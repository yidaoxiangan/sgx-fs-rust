use core::mem::MaybeUninit;

use crate::{
    protected_fs_file::*,
    protected_fs_node::{ATTACHED_DATA_NODES_COUNT, MD_USER_DATA_SIZE, NODE_SIZE},
};
use core::mem::MaybeUninit;
use sgx_types::*;
impl protected_fs_file {
    fn read_data_node() -> *mut file_data_node_t {
        let data_node_number: uint64_t;
        let physical_node_number: uint64_t;
        let result32: int32_t;
        let status: sgx_status_t;
    // TODO
    }

    fn get_node_numbers(
        offset: uint64_t,
        mht_node_number: *mut uint64_t,
        data_node_number: *mut uint64_t,
        physical_mht_node_number: *mut uint64_t,
        physical_data_node_number: *mut uint64_t,
    ) {
        let mut _mht_node_number = MaybeUninit::<i32>::uninit();
        let mut _data_node_number = MaybeUninit::<i32>::uninit();
        let mut _physical_mht_node_number = MaybeUninit::<i32>::uninit();
        let mut _physical_data_node_number = MaybeUninit::<i32>::uninit();

        assert!(offset >= MD_USER_DATA_SIZE);

        _data_node_number.write(offset - MD_USER_DATA_SIZE / NODE_SIZE);
        _mht_node_number.write(val_data_node_number / ATTACHED_DATA_NODES_COUNT);
        _physical_data_node_number.write(_data_node_number + 1 + 1 + _mht_node_number);
        _physical_mht_node_number
            .write(_physical_data_node_number - _data_node_number % ATTACHED_DATA_NODES_COUNT - 1);

        let mut _mht_node_number = unsafe { _mht_node_number.assume_init() };
        let mut _data_node_number = unsafe { _mht_node_number.assume_init() };
        let mut _physical_mht_node_number = unsafe { _mht_node_number.assume_init() };
        let mut _physical_data_node_number = unsafe { _mht_node_number.assume_init() };

        unsafe {
            if !mht_node_number.is_null() {
                (*mht_node_number) = _mht_node_number;
            }
            if !data_node_number.is_null() {
                (*data_node_number) = _data_node_number;
            }
            if !physical_mht_node_number.is_null() {
                (*physical_mht_node_number) = _physical_mht_node_number;
            }
            if !physical_data_node_number.is_null() {
                (*physical_data_node_number) = _physical_data_node_number;
            }
        }
    }
}
