use crate::parser::{
    BlockType, FuncIdx, FuncType, GlobalIdX,
    Instr::{self, *},
    LabelIdX, LocalIdX, NumType, TypeIdX, ValType,
};
#[derive(Debug)]
#[allow(unused)]
pub struct TypingRules {
    pub input: Vec<ValType>,
    pub output: Vec<ValType>,
}
// This could be a function but hihi
macro_rules! t {
    () => {
        TypingRules {
            input: Vec::new(),
            output: Vec::new(),
        }
    };
    (() -> $($output:tt),*) => {
        TypingRules {
            input: [].into(),
            output: [$(ty_to_val!($output), )*].into(),
        }
    };
    ($($input:tt),* -> ()) => {
        TypingRules {
            input: [$(ty_to_val!($input), )*].into(),
            output: [].into(),
        }
    };
    ($($input:tt),* -> $($output:tt),*) => {
        TypingRules {
            input: [$(ty_to_val!($input), )*].into(),
            output: [$(ty_to_val!($output), )*].into(),
        }
    };

    (($e:expr => $($output:tt),*)) => {
        TypingRules {
            input: $e.into(),
            output: [$(ty_to_val!($input), )*].into(),
        }
    };
    (($($input:tt),*) => $e:expr) => {
        TypingRules {
            input: [$(ty_to_val!($input), )*].into(),
            output: $e.into(),
        }
    };

    ($a:expr => $b:expr) => {
        TypingRules {
            input: $a.into(),
            output: $b.into(),
        }
    };
}
#[rustfmt::skip]
macro_rules! ty_to_val {
    (i32) =>    {ValType::Num(NumType::I32)};
    (i64) =>    {ValType::Num(NumType::I64)};
    (f32) =>    {ValType::Num(NumType::F32)};
    (f64) =>    {ValType::Num(NumType::F64)};
    (T)   =>    {ValType::Poly};
    (Vec128) => {ValType::Vec128};
    (Fn*)    => {ValType::Ref(RefTyp::FuncRef)};
    (Ex*)    => {ValType::Ref(RefTyp::ExternRef)};
    ([$e:expr]) => {$e}
}

#[derive(Debug)]
#[allow(unused)]
pub enum TypeCheckError {
    WrongTypeOnStack,
    EmptyStack,
    MissingFunction,
    MissingType,
    IfElseTypeMismatch(Vec<ValType>, Vec<ValType>),
    ReturnTypeMismatch(Vec<ValType>, Vec<ValType>),
    MissingLocal,
    UnknownFunction,
    UnknownTable,
    UnknownType,
}

