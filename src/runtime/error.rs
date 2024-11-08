#[allow(unused)]
pub enum RuntimeError {
    ParseError(String),
    Exit(i32),
    GlobalWithoutValue,
    ActiveDataWithoutOffset,
    UnknownFunction(String, String),
    NoFrame(&'static str, u32, u32),
    WrongType(&'static str, u32, u32),
    EmptyStack(&'static str, u32, u32),
    Impossible(&'static str, u32, u32),
    MissingLocal(&'static str, u32, u32),
    MissingFunction(&'static str, u32, u32),
    MissingJumpLabel(&'static str, u32, u32),
    MissingTableIndex(&'static str, u32, u32),
    OutOfBoundsMemoryAccess,
}

impl std::fmt::Debug for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OutOfBoundsMemoryAccess => write!(f, "out of bounds memory access"),
            Self::ParseError(string) => write!(f, "ParseError({string})"),
            Self::Exit(arg0) => f.debug_tuple("Exit").field(arg0).finish(),
            Self::GlobalWithoutValue => write!(f, "GlobalWithoutOffset"),
            Self::ActiveDataWithoutOffset => write!(f, "ActiveDataWithoutOffset"),
            Self::UnknownFunction(arg0, arg1) => write!(f, "unknown function: {arg0}::{arg1}"),
            Self::NoFrame(arg0, arg1, arg2) => {
                write!(f, "ran out of stack frames: {arg0}:{arg1}:{arg2}")
            }
            Self::WrongType(arg0, arg1, arg2) => {
                write!(f, "wrong type popped from stack: {arg0}:{arg1}:{arg2}")
            }
            Self::EmptyStack(arg0, arg1, arg2) => {
                write!(f, "empty stack: {arg0}:{arg1}:{arg2}")
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
            Self::MissingJumpLabel(arg0, arg1, arg2) => {
                write!(f, "missing jump label: {arg0}:{arg1}:{arg2}")
            }
            Self::MissingTableIndex(arg0, arg1, arg2) => {
                write!(f, "missing table index: {arg0}:{arg1}:{arg2}")
            }
        }
    }
}
