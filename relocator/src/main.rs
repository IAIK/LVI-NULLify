#![feature(split_inclusive)]
#![feature(option_result_contains)]
#![feature(iter_partition_in_place)]

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::fs::File;
use std::io::prelude::*;
use clap::{Arg, App, AppSettings, SubCommand};
use std::ffi::CStr;
use itertools::Itertools;
use std::path::Path;
use regex::Regex;

// definitions of classical ELF structs
#[repr(C)]
#[derive(Debug)]
struct Elf64_Ehdr {
    e_ident: [u8; 16],
    e_type : u16,
    e_machine : u16,
    e_version : u32,
    e_entry : u64,
    e_phoff : u64,
    e_shoff : u64,
    e_flags : u32,
    e_ehsize : u16,
    e_phentsize : u16,
    e_phnum : u16,
    e_shentsize : u16,
    e_shnum : u16,
    e_shstrndx : u16
}

#[repr(C)]
#[derive(Debug)]
struct Elf64_Phdr {
    p_type : u32,
    p_flags : u32,
    p_offset : u64,
    p_vaddr : u64,
    p_paddr : u64,
    p_filesz : u64,
    p_memsz : u64,
    p_align : u64
} 

#[repr(C)]
#[derive(Debug,Clone)]
struct Elf64_Shdr {
    sh_name : u32,
    sh_type : u32,
    sh_flags : u64,
    sh_addr : u64,
    sh_offset : u64,
    sh_size : u64,
    sh_link : u32,
    sh_info : u32,
    sh_addralign : u64,
    sh_entsize : u64
}

/*
    A: Addend of Elfxx_Rela entries.
    B: Image base where the shared object was loaded in process virtual address space.
    G: Offset to the GOT relative to the address of the correspondent relocation
    entry’s symbol.
    GOT: Address of the Global Offset Table
    L: Section offset or address of the procedure linkage table (PLT, .got.plt).
    P: The section offset or address of the storage unit being relocated.
    retrieved via r_offset relocation entry’s field.
    S: Relocation entry’s correspondent symbol value.
    Z: Size of Relocations entry’s symbol.
*/
#[derive(Debug, PartialEq, FromPrimitive)]
#[allow(non_camel_case_types,dead_code)]
enum RelaType {
    R_X86_64_NONE = 0,      //None
    R_X86_64_64 = 1,        //S+A
    R_X86_64_PC32 = 2,      //S+A–P
    R_X86_64_GOT32 = 3,     //G+A
    R_X86_64_PLT32 = 4,     //L+A–P
    R_X86_64_COPY = 5,      //copy
    R_X86_64_GLOB_DAT = 6,  //S
    R_X86_64_JUMP_SLOT = 7, //S
    R_X86_64_RELATIVE = 8,  //B+A
    R_X86_64_GOTPCREL = 9,  //G+GOT+A–P
    R_X86_64_32 = 10,       //S+A
    R_X86_64_32S = 11,      //S+A
    R_X86_64_16 = 12,       //S+A
    R_X86_64_PC16 = 13,     //S+A–P
    R_X86_64_8 = 14,        //S+A
    R_X86_64_PC8 = 15,      //S+A–P
    R_X86_64_PC64 = 24,     //S+A–P
    R_X86_64_GOTOFF64 = 25, //S+A–GOT
    R_X86_64_GOTPC32 = 26,  //GOT+A–P
    R_X86_64_SIZE32 = 32,   //Z+A
    R_X86_64_SIZE64 = 33,   //Z+A
}

impl RelaType {
    fn explain(&self) -> &'static str{
        use RelaType::*;
        match self {
            R_X86_64_NONE => "",
            R_X86_64_64 | R_X86_64_32 | R_X86_64_32S | R_X86_64_16 | R_X86_64_8 => "S+A",
            R_X86_64_PC64 | R_X86_64_PC32 | R_X86_64_PC16 | R_X86_64_PC8 => "S+A–P",
            R_X86_64_GOT32 => "G+A",
            R_X86_64_PLT32 => "L+A–P",
            R_X86_64_COPY => "copy",
            R_X86_64_GLOB_DAT | R_X86_64_JUMP_SLOT => "S",
            R_X86_64_RELATIVE => "B+A",
            R_X86_64_GOTPCREL => "G+GOT+A–P",
            R_X86_64_GOTOFF64 => "S+A–GOT",
            R_X86_64_GOTPC32 => "GOT+A–P",
            R_X86_64_SIZE32 | R_X86_64_SIZE64 => "Z+A"
        }
    }
}

