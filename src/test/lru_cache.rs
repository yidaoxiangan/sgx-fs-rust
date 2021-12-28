use crate::lru_cache::lru_cache;

use super::super::protected_fs_file::*;
use super::super::util::*;
use sgx_tstd::vec::Vec;
use sgx_tstd::io::{self, Write};
pub fn test_lru_cache_add_and_remove() {
    let mut my_cache = lru_cache::new();

    let entry_1 = vec![1];
    let entry_2 = vec![1,2];
    my_cache.add(1, entry_1);
    my_cache.add(2, entry_2);
    my_cache.add(3,vec![1,2,3]);
    my_cache.print_all_keys_and_all_values();
    // let a = my_cache.get(1);
    my_cache.print_all_keys_and_all_values();
    my_cache.add(2,vec![2,1]);
    my_cache.print_all_keys_and_all_values();
    my_cache.remove_last();
    my_cache.print_all_keys_and_all_values();
    my_cache.remove_last();
    my_cache.print_all_keys_and_all_values();
    my_cache.remove_last();
    my_cache.print_all_keys_and_all_values();
    my_cache.remove_last();
    my_cache.print_all_keys_and_all_values();
 
}

pub fn test_lru_cache_iter() {
    let mut cache = lru_cache::new();
    cache.add(1,vec![1,1,1]);
    cache.add(2,vec![2,2,2]);
    cache.add(3,vec![3,3,3]);
    cache.add(4,vec![4,4,4]);
    cache.add(5,vec![5,5,5]);
    cache.add(6,vec![6,6,6]);
    cache.add(7,vec![7,7,7]);

    cache.print_all_keys_and_all_values();

    let mut a: &Vec<u8> = cache.get_first().unwrap() as &Vec<u8>;

    print_vec_u8(a);
    println!();
    a = cache.get_next().unwrap();
    print_vec_u8(a);
    println!();
    a = cache.get_next().unwrap();
    print_vec_u8(a);
    println!();
    a = cache.get_last().unwrap();
    print_vec_u8(a);
    println!();
}
