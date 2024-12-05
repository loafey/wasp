use crate::parser::MemArg;
use std::{collections::HashMap, fmt::Debug, mem, num::Wrapping};

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
impl<const PAGE_SIZE: usize> Debug for Memory<PAGE_SIZE> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Memory<{PAGE_SIZE}>")
    }
}
impl<const PAGE_SIZE: usize> Memory<PAGE_SIZE> {
    pub fn grow(&mut self, m: usize) -> i32 {
        let calc = (Wrapping(self.current_pages) + Wrapping(m)).0;
        if calc < self.current_pages || calc > self.max_pages {
            -1
        } else {
            self.current_pages += m;
            (self.current_pages - m) as i32
        }
    }

    pub fn pages(&self) -> (usize, usize) {
        (self.current_pages, self.max_pages)
    }

    #[allow(unused)]
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
        // println!("writing {byte} to {address}");
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
        if self.current_pages == 0 {
            return Err(RuntimeError::OutOfBoundsMemoryAccess);
        }
        // println!("setting {}", address + mem_arg.offset as usize);
        // let align = 2usize.pow(align);

        {
            let start_address = address + mem_arg.offset as usize;
            let start_block = start_address / PAGE_SIZE;
            let end_address = address + mem_arg.offset as usize + mem::size_of::<T>() - 1;
            let end_block = end_address / PAGE_SIZE;
            if start_block > self.current_pages
                || end_block >= self.current_pages
                || end_block > self.max_pages
            {
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
        let byte = if let Some(v) = self.map.get(&block) {
            v.data[index]
        } else {
            0
        };
        // println!("got {byte} from {address}");
        byte
    }

    // Very nice function! :)
    pub fn get<T: std::fmt::Debug>(
        &self,
        address: usize,
        mem_arg: MemArg,
    ) -> Result<T, RuntimeError> {
        if self.current_pages == 0 {
            return Err(RuntimeError::OutOfBoundsMemoryAccess);
        }
        // println!("getting {}", address + mem_arg.offset as usize);

        let mut val = unsafe { mem::zeroed::<T>() };
        let r = &mut val as *mut T as *mut u8;

        {
            let start_address = address + mem_arg.offset as usize;
            let start_block = start_address / PAGE_SIZE;
            let end_address = address + mem_arg.offset as usize + mem::size_of::<T>() - 1;
            let end_block = end_address / PAGE_SIZE;

            if start_block > self.current_pages
                || end_block >= self.current_pages
                || end_block >= self.max_pages
            {
                return Err(RuntimeError::OutOfBoundsMemoryAccess);
            }
        }

        for i in 0..mem::size_of::<T>() {
            let b = self.get_u8(address + i, mem_arg);
            unsafe { *r.add(i) = b };
        }

        // println!("read got {val:?}");

        Ok(val)
    }

    pub fn copy(
        &mut self,
        source: usize,
        amount: usize,
        destination: usize,
    ) -> Result<(), RuntimeError> {
        if self.current_pages == 0 {
            return Err(RuntimeError::OutOfBoundsMemoryAccess);
        }
        let start_block = source / PAGE_SIZE;
        let end_block = (source + amount) / PAGE_SIZE;

        let destination_block = destination / PAGE_SIZE;
        let destination_block_end = (destination + amount) / PAGE_SIZE;

        println!(
            "{source}|{} {destination}|{} : {start_block}|{end_block} {destination_block}|{destination_block_end} {}|{}",
            source + amount, destination + amount, self.current_pages, self.max_pages
        );

        if start_block > self.current_pages
            || end_block > self.current_pages
            || destination_block >= self.current_pages
            || destination_block_end >= self.current_pages
        {
            return Err(RuntimeError::OutOfBoundsMemoryAccess);
        }

        let mut buf = Vec::new();
        for i in 0..amount {
            buf.push(self.get_u8(source + i, MemArg::default()));
        }
        #[allow(clippy::needless_range_loop)]
        for i in 0..amount {
            self.set_u8(destination + i, MemArg::default(), buf[i])?
        }

        Ok(())
    }

    pub fn slice_write(&mut self, address: usize, slice: &[u8]) -> Result<(), RuntimeError> {
        if self.current_pages == 0 {
            return Err(RuntimeError::OutOfBoundsMemoryAccess);
        }
        let start_address = address;
        let start_block = start_address / PAGE_SIZE;
        let end_address = address + slice.len();
        let end_block = end_address / PAGE_SIZE;

        if start_block > self.current_pages || end_block >= self.current_pages {
            return Err(RuntimeError::OutOfBoundsMemoryAccess);
        }

        for (i, b) in slice.iter().enumerate() {
            self.set_u8(address + i, MemArg::default(), *b)?;
        }

        Ok(())
    }

    pub fn bulk_write(&mut self, address: usize, end: usize, val: u8) -> Result<(), RuntimeError> {
        if self.current_pages == 0 {
            return Err(RuntimeError::OutOfBoundsMemoryAccess);
        }
        let start_address = address;
        let start_block = start_address / PAGE_SIZE;
        let end_address = address + end - 1;
        let end_block = end_address / PAGE_SIZE;

        if start_block > self.current_pages || end_block >= self.current_pages {
            return Err(RuntimeError::OutOfBoundsMemoryAccess);
        }

        for i in address..address + end {
            self.set_u8(i, MemArg::default(), val)?;
        }

        Ok(())
    }
}
