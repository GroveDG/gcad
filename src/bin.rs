#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod parse;

use std::{
    collections::HashMap,
    hash::{DefaultHasher, Hash, Hasher, RandomState},
};

use eframe::{
    egui::{self, Color32, FontId, Pos2, Rect, Sense, Stroke, TextEdit, Ui},
    emath::RectTransform,
};
use gsolve::math::Vector;
use parse::{
    Figure,
    ParseErr::{self, *},
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
    prev_str_hash: u64,
    prev_parsed_hash: u64,
    solution: HashMap<String, Vector>,
}
impl MyApp {
    fn update_fig(&mut self) -> Result<(), ParseErr> {
        let str_hash = {
            let mut hasher = DefaultHasher::new();
            self.document.hash(&mut hasher);
            hasher.finish()
        };
        if self.prev_str_hash == str_hash {
            return Ok(());
        }
        self.prev_str_hash = str_hash;

        let statements = parse::parse(&self.document)?;

        let parsed_hash = {
            let mut hasher = DefaultHasher::new();
            statements.hash(&mut hasher);
            hasher.finish()
        };
        if self.prev_parsed_hash == parsed_hash {
            return Ok(());
        }
        self.prev_parsed_hash = parsed_hash;

        let fig = Figure::from_statements(statements)?;
        let pos = fig.order.solve().map_err(|_| Invalid)?;

        self.solution =
            HashMap::from_iter(fig.point_map.into_iter().map(|(point, i)| (point, pos[i])));

        Ok(())
    }
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
            if let Err(e) = self.update_fig() {
                println!("{}", e);
                return;
            };

            let solution: HashMap<&String, Pos2, RandomState> = HashMap::from_iter(self.solution.iter().map(|(point, p)| {
                (
                    point,
                    Pos2 {
                        x: p.x as f32,
                        y: p.y as f32,
                    },
                )
            }));
            let Some(mut bb) = bounding_box(solution.values().copied()) else {
                return;
            };
            std::mem::swap(&mut bb.min, &mut bb.max);
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
                for (point, p) in solution {
                    let p = transform.transform_pos(p);
                    canvas.circle(p, 1., Color32::WHITE, Stroke::NONE);
                    canvas.text(p, egui::Align2::LEFT_TOP, point, FontId::default(), Color32::GRAY);
                }
                response
            });
        });
    }
}

fn bounding_box(mut pos: impl Iterator<Item = Pos2>) -> Option<Rect> {
    let mut min = pos.next()?;
    let mut max = min;
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
