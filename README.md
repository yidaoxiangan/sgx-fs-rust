+ TODO

### protected_fs_file.h
#### protected_fs_file class
##### private

+ [ ] `void init_fields();`
+ [ ] `bool cleanup_filename(const char* src, char* dest);`
+ [ ] `bool parse_mode(const char* mode);`
+ [ ] `bool file_recovery(const char* filename);`
+ [ ] `bool init_existing_file(const char* filename, const char* clean_filename, const sgx_aes_gcm_128bit_key_t* import_key);`
+ [ ] `bool init_new_file(const char* clean_filename);`
  
+ [ ] `bool generate_secure_blob(sgx_aes_gcm_128bit_key_t* key, const char* label, uint64_t physical_node_number, sgx_aes_gcm_128bit_tag_t* output);`
+ [ ] `bool generate_secure_blob_from_user_kdk(bool restore);`
+ [ ] `bool init_session_master_key();`
+ [ ] `bool derive_random_node_key(uint64_t physical_node_number);`
+ [ ] `bool generate_random_meta_data_key();`
+ [ ] `bool restore_current_meta_data_key(const sgx_aes_gcm_128bit_key_t* import_key);`
  
  
+ [ ] `file_data_node_t* get_data_node();`
+ [ ] `file_data_node_t* read_data_node();`
+ [ ] `file_data_node_t* append_data_node();`
+ [ ] `file_mht_node_t* get_mht_node();`
+ [ ] `file_mht_node_t* read_mht_node(uint64_t mht_node_number);`
+ [ ] `file_mht_node_t* append_mht_node(uint64_t mht_node_number);`
+ [ ] `bool write_recovery_file();`
+ [ ] `bool set_update_flag(bool flush_to_disk);`
+ [ ] `void clear_update_flag();`
+ [ ] `bool update_all_data_and_mht_nodes();`
+ [ ] `bool update_meta_data_node();`
+ [ ] `bool write_all_changes_to_disk(bool flush_to_disk);`
+ [ ] `void erase_recovery_file();`
+ [ ] `bool internal_flush(/*bool mc,*/ bool flush_to_disk);`