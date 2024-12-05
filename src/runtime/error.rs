use super::{typecheck::TypeCheckError, Value};

#[allow(unused)]
pub enum RuntimeError {
    ParseError(String),
    Exit(i32),
    GlobalWithoutValue,
    ActiveDataWithoutOffset,
    UnknownFunction(String, String),
    ReturnedToNoFrame(Vec<Value>, &'static str, u32, u32),
    UnknownImport(&'static str, u32, u32),
    IncompatibleImportType(&'static str, u32, u32),
    NoFrame(&'static str, u32, u32),
    NoModule(String, &'static str, u32, u32),
    MissingGlobal(&'static str, u32, u32),
    WrongType(&'static str, &'static str, &'static str, u32, u32),
    EmptyStack(&'static str, u32, u32),
    Unreachable(&'static str, u32, u32),
    Impossible(&'static str, u32, u32),
    MissingLocal(&'static str, u32, u32),
    MissingFunction(&'static str, u32, u32),
    MissingFunctionImport(String, &'static str, u32, u32),
    MissingJumpLabel(&'static str, u32, u32),
    MissingTableIndex(&'static str, u32, u32),
    OutOfBoundsTableAccess(&'static str, u32, u32),
    MissingData(&'static str, u32, u32),
    DataInitOutOfRange(&'static str, u32, u32),
    MissingElementIndex(&'static str, u32, u32),
    MissingType(&'static str, u32, u32),
    TypeError(TypeCheckError),
    UninitializedElement(&'static str, u32, u32),
    UndefinedElement(&'static str, u32, u32),
    IndirectCallTypeMismatch(&'static str, u32, u32),
    IntegerOverflow(&'static str, u32, u32),
    InvalidConversionToInteger(&'static str, u32, u32),
    IntegerDivideByZero(&'static str, u32, u32),
    MultipleMemories(&'static str, u32, u32),
    UnknownLabel,
    UnknownGlobal,
    UnknownMemory,
    OutOfBoundsMemoryAccess,
    StackExhaustion(usize, usize),
}

impl From<TypeCheckError> for RuntimeError {
    fn from(value: TypeCheckError) -> Self {
        Self::TypeError(value)
    }
}

impl std::fmt::Debug for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownGlobal => write!(f, "unknown global"),
            Self::UnknownMemory => write!(f, "unknown memory"),
            Self::StackExhaustion(cur, max) => write!(f, "stack overflow ({cur}/{max})"),
            Self::IndirectCallTypeMismatch(..) => write!(f, "indirect call type mismatch"),
            Self::NoModule(module, arg0, arg1, arg2) => {
                write!(
                    f,
                    "tried to use unloaded module \"{module}\": {arg0}:{arg1}:{arg2}"
                )
            }
            Self::IntegerOverflow(arg0, arg1, arg2) => {
                write!(f, "integer overflow: {arg0}:{arg1}:{arg2}")
            }
            Self::IntegerDivideByZero(arg0, arg1, arg2) => {
                write!(f, "integer divide by zero: {arg0}:{arg1}:{arg2}")
            }
            Self::MultipleMemories(arg0, arg1, arg2) => {
                write!(f, "multiple memories: {arg0}:{arg1}:{arg2}")
            }
            Self::IncompatibleImportType(arg0, arg1, arg2) => {
                write!(f, "incompatible import type: {arg0}:{arg1}:{arg2}")
            }
            Self::UnknownImport(arg0, arg1, arg2) => {
                write!(f, "unknown import: {arg0}:{arg1}:{arg2}")
            }
            Self::MissingData(arg0, arg1, arg2) => {
                write!(f, "tried to get non-existent data: {arg0}:{arg1}:{arg2}")
            }
            Self::TypeError(t) => write!(f, "type error: {t:?}"),
            Self::OutOfBoundsMemoryAccess => write!(f, "out of bounds memory access"),
            Self::ParseError(string) => write!(f, "ParseError({string})"),
            Self::Exit(arg0) => f.debug_tuple("Exit").field(arg0).finish(),
            Self::GlobalWithoutValue => write!(f, "GlobalWithoutOffset"),
            Self::ActiveDataWithoutOffset => write!(f, "ActiveDataWithoutOffset"),
            Self::UnknownFunction(arg0, arg1) => write!(f, "unknown function: {arg0}::{arg1}"),
            Self::ReturnedToNoFrame(stack, arg0, arg1, arg2) => {
                write!(
                    f,
                    "returned, but no more frames ({stack:?}): {arg0}:{arg1}:{arg2}"
                )
            }
            Self::MissingType(arg0, arg1, arg2) => {
                write!(f, "missing type: {arg0}:{arg1}:{arg2}")
            }
            Self::MissingGlobal(arg0, arg1, arg2) => {
                write!(f, "missing global: {arg0}:{arg1}:{arg2}")
            }
            Self::NoFrame(arg0, arg1, arg2) => {
                write!(f, "ran out of stack frames: {arg0}:{arg1}:{arg2}")
            }
            Self::WrongType(arg0, exp, got, arg1, arg2) => {
                write!(f, "wrong type popped from stack (got {got}, expected {exp}): {arg0}:{arg1}:{arg2}")
            }
            Self::EmptyStack(arg0, arg1, arg2) => {
                write!(f, "empty stack: {arg0}:{arg1}:{arg2}")
            }
            Self::Unreachable(arg0, arg1, arg2) => {
                write!(f, "hit an unreachable code segment: {arg0}:{arg1}:{arg2}")
            }
            Self::Impossible(arg0, arg1, arg2) => {
                write!(f, "an impossible case happened: {arg0}:{arg1}:{arg2}")
            }
            Self::MissingLocal(arg0, arg1, arg2) => {
                write!(f, "a local is missing: {arg0}:{arg1}:{arg2}")
            }
            Self::MissingFunction(arg0, arg1, arg2) => {
                write!(f, "missing function index: {arg0}:{arg1}:{arg2}")
            }
            Self::MissingFunctionImport(name, arg0, arg1, arg2) => {
                write!(f, "missing function {name:?}: {arg0}:{arg1}:{arg2}")
            }
            Self::MissingJumpLabel(arg0, arg1, arg2) => {
                write!(f, "missing jump label: {arg0}:{arg1}:{arg2}")
            }
            Self::MissingTableIndex(arg0, arg1, arg2) => {
                write!(f, "missing table index: {arg0}:{arg1}:{arg2}")
            }
            Self::MissingElementIndex(arg0, arg1, arg2) => {
                write!(f, "missing element vector index: {arg0}:{arg1}:{arg2}")
            }
            Self::DataInitOutOfRange(_, _, _) => {
                write!(f, "out of bounds memory access")
            }
            Self::OutOfBoundsTableAccess(_, _, _) => {
                write!(f, "out of bounds table access")
            }
            Self::UnknownLabel => {
                write!(f, "unknown label")
            }
            Self::UndefinedElement(_arg0, _arg1, _arg2) => {
                write!(f, "undefined element")
            }
            Self::UninitializedElement(_arg0, _arg1, _arg2) => {
                write!(f, "uninitialized element")
            }
            Self::InvalidConversionToInteger(_arg0, _arg1, _arg2) => {
                write!(f, "invalid conversion to integer")
            }
        }
    }
}
