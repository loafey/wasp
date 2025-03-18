use crate::runtime::{clean_model::Function, Frame, FuncId, Runtime, RuntimeError};

impl Runtime {
    pub fn execute_start(&mut self) -> Result<(), RuntimeError> {
        if let Some(module) = self.modules.get("_$_main_$_") {
            let ws = unsafe { module.as_ws() };
            // If a start function exists, run it
            if let Some(start) = ws.start {
                let mut local_n = 0;
                let locals = ws
                    .functions
                    .get(*start as usize)
                    .and_then(|v| match v.as_ref() {
                        Function::WS { locals, .. } => Some(
                            locals
                                .iter()
                                .flat_map(move |l| {
                                    local_n += 1;
                                    (0..l.n).map(move |i| (local_n + i - 1, l.t.default_value()))
                                })
                                .collect(),
                        ),
                        Function::IO { .. } => None,
                    })
                    .unwrap_or_default();
                self.stack.push(Frame {
                    func_id: FuncId::Id(*start),
                    pc: 0,
                    module: "_$_main_$_".to_string(),
                    stack: Vec::new(),
                    locals,
                    depth_stack: Vec::new(),
                });
                loop {
                    match self.step() {
                        Ok(_) => (),
                        Err(RuntimeError::NoFrame(_, _, _)) => break,
                        Err(e) => return Err(e),
                    }
                }
            }
        }
        Ok(())
    }
}