#[repr(C)]
#[derive(Debug,Clone)]
struct Elf64_Rela {
    r_offset : u64,
    r_info_type : u32,
    r_info_symbol : u32,
    r_addend : i64
}

#[repr(C)]
#[derive(Debug)]
struct Elf64_Rel {
    r_offset : u64,
    r_info : u64
}

#[repr(C)]
#[derive(Debug)]
struct Elf64_Sym{
    st_name : u32,
    st_info : u8,
    st_other : u8,
    st_shndx : u16,
    st_value : u64,
    st_size : u64
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
enum Elf64SymType {
    STT_NOTYPE = 0,
    STT_OBJECT = 1,
    STT_FUNC = 2,
    STT_SECTION = 3,
    STT_FILE = 4,
    STT_COMMON = 5,
    STT_LOOS = 10,
    STT_HIOS = 12,
    STT_LOPROC = 13,
    STT_HIPROC = 15 
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
enum Elf64ShdrFlags {
    SHF_WRITE = 0x1,
    SHF_ALLOC = 0x2,
    SHF_EXECINSTR = 0x4,
    SHF_MERGE = 0x10,
    SHF_STRINGS = 0x20,
    SHF_INFO_LINK = 0x40,
    SHF_LINK_ORDER = 0x80,
    SHF_OS_NONCONFORMING = 0x100,
    SHF_GROUP = 0x200,
    SHF_MASKOS = 0x0ff00000,
    SHF_ORDERED = 0x40000000,
    SHF_EXCLUDE = 0x80000000,
    SHF_MASKPROC = 0xf0000000 
}

impl Elf64_Sym {
    fn is_func(&self) -> bool{
        self.get_type() == (Elf64SymType::STT_FUNC as u8)
    }

    fn is_object(&self) -> bool {
        self.get_type() == (Elf64SymType::STT_OBJECT as u8)
    }

    fn is_no_type(&self) -> bool {
        self.get_type() == (Elf64SymType::STT_NOTYPE as u8)
    }

    fn is_section(&self) -> bool {
        self.get_type() == (Elf64SymType::STT_SECTION as u8)
    }

    fn get_type(&self) -> u8 {
        self.st_info & 0xF
    }

    fn get_binding(&self) -> u8 {
        self.st_info >> 4
    }

    fn is_text(&self, shdrs_copy : &Vec<Elf64_Shdr>) -> bool {
        if !self.is_section() { return false; }

        let is_exec = 
            shdrs_copy[self.st_shndx as usize].sh_flags &
            (Elf64ShdrFlags::SHF_EXECINSTR as u64);

        is_exec != 0
    }

    fn is_undefined<'a>(
        &self, 
        symbol_table : &'a ElfSymTab<'a>,
        string_tables : &Vec<ElfStringTab<'a>>) -> bool {

        let undefined_vars_list = vec!["__ImageBase"];
        
        let name = string_tables.iter()
            .filter_map(|x| x.get_symbol_name(&self))
            .find(|x| undefined_vars_list.contains(x) );
        
        if let Some(x) = name {
            println!("found undefined defined symbol {}", x);
            return false;
        }
        self.is_no_type()
    }
}

#[derive(Debug)]
struct ElfSymTab<'a> {
    header : &'a mut Elf64_Shdr,
    symbols : &'a mut [Elf64_Sym]
}

#[derive(Debug)]
struct ElfStringTab<'a> {
    header : &'a mut Elf64_Shdr,
    raw : &'a mut [u8],
    shdr_index : u16
}

#[derive(Debug)]
struct ElfRelocations<'a> {
    header : &'a mut Elf64_Shdr,
    relocations : &'a mut [Elf64_Rela]
}

impl Elf64_Rela {
    fn get_symbol<'a>(&self, symbol_tab : &'a ElfSymTab<'a>) -> Option<&'a Elf64_Sym> {
        symbol_tab.get_symbol(self.r_info_symbol)
    }

    fn get_symbol_name<'a>(
        &self,
        symbol_tab : &'a ElfSymTab<'a>, 
        symbol_names : &'a ElfStringTab<'a>
    ) -> Option<&'a str> {
        if self.r_info_symbol == 0 { return None; }
        let symbol = self.get_symbol(&symbol_tab)?;
        symbol_names.get_symbol_name(symbol)
    }

    fn get_symbol_from_addend<'a>(
        &self,
        symbol_tab : &'a ElfSymTab<'a>
    ) -> Option<&'a Elf64_Sym> {
        if self.r_info_type != RelaType::R_X86_64_RELATIVE as u32 {
            return None;
        }
        symbol_tab.symbols.iter().find(|sym| sym.st_value == self.r_addend as u64)
    }
}

