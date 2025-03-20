#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod parse;
use eframe::{egui::{self, Color32, Pos2, Rect, Sense, Spacing, Stroke, TextEdit}, emath::RectTransform};

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([640.0, 480.0]),
        ..Default::default()
    };
    eframe::run_native("GCAD", options, Box::new(|_| Ok(Box::<MyApp>::default())))
}

#[derive(Default)]
struct MyApp {
    document: String,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("document").show(ctx, |ui| {
            ui.add_sized(
                ui.available_size(),
                TextEdit::multiline(&mut self.document)
                    .code_editor()
                    .margin(0.),
            );
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            let fig = match parse::parse(&self.document) {
                Ok(fig) => fig,
                Err(e) => return println!("{}", e),
            };
            let pos = match fig.order.solve() {
                Ok(pos) => pos,
                Err(e) => return println!("{}", e),
            };
            for p in pos {
                println!("{}", p);
            }
            let (response, canvas) = ui.allocate_painter(ui.available_size(), Sense::empty());
            // let transform = RectTransform::from_to(, response.rect);
            canvas.circle(Pos2::ZERO, 2000., Color32::WHITE, Stroke::new(10., Color32::RED));
        });
    }
}
