#include "enclave_t.h"
#include <stdarg.h>
#include <stdio.h>
#include <string.h>

#include <vector>
#include <string>
#include <iterator>
#include <algorithm>

int printf(const char* fmt, ...)
{
    char buf[BUFSIZ] = { '\0' };
    va_list ap;
    va_start(ap, fmt);
    vsnprintf(buf, BUFSIZ, fmt, ap);
    va_end(ap);
    ocall_print_string(buf);
    return (int)strnlen(buf, BUFSIZ - 1) + 1;
}

void test() {}

void ecall_naked_function(void) {
    test();
    ocall_print_string("inside: ecall_naked_function\n");
}

void ecall_test_enclave(void) {
    int in = 10;
    int out = 0;
    sgx_status_t status;

    status = ocall_ptr_pass(&in, &out);
    if (status != SGX_SUCCESS) {
        ocall_print_string("ERROR: ocall_ptr_pass STATUS: ");
        ocall_print_status(status);
        ocall_print_string("\n");
    } else if (out != 12) {
        ocall_print_string("ERROR: ocall_ptr_pass\n");
    } else {
        ocall_print_string("PASS: ocall_ptr_pass\n");
    }

    in = 10;
    status = ocall_ptr_inout(&in);
    if (status != SGX_SUCCESS) {
        ocall_print_string("ERROR: ocall_ptr_inout STATUS: ");
        ocall_print_status(status);
        ocall_print_string("\n");
    } else if (in != 12) {
        ocall_print_string("ERROR: ocall_ptr_inout\n");
    } else {
        ocall_print_string("PASS: ocall_ptr_inout\n");
    }

    in = 10;
    status = ocall_ptr_user(&in);
    if (status != SGX_SUCCESS) {
        ocall_print_string("ERROR: ocall_ptr_user STATUS: ");
        ocall_print_status(status);
        ocall_print_string("\n");
    } else {
        ocall_print_string("PASS: ocall_ptr_user\n");
    }
}

void ecall_ptr_pass(int *in, int *out) {
    *out = *in + 2;
}

void ecall_ptr_inout(int *val) {
    *val += 2;
}

void ecall_ptr_user(int *val) {
    *val += 2;
}

void test_large_malloc() {
    int size = 10 * 1024;
    int *large = (int*)malloc(size);
    if (large == NULL) {
        ocall_print_string("ERROR: test_large_malloc\n");
    }
    memset(large, 0xFF, size);
    free(large);
    ocall_print_string("PASS: test_large_malloc\n");
}

void test_printf() {
    printf("%s%c %s\n", "PASS", ':', "test_printf");
}

void test_vector() {
    std::vector<int> vec;
    vec.push_back(1);
}

void test_string() {
    std::vector<char> raw_str_rev = { 'o', 'l', 'l', 'e', 'H'};
    std::vector<char> raw_str;
    std::copy(raw_str_rev.rbegin(), raw_str_rev.rend(), std::back_insert_iterator<std::vector<char>>(raw_str));
    std::string str(raw_str.begin(), raw_str.end());
    printf("PASS: test_string %s World\n", str.c_str());
}

void ecall_test_libc() {
    test_large_malloc();
    test_printf();
    test_vector();
    test_string();
}

void ecall_struct_inout(NonePOD_t *npod, POD_t pod) {

}