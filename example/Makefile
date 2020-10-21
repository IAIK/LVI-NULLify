TR_CC = ../compiler/clang-gs
TR_CXX = ../compiler/clang-gs++

RELOCATOR=../relocator/target/release/relocator

SGX_SDK ?= /opt/intel/sgxsdk
SGX_MODE ?= HW
SGX_ARCH ?= x64
SGX_DEBUG ?= 1

include $(SGX_SDK)/buildenv.mk

ifeq ($(shell getconf LONG_BIT), 32)
	SGX_ARCH := x86
else ifeq ($(findstring -m32, $(CXXFLAGS)), -m32)
	SGX_ARCH := x86
endif

ifeq ($(SGX_ARCH), x86)
	SGX_COMMON_FLAGS := -m32
	SGX_LIBRARY_PATH := $(SGX_SDK)/lib
	SGX_ENCLAVE_SIGNER := $(SGX_SDK)/bin/x86/sgx_sign
	SGX_EDGER8R := $(SGX_SDK)/bin/x86/sgx_edger8r
else
	SGX_COMMON_FLAGS := -m64
	SGX_LIBRARY_PATH := $(SGX_SDK)/lib64
	SGX_ENCLAVE_SIGNER := $(SGX_SDK)/bin/x64/sgx_sign
	SGX_EDGER8R := $(SGX_SDK)/bin/x64/sgx_edger8r
endif

ifeq ($(SGX_DEBUG), 1)
ifeq ($(SGX_PRERELEASE), 1)
$(error Cannot set SGX_DEBUG and SGX_PRERELEASE at the same time!!)
endif
endif

ifeq ($(SGX_DEBUG), 1)
        SGX_COMMON_FLAGS += -O0
else
        SGX_COMMON_FLAGS += -O0 
endif

#

SGX_COMMON_FLAGS += -Wall -Wextra -Winit-self -Wpointer-arith -Wreturn-type \
                    -Waddress -Wsequence-point -Wformat-security \
                    -Wmissing-include-dirs -Wfloat-equal -Wundef -Wshadow \
                    -Wcast-align -Wcast-qual -Wconversion -Wredundant-decls
SGX_COMMON_CFLAGS := $(SGX_COMMON_FLAGS) -Wstrict-prototypes -Wunsuffixed-float-constants #-Wjump-misses-init 
SGX_COMMON_CXXFLAGS := $(SGX_COMMON_FLAGS) -Wnon-virtual-dtor -std=c++11

######## App Settings ########

ifneq ($(SGX_MODE), HW)
	Urts_Library_Name := sgx_urts_sim
else
	Urts_Library_Name := sgx_urts
endif

App_Cpp_Files := app/app.cpp
App_Include_Paths := -Iapp -I$(SGX_SDK)/include

App_C_Flags := -fPIC -Wno-attributes $(App_Include_Paths) -g

# Three configuration modes - Debug, prerelease, release
#   Debug - Macro DEBUG enabled.
#   Prerelease - Macro NDEBUG and EDEBUG enabled.
#   Release - Macro NDEBUG enabled.
ifeq ($(SGX_DEBUG), 1)
        App_C_Flags += -DDEBUG -UNDEBUG -UEDEBUG
else ifeq ($(SGX_PRERELEASE), 1)
        App_C_Flags += -DNDEBUG -DEDEBUG -UDEBUG
else
        App_C_Flags += -DNDEBUG -UEDEBUG -UDEBUG
endif

App_Cpp_Flags := $(App_C_Flags)
App_Link_Flags := -L$(SGX_LIBRARY_PATH) -l$(Urts_Library_Name) -lpthread 

App_Cpp_Objects := $(App_Cpp_Files:.cpp=.o)

App_Name := main

######## Enclave Settings ########

ifneq ($(SGX_MODE), HW)
	Trts_Library_Name := sgx_trts_sim
	Service_Library_Name := sgx_tservice_sim
else
	Trts_Library_Name := sgx_trts
	Service_Library_Name := sgx_tservice
endif
Crypto_Library_Name := sgx_tcrypto

