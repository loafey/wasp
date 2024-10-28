#![feature(const_type_name)]

use egui::{Id, Vec2};
use hex::Hex;
use parser::{Module, Parsable};
use runtime::{clean_model::Function, Runtime};
use std::{
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

struct App {
    runtime: Runtime,
    current_frame: usize,
    frame_count: usize,
    last_tick: Instant,
    frame_duration: f64,
}
impl App {
    pub fn new(_xcc: &eframe::CreationContext<'_>) -> Self {
        let bin: &[u8] = include_bytes!("../examples/hello_world.wasm");
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

        Self {
            runtime: Runtime::new(module),
            current_frame: 0,
            frame_count: 1,
            frame_duration: 0.25,
            last_tick: Instant::now(),
        }
    }
}
impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // if self.last_tick.elapsed().as_secs_f64() > self.frame_duration {
        //     self.last_tick = Instant::now();
        //     self.runtime.step();
        //     self.current_frame = self.runtime.stack.len() - 1;
        // }

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            if ui.button("Step").clicked() {
                self.runtime.step();
                if self.frame_count != self.runtime.stack.len() {
                    self.current_frame = self.runtime.stack.len() - 1;
                }
                self.frame_count = self.runtime.stack.len();
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
            let label =
                egui::Label::new(format!("{:#?}", self.runtime.stack[self.current_frame])).extend();
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

                        let func = &self.runtime.module.functions[&(func as u32)];
                        let Function::Local { code, .. } = func else {
                            return;
                        };
                        let var_name = format!("{}:", code.len());
                        let var_name = &var_name;
                        let longest = var_name.len() + 2;

                        for (i, ins) in code.iter().enumerate() {
                            let label = format!("{i}:");
                            let space = if label.len() < longest {
                                " ".repeat(longest - label.len())
                            } else {
                                " ".to_string()
                            };
                            let icon = if i == pc { "├╼ " } else { "│  " };

                            ui.label(
                                egui::RichText::new(format!("{icon}{label}{space}{ins:?}"))
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

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_min_inner_size([300.0, 220.0]),
        ..Default::default()
    };
    eframe::run_native(
        "sasm",
        native_options,
        Box::new(|cc| Ok(Box::new(App::new(cc)))),
    )
    .unwrap();
}
