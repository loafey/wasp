use crate::parser::{FuncType, Instr, Locals, NumType, TypingRuleError, TypingRules, ValType};

#[derive(Debug)]
pub enum TypeCheckError {
    WrongTypeOnStack,
    EmptyStack,
    MissingFunction,
    GetTypeError(TypingRuleError),
    ReturnTypeMismatch(Vec<ValType>, Vec<ValType>),
}
impl From<TypingRuleError> for TypeCheckError {
    fn from(value: TypingRuleError) -> Self {
        TypeCheckError::GetTypeError(value)
    }
}

pub fn check(
    locals: &[ValType],
    instrs: &[Instr],
    function_types: &[FuncType],
    raw_types: &[FuncType],
    globals: &[ValType],
    return_types: Option<Vec<ValType>>,
) -> Result<(), TypeCheckError> {
    let mut context = Vec::new();
    // println!("{instrs:#?}");
    for inst in instrs {
        // println!("    {inst:?}:\n    locals: {locals:?}\n    context: {context:?}");
        match inst {
            Instr::x1b_select => {
                let top = context.pop().ok_or(TypeCheckError::EmptyStack)?;
                if top != ValType::Num(NumType::I32) {
                    return Err(TypeCheckError::WrongTypeOnStack);
                }

                let input = [
                    context.pop().ok_or(TypeCheckError::EmptyStack)?,
                    context.pop().ok_or(TypeCheckError::EmptyStack)?,
                ];
                context.push(input[0]);
            }
            inst => {
                let TypingRules { input, output } =
                    inst.get_types(locals, function_types, raw_types, globals)?;
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
        }
        // println!()
    }

    if let Some(return_types) = return_types {
        println!("{return_types:?} {context:?}");
        if return_types != context {
            return Err(TypeCheckError::ReturnTypeMismatch(return_types, context));
        }
    }

    Ok(())
}
