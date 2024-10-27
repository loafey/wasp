use std::collections::HashMap;

use crate::parser::{
    self, ExportDesc, FuncIdx, GlobalIdX, ImportDesc, Instr::*, LocalIdX, MemArg, Module, TypeIdX,
};

#[derive(Clone, Copy)]
pub enum Value {
    I32(i32),
}

impl std::fmt::Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::I32(arg0) => write!(f, "i32({arg0})"),
        }
    }
}

#[derive(Debug)]
pub struct Frame {
    pub func_id: usize,
    pub pc: usize,
    pub stack: Vec<Value>,
    pub locals: HashMap<u32, Value>,
    pub labels: HashMap<u32, u32>,
    pub block_count: Vec<usize>,
}

pub struct Runtime {
    pub module: Module,
    pub stack: Vec<Frame>,
    pub globals: HashMap<u32, Value>,
    pub data: Vec<u8>,
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

        let ExportDesc::Func(TypeIdX(main_id)) = module
            .exports
            .exports
            .iter()
            .find(|s| s.nm.0 == "main")
            .map(|f| f.d)
            .unwrap()
        else {
            panic!("no main :(")
        };
        Self {
            module,
            stack: vec![Frame {
                func_id: main_id as usize,
                pc: 0,
                stack: Vec::new(),
                locals: [(0, Value::I32(0)), (1, Value::I32(0))].into(),
                labels: HashMap::new(),
                block_count: Vec::new(),
            }],
            data,
            globals: HashMap::new(),
        }
    }
    pub fn step(&mut self) {
        let f = self.stack.last_mut().unwrap();

        let import_funcs: usize = self
            .module
            .imports
            .imports
            .iter()
            .map(|i| matches!(i.desc, ImportDesc::Func(_)) as usize)
            .sum();

        if f.func_id < import_funcs {
            let func = &self.module.imports.imports[f.func_id];
            match &*func.module.0 {
                m @ "console" => match &*func.name.0 {
                    "log" => {
                        let st = self.stack.last_mut().unwrap();
                        let Some((Value::I32(y), Value::I32(x))) =
                            st.stack.pop().zip(st.stack.pop())
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
            let index = if import_funcs == 0 {
                f.func_id
            } else {
                import_funcs - f.func_id
            };
            let instrs = &self.module.code.code[index].code.e.instrs;
            {
                let instr = &instrs[f.pc];
                f.pc += 1;

                match instr {
                    x02_block(bt, ins) => {
                        f.block_count.push(f.stack.len());
                        match bt {
                            parser::BlockType::Eps => {}
                            parser::BlockType::T(_) => todo!(),
                            parser::BlockType::X(_) => todo!(),
                        }

                        let c = ins.clone();

                        self.module.code.code[index].code.e.instrs.remove(f.pc - 1);
                        println!("pop {}", f.pc - 1);
                        for i in c.into_iter().rev() {
                            self.module.code.code[index]
                                .code
                                .e
                                .instrs
                                .insert(f.pc - 1, i);
                        }
                        self.module.code.code[index]
                            .code
                            .e
                            .instrs
                            .insert(f.pc - 1, block_start);
                    }
                    x10_i32_const(i) => f.stack.push(Value::I32(*i)),
                    x20_local_get(LocalIdX(id)) => f.stack.push(*f.locals.get(id).unwrap()),
                    x22_local_tee(LocalIdX(id)) => {
                        let last = f.stack.last().unwrap();
                        f.locals.insert(*id, *last);
                    }
                    x23_global_get(GlobalIdX(id)) => f
                        .stack
                        .push(self.globals.get(id).copied().unwrap_or(Value::I32(0))),
                    x24_global_set(GlobalIdX(id)) => {
                        let pop = f.stack.pop().unwrap();
                        self.globals.insert(*id, pop);
                    }
                    x36_i32_store(MemArg { align, offset }) => {
                        let addr = (align * offset) as usize;
                        let end_pos = addr + size_of::<i32>();
                        if self.module.mems.len() < end_pos {
                            self.module
                                .mems
                                .append(&mut vec![0; end_pos - self.module.mems.len()]);
                        }
                        #[allow(irrefutable_let_patterns)]
                        let Value::I32(v) = f.stack.pop().unwrap() else {
                            panic!()
                        };
                        let bytes = v.to_le_bytes();
                        for (i, b) in bytes.into_iter().enumerate() {
                            self.module.mems[addr + i] = b;
                        }
                    }
                    x6a_i32_add => {
                        let y = f.stack.pop().unwrap();
                        let x = f.stack.pop().unwrap();
                        match (x, y) {
                            (Value::I32(x), Value::I32(y)) => f.stack.push(Value::I32(y + x)),
                        }
                    }
                    x6b_i32_sub => {
                        let y = f.stack.pop().unwrap();
                        let x = f.stack.pop().unwrap();
                        match (x, y) {
                            (Value::I32(x), Value::I32(y)) => f.stack.push(Value::I32(y - x)),
                        }
                    }
                    x41_call(FuncIdx(id)) => {
                        println!("In frame");
                        self.stack.push(Frame {
                            func_id: *id as usize,
                            pc: 0,
                            stack: Vec::new(),
                            locals: HashMap::new(),
                            labels: HashMap::new(),
                            block_count: Vec::new(),
                        });
                        // self.call_by_id(*id);
                    }
                    f => {
                        unimplemented!("instruction not supported : {f:?}")
                    }
                }
            }
        }
    }
}