impl<'a> ElfSymTab<'a> {
    fn get_symbol(&'a self, index : u32) -> Option<&'a Elf64_Sym> {
        self.symbols.get(index as usize)
    }
    fn print(&'a self, symbol_names : &'a Vec<ElfStringTab<'a>>) {
        for (i, sym) in self.symbols.iter().enumerate() {

            let names : Vec<String> = symbol_names.iter()
                .filter_map(|st| st.get_symbol_name(sym))
                .map(|x| format!("{:>10}", x))
                .collect();

            println!("{:2} {:?} {} ", i, sym, names.iter().format(", ")); 
        }
    }
}

impl<'a> ElfStringTab<'a> {
    fn get_string(&'a self, offset : u32) -> Option<&'a str> {
        // check if offset is out of bounds
        self.raw.get(offset as usize..)
            // check if section is null terminated
            .and_then(|slice| slice.last().filter(|&&x| x == 0).and(Some(slice)) )
            // convert raw poiter to string
            .and_then(|slice| unsafe{ CStr::from_ptr(slice.as_ptr().cast()) }.to_str().ok() )
    }

    fn get_strings(&'a self) -> Vec<&'a str> {
        // split section by nullbytes
        self.raw.split_inclusive(|&x| x == 0)
            // convert to strings
            .map(|x| CStr::from_bytes_with_nul(&x).ok().and_then(|cstr| cstr.to_str().ok()) )
            // remove error strings
            .flatten().collect()
    }

    fn get_symbol_name(&'a self, symbol : &Elf64_Sym) -> Option<&'a str> {
        if symbol.st_name == 0 {
            return None;
        }
        self.get_string(symbol.st_name)
    }

    fn get_section_name(&'a self, header : &Elf64_Shdr) -> Option<&'a str> {
        if self.contains_section_headers() {
            self.get_string(header.sh_name)
        } else {
            None
        }
    }

    fn contains_section_headers(&'a self) -> bool {
        let name = self.get_string(self.header.sh_name);
        name.contains(&".strtab") || name.contains(&".shstrtab")
    }

    fn print(&'a self) {
        println!("{:?} {} \t{}", self.header, self.shdr_index, self.get_strings().iter().format("\n\t"));
    }
}


impl<'a> ElfRelocations<'a> {
    fn change_relocations(
        &'a mut self, 
        symbol_table : &'a ElfSymTab<'a>,
        shdrs_copy : &Vec<Elf64_Shdr>,
        string_tables : &Vec<ElfStringTab<'a>>
    ) {
        let to_replace = [
            RelaType::R_X86_64_64 as u32,
            RelaType::R_X86_64_32 as u32,
            RelaType::R_X86_64_32S as u32, 
            RelaType::R_X86_64_16 as u32,
            RelaType::R_X86_64_8 as u32,
        ];

        for rela in self.relocations.iter_mut() {
            if !to_replace.contains(&rela.r_info_type) {
                continue;
            }

            let sym = rela.get_symbol(&symbol_table);
            if sym.is_none() {
                continue;
            }
            let sym = sym.unwrap();

            if sym.is_text(&shdrs_copy) || sym.is_func() || sym.is_undefined(&symbol_table, &string_tables) {
                continue;
            }

            /*if sym.is_no_type() {
                continue;
            }*/
            
            let name : Vec<String> = string_tables.iter()
                .filter_map(|x| x.get_symbol_name(&sym)).map(|x| x.to_string()).collect();

            rela.r_info_type = RelaType::R_X86_64_COPY as u32;
            println!("addend: {}", rela.r_addend);
            
            if !name.is_empty() {
                println!("replacing {:?}", name)
            }
            
        }
        for rela in self.relocations.iter_mut() {
            if RelaType::R_X86_64_RELATIVE as u32 != rela.r_info_type {
                continue;
            }

            let sym = rela.get_symbol_from_addend(&symbol_table);
            if sym.is_none() {
                continue;
            }
            let sym = sym.unwrap();

            if sym.is_text(&shdrs_copy) || sym.is_func() || sym.is_undefined(&symbol_table, &string_tables) {
                continue;
            }

            /*if sym.is_no_type() {
                continue;
            }*/
            
            let name : Vec<String>= string_tables.iter()
                .filter_map(|x| x.get_symbol_name(&sym)).map(|x| x.to_string()).collect();
            
            rela.r_info_type = RelaType::R_X86_64_COPY as u32;
            if !name.is_empty() {
                println!("replacing {:?}", name)
            }
        }
    }

    fn print(
        &'a self, 
        symbol_tab : &'a ElfSymTab<'a>, 
        symbol_names : &'a Vec<ElfStringTab<'a>>
    ) {
        let section_name = symbol_names.iter()
            .find(|st| st.contains_section_headers())
            .and_then(|st| st.get_section_name(self.header))
            .unwrap_or("UNKOWN!");

        println!("{}", section_name);
        for rela in self.relocations.iter() {

            let rela_description = RelaType::from_u32(rela.r_info_type)
                .map(|x| format!("{:<20?} {:<10}", x, x.explain()) )
                .unwrap_or(format!("{:<20} UNKNOWN!!!", rela.r_info_type));

            let names : Vec<String> = if let Some(sym) = rela.get_symbol(&symbol_tab) {
                symbol_names.iter()
                    .filter_map(|st| st.get_symbol_name(sym))
                    .map(|x| format!("{} {:>10}", sym.st_info & 0xF, x))
                    .collect()

            } else {
                Vec::new()
            };

            let sym = rela.get_symbol(&symbol_tab).or(rela.get_symbol_from_addend(&symbol_tab));

            println!("\t: {} [{:x}] {:x<10?} r_info_type: {} r_offset: {:x} r_addend: {:x}",
                sym.map(Elf64_Sym::get_type)
                    .unwrap_or(0xffu8),
                sym.map(|x| x.st_info).unwrap_or(0xff),
                names.iter().format(", "),
                rela_description,
                rela.r_offset,
                rela.r_addend
            );
        }
    }
}

#[derive(Debug)]
struct ElfFile<'a> {
    shdrs_copy : Vec<Elf64_Shdr>, 
    elf_header : &'a mut Elf64_Ehdr,
    symbol_table : ElfSymTab<'a>,
    string_tables : Vec<ElfStringTab<'a>>,
    relocation_sections : Vec<ElfRelocations<'a>>,
}

impl<'a> ElfFile<'a> {
    fn change_relocations(&'a mut self) {
        for rs in self.relocation_sections.iter_mut() {
            rs.change_relocations(&self.symbol_table, &self.shdrs_copy, &self.string_tables);
        }
    }

    fn print(&self) {
        println!("string tables names:");
        for st in self.string_tables.iter() {
            st.print();
        }

        println!("symbols:");
        self.symbol_table.print(&self.string_tables);

        println!("relocation sections:");
        for rs in self.relocation_sections.iter() {
            rs.print(&self.symbol_table, &self.string_tables);
            println!("");
        }
    }
}

fn get_mut_slice_from_buffer<'a, T>(slice : & mut [u8], offset : u64, count : u64) -> Option<&'a mut [T]> {
    let size = count as usize * std::mem::size_of::<T>();
    let offset = offset as usize;

    slice.get_mut(offset..offset+size).map(|s| unsafe { 
        std::slice::from_raw_parts_mut(
            s.as_mut_ptr().cast(),
            count as usize
        )
    })
}

enum ElfSection<'a> {
    StringTable(ElfStringTab<'a>),
    RelocationSection(ElfRelocations<'a>),
    SymbolTable(ElfSymTab<'a>)
}


fn convert_raw_to_elf<'a>(buf : &'a mut Vec<u8>) -> Option<ElfFile<'a>> {
    let slice = buf.as_mut_slice();

    let ehdr : &mut Elf64_Ehdr = &mut get_mut_slice_from_buffer(slice, 0, 1)?[0];

    let shdrs : &mut [Elf64_Shdr] = get_mut_slice_from_buffer(slice, ehdr.e_shoff, ehdr.e_shnum as u64)?;
    
    assert_eq!(ehdr.e_shentsize as usize, std::mem::size_of::<Elf64_Shdr>());

    const SHT_SYMTAB : u32 = 2;
    const SHT_STRTAB : u32 = 3;
    const SHT_RELA : u32 = 4; 

    let mut shdrs_copy : Vec<Elf64_Shdr> = Vec::new();

    let mut sections : Vec<ElfSection<'a>> = shdrs.iter_mut().enumerate().filter_map(|(i, shdr)| {
        let names = ["SHT_NULL ","SHT_PROGBITS ","SHT_SYMTAB ","SHT_STRTAB ","SHT_RELA ","SHT_HASH ","SHT_DYNAMIC ","SHT_NOTE ","SHT_NOBITS ","SHT_REL ","SHT_SHLIB ","SHT_DYNSYM "];

        shdrs_copy.push(shdr.clone());

        match shdr.sh_type {
            SHT_SYMTAB => {
                let x = get_mut_slice_from_buffer(slice, shdr.sh_offset, shdr.sh_size / std::mem::size_of::<Elf64_Sym>() as u64)?;
                Some(ElfSection::SymbolTable(ElfSymTab{
                    header : shdr,
                    symbols : x
                }))
            },
            SHT_STRTAB => {
                let x = get_mut_slice_from_buffer(slice, shdr.sh_offset, shdr.sh_size)?;
                Some(ElfSection::StringTable(ElfStringTab{
                    header : shdr,
                    raw : x,
                    shdr_index : i as u16 + 1 as u16
                }))
            },
            SHT_RELA => {
                let x = get_mut_slice_from_buffer(slice, shdr.sh_offset, shdr.sh_size / std::mem::size_of::<Elf64_Rela>() as u64)?;
                Some(ElfSection::RelocationSection(ElfRelocations{
                    header : shdr,
                    relocations : x
                }))
            },
            _ => {
                None
            },
        }
    }).collect();


    let mut string_tables : Vec<ElfStringTab<'a>> = vec![];
    let mut relocation_sections : Vec<ElfRelocations<'a>> = vec![];
    let mut symbol_tables : Option<ElfSymTab<'a>> = None;

    for sec in sections.drain(..) {
        match sec {
            ElfSection::SymbolTable(x) => symbol_tables = Some(x),
            ElfSection::StringTable(x) => string_tables.push(x),
            ElfSection::RelocationSection(x) => relocation_sections.push(x),
        }
    }
   
    Some(ElfFile{
        shdrs_copy : shdrs_copy,
        elf_header : ehdr,
        symbol_table : symbol_tables?,
        string_tables : string_tables,
        relocation_sections :  relocation_sections
    })
}

#[derive(Debug)]
enum RunErrors {
    CannotOpenInputFile,
    CannotReadInputFile,
    NoValidElfFormat,
    CannotCreateOuputFile,
    CannotWriteOutputFile
}

fn run(input_file_name : &str, output_file_name : &str, verbose : bool) -> Result<(), RunErrors>{
    let input_path = Path::new(input_file_name);
    let output_path = Path::new(output_file_name);

    File::open(input_path).or(Err(RunErrors::CannotOpenInputFile))
        // read raw
        .and_then(|mut input| {
            let mut buf = Vec::new();
            input.read_to_end(&mut buf).or(Err(RunErrors::CannotReadInputFile))?;
            Ok(buf)
        })
        // convert elf and modify
        .and_then(|mut buf| {
            let mut elf_file = convert_raw_to_elf(&mut buf).ok_or(RunErrors::NoValidElfFormat)?;
            //elf_file.print();
            elf_file.change_relocations();
                
            Ok(buf)
        })
        // store
        .and_then(|buf| {
            File::create(output_path).or(Err(RunErrors::CannotCreateOuputFile))
                .and_then(|mut output| output.write_all(&buf).or(Err(RunErrors::CannotWriteOutputFile)) )
        })
}

fn main() {
    let matches = App::new("relocator")
        .setting(AppSettings::AllowExternalSubcommands)
        .version("0.1.0")
        .author("")
        .about("")
        .arg(Arg::with_name("input")
            .short("i")
            .long("input")
            .takes_value(true)
            .help("intput elf file name")
        )
        .arg(Arg::with_name("output")
            .short("o")
            .long("output")
            .takes_value(true)
            .help("output elf file name")
        )
        .arg(Arg::with_name("verbose")
            .short("v")
            .long("verbose")
            .help("print details")
        )
        .get_matches();

    match matches.subcommand() {
        (sub_cmd, Some(y) ) => {
            let args = &y.args[""].vals;
            let output = args.iter().position(|x| x == "-o");
            if let Some(index) = output {
                let output_name = args.get(index+1)
                    .and_then(|x| x.to_str());

                if let Some(x) = output_name {
                    println!("using raw input {}", x);
                    match run(x, x, false) {
                        Ok(()) => println!("SUCCESSFULL!"),
                        Err(x) => println!("FAILED! {:?}", x),
                    };
                }
            }
        },

        (_, _) => {
            
        },
    };
}
