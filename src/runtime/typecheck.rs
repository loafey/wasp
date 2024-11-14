use crate::parser::{FuncType, Instr, Locals, NumType, TypingRuleError, TypingRules, ValType};

#[derive(Debug)]
pub enum TypeCheckError {
    WrongTypeOnStack,
    EmptyStack,
    MissingFunction,
    GetTypeError(TypingRuleError),
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
) -> Result<(), TypeCheckError> {
    let mut context = Vec::new();
    for inst in instrs {
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
                println!("{inst:?} {locals:?}");
                let TypingRules { input, output } = inst.get_types(locals, function_types)?;
                for inp in input {
                    let Some(p) = context.pop() else {
                        return Err(TypeCheckError::EmptyStack);
                    };
                    if p != inp && !matches!(inp, ValType::Nil) {
                        return Err(TypeCheckError::WrongTypeOnStack);
                    }
                }
                for out in output {
                    context.push(out);
                }
            }
        }
    }

    Ok(())
}
