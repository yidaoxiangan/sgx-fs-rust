use sgx_tstd::io::{self, Write};
use sgx_tstd::vec::Vec;

pub fn print_vec_u8(vec: &Vec<u8>) {
    print!("[ ");
    for i in &**vec {
        print!("{} ", i);
    }
    print!("]");
}
pub fn print_vec_i32(vec: &Vec<i32>) {
    print!("[ ");
    for i in &**vec {
        print!("{} ", i);
    }
    print!("]");
}
