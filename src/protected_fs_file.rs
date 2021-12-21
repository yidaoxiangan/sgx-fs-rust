use sgx_types::*;

use crate::protected_fs_node::*;

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
struct _file_mht_node {
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

struct _file_data_node {
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
