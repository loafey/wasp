#![feature(const_type_name)]

use egui::{Id, Vec2};
use hex::Hex;
use parser::{Instr, Module, Parsable};
use runtime::{clean_model::Function, Runtime};
use std::{
    env::args,
    io::Cursor,
    mem::MaybeUninit,
    time::{Duration, Instant},
};
mod hex;
mod parser;
mod runtime;

#[allow(unused_imports)]
#[macro_use]
extern crate log;

fn alloc<const N: usize>() -> Hex<N> {
    #[allow(clippy::uninit_assumed_init)]
    #[allow(invalid_value)]
    unsafe {
        MaybeUninit::uninit().assume_init()
    }
}

fn runtime() -> Runtime {
    let bin: &[u8] = include_bytes!("../examples/c_addition.wasm");
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
            std::process::exit(1);
        }
    };
    Runtime::new(module)
}

struct App {
    runtime: Runtime,
    current_frame: usize,
    frame_count: usize,
    last_tick: Instant,
    frame_duration: f64,
    auto: bool,
}
impl App {
    pub fn new(_xcc: &eframe::CreationContext<'_>) -> Self {
        Self {
            runtime: runtime(),
            current_frame: 0,
            frame_count: 1,
            frame_duration: 0.05, //0.5,
            last_tick: Instant::now(),
            auto: false,
        }
    }
}
impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.auto && self.last_tick.elapsed().as_secs_f64() > self.frame_duration {
            self.last_tick = Instant::now();
            self.runtime.step();
            self.current_frame = self.runtime.stack.len() - 1;
        }

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            if ui.button("Step").clicked() {
                self.runtime.step();
                if self.frame_count != self.runtime.stack.len() {
                    self.current_frame = self.runtime.stack.len() - 1;
                }
                self.frame_count = self.runtime.stack.len();
            }

            if ui.button("Auto").clicked() {
                self.auto = !self.auto;
            }
        });

        egui::SidePanel::new(egui::panel::Side::Left, Id::new("side_panel")).show(ctx, |ui| {
            ui.heading("Stack frames:");
            let width = ui.max_rect().width();
            for i in (0..self.runtime.stack.len()).rev() {
                let button =
                    egui::Button::new(format!("Frame {i:#?}")).min_size(Vec2::new(width, 16.0));
                if ui.add(button).clicked() {
                    self.current_frame = i;
                }
            }
        });

        egui::SidePanel::new(egui::panel::Side::Left, Id::new("stack_info")).show(ctx, |ui| {
            ui.heading("Frame info:");
            let text = &self.runtime.stack[self.current_frame];
            // if text.func_id == 3 {
            //     self.auto = false;
            // }

            if let Function::Local { labels, code, .. } =
                &self.runtime.module.functions[&text.func_id]
            {
                if matches!(
                    &code[..],
                    [
                        Instr::x23_global_get(_),
                        Instr::x41_i32_const(_),
                        Instr::x6b_i32_sub,
                        ..
                    ]
                ) {
                    self.auto = false;
                }
                let label = egui::Label::new(format!("Labels: {labels:#?}")).extend();
                ui.add(label);
            }

            let label = egui::Label::new(format!("{text:#?}")).extend();
            ui.add(label);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Instructions");
            ui.with_layout(
                egui::Layout::top_down(egui::Align::Min).with_cross_justify(true),
                |ui| {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        let pc = self.runtime.stack[self.current_frame].pc;
                        let func = self.runtime.stack[self.current_frame].func_id;

                        let func = self.runtime.module.functions.get(&func).unwrap();
                        let Function::Local { code, .. } = func else {
                            return;
                        };
                        let var_name = format!("{}:", code.len());
                        let var_name = &var_name;
                        let longest = var_name.len() + 2;

                        let mut indent = 0;

                        for (i, ins) in code.iter().enumerate() {
                            let label = format!("{i}:");
                            let space = if label.len() < longest {
                                " ".repeat(longest - label.len())
                            } else {
                                " ".to_string()
                            };
                            let icon = if i == pc { "├╼ " } else { "│  " };

                            if matches!(ins, Instr::block_end) {
                                indent -= 1;
                            }
                            let ind = "  ".repeat(indent);
                            if matches!(ins, Instr::block_start) {
                                indent += 1;
                            }

                            let ins = match ins {
                                Instr::block_start => "{".to_string(),
                                Instr::block_end => "}".to_string(),
                                ins => format!("{ins:?}"),
                            };

                            ui.label(
                                egui::RichText::new(format!("{icon}{label}{space}{ind}{ins}"))
                                    .monospace(),
                            );
                        }
                    });
                },
            );
        });
        ctx.request_repaint_after(Duration::from_secs_f64(self.frame_duration / 2.0));
    }
}

fn main() {
    pretty_env_logger::init();

    if args().any(|s| s == "--gui") {
        let native_options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_inner_size([400.0, 300.0])
                .with_min_inner_size([300.0, 220.0])
                .with_maximized(true),
            ..Default::default()
        };
        eframe::run_native(
            "sasm",
            native_options,
            Box::new(|cc| Ok(Box::new(App::new(cc)))),
        )
        .unwrap();
    } else {
        let mut runtime = runtime();
        loop {
            runtime.step();
        }
    }
}
