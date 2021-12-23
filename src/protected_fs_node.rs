use core::default;

use sgx_tstd::mem::*;
use sgx_types::*;

pub const NODE_SIZE: usize = 4096;

pub type sgx_iv_t = [uint8_t; SGX_AESGCM_IV_SIZE];

pub const SGX_FILE_ID: uint64_t = 0x5347585F46494C45;
pub const SGX_FILE_MAJOR_VERSION: uint8_t = 0x01;
pub const SGX_FILE_MINOR_VERSION: uint8_t = 0x00;

//TODO: COMPILE_TIME_ASSERT

#[repr(C)]
#[derive(Default)]
struct _meta_data_plain {
    fild_id: uint64_t,
    major_version: uint8_t,
    minor_version: uint8_t,

    meta_data_key_id: sgx_key_id_t,
    cpu_svn: sgx_cpu_svn_t,
    isv_svn: sgx_isv_svn_t,
    use_user_kdk_key: uint8_t,

    meta_data_gmac: sgx_aes_gcm_128bit_tag_t,

    update_flag: uint8_t,
}
pub type meta_data_plain_t = _meta_data_plain;

pub const FILENAME_MAX_LEN: size_t = 260;
pub const MD_USER_DATA_SIZE: size_t = NODE_SIZE * 3 / 4;

struct _mc_uuid {
    mcuuid: [uint8_t; 16],
}

pub type sgx_mc_uuid_t = _mc_uuid;

#[repr(C)]
pub struct _meta_data_encrypted {
    clean_filename: [c_char; FILENAME_MAX_LEN],
    size: int64_t,

    mc_uuid: sgx_mc_uuid_t,
    mc_value: uint32_t,

    mht_key: sgx_aes_gcm_128bit_key_t,
    mht_gmac: sgx_aes_gcm_128bit_tag_t,

    data: [uint8_t; MD_USER_DATA_SIZE],
}

pub type meta_data_encrypted_t = _meta_data_encrypted;

pub type meta_data_encrypted_blob_t = [uint8_t; size_of::<meta_data_encrypted_t>()];
pub const META_DATA_NODE_SIZE: usize = NODE_SIZE;
pub type meta_data_padding_t = [uint8_t;
    META_DATA_NODE_SIZE
        - (size_of::<meta_data_plain_t>() + size_of::<meta_data_encrypted_blob_t>())];

#[repr(C)]
pub struct _meta_data_node {
    plain_part: meta_data_plain_t,
    encrypted_part: meta_data_encrypted_blob_t,
    padding: meta_data_padding_t,
}
pub type meta_data_node_t = _meta_data_node;

impl Default for meta_data_node_t {
    fn default() -> Self {
        Self {
            plain_part: meta_data_plain_t::default(),
            padding: [
                0 as uint8_t;
                META_DATA_NODE_SIZE
                    - (size_of::<meta_data_plain_t>() + size_of::<meta_data_encrypted_blob_t>())
            ],
            encrypted_part: [0 as uint8_t; size_of::<meta_data_encrypted_t>()],
        }
    }
}

#[repr(C)]
struct _data_node_crypto {
    key: sgx_aes_gcm_128bit_key_t,
    gmac: sgx_aes_gcm_128bit_tag_t,
}
pub type gcm_crypto_data_t = _data_node_crypto;

pub const ATTACHED_DATA_NODES_COUNT: size_t = NODE_SIZE / size_of::<gcm_crypto_data_t>() * 3 / 4;
//TODO: COMPILE_TIME_ASSERT
pub const CHILD_MHT_NODES_COUNT: size_t = NODE_SIZE / size_of::<gcm_crypto_data_t>() * 1 / 4;
//TODO: COMPILE_TIME_ASSERT

#[repr(C)]
struct _mht_node {
    data_nodes_crypto: [gcm_crypto_data_t; ATTACHED_DATA_NODES_COUNT],
    mht_nodes_crypto: [gcm_crypto_data_t; CHILD_MHT_NODES_COUNT],
}
pub type mht_node_t = _mht_node;

#[repr(C)]
pub struct _data_node {
    data: [uint8_t; NODE_SIZE],
}
pub type data_node_t = _data_node;

pub struct _encrypted_node {
    cipher: [uint8_t; NODE_SIZE],
}
pub type encrypted_node_t = _encrypted_node;

#[repr(C)]
pub struct _recovery_node {
    physical_node_number: uint64_t,
    node_data: [uint8_t; NODE_SIZE],
}

pub type recovery_node_t = _recovery_node;
