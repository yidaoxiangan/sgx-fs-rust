use core::{mem, ptr};
use sgx_tstd::boxed::Box;
use sgx_tstd::collections::HashMap;
use sgx_tstd::io::{self, Write};
use sgx_tstd::vec::Vec;
use super::util::print_vec_u8;

struct lru_entry<T> {
    key: mem::MaybeUninit<u64>,
    val: mem::MaybeUninit<T>,
    prev: *mut lru_entry<T>,
    next: *mut lru_entry<T>,
}

impl<T> lru_entry<T> {
    fn new(key: u64, val: T) -> Self {
        lru_entry {
            key: mem::MaybeUninit::new(key),
            val: mem::MaybeUninit::new(val),
            prev: ptr::null_mut(),
            next: ptr::null_mut(),
        }
    }

    fn new_sigil() -> Self {
        lru_entry {
            key: mem::MaybeUninit::uninit(),
            val: mem::MaybeUninit::uninit(),
            prev: ptr::null_mut(),
            next: ptr::null_mut(),
        }
    }
}

pub struct lru_cache<T> {
    map: HashMap<u64, Box<lru_entry<T>>>,
    head: *mut lru_entry<T>,
    tail: *mut lru_entry<T>,

    m_it: *mut lru_entry<T>,
}

impl<T> lru_cache<T> {
    pub fn new() -> Self {
        let cache = lru_cache {
            map: HashMap::default(),
            head: Box::into_raw(Box::new(lru_entry::<T>::new_sigil())),
            tail: Box::into_raw(Box::new(lru_entry::<T>::new_sigil())),
            m_it: ptr::null_mut(),
        };
        unsafe {
            (*(cache.head)).next = cache.tail;
            (*(cache.tail)).prev = cache.head;
        }
        cache
    }

    pub fn add(&mut self, k: u64, mut v: T) -> Option<T> {
        let node_ptr = self.map.get_mut(&k).map(|node| {
            let node_ptr: *mut lru_entry<T> = &mut **node;
            node_ptr
        });

        match node_ptr {
            Some(node_ptr) => {
                unsafe {
                    mem::swap(&mut v, &mut (*(*node_ptr).val.as_mut_ptr()) as &mut T);
                }
                self.detach(node_ptr);
                self.attach(node_ptr);

                Some(v)
            }

            None => {
                let mut node = Box::new(lru_entry::new(k, v));
                let node_ptr: *mut lru_entry<T> = &mut *node;
                self.attach(node_ptr);
                self.map.insert(k, node);
                None
            }
        }
    }

    fn detach(&mut self, node: *mut lru_entry<T>) {
        unsafe {
            (*(*node).prev).next = (*node).next;
            (*(*node).next).prev = (*node).prev;
        }
    }
    fn attach(&mut self, node: *mut lru_entry<T>) {
        unsafe {
            (*node).next = (*self.head).next;
            (*node).prev = self.head;
            (*self.head).next = node;
            (*(*node).next).prev = node;
        }
    }

    pub fn get<'a>(&'a mut self, key: u64) -> Option<&'a T> {
        if let Some(node) = self.map.get_mut(&key) {
            let node_ptr: *mut lru_entry<T> = &mut **node;
            self.detach(node_ptr);
            self.attach(node_ptr);

            unsafe { Some(&mut *((*node_ptr).val).as_mut_ptr() as &T) }
        } else {
            None
        }
    }

    pub fn find<'a>(&'a mut self, key: u64) -> Option<&'a T> {
        if let Some(node) = self.map.get_mut(&key) {
            let node_ptr: *mut lru_entry<T> = &mut **node;

            unsafe { Some(&mut *((*node_ptr).val).as_mut_ptr() as &T) }
        } else {
            None
        }
    }

    pub fn size(&self) -> usize {
        self.map.len()
    }

    pub fn get_first<'a>(&mut self) -> Option<&'a T> {
        if self.size() == 0 {
            return None;
        }
        self.m_it = unsafe { (*self.head).next };

        unsafe { Some(&mut *((*self.m_it).val).as_mut_ptr() as &T) }
    }

    pub fn get_last<'a>(&mut self) -> Option<&'a T> {
        if self.size() == 0 {
            return None;
        }
        self.m_it = unsafe { (*self.tail).prev };

        unsafe { Some(&mut *((*self.m_it).val).as_mut_ptr() as &T) }
    }

    pub fn get_next<'a>(&mut self) -> Option<&'a T> {
        if self.size() == 0 {
            return None;
        }

        self.m_it = unsafe { (*self.m_it).next };

        if self.m_it == self.tail {
            return None;
        }
        unsafe { Some(&mut *((*self.m_it).val).as_mut_ptr() as &T) }
    }

    pub fn remove_last(&mut self) {
        let prev;
        unsafe { prev = (*self.tail).prev }
        if prev != self.head {
            let old_key;
            old_key = unsafe { &(*(*(*self.tail).prev).key.as_ptr()) };
            let mut old_node = self.map.remove(&old_key).unwrap();
            let node_ptr: *mut lru_entry<T> = &mut *old_node;
            self.detach(node_ptr);
        }
    }
}

impl lru_cache<Vec<u8>> {
    pub fn print_all_keys_and_all_values(&mut self) {
        println!("The length of current lru cache is {}.", self.size());
        println!("-----START--------");
        if self.size() == 0 {
            println!("NOTHING HERE");
            println!("------------------");
            return;
        }
        let head = self.head;

        let mut ptr;
        let mut key;
        unsafe {
            ptr = (*head).next;
            key = *(*ptr).key.as_ptr();
        }

        for _x in 0..self.map.len() {
            print!("[{}] ==> ", key);

            if let Some(ptr) = self.find(key) {
                print_vec_u8(ptr);
            }
            println!("");
            unsafe {
                ptr = (*ptr).next;
                key = *(*ptr).key.as_ptr();
            }
        }
        println!("-----END----------");
    }
}
