use crate::parser::MemArg;
use std::{collections::HashMap, fmt::Debug, mem, ptr};

pub struct Memory<const PAGE_SIZE: usize> {
    map: HashMap<usize, [u8; PAGE_SIZE]>,
    pub stack_ptr: usize,
    pub stack_base: usize,
    pub stack_end: usize,
}
impl<const PAGE_SIZE: usize> Debug for Memory<PAGE_SIZE> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut d = self.map.iter().collect::<Vec<_>>();
        d.sort_by_key(|(d, _)| *d);
        for (l, r) in d {
            write!(f, "{l},")?;
            for c in r {
                write!(f, "{},", *c as char)?;
            }
            writeln!(f)?
        }
        writeln!(f)
    }
}
impl<const PAGE_SIZE: usize> Memory<PAGE_SIZE> {
    pub fn size(&self) -> usize {
        self.map.len() * PAGE_SIZE
    }

    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            stack_ptr: 0,
            stack_base: 0,
            stack_end: 0,
        }
    }

    fn set_u8(&mut self, address: usize, MemArg { align: _, offset }: MemArg, byte: u8) {
        // let align = 2usize.pow(align);
        let address = address + offset as usize;
        let block = address / PAGE_SIZE;
        let index = address % PAGE_SIZE;
        self.map.entry(block).or_insert([0; PAGE_SIZE])[index] = byte;
    }

    pub fn set<T>(&mut self, address: usize, mem_arg: MemArg, val: T) {
        // println!("setting {}", address + mem_arg.offset as usize);
        // let align = 2usize.pow(align);
        let t = &val as *const T as *const u8;
        for i in 0..mem::size_of::<T>() {
            self.set_u8(address + i, mem_arg, unsafe { *t.add(i) })
        }
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
        // println!("getting {}", address + mem_arg.offset as usize);
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
