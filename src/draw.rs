use std::collections::HashMap;

// use draw::{render, Canvas, Drawing, Style, SvgRenderer, RGB};
use rsille::Canvas;

use crate::{
    constraints::elements::Point,
    math::vector::{bounding_box, Number, Vector},
};

pub fn draw(positions: HashMap<Point, Vector>) {
    let (mut min, mut max) = bounding_box(positions.values());

    let mut size = max - min;
    min -= size * 0.25;
    max += size * 0.25;
    size *= 1.5;

    let mut canvas = Canvas::new();
    let t_size = term_size::dimensions().unwrap();
    let t_size = Vector {
        x: t_size.0 as Number * 2.0,
        y: (t_size.1 - 1) as Number * 3.0,
    };
    let scale = t_size / size;
    let scale = if t_size.y > scale.x * size.y {
        scale.x
    } else {
        scale.y
    };

    canvas.set_size(size.x * scale, size.y * scale);
    for (_, pos) in positions {
        let pos = (pos - min) * scale;
        canvas.set(pos.x, pos.y);
    }
    canvas.print();

    //const dpu: Number = 96.0;
    // let mut canvas = Canvas::new(
    //     (size.x*dpu) as u32,
    //     (size.y*dpu) as u32
    // );

    // for (_point, pos) in positions {
    //     let can_pos = (pos - min) * dpu;
    //     canvas.display_list.add(
    //         Drawing::new()
    //         .with_shape(draw::Shape::Circle { radius: 2 })
    //         .with_xy(can_pos.x as f32, can_pos.y as f32)
    //         .with_style(Style::filled(RGB { r: 0, g: 0, b: 0 }))
    //     );
    // }

    // render::save(
    //     &canvas,
    //     "figure.svg",
    //     SvgRenderer::new()
    // ).or(Err("render failed".to_string())).unwrap();
}
