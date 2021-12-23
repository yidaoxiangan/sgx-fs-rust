use crate::{protected_fs_file::*, protected_fs_node::meta_data_node_t};

impl protected_fs_file {
    pub fn init_fields(&mut self) {
        self.node = protected_fs_file_node::meta_data_node {
            meta_data_node_number: 0,
            file_meta_data: meta_data_node_t::default(),
            // TODO: CONTINUE
            // encyrpted_part_plain: 
        }
    }
}
