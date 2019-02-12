#![no_std]
#[macro_use]
extern crate sgx_tstd as std;
extern crate sgx_types;

use sgx_types::*;
use std::string::String;

#[no_mangle]
pub extern "C" fn ecall_print_square(n: i32) -> sgx_status_t {
    let n_squared = n * n;
    println!("n: {}, n^2: {}", n, n_squared);
    let enclave_string = String::from("String inside enclave!");
    println!("{}", enclave_string);
    sgx_status_t::SGX_SUCCESS
}