# General

## Setup
POCs require the PTEditor module to be loaded, available at https://github.com/misc0110/PTEditor/  
You can follow the instructions in the readme there to install from PPA or source.
Loading unsigned kernel modules may not work with secure boot enabled, thus you will also need to disable it, or self-sign the module.

If you installed you SDK and LVINullify SDK anywhere other than the recommended paths, you will have to adjust the corresponding lines in the Makefiles of the POCs.

If you want to see anything (interesting) at all, you need a CPU that supports SGX and is vulnerable to LVI Null (https://software.intel.com/content/www/us/en/develop/topics/software-security-guidance/processors-affected-consolidated-product-cpu-model.html).

For best results, run the POCs on a quiet system or isolated cores (start pocs with taskset on those cores) and fix the CPU frequency (e.g. cpupower -c all frequency-set). 

## Building

You can build all POCs with  
`make clean && GS=0 make` without LVINullify or  
`make clean && GS=1 make` with LVINullify


### the virtual zero page

The indirect POCs need the virtual zero page, which we can get by starting the program with `setarch x86_64 -Z ./app`.
On ubuntu, setarch needs to either be run with sudo each time, or you can run `sudo sysctl -w vm.mmap_min_addr=0` once per session.

### config

All POCs have a `conf.h` file that defines a LEAKAGE_CHAR that can be changed.

All POCs calibrate a cache-miss threshold in the first line of main in `App/App.cpp`. If the automated calibration is not working for you, you can manually define a threshold there.

# Switch Enclave

This POC demonstrates a direct injection into a switch condition.
It also doubles as an LVI test, as it tries to inject '6' as well as '0'.

Run with `./app`

sample output without LVINullify:  
```
transient 0: 14288 (95.25%) (should be fairly high)
transient lvi t: 42 (0.28%) (should show *something* on LVI vulnerable machines)
architectural: 14142 (94.28%)  (should be fairly high, comparable to transient 0)
other: 1 (0.01%) (should be very low, otherwise the cache treshold is probably wrong)
```

sample output with LVINullify:  
```
transient 0: 0 (0.00%) (this should now be 0)
transient lvi t: 0 (0.00%) (this could still show something)
architectural: 13817 (92.11%) (should be similar to before)
other: 0 (0.00%) (same as before)
```

# Direct Load Enclave

This is an example of an LVI-Null attack that is not mitigated by our approach, and is left to algorithm developers.  
With or without LVINullify, LEAKAGE_CHAR should show up on the output.  
It can be mitigated by adding a fence in line 102.

Run with `./app`

# Indirect Load Enclave

This POC shows injection of arbitrary values into indirect loads, i.e. loads where the pointer to the target is also loaded from memory.  
With LVINullify, we see a redirection of transient loads from the Nullpage to the beginning of the enclave.

Run with `setarch x86_64 -Z ./app`

sample output without LVINullify:  
```
transient t: 4030 (26.87%) (varies, but should be clearly visible)
architectural: 13675 (91.17%) (should be high)
enclave start: 0 (0.00%) (should be 0)
other: 0 (0.00%) (should be very low, otherwise the cache treshold is probably wrong)
```

sample output with LVINullify:  
```
transient t: 0 (0.00%) (should be 0)
architectural: 14231 (94.87%) (should be high)
enclave start: 3916 (26.11%) (should be similar to transient t before)
other: 0 (0.00%) (same as before)
```

# Null Redirect Enclave

This example shows the common C++ pattern of calling a virtual function.
Without LVINullify, the function address lookup is redirected to the Nullpage, which can redirect control flow to anywhere in the enclave binary.
With LVINullify, we see no more leakage, as the start of the enclave is not a valid pointer.

sample output without LVINullify:  
```
transient t: 3219 (21.46%) (varies, but should be clearly visible
other: 0 (0.00%) (should be very low, otherwise the cache treshold is probably wrong)
```


sample output with LVINullify:  
```
transient t: 0 (0.00%) (should be 0)
other: 0 (0.00%) (same as before)
```


# Notes

As these are only proofs of concept and not end-to-end exploits, we take several shortcuts.  
Most notably, our enclaves cooperate with the attacker instead of relying on a framework like SGX-Step. But importantly: all mechanisms we use can be replicated in a real attack.  
Another example is our use of clflush within the enclave. Because our mitigation does not support assembly, we have to manually make clflush gs-relative.