Enclave_Cpp_Files := enclave/enclave.cpp
Enclave_Include_Paths := -Ienclave -I$(SGX_SDK)/include -I$(SGX_SDK)/include/tlibc -I$(SGX_SDK)/include/libcxx

Enclave_C_Flags := $(Enclave_Include_Paths) -nostdinc -fvisibility=hidden  -fno-pic -fno-pie  -ffunction-sections -fdata-sections $(MITIGATION_CFLAGS) -mcmodel=large 
CC_BELOW_4_9 := $(shell expr "`$(CC) -dumpversion`" \< "4.9")
ifeq ($(CC_BELOW_4_9), 1)
	Enclave_C_Flags += -fstack-protector
else
	Enclave_C_Flags += -fstack-protector-strong
endif

Enclave_Cpp_Flags := $(Enclave_C_Flags) -nostdinc++

# Enable the security flags
Enclave_Security_Link_Flags := -Wl,-z,relro,-z,now,-z,noexecstack

Enclave_Cpp_Objects := $(sort $(Enclave_Cpp_Files:.cpp=.o))

Enclave_Name := enclave.so
Signed_Enclave_Name := enclave.signed.so
Enclave_Config_File := enclave/enclave.config.xml

ifeq ($(SGX_MODE), HW)
ifeq ($(SGX_DEBUG), 1)
	Build_Mode = HW_DEBUG
else ifeq ($(SGX_PRERELEASE), 1)
	Build_Mode = HW_PRERELEASE
else
	Build_Mode = HW_RELEASE
endif
else
ifeq ($(SGX_DEBUG), 1)
	Build_Mode = SIM_DEBUG
else ifeq ($(SGX_PRERELEASE), 1)
	Build_Mode = SIM_PRERELEASE
else
	Build_Mode = SIM_RELEASE
endif
endif

.SILENT:

.PHONY: all target run
all: .config_$(Build_Mode)_$(SGX_ARCH)
	@$(MAKE) target


debug: all
	$(SGX_SDK)/bin/sgx-gdb ./main

ifeq ($(Build_Mode), HW_RELEASE)
target:  $(App_Name) $(Enclave_Name)
	@echo "The project has been built in release hardware mode."
	@echo "Please sign the $(Enclave_Name) first with your signing key before you run the $(App_Name) to launch and access the enclave."
	@echo "To sign the enclave use the command:"
	@echo "   $(SGX_ENCLAVE_SIGNER) sign -key <your key> -enclave $(Enclave_Name) -out <$(Signed_Enclave_Name)> -config $(Enclave_Config_File)"
	@echo "You can also sign the enclave using an external signing tool."
	@echo "To build the project in simulation mode set SGX_MODE=SIM. To build the project in prerelease mode set SGX_PRERELEASE=1 and SGX_MODE=HW."

else
target: $(App_Name) $(Signed_Enclave_Name)
ifeq ($(Build_Mode), HW_DEBUG)
	@echo "The project has been built in debug hardware mode."
else ifeq ($(Build_Mode), SIM_DEBUG)
	@echo "The project has been built in debug simulation mode."
else ifeq ($(Build_Mode), HW_PRERELEASE)
	@echo "The project has been built in pre-release hardware mode."
else ifeq ($(Build_Mode), SIM_PRERELEASE)
	@echo "The project has been built in pre-release simulation mode."
else
	@echo "The project has been built in release simulation mode."
endif
endif
	

run: all
ifneq ($(Build_Mode), HW_RELEASE)
	@$(CURDIR)/$(App_Name)
	@echo "RUN  =>  $(App_Name) [$(SGX_MODE)|$(SGX_ARCH), OK]"
endif

.config_$(Build_Mode)_$(SGX_ARCH):
	@rm -f .config_* app/$(App_Name) $(Enclave_Name) $(Signed_Enclave_Name) $(App_Cpp_Objects) app/enclave_u.* $(Enclave_Cpp_Objects) enclave/enclave_t.*
	@touch .config_$(Build_Mode)_$(SGX_ARCH)

######## App Objects ########

