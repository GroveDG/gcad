use std::collections::HashMap;

use draw::{Canvas, Drawing, Style, RGB};

use crate::{constraints::elements::Point, math::vector::{bounding_box, Vector}};

pub fn draw(positions: HashMap<Point, Vector>, dpu: f64) -> Canvas {
    let (mut min, mut max) = bounding_box(positions.values());

    let mut size = max - min;
    min -= size*0.25;
    max += size*0.25;
    size *= 1.5;

    let mut canvas = Canvas::new(
        (size.x*dpu) as u32,
        (size.y*dpu) as u32
    );

    for (_point, pos) in positions {
        let can_pos = (pos - min) * dpu;
        canvas.display_list.add(
            Drawing::new()
            .with_shape(draw::Shape::Circle { radius: 2 })
            .with_xy(can_pos.x as f32, can_pos.y as f32)
            .with_style(Style::filled(RGB { r: 0, g: 0, b: 0 }))
        );
    }

    canvas
}