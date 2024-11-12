use crate::parser::MemArg;
use std::{collections::HashMap, mem, num::Wrapping};

use super::RuntimeError;

struct Page<const PAGE_SIZE: usize> {
    _zero: usize,
    data: [u8; PAGE_SIZE],
}

pub struct Memory<const PAGE_SIZE: usize> {
    current_pages: usize,
    max_pages: usize,
    map: HashMap<usize, Page<PAGE_SIZE>>,
}
impl<const PAGE_SIZE: usize> Memory<PAGE_SIZE> {
    pub fn grow(&mut self, m: usize) -> i32 {
        let calc = (Wrapping(self.current_pages) + Wrapping(m)).0;
        if calc < self.current_pages || calc > self.max_pages {
            -1
        } else {
            self.current_pages += m;
            1
        }
    }

    pub fn size(&self) -> usize {
        self.map.len() * PAGE_SIZE
    }

    pub fn new(current_pages: usize, max_pages: usize) -> Self {
        Self {
            current_pages,
            max_pages,
            map: HashMap::new(),
        }
    }

    fn set_u8(
        &mut self,
        address: usize,
        MemArg { align: _, offset }: MemArg,
        byte: u8,
    ) -> Result<(), RuntimeError> {
        // let align = 2usize.pow(align);
        let address = address + offset as usize;
        let block = address / PAGE_SIZE;
        if block >= self.current_pages {
            return Err(RuntimeError::OutOfBoundsMemoryAccess);
        }
        let index = address % PAGE_SIZE;
        let entry = self.map.entry(block).or_insert(Page {
            _zero: 1,
            data: [0; PAGE_SIZE],
        });
        entry.data[index] = byte;
        Ok(())
    }

    pub fn set<T>(&mut self, address: usize, mem_arg: MemArg, val: T) -> Result<(), RuntimeError> {
        // println!("setting {}", address + mem_arg.offset as usize);
        // let align = 2usize.pow(align);

        {
            let start_address = address + mem_arg.offset as usize;
            let start_block = start_address / PAGE_SIZE;
            let end_address = address + mem_arg.offset as usize + mem::size_of::<T>();
            let end_block = end_address / PAGE_SIZE;

            if start_block >= self.current_pages || end_block >= self.current_pages {
                return Err(RuntimeError::OutOfBoundsMemoryAccess);
            }
        }

        let t = &val as *const T as *const u8;
        for i in 0..mem::size_of::<T>() {
            self.set_u8(address + i, mem_arg, unsafe { *t.add(i) })?
        }
        Ok(())
    }

    fn get_u8(&self, address: usize, MemArg { align: _, offset }: MemArg) -> u8 {
        // let align = 2usize.pow(align);
        let address = address + offset as usize;
        let block = address / PAGE_SIZE;
        let index = address % PAGE_SIZE;
        if let Some(v) = self.map.get(&block) {
            v.data[index]
        } else {
            0
        }
    }

    // Very nice function! :)
    pub fn get<T>(&self, address: usize, mem_arg: MemArg) -> Result<T, RuntimeError> {
        // println!("getting {}", address + mem_arg.offset as usize);

        let mut val = unsafe { mem::zeroed::<T>() };
        let r = &mut val as *mut T as *mut u8;

        {
            let start_address = address + mem_arg.offset as usize;
            let start_block = start_address / PAGE_SIZE;
            let end_address = address + mem_arg.offset as usize + mem::size_of::<T>();
            let end_block = end_address / PAGE_SIZE;

            if start_block >= self.current_pages || end_block >= self.current_pages {
                return Err(RuntimeError::OutOfBoundsMemoryAccess);
            }
        }

        for i in 0..mem::size_of::<T>() {
            let b = self.get_u8(address + i, mem_arg);
            unsafe { *r.add(i) = b };
        }

        Ok(val)
    }
}
