extern crate sgx_types;
extern crate sgx_urts;
use sgx_types::*;
use sgx_urts::SgxEnclave;

static ENCLAVE_FILE: &'static str = "enclave.signed.so";

extern {
	fn ecall_print_square(eid: sgx_enclave_id_t, retval: *mut sgx_status_t, n: i32) -> sgx_status_t;
}

fn init_enclave(debug: bool) -> SgxResult<SgxEnclave> {
	let mut launch_token: sgx_launch_token_t = [0; 1024];
	let debug = if debug { 1 } else { 0 };
	let mut misc_attr = sgx_misc_attribute_t {secs_attr: sgx_attributes_t { flags:0, xfrm:0}, misc_select:0};
	SgxEnclave::create(ENCLAVE_FILE, debug, &mut launch_token, &mut 0, &mut misc_attr)
}

fn main() {
	let enclave = init_enclave(true).unwrap();
	println!("Enclave initialized, eid: {}", enclave.geteid());

	let n = 7;
	let mut retval = sgx_status_t::SGX_SUCCESS;
	let result = unsafe { ecall_print_square(enclave.geteid(), &mut retval, n) };

	match result {
		sgx_status_t::SGX_SUCCESS if retval == sgx_status_t::SGX_SUCCESS => {
			println!("ECALL success!")
		},
		_ => println!("ECALL failed!")
	}

	enclave.destroy();
}
