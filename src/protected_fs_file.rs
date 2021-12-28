use sgx_tstd::{*, ffi::CString};
use sgx_types::*;

use crate::{lru_cache::*, protected_fs_node::*};

enum protected_fs_status_e {
    SGX_FILE_STATUS_OK = 0,
    SGX_FILE_STATUS_NOT_INITIALIZED,
    SGX_FILE_STATUS_FLUSH_ERROR,
    SGX_FILE_STATUS_WRITE_TO_DISK_FAILED,
    SGX_FILE_STATUS_CRYPTO_ERROR,
    SGX_FILE_STATUS_CORRUPTED,
    SGX_FILE_STATUS_MEMORY_CORRUPTEFD,
    //SGX_FILE_STATUS_WRITE_TO_DISK_FAILED_NEED_MC,
    //SGX_FILE_STATUS_MC_NOT_INCREMENTED,
    SGX_FILE_STATUS_CLOSED,
}

pub const MAX_PAGES_IN_CACHE: uint8_t = 48;

struct _mode {
    read: uint8_t,
    write: uint8_t,
    append: uint8_t,
    binary: uint8_t,
    update: uint8_t,
}
impl Default for _mode {
    fn default() -> Self {
        Self {
            read: 1,
            write: 1,
            append: 1,
            binary: 1,
            update: 1,
        }
    }
}
enum open_mode_t {
    mode(_mode),
    raw(uint8_t),
}

const FILE_MHT_NODE_TYPE: size_t = 1;
const FILE_DATA_NODE_TYPE: size_t = 2;

const PATHNAME_MAX_LEN: size_t = 512;
const FULLNAME_MAX_LEN: size_t = PATHNAME_MAX_LEN + FILENAME_MAX_LEN;
const RECOVERY_FILE_MAX_LEN: size_t = FULLNAME_MAX_LEN + 10;

// TODO: ELEGENTLY COMPLETE IT
enum current_node {
    encrypted_node {
        physical_node_number: uint64_t,
        encrypted: encrypted_node_t,
    },
    recovery_node(recovery_node_t),
}
pub struct _file_mht_node {
    _type: uint8_t,
    mht_node_number: uint64_t,
    parent: *mut _file_mht_node,
    need_writing: bool,
    new_node: bool,
    // TODO: change the name. Node is fuzzy.
    node: current_node,
    plain: data_node_t,
}

pub type file_mht_node_t = _file_mht_node;

#[allow(non_camel_case_types)]
pub struct _file_data_node {
    _type: uint8_t,
    mht_node_number: uint64_t,
    parent: *mut _file_data_node,
    need_writing: bool,
    new_node: bool,
    // TODO: change the name. Node is fuzzy.
    node: current_node,
    plain: data_node_t,
}

pub type file_data_node_t = _file_data_node;

pub enum protected_fs_file_node {
    meta_data_node {meta_data_node_number: uint64_t, file_meta_data: meta_data_node_t},
    meta_data_recovery_node,
}

pub struct protected_fs_file {
    pub node: protected_fs_file_node,
    encyrpted_part_plain: meta_data_encrypted_t,

    root_mht: file_mht_node_t,

    // TODO: FILE
    file: CString,
    open_mode: open_mode_t,
    read_only: uint8_t,
    offset: int64_t,
    end_of_file: bool,

    real_file_size: int64_t,

    need_writing: bool,
    last_error: uint32_t,
    file_status: protected_fs_status_e,

    mutex: sgx_thread_mutex_t,

    use_user_kdk_key: uint8_t,
    user_kdk_key: sgx_aes_gcm_128bit_key_t,

    cur_key: sgx_aes_gcm_128bit_key_t,
    session_master_key: sgx_aes_gcm_128bit_key_t,
    master_key_count: uint32_t,

    recovery_filename: [c_char; RECOVERY_FILE_MAX_LEN],

    cache: lru_cache<u8>,

    empty_iv: sgx_iv_t,
    report: sgx_report_t,
}