app/enclave_u.h: $(SGX_EDGER8R) enclave/enclave.edl
	@cd app && $(SGX_EDGER8R) --untrusted ../enclave/enclave.edl --search-path ../enclave --search-path $(SGX_SDK)/include
	@echo "GEN  =>  $@"

app/enclave_u.c: app/enclave_u.h

app/enclave_u.o: app/enclave_u.c
	@$(CC) $(SGX_COMMON_CFLAGS) $(App_C_Flags) -c $< -o $@
	@echo "CC   <=  $<"

app/%.o: app/%.cpp  app/enclave_u.h
	@$(CXX) $(SGX_COMMON_CXXFLAGS) $(App_Cpp_Flags) -c $< -o $@
	@echo "CXX  <=  $<"

$(App_Name): app/enclave_u.o $(App_Cpp_Objects)
	@$(CXX) $^ -o $@ $(App_Link_Flags)
	@echo "LINK =>  $@"

######## Enclave Objects ########

enclave/enclave_t.h: $(SGX_EDGER8R) enclave/enclave.edl
	@cd enclave && $(SGX_EDGER8R) --trusted ../enclave/enclave.edl --search-path ../enclave --search-path $(SGX_SDK)/include
	@echo "GEN  =>  $@"

enclave/enclave_t.c: enclave/enclave_t.h

enclave/enclave_t.o: enclave/enclave_t.c
	@$(TR_CC) $(SGX_COMMON_CFLAGS) $(Enclave_C_Flags) -c $< -o $@
	@echo "CC   <=  $<"

enclave/%.o: enclave/%.cpp enclave/enclave_t.h
	echo $(SGX_COMMON_CXXFLAGS) $(Enclave_Cpp_Flags)
	@$(TR_CXX) $(SGX_COMMON_CXXFLAGS) $(Enclave_Cpp_Flags) -c $< -o $@
	@echo "CXX  <=  $<"

monolith.o: enclave/enclave_t.o $(Enclave_Cpp_Objects) 
	-rm -r tmp_obj
	mkdir tmp_obj
	cd tmp_obj; \
	ar x $(SGX_TRUSTED_LIBRARY_PATH)/libsgx_trts.a; \
	ar x $(SGX_TRUSTED_LIBRARY_PATH)/libsgx_tstdc.a; \
	ar x $(SGX_TRUSTED_LIBRARY_PATH)/libsgx_tservice.a; \
	ar x $(SGX_TRUSTED_LIBRARY_PATH)/liblsgx_tcxx.a; \
	ld -r *.o -o monolith_libs.o

	ld -r tmp_obj/monolith_libs.o enclave/enclave_t.o $(Enclave_Cpp_Objects) -o monolith.o

	$(RELOCATOR) -i monolith.o -o monolith.o
	
$(Enclave_Name): monolith.o

	@$(TR_CXX) monolith.o -o $@ $(MITIGATION_LDFLAGS) $(Enclave_Security_Link_Flags) \
	-nostdlib -nodefaultlibs -nostartfiles -L$(SGX_TRUSTED_LIBRARY_PATH) \
	-Wl,--start-group -Wl,--end-group \
	-Wl,-Bstatic -Wl,-Bsymbolic -Wl,--no-undefined \
	-Wl,-pie,-eenclave_entry -Wl,--export-dynamic  \
	-Wl,--defsym,__ImageBase=0 -Wl,--gc-sections \
	-Wl,--version-script=enclave/enclave.lds

$(Signed_Enclave_Name): $(Enclave_Name)
	@openssl genrsa -out enclave/private.pem -3 3072
	@openssl rsa -in enclave/private.pem -pubout -out enclave/public.pem
	@$(SGX_ENCLAVE_SIGNER) sign -ignore-rel-error -key enclave/private.pem -enclave $(Enclave_Name) -out $@ -config  $(Enclave_Config_File)
	@echo "SIGN =>  $@"

.PHONY: clean

clean:
	@rm -f .config_* $(App_Name) $(Enclave_Name) $(Signed_Enclave_Name) $(App_Cpp_Objects) app/enclave_u.* $(Enclave_Cpp_Objects) enclave/enclave_t.* monolith.o enclave/private.pem enclave/public.pem
	@rm -rf tmp_obj
