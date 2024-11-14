use crate::parser::{
    BlockType, FuncIdx, FuncType, GlobalIdX,
    Instr::{self, *},
    LocalIdX, NumType, TypeIdX, ValType,
};
#[derive(Debug, Default)]
pub struct TypingRules {
    pub input: Vec<ValType>,
    pub output: Vec<ValType>,
}
impl TypingRules {
    pub fn single_input(valtype: ValType) -> TypingRules {
        Self {
            input: vec![valtype],
            output: Vec::new(),
        }
    }
    pub fn single_output(valtype: ValType) -> TypingRules {
        Self {
            output: vec![valtype],
            input: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub enum TypeCheckError {
    WrongTypeOnStack,
    EmptyStack,
    MissingFunction,
    IfElseTypeMismatch(Vec<ValType>, Vec<ValType>),
    ReturnTypeMismatch(Vec<ValType>, Vec<ValType>),
    MissingLocal,
}

pub fn check(
    mut context: Vec<ValType>,
    locals: &[ValType],
    instrs: &[Instr],
    function_types: &[FuncType],
    raw_types: &[FuncType],
    globals: &[ValType],
    return_types: Option<Vec<ValType>>,
) -> Result<Vec<ValType>, TypeCheckError> {
    // println!("{instrs:#?}");
    for inst in instrs {
        // println!("    {inst:?}:\n    locals: {locals:?}\n    context: {context:?}");
        let TypingRules { input, output } = {
            let this = &inst;
            match this {
                x00_unreachable => TypingRules::default(),
                x01_nop => TypingRules::default(),
                x02_block(bt, b) | x03_loop(bt, b) => {
                    let (rt, inputs) = match bt {
                        BlockType::Eps => (TypingRules::default(), Vec::new()),
                        BlockType::T(val_type) => (
                            TypingRules {
                                input: Vec::new(),
                                output: vec![*val_type],
                            },
                            Vec::new(),
                        ),
                        BlockType::TypIdx(i) => {
                            let ft = raw_types
                                .get(*i as usize)
                                .ok_or(TypeCheckError::MissingFunction)?;
                            (
                                TypingRules {
                                    input: Vec::new(),
                                    output: ft.output.types.clone(),
                                },
                                ft.input.types.clone(),
                            )
                        }
                    };
                    let mut pass = Vec::new();
                    for input in inputs {
                        let p = context.pop().ok_or(TypeCheckError::EmptyStack)?;
                        if p != input {
                            return Err(TypeCheckError::WrongTypeOnStack);
                        }
                        pass.push(p);
                    }
                    check(pass, locals, b, function_types, raw_types, globals, None)?;
                    rt
                }
                x04_if_else(_, a, b) => {
                    let a = check(
                        Vec::new(),
                        locals,
                        a,
                        function_types,
                        raw_types,
                        globals,
                        None,
                    )?;
                    if let Some(b) = b {
                        let b = check(
                            Vec::new(),
                            locals,
                            b,
                            function_types,
                            raw_types,
                            globals,
                            None,
                        )?;
                        if a != b {
                            return Err(TypeCheckError::IfElseTypeMismatch(a, b));
                        }
                        TypingRules {
                            input: vec![ValType::Num(NumType::I32)],
                            output: a,
                        }
                    } else {
                        TypingRules::single_input(ValType::Num(NumType::I32))
                    }
                }
                x05 => todo!(),
                x06 => todo!(),
                x07 => todo!(),
                x08 => todo!(),
                x09 => todo!(),
                x0a => todo!(),
                x0b => todo!(),
                x0c_br(_) => TypingRules::default(),
                x0d_br_if(_) => TypingRules::single_input(ValType::Num(NumType::I32)),
                x0e_br_table(_, _) => TypingRules::single_input(ValType::Num(NumType::I32)),
                x0f_return => TypingRules::default(),
                x10_call(FuncIdx(i)) => {
                    let ft = function_types
                        .get(*i as usize)
                        .ok_or(TypeCheckError::MissingFunction)?;
                    TypingRules {
                        input: ft.input.types.clone(),
                        output: ft.output.types.clone(),
                    }
                }
                x11_call_indirect(TypeIdX(i), _) => {
                    let ft = raw_types
                        .get(*i as usize)
                        .ok_or(TypeCheckError::MissingFunction)?;
                    TypingRules {
                        input: ft.input.types.clone(),
                        output: ft.output.types.clone(),
                    }
                }
                x12 => todo!(),
                x13 => todo!(),
                x14 => todo!(),
                x15 => todo!(),
                x16 => todo!(),
                x17 => todo!(),
                x18 => todo!(),
                x19 => todo!(),
                x1a_drop => TypingRules {
                    input: vec![ValType::Nil],
                    output: Vec::new(),
                },
                x1b_select => {
                    let top = context.pop().ok_or(TypeCheckError::EmptyStack)?;
                    if top != ValType::Num(NumType::I32) {
                        return Err(TypeCheckError::WrongTypeOnStack);
                    }

                    let input = [
                        context.pop().ok_or(TypeCheckError::EmptyStack)?,
                        context.pop().ok_or(TypeCheckError::EmptyStack)?,
                    ];
                    TypingRules::single_output(input[0])
                }
                x1c => todo!(),
                x1d => todo!(),
                x1e => todo!(),
                x1f => todo!(),
                x20_local_get(LocalIdX(i)) => locals
                    .get(*i as usize)
                    .map(|l| TypingRules::single_output(*l))
                    .ok_or(TypeCheckError::MissingLocal)?,
                x21_local_set(LocalIdX(i)) => TypingRules::single_input(locals[*i as usize]),
                x22_local_tee(LocalIdX(i)) => TypingRules {
                    input: vec![locals[*i as usize]],
                    output: vec![locals[*i as usize]],
                },
                x23_global_get(GlobalIdX(i)) => TypingRules::single_output(globals[*i as usize]),
                x24_global_set(GlobalIdX(i)) => TypingRules::single_input(globals[*i as usize]),
                x25 => todo!(),
                x26_table_set(_) => todo!(),
                x27 => todo!(),
                x28_i32_load(_) => TypingRules {
                    input: vec![ValType::Num(NumType::I32)],
                    output: vec![ValType::Num(NumType::I32)],
                },
                x29_i64_load(_) => TypingRules {
                    input: vec![ValType::Num(NumType::I32)],
                    output: vec![ValType::Num(NumType::I64)],
                },
                x2a_f32_load(_) => TypingRules {
                    input: vec![ValType::Num(NumType::I32)],
                    output: vec![ValType::Num(NumType::F32)],
                },
                x2b_f64_load(_) => TypingRules {
                    input: vec![ValType::Num(NumType::I32)],
                    output: vec![ValType::Num(NumType::F64)],
                },
                x2c_i32_load8_s(_) => TypingRules {
                    input: vec![ValType::Num(NumType::I32)],
                    output: vec![ValType::Num(NumType::I32)],
                },
                x2d_i32_load8_u(_) => TypingRules {
                    input: vec![ValType::Num(NumType::I32)],
                    output: vec![ValType::Num(NumType::I32)],
                },
                x2e_i32_load16_s(_) => TypingRules {
                    input: vec![ValType::Num(NumType::I32)],
                    output: vec![ValType::Num(NumType::I32)],
                },
                x2f_i32_load16_u(_) => TypingRules {
                    input: vec![ValType::Num(NumType::I32)],
                    output: vec![ValType::Num(NumType::I32)],
                },
                x30_i64_load8_s(_) => TypingRules {
                    input: vec![ValType::Num(NumType::I32)],
                    output: vec![ValType::Num(NumType::I64)],
                },
                x31_i64_load8_u(_) => TypingRules {
                    input: vec![ValType::Num(NumType::I32)],
                    output: vec![ValType::Num(NumType::I64)],
                },
                x32_i64_load16_s(_) => TypingRules {
                    input: vec![ValType::Num(NumType::I32)],
                    output: vec![ValType::Num(NumType::I64)],
                },
                x33_i64_load16_u(_) => TypingRules {
                    input: vec![ValType::Num(NumType::I32)],
                    output: vec![ValType::Num(NumType::I64)],
                },
                x34_i64_load32_s(_) => TypingRules {
                    input: vec![ValType::Num(NumType::I32)],
                    output: vec![ValType::Num(NumType::I64)],
                },
                x35_i64_load32_u(_) => TypingRules {
                    input: vec![ValType::Num(NumType::I32)],
                    output: vec![ValType::Num(NumType::I64)],
                },
                x36_i32_store(_) => TypingRules {
                    input: vec![ValType::Num(NumType::I32), ValType::Num(NumType::I32)],
                    output: Vec::new(),
                },
                x37_i64_store(_) => todo!(),
                x38_f32_store(_) => todo!(),
                x39_f64_store(_) => todo!(),
                x3a_i32_store8(_) => todo!(),
                x3b_i32_store16(_) => todo!(),
                x3c_i64_store8(_) => todo!(),
                x3d_i64_store16(_) => todo!(),
                x3e_i64_store32(_) => todo!(),
                x3f => todo!(),
                x40_grow => TypingRules {
                    input: vec![ValType::Num(NumType::I32)],
                    output: vec![ValType::Num(NumType::I32)],
                },
                x41_i32_const(_) => TypingRules::single_output(ValType::Num(NumType::I32)),
                x42_i64_const(_) => TypingRules::single_output(ValType::Num(NumType::I64)),
                x43_f32_const(_) => TypingRules::single_output(ValType::Num(NumType::F32)),
                x44_f64_const(_) => TypingRules::single_output(ValType::Num(NumType::F64)),
                x45_i32_eqz => TypingRules {
                    input: vec![ValType::Num(NumType::I32)],
                    output: vec![ValType::Num(NumType::I32)],
                },
                x46_i32_eq => TypingRules {
                    input: vec![ValType::Num(NumType::I32), ValType::Num(NumType::I32)],
                    output: vec![ValType::Num(NumType::I32)],
                },
                x47_i32_ne => todo!(),
                x48_i32_lt_s => todo!(),
                x49_i32_lt_u => todo!(),
                x4a_i32_gt_s => todo!(),
                x4b_i32_gt_u => todo!(),
                x4c_i32_le_s => todo!(),
                x4d_i32_le_u => todo!(),
                x4e_i32_ge_s => todo!(),
                x4f_i32_ge_u => todo!(),
                x50_i64_eqz => todo!(),
                x51_i64_eq => todo!(),
                x52_i64_ne => todo!(),
                x53_i64_lt_s => todo!(),
                x54_i64_lt_u => todo!(),
                x55_i64_gt_s => todo!(),
                x56_i64_gt_u => todo!(),
                x57 => todo!(),
                x58 => todo!(),
                x59 => todo!(),
                x5a_i64_ge_u => todo!(),
                x5b => todo!(),
                x5c => todo!(),
                x5d => todo!(),
                x5e_f32_gt => TypingRules {
                    input: vec![ValType::Num(NumType::F32), ValType::Num(NumType::F32)],
                    output: vec![ValType::Num(NumType::I32)],
                },
                x5f => todo!(),
                x60 => todo!(),
                x61_f64_eq => todo!(),
                x62_f64_ne => todo!(),
                x63_f64_lt => todo!(),
                x64 => todo!(),
                x65 => todo!(),
                x66_f64_ge => todo!(),
                x67_i32_clz => todo!(),
                x68_i32_ctz => TypingRules {
                    input: vec![ValType::Num(NumType::I32)],
                    output: vec![ValType::Num(NumType::I32)],
                },
                x69 => todo!(),
                x6a_i32_add => TypingRules {
                    input: vec![ValType::Num(NumType::I32), ValType::Num(NumType::I32)],
                    output: vec![ValType::Num(NumType::I32)],
                },
                x6b_i32_sub => TypingRules {
                    input: vec![ValType::Num(NumType::I32), ValType::Num(NumType::I32)],
                    output: vec![ValType::Num(NumType::I32)],
                },
                x6c_i32_mul => TypingRules {
                    input: vec![ValType::Num(NumType::I32), ValType::Num(NumType::I32)],
                    output: vec![ValType::Num(NumType::I32)],
                },
                x6d_i32_div_s => todo!(),
                x6e_i32_div_u => todo!(),
                x6f => todo!(),
                x70_i32_rem_u => todo!(),
                x71_i32_and => todo!(),
                x72_i32_or => todo!(),
                x73_i32_xor => todo!(),
                x74_i32_shl => todo!(),
                x75_i32_shr_s => todo!(),
                x76_i32_shr_u => todo!(),
                x77_i32_rotl => todo!(),
                x78 => todo!(),
                x79 => todo!(),
                x7a => todo!(),
                x7c_i64_add => todo!(),
                x7d_i64_sub => todo!(),
                x7e_i64_mul => todo!(),
                x7f_i64_div_s => todo!(),
                x80_i64_div_u => todo!(),
                x81 => todo!(),
                x82 => todo!(),
                x83_i64_and => todo!(),
                x84_i64_or => todo!(),
                x85_i64_xor => todo!(),
                x86_i64_shl => todo!(),
                x87 => todo!(),
                x88_i64_shr_u => todo!(),
                x89 => todo!(),
                x8a => todo!(),
                x8b => todo!(),
                x8c => todo!(),
                x8d => todo!(),
                x8e => todo!(),
                x8f => todo!(),
                x90 => todo!(),
                x91 => todo!(),
                x92 => todo!(),
                x93 => todo!(),
                x94 => todo!(),
                x95 => todo!(),
                x96 => todo!(),
                x97 => todo!(),
                x98 => todo!(),
                x99_f64_abs => todo!(),
                x9a_f64_neg => todo!(),
                x9b => todo!(),
                x9c => todo!(),
                x9d => todo!(),
                x9e => todo!(),
                x9f => todo!(),
                xa0_f64_add => todo!(),
                xa1_f64_sub => todo!(),
                xa2_f64_mul => todo!(),
                xa3 => todo!(),
                xa4 => todo!(),
                xa5 => todo!(),
                xa6 => todo!(),
                xa7_i32_wrap_i64 => todo!(),
                xa8 => todo!(),
                xa9 => todo!(),
                xaa_i32_trunc_f64_s => todo!(),
                xab_i32_trunc_f64_u => todo!(),
                xac_i64_extend_i32_s => todo!(),
                xad_i64_extend_i32_u => todo!(),
                xae => todo!(),
                xaf => todo!(),
                xb0 => todo!(),
                xb1 => todo!(),
                xb2 => todo!(),
                xb3 => todo!(),
                xb4 => todo!(),
                xb5 => todo!(),
                xb6 => todo!(),
                xb7_f64_convert_i32_s => todo!(),
                xb8_f64_convert_i32_u => todo!(),
                xb9 => todo!(),
                xba => todo!(),
                xbb => todo!(),
                xbd_i64_reinterpret_f64 => todo!(),
                xbe => todo!(),
                xbf_f64_reinterpret_i64 => todo!(),
                xc0 => todo!(),
                xc1 => todo!(),
                xc2 => todo!(),
                xc3 => todo!(),
                xc4 => todo!(),
                xc5 => todo!(),
                xc6 => todo!(),
                xc7 => todo!(),
                xc8 => todo!(),
                xc9 => todo!(),
                xca => todo!(),
                xcb => todo!(),
                xcc => todo!(),
                xcd => todo!(),
                xce => todo!(),
                xcf => todo!(),
                xd0_ref_null(_) => todo!(),
                xd1 => todo!(),
                xd2_ref_func(_) => todo!(),
                xd3 => todo!(),
                xd4 => todo!(),
                xd5 => todo!(),
                xd6 => todo!(),
                xd7 => todo!(),
                xd8 => todo!(),
                xd9 => todo!(),
                xda => todo!(),
                xdb => todo!(),
                xdc => todo!(),
                xdd => todo!(),
                xde => todo!(),
                xdf => todo!(),
                xe0 => todo!(),
                xe1 => todo!(),
                xe2 => todo!(),
                xe3 => todo!(),
                xe4 => todo!(),
                xe5 => todo!(),
                xe6 => todo!(),
                xe7 => todo!(),
                xe8 => todo!(),
                xe9 => todo!(),
                xea => todo!(),
                xeb => todo!(),
                xec => todo!(),
                xed => todo!(),
                xee => todo!(),
                xef => todo!(),
                xf0 => todo!(),
                xf1 => todo!(),
                xf2 => todo!(),
                xf3 => todo!(),
                xf4 => todo!(),
                xf5 => todo!(),
                xf6 => todo!(),
                xf7 => todo!(),
                xf8 => todo!(),
                xf9 => todo!(),
                xfa => todo!(),
                xfb => todo!(),
                xfc_0_i32_trunc_sat_f32_s => todo!(),
                xfc_1_i32_trunc_sat_f32_u => todo!(),
                xfc_2_i32_trunc_sat_f64_u => todo!(),
                xfc_3_i32_trunc_sat_f64_s => todo!(),
                xfc_4_i64_trunc_sat_f32_s => todo!(),
                xfc_5_i64_trunc_sat_f32_u => todo!(),
                xfc_6_i64_trunc_sat_f64_s => todo!(),
                xfc_7_i64_trunc_sat_f64_u => todo!(),
                xfc_8_memory_init(_) => todo!(),
                xfc_9_data_drop(_) => todo!(),
                xfc_10_memory_copy(_, _) => todo!(),
                xfc_11_memory_fill(_) => todo!(),
                xfd => todo!(),
                xfe => todo!(),
                xff => todo!(),
                block_start(_, _, _) => todo!(),
                block_end(_, _, _) => todo!(),
                comment(_, _) => todo!(),
            }
        };
        // println!(
        //     "\n{inst:?} ({input:?}, {output:?})\nlocals: {locals:?}\ncontext: {context:?}"
        // );
        for inp in input {
            let Some(p) = context.pop() else {
                return Err(TypeCheckError::EmptyStack);
            };
            if p != inp && !matches!(inp, ValType::Nil) {
                return Err(TypeCheckError::WrongTypeOnStack);
            }
        }
        for out in output {
            // println!("        push: {out:?}");
            context.push(out);
        }
    }
    // println!()

    if let Some(return_types) = return_types {
        // println!("{return_types:?} {context:?}");
        if return_types != context {
            return Err(TypeCheckError::ReturnTypeMismatch(return_types, context));
        }
    }

    Ok(context)
}
