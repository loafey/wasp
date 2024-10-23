#![feature(const_type_name)]

use hex::Hex;
use parser::{FuncIdx, ImportDesc, Instr::*, Module, Parsable};
use std::{io::Cursor, mem::MaybeUninit};
mod hex;
mod parser;

fn alloc<const N: usize>() -> Hex<N> {
    #[allow(clippy::uninit_assumed_init)]
    #[allow(invalid_value)]
    unsafe {
        MaybeUninit::uninit().assume_init()
    }
}

#[derive(Debug)]
enum StackValue {
    I32(i32),
}
struct Runtime {
    module: Module,
    stack: Vec<StackValue>,
    data: Vec<u8>,
}
impl Runtime {
    pub fn new(module: Module) -> Self {
        let mut data = Vec::new();
        for d in &module.datas.data {
            match d {
                parser::Data::Mem0(_, vec) => data.append(&mut vec.clone()),
                parser::Data::MemB(_) => todo!(),
                parser::Data::MemX(_, _, _) => todo!(),
            }
        }
        Self {
            module,
            stack: Vec::new(),
            data,
        }
    }
    pub fn call_by_id(&mut self, id: u32) {
        let import_funcs: usize = self
            .module
            .imports
            .imports
            .iter()
            .map(|i| matches!(i.desc, ImportDesc::Func(_)) as usize)
            .sum();

        if (id as usize) < import_funcs {
            let func = &self.module.imports.imports[id as usize];
            match &*func.module.0 {
                m @ "console" => match &*func.name.0 {
                    "log" => {
                        let Some((StackValue::I32(y), StackValue::I32(x))) =
                            self.stack.pop().zip(self.stack.pop())
                        else {
                            unimplemented!()
                        };
                        let (x, y) = (x as usize, y as usize);

                        let str = String::from_utf8_lossy(&self.data[x..y]);
                        println!("{str}");
                    }
                    n => panic!("no function named {n:?} in module {m:?}"),
                },
                m => panic!("unknown module {m:?}"),
            }
        } else {
            println!("{import_funcs} | {id}");
            let index = if import_funcs == 0 {
                id as usize
            } else {
                import_funcs - id as usize
            };
            let instrs = self.module.code.code[index].code.e.instrs.clone();
            #[allow(clippy::needless_range_loop)]
            for pc in 0..instrs.len() {
                match &instrs[pc] {
                    x00_unreachable => todo!("x00_unreachable"),
                    x02_block(_, _) => todo!("x02_block"),
                    x03_loop(_, _) => todo!("x03_loop"),
                    x0c_br(_) => todo!("x0c_br"),
                    x0d_br_if(_) => todo!("x0d_br_if"),
                    x0e_br_table(_, _) => todo!("x0e_br_table"),
                    x0f_return => todo!("x0f_return"),
                    x10_i32_const(i) => self.stack.push(StackValue::I32(*i)),
                    x11_call_indirect(_, _) => todo!("x11_call_indirect"),
                    x1a_drop => todo!("x1a_drop"),
                    x1b_select => todo!("x1b_select"),
                    x20_local_get(_) => todo!("x20_local_get"),
                    x21_local_set(_) => todo!("x21_local_set"),
                    x22_local_tee(_) => todo!("x22_local_tee"),
                    x23_global_get(_) => todo!("x23_global_get"),
                    x24_global_set(_) => todo!("x24_global_set"),
                    x26_table_set(_) => todo!("x26_table_set"),
                    x28_i32_load(_) => todo!("x28_i32_load"),
                    x29_i64_load(_) => todo!("x29_i64_load"),
                    x2c_i32_load8_s(_) => todo!("x2c_i32_load8_s"),
                    x2d_i32_load8_u(_) => todo!("x2d_i32_load8_u"),
                    x2f_i32_load16_u(_) => todo!("x2f_i32_load16_u"),
                    x36_i32_store(_) => todo!("x36_i32_store"),
                    x37_i64_store(_) => todo!("x37_i64_store"),
                    x3a_i32_store8(_) => todo!("x3a_i32_store8"),
                    x3b_i32_store16(_) => todo!("x3b_i32_store16"),
                    x40_grow => todo!("x40_grow"),
                    x41_call(FuncIdx(id)) => self.call_by_id(*id),
                    x42_i64_const(_) => todo!("x42_i64_const"),
                    x45_i32_eqz => todo!("x45_i32_eqz"),
                    x46_i32_eq => todo!("x46_i32_eq"),
                    x47_i32_ne => todo!("x47_i32_ne"),
                    x48_lt_s => todo!("x48_lt_s"),
                    x49_i32_lt_u => todo!("x49_i32_lt_u"),
                    x4a_i32_gt_s => todo!("x4a_i32_gt_s"),
                    x4b_i32_gt_u => todo!("x4b_i32_gt_u"),
                    x4c_i32_le_s => todo!("x4c_i32_le_s"),
                    x4d_i32_le_u => todo!("x4d_i32_le_u"),
                    x4e_i32_ge_s => todo!("x4e_i32_ge_s"),
                    x4f_i32_ge_u => todo!("x4f_i32_ge_u"),
                    x51_i64_eq => todo!("x51_i64_eq"),
                    x52_i64_ne => todo!("x52_i64_ne"),
                    x67_i32_clz => todo!("x67_i32_clz"),
                    x68_i32_ctz => todo!("x68_i32_ctz"),
                    x6a_i32_add => todo!("x6a_i32_add"),
                    x6b_i32_sub => todo!("x6b_i32_sub"),
                    x6c_i32_mul => todo!("x6c_i32_mul"),
                    x6e_i32_div_u => todo!("x6e_i32_div_u"),
                    x71_i32_and => todo!("x71_i32_and"),
                    x72_i32_or => todo!("x72_i32_or"),
                    x73_i32_xor => todo!("x73_i32_xor"),
                    x74_i32_shl => todo!("x74_i32_shl"),
                    x76_i32_shr_u => todo!("x76_i32_shr_u"),
                    x77_i32_rotl => todo!("x77_i32_rotl"),
                    x7c_i64_add => todo!("x7c_i64_add"),
                    x7e_i64_mul => todo!("x7e_i64_mul"),
                    x84_i64_or => todo!("x84_i64_or"),
                    x86_i64_shl => todo!("x86_i64_shl"),
                    x88_i64_shr_u => todo!("x88_i64_shr_u"),
                    xa7_i32_wrap_i64 => todo!("xa7_i32_wrap_i64"),
                    xad_i64_extend_i32_u => todo!("xad_i64_extend_i32_u"),
                }
            }
        }
    }
}

fn main() {
    let bin: &[u8] = include_bytes!("../examples/rust_addition.wasm");
    let mut cursor = Cursor::new(bin);
    let mut stack = Vec::new();
    let module = match Module::parse(&mut cursor, &mut stack) {
        Ok(o) => o,
        Err(e) => {
            stack.reverse();
            eprintln!(
                "Error: {e:?}, bin pos: {}, stack: {stack:#?}",
                cursor.position()
            );
            return;
        }
    };
    // println!("{module:#?}");
    Runtime::new(module).call_by_id(1);
}
