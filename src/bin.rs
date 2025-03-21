#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod parse;

use eframe::{
    egui::{self, Color32, Pos2, Rect, Sense, Stroke, TextEdit, Ui},
    emath::RectTransform,
};

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
            let pos: Vec<_> = pos
                .into_iter()
                .map(|p| Pos2 {
                    x: p.x as f32,
                    y: p.y as f32,
                })
                .collect();
            let Some(bb) = bounding_box(&pos) else { return };
            let a = bb.size().x / bb.size().y;
            let mut size = ui.available_size();
            let b = size.x / size.y;
            let centerer = if a <= b {
                size.x = size.y * a;
                Ui::vertical_centered
            } else {
                size.y = size.x / a;
                Ui::horizontal_centered
            };
            (centerer)(ui, |ui: &mut Ui| {
                let (response, canvas) = ui.allocate_painter(size, Sense::empty());
                let transform = RectTransform::from_to(bb, response.rect);
                for p in pos {
                    canvas.circle(transform.transform_pos(p), 1., Color32::WHITE, Stroke::NONE);
                }
                response
            });
        });
    }
}

fn bounding_box(pos: &[Pos2]) -> Option<Rect> {
    if pos.is_empty() {
        return None;
    }
    let mut min = pos[0];
    let mut max = pos[0];
    for p in pos {
        min.x = min.x.min(p.x);
        min.y = min.y.min(p.y);
        max.x = max.x.max(p.x);
        max.y = max.y.max(p.y);
    }
    let size = max - min;
    let margin = (size.x.max(size.y) * 0.25).max(1.);
    min.x -= margin;
    min.y -= margin;
    max.x += margin;
    max.y += margin;
    Some(Rect { min, max })
}
