use crate::{
    parser::Instr,
    runtime::{clean_model::Function, Runtime, Value},
};
use egui::{Id, Vec2};
use std::{
    path::PathBuf,
    time::{Duration, Instant},
};

pub fn gui(path: String) {
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
        Box::new(|cc| Ok(Box::new(App::new(cc, path.into())))),
    )
    .expect("Failed to run window");
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
    pub fn new(_xcc: &eframe::CreationContext<'_>, path: PathBuf) -> Self {
        Self {
            runtime: Runtime::new(path).expect("Failed to load runtime"),
            current_frame: 0,
            frame_count: 1,
            frame_duration: 0.01, //0.5,
            last_tick: Instant::now(),
            auto: false,
        }
    }
}
impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.auto && self.last_tick.elapsed().as_secs_f64() > self.frame_duration {
            self.last_tick = Instant::now();
            self.runtime.step().expect("Failed to step in runtime");
            self.current_frame = self.runtime.stack.len() - 1;
        }

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.with_layout(egui::Layout::left_to_right(egui::Align::Min), |ui| {
                if ui.button("Step").clicked() {
                    self.runtime.step().expect("Failed to step in runtime");
                    if self.frame_count != self.runtime.stack.len() {
                        self.current_frame = self.runtime.stack.len() - 1;
                    }
                    self.frame_count = self.runtime.stack.len();
                }

                if ui.button("Auto").clicked() {
                    self.auto = !self.auto;
                }

                ui.label("Tick delay:");
                ui.add(egui::Slider::new(&mut self.frame_duration, 0.0..=2.0));
            });
        });

        egui::SidePanel::new(egui::panel::Side::Left, Id::new("side_panel")).show(ctx, |ui| {
            ui.heading("Ram usage:");
            let mem = self.runtime.module.memory.size();
            if mem > 1_000_000_000 {
                ui.label(format!("{}GB", mem as f64 / 1_000_000_000.0))
            } else if mem > 1_000_000 {
                ui.label(format!("{}MB", mem as f64 / 1_000_000.0))
            } else if mem > 1_000 {
                ui.label(format!("{}KB", mem as f64 / 1_000.0))
            } else {
                ui.label(format!("{}B", mem))
            };

            ui.heading("Stack frames:");
            let width = ui.max_rect().width();
            for i in (0..self.runtime.stack.len()).rev() {
                let button =
                    egui::Button::new(format!("Frame {i:#?}")).min_size(Vec2::new(width, 16.0));
                if ui.add(button).clicked() {
                    self.current_frame = i;
                }
            }

            ui.heading("Globals:");
            for (k, v) in &self.runtime.module.globals {
                ui.label(format!("{k}: {v:?}"));
            }
        });

        egui::SidePanel::new(egui::panel::Side::Left, Id::new("stack_info")).show(ctx, |ui| {
            ui.heading("Frame info:");
            let text = &self.runtime.stack[self.current_frame];
            // self.auto = text.func_id != 29;
            if text
                .stack
                .iter()
                .filter(|v| matches!(v, Value::BlockLock))
                .count()
                > text.depth_stack.len()
            {
                self.auto = false;
                error!("!LOCK!");
            }
            if let Function::Local { labels, .. } = &self.runtime.module.functions[&text.func_id] {
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

                        let func = self
                            .runtime
                            .module
                            .functions
                            .get(&func)
                            .expect("Failed to get function");
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

                            if matches!(ins, Instr::block_end(_, _, _bt)) {
                                indent -= 1;
                            }
                            let ind = "│ ".repeat(indent);
                            if matches!(ins, Instr::block_start(_, _, _bt)) {
                                indent += 1;
                            }

                            let ins = match ins {
                                Instr::block_start(bt, end, _bt) => {
                                    format!("{{ -- {bt:?} > {end}")
                                }
                                Instr::block_end(bt, start, _bt) => {
                                    format!("}} -- {bt:?} > {start}")
                                }
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