#[allow(unused)]
pub fn check(
    mut context: Vec<ValType>,
    locals: &[ValType],
    instrs: &[Instr],
    function_types: &[FuncType],
    raw_types: &[FuncType],
    globals: &[ValType],
    return_types: Option<Vec<ValType>>,
) -> Result<Vec<ValType>, TypeCheckError> {
    let mut polymorphic_stack = None;
    // println!("{instrs:#?}");
    // println!("ts: {instrs:#?}");
    for inst in instrs {
        // println!("    {inst:?}:\n    locals: {locals:?}\n    context: {context:?}");
        let TypingRules { input, output } = {
            // println!(" @ {inst:?}");
            match &inst {
                x00_unreachable => {
                    polymorphic_stack = Some(context.clone());
                    t!()
                }
                x01_nop => t! {},
                x02_block(bt, b) | x03_loop(bt, b) => {
                    let (rt, inputs) = match bt {
                        BlockType::Eps => (t! {}, Vec::new()),
                        BlockType::T(val_type) => (t!(() => [*val_type]), Vec::new()),
                        BlockType::TypIdx(i) => {
                            let ft = raw_types
                                .get(*i as usize)
                                .ok_or(TypeCheckError::MissingFunction)?;
                            (t!(() => ft.output.types.clone()), ft.input.types.clone())
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
                    let mut r = check(
                        pass,
                        locals,
                        b,
                        function_types,
                        raw_types,
                        globals,
                        Some(rt.output.clone()),
                    )?;
                    context.append(&mut r);
                    if let Some(return_types) = &return_types {
                        // println!(" - Block return: {return_types:?} {context:?}");
                        if return_types != &context {
                            return Err(TypeCheckError::ReturnTypeMismatch(
                                return_types.to_vec(),
                                context,
                            ));
                        }
                    }
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
                        t!((i32) => a)
                    } else {
                        t!(i32 -> ())
                    }
                }
                x05 => todo!(),
                x06 => todo!(),
                x07 => todo!(),
                x08 => todo!(),
                x09 => todo!(),
                x0a => todo!(),
                x0b => todo!(),
                x0c_br(LabelIdX(_)) => {
                    polymorphic_stack = Some(context.clone());
                    t!()
                    // if let Some(return_types) = return_types {
                    //     // println!("{return_types:?} {context:?}");
                    //     if return_types != context {
                    //         return Err(TypeCheckError::ReturnTypeMismatch(return_types, context));
                    //     }
                    // }
                    // return Ok((*i as usize, context));
                }
                x0d_br_if(_) => t!(i32 -> ()),
                x0e_br_table(_, _) => t!(i32 -> ()),
                x0f_return => t!(),
                x10_call(FuncIdx(i)) => {
                    let ft = function_types
                        .get(*i as usize)
                        .ok_or(TypeCheckError::MissingFunction)?;
                    t!(ft.input.types.clone() => ft.output.types.clone())
                }
                x11_call_indirect(TypeIdX(i), _) => {
                    let ft = raw_types
                        .get(*i as usize)
                        .ok_or(TypeCheckError::MissingFunction)?;
                    let mut input = vec![ValType::Num(NumType::I32)];
                    input.append(&mut ft.input.types.clone());
                    t!(input => ft.output.types.clone())
                }
                x12 => todo!(),
                x13 => todo!(),
                x14 => todo!(),
                x15 => todo!(),
                x16 => todo!(),
                x17 => todo!(),
                x18 => todo!(),
                x19 => todo!(),
                x1a_drop => t!(T -> ()),
                x1b_select => {
                    if polymorphic_stack.is_none() {
                        let top = context.pop().ok_or(TypeCheckError::EmptyStack)?;
                        if top != ValType::Num(NumType::I32) {
                            return Err(TypeCheckError::WrongTypeOnStack);
                        }

                        let input = [
                            context.pop().ok_or(TypeCheckError::EmptyStack)?,
                            context.pop().ok_or(TypeCheckError::EmptyStack)?,
                        ];
                        t!([] => [input[0]])
                    } else {
                        t!()
                    }
                }
                x1c => todo!(),
                x1d => todo!(),
                x1e => todo!(),
                x1f => todo!(),
                x20_local_get(LocalIdX(i)) => locals
                    .get(*i as usize)
                    .map(|l| t!([] => [*l]))
                    .ok_or(TypeCheckError::MissingLocal)?,
                x21_local_set(LocalIdX(i)) => t!([locals[*i as usize]] => []),
                x22_local_tee(LocalIdX(i)) => t!([locals[*i as usize]] => [locals[*i as usize]]),
                x23_global_get(GlobalIdX(i)) => t!([] => [globals[*i as usize]]),
                x24_global_set(GlobalIdX(i)) => t!([globals[*i as usize]] => []),
                x25 => todo!(),
                x26_table_set(_) => todo!(),
                x27 => todo!(),
                x28_i32_load(_) => t!(i32 -> i32),
                x29_i64_load(_) => t!(i32 -> i64),
                x2a_f32_load(_) => t!(i32 -> f32),
                x2b_f64_load(_) => t!(i32 -> f64),
                x2c_i32_load8_s(_) => t!(i32 -> i32),
                x2d_i32_load8_u(_) => t!(i32 -> i32),
                x2e_i32_load16_s(_) => t!(i32 -> i32),
                x2f_i32_load16_u(_) => t!(i32 -> i32),
                x30_i64_load8_s(_) => t!(i32 -> i64),
                x31_i64_load8_u(_) => t!(i32 -> i64),
                x32_i64_load16_s(_) => t!(i32 -> i64),
                x33_i64_load16_u(_) => t!(i32 -> i64),
                x34_i64_load32_s(_) => t!(i32 -> i64),
                x35_i64_load32_u(_) => t!(i32 -> i64),
                x36_i32_store(_) => t!(i32, i32 -> ()),
                x37_i64_store(_) => t!(i64, i32 -> ()),
                x38_f32_store(_) => t!(f32, i32 -> ()),
                x39_f64_store(_) => t!(f64, i32 -> ()),
                x3a_i32_store8(_) => t!(i32, i32 -> ()),
                x3b_i32_store16(_) => t!(i32, i32 -> ()),
                x3c_i64_store8(_) => t!(i64, i32 -> ()),
                x3d_i64_store16(_) => t!(i64, i32 -> ()),
                x3e_i64_store32(_) => t!(i64, i32 -> ()),
                x3f => todo!(),
                x40_grow => t!(i32 -> i32),
                x41_i32_const(_) => t!(() -> i32),
                x42_i64_const(_) => t!(() -> i64),
                x43_f32_const(_) => t!(() -> f32),
                x44_f64_const(_) => t!(() -> f64),
                x45_i32_eqz => t!(i32 -> i32),
                x46_i32_eq => t!(i32, i32 -> i32),
                x47_i32_ne => t!(i32, i32 -> i32),
                x48_i32_lt_s => t!(i32, i32 -> i32),
                x49_i32_lt_u => t!(i32, i32 -> i32),
                x4a_i32_gt_s => t!(i32, i32 -> i32),
                x4b_i32_gt_u => t!(i32, i32 -> i32),
                x4c_i32_le_s => t!(i32, i32 -> i32),
                x4d_i32_le_u => t!(i32, i32 -> i32),
                x4e_i32_ge_s => t!(i32, i32 -> i32),
                x4f_i32_ge_u => t!(i32, i32 -> i32),
                x50_i64_eqz => t!(i64 -> i32),
                x51_i64_eq => t!(i64, i64 -> i32),
                x52_i64_ne => t!(i64, i64 -> i32),
                x53_i64_lt_s => t!(i64, i64 -> i32),
                x54_i64_lt_u => t!(i64, i64 -> i32),
                x55_i64_gt_s => t!(i64, i64 -> i32),
                x56_i64_gt_u => t!(i64, i64 -> i32),
                x57_i64_le_s => todo!(),
                x58_i64_le_u => t!(i64, i64 -> i32),
                x59_i64_ge_s => todo!(),
                x5a_i64_ge_u => t!(i64, i64 -> i32),
                x5b_f32_eq => t!(f32, f32 -> i32),
                x5c_f32_ne => t!(f32, f32 -> i32),
                x5d_f32_lt => todo!(),
                x5e_f32_gt => t!(f32, f32 -> i32),
                x5f_f32_le => t!(f32, f32 -> i32),
                x60_f32_ge => todo!(),
                x61_f64_eq => t!(f64, f64 -> i32),
                x62_f64_ne => t!(f64, f64 -> i32),
                x63_f64_lt => t!(f64, f64 -> i32),
                x64_f64_gt => todo!(),
                x65_f64_le => t!(f64, f64 -> i32),
                x66_f64_ge => t!(f64, f64 -> i32),
                x67_i32_clz => t!(i32 -> i32),
                x68_i32_ctz => t!(i32 -> i32),
                x69_i32_popcnt => todo!(),
                x6a_i32_add => t!(i32, i32 -> i32),
                x6b_i32_sub => t!(i32, i32 -> i32),
                x6c_i32_mul => t!(i32, i32 -> i32),
                x6d_i32_div_s => t!(i32, i32 -> i32),
                x6e_i32_div_u => t!(i32, i32 -> i32),
                x6f_i32_rem_s => todo!(),
                x70_i32_rem_u => t!(i32, i32 -> i32),
                x71_i32_and => todo!(),
                x72_i32_or => todo!(),
                x73_i32_xor => todo!(),
                x74_i32_shl => todo!(),
                x75_i32_shr_s => todo!(),
                x76_i32_shr_u => todo!(),
                x77_i32_rotl => todo!(),
                x78_i32_rotr => todo!(),
                x79_i64_clz => todo!(),
                x7a_i64_ctz => t!(i64 -> i64),
                x7b_i64_popcnt => t!(i64 -> i64),
                x7c_i64_add => t!(i64, i64 -> i64),
                x7d_i64_sub => t!(i64, i64 -> i64),
                x7e_i64_mul => t!(i64, i64 -> i64),
                x7f_i64_div_s => t!(i64, i64 -> i64),
                x80_i64_div_u => t!(i64, i64 -> i64),
                x81_i64_rem_s => todo!(),
                x82_i64_rem_u => todo!(),
                x83_i64_and => todo!(),
                x84_i64_or => todo!(),
                x85_i64_xor => todo!(),
                x86_i64_shl => todo!(),
                x87_i64_shr_s => todo!(),
                x88_i64_shr_u => todo!(),
                x89_i64_rotl => todo!(),
                x8a_i64_rotr => todo!(),
                x8b_f32_abs => todo!(),
                x8c_f32_neg => t!(f32 -> f32),
                x8d_f32_ceil => todo!(),
                x8e_f32_floor => todo!(),
                x8f_f32_trunc => todo!(),
                x90_f32_nearest => todo!(),
                x91_f32_sqrt => t!(f32 -> f32),
                x92_f32_add => t!(f32, f32 -> f32),
                x93_f32_sub => t!(f32, f32 -> f32),
                x94_f32_mul => t!(f32, f32 -> f32),
                x95_f32_div => todo!(),
                x96_f32_min => todo!(),
                x97_f32_max => todo!(),
                x98_f32_copysign => todo!(),
                x99_f64_abs => t!(f64 -> f64),
                x9a_f64_neg => t!(f64 -> f64),
                x9b_f64_ceil => todo!(),
                x9c_f64_floor => todo!(),
                x9d => todo!(),
                x9e_f64_nearest => todo!(),
                x9f_f64_sqrt => todo!(),
                xa0_f64_add => t!(f64, f64 -> f64),
                xa1_f64_sub => t!(f64, f64 -> f64),
                xa2_f64_mul => t!(f64, f64 -> f64),
                xa3_f64_div => todo!(),
                xa4_f64_min => todo!(),
                xa5_f64_max => todo!(),
                xa6_f64_copysign => todo!(),
                xa7_i32_wrap_i64 => t!(i64 -> i32),
                xa8_i32_trunc_f32_s => t!(f32 -> i32),
                xa9_i32_trunc_f32_u => t!(f32 -> i32),
                xaa_i32_trunc_f64_s => t!(f64 -> i32),
                xab_i32_trunc_f64_u => t!(f64 -> i32),
                xac_i64_extend_i32_s => t!(i32 -> i64),
                xad_i64_extend_i32_u => t!(i32 -> i64),
                xae_i64_trunc_f32_s => t!(f32 -> i64),
                xaf_i64_trunc_f32_u => t!(f32 -> i64),
                xb0_i64_trunc_f64_s => t!(f64 -> i64),
                xb1_i64_trunc_f64_u => t!(f64 -> i64),
                xb2_f32_convert_i32_s => t!(i32 -> f32),
                xb3_f32_convert_i32_u => t!(i32 -> f32),
                xb4_f32_convert_i64_s => t!(i64 -> f32),
                xb5_f32_convert_i64_u => t!(i64 -> f32),
                xb6_f32_demote_f64 => t!(f64 -> f32),
                xb7_f64_convert_i32_s => t!(i32 -> f64),
                xb8_f64_convert_i32_u => t!(i32 -> f64),
                xb9_f64_convert_i64_s => t!(i64 -> f64),
                xba_f64_convert_i64_u => t!(i64 -> f64),
                xbb_f64_promote_f32 => t!(f32 -> f64),
                xbc_i32_reinterpret_f32 => t!(f32 -> i32),
                xbd_i64_reinterpret_f64 => t!(f64 -> i32),
                xbe_f32_reinterpret_i32 => t!(i32 -> f32),
                xbf_f64_reinterpret_i64 => t!(i64 -> f64),
                xc0_i32_extend8_s => todo!(),
                xc1_i32_extend16_s => todo!(),
                xc2_i64_extend8_s => todo!(),
                xc3_i64_extend16_s => todo!(),
                xc4_i64_extend32_s => todo!(),
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
                xfc_0_i32_trunc_sat_f32_s => t!(f32 -> i32),
                xfc_1_i32_trunc_sat_f32_u => t!(f32 -> i32),
                xfc_2_i32_trunc_sat_f64_s => t!(f64 -> i32),
                xfc_3_i32_trunc_sat_f64_u => t!(f64 -> i32),
                xfc_4_i64_trunc_sat_f32_s => t!(f32 -> i64),
                xfc_5_i64_trunc_sat_f32_u => t!(f32 -> i64),
                xfc_6_i64_trunc_sat_f64_s => t!(f64 -> i64),
                xfc_7_i64_trunc_sat_f64_u => t!(f64 -> i64),
                xfc_8_memory_init(_, _) => t!(i32, i32, i32 -> ()),
                xfc_9_data_drop(_) => t!(),
                xfc_10_memory_copy(_, _) => t!(i32, i32, i32 -> ()),
                xfc_11_memory_fill(_) => t!(i32, i32, i32 -> ()),
                xfc_12_table_init(_, _) => t!(i32, i32, i32 -> ()),
                xfc_13_elem_drop(_) => t!(),
                xfc_14_table_copy(_, _) => t!(i32, i32, i32 -> ()),
                xfc_15_table_grow(_) => todo!(),
                xfc_16_table_size(_) => todo!(),
                xfc_17_table_fill(_) => todo!(),
                xfd => todo!(),
                xfe => todo!(),
                xff => todo!(),
                block_start(_, _, _) => todo!(),
                block_end(_, _, _) => todo!(),
                comment(_, _) => todo!(),
                if_then_else(_) => todo!(),
                else_jump(_) => todo!(),
            }
        };
        // println!(
        //     "\n{inst:?} ({input:?}, {output:?})\nlocals: {locals:?}\ncontext: {context:?}"
        // );
        for inp in input {
            let p = match context.pop() {
                Some(p) => p,
                None if polymorphic_stack.is_some() => inp,
                None => return Err(TypeCheckError::EmptyStack),
            };
            if p != inp && !matches!(inp, ValType::Poly) {
                return Err(TypeCheckError::WrongTypeOnStack);
            }
        }
        for out in output {
            // println!("        push: {out:?}");
            context.push(out);
        }
    }
    // println!()

    // println!("rs: {context:?}");
    // println!(" - End return {return_types:?} {polymorphic_stack:?}");
    if let Some(return_types) = return_types {
        if let Some(polymorphic_stack) = polymorphic_stack {
            if return_types != polymorphic_stack {
                return Err(TypeCheckError::ReturnTypeMismatch(return_types, context));
            }
            return Ok(polymorphic_stack);
        }
        // println!("{return_types:?} {context:?}");
        if return_types != context {
            return Err(TypeCheckError::ReturnTypeMismatch(return_types, context));
        }
    }

    Ok(context)
}
