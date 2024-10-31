use std::{collections::HashMap, mem, ptr};

use crate::parser::MemArg;

pub struct Memory<const PAGE_SIZE: usize> {
    map: HashMap<usize, [u8; PAGE_SIZE]>,
}
impl<const PAGE_SIZE: usize> Memory<PAGE_SIZE> {
    pub fn size(&self) -> usize {
        self.map.len() * PAGE_SIZE
    }

    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn set_u8(&mut self, address: usize, MemArg { align: _, offset }: MemArg, byte: u8) {
        // let align = 2usize.pow(align);
        let address = address + offset as usize;
        let block = address / PAGE_SIZE;
        let index = address % PAGE_SIZE;
        self.map.entry(block).or_insert([0; PAGE_SIZE])[index] = byte;
    }

    fn get_u8(&self, address: usize, MemArg { align: _, offset }: MemArg) -> u8 {
        // let align = 2usize.pow(align);
        let address = address + offset as usize;
        let block = address / PAGE_SIZE;
        let index = address % PAGE_SIZE;
        if let Some(v) = self.map.get(&block) {
            v[index]
        } else {
            0
        }
    }

    // Very nice function! :)
    pub fn get<T>(&self, address: usize, mem_arg: MemArg) -> T {
        let mut val = unsafe { mem::zeroed::<T>() };
        let r = &mut val as *mut T as *mut u8;
        for i in 0..mem::size_of::<T>() {
            let b = self.get_u8(address + i, mem_arg);
            unsafe {
                ptr::write(r.add(i), b);
            }
        }

        val
    }
}
