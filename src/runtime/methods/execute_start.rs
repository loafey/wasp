use std::collections::HashMap;

use crate::runtime::{Frame, FuncId, Runtime, RuntimeError};

impl Runtime {
    pub fn execute_start(&mut self) -> Result<(), RuntimeError> {
        if let Some(module) = self.modules.get("_$_main_$_") {
            let ws = unsafe { module.as_ws() };
            // If a start function exists, run it
            if let Some(start) = ws.start {
                self.stack.push(Frame {
                    func_id: FuncId::Id(*start),
                    pc: 0,
                    module: "_$_main_$_".to_string(),
                    stack: Vec::new(),
                    locals: HashMap::new(),
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
