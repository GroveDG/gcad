use std::{
    collections::HashMap,
    hash::{DefaultHasher, Hash, Hasher, RandomState},
};

use dioxus::prelude::*;
use gsolve::math::Vector;
use parse::Figure;

mod parse;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link {
            rel: "stylesheet",
            href: asset!("/assets/main.css")
        },
        GCADDoc { }
    }
}

#[component]
fn GCADDoc() -> Element {
    let mut doc = use_signal(String::default);
    let mut hash = use_hook(|| 0u64);
    let mut solution: Signal<HashMap<String, gsolve::math::Vector, RandomState>> =
        use_signal(HashMap::default);
    let mut err = use_signal(String::default);
    let (min, size) = bounding_box(solution.read().values().copied()).unwrap_or_default();
    let svg_font_size = size.y / 30.;

    rsx! {
        textarea {
            id: "gcad-document",
            value: "{doc}",
            oninput: move |event| {
                doc.set(event.value());

                err.set('parse: {
                    let statements = match parse::parse(&doc.cloned()) {
                        Err(e) => break 'parse e.to_string(),
                        Ok(s) => s,
                    };
                    let new_hash = {
                        let mut hasher = DefaultHasher::new();
                        statements.hash(&mut hasher);
                        hasher.finish()
                    };
                    if hash == new_hash {
                        break 'parse String::default();
                    }
                    hash = new_hash;
                    let fig = match Figure::from_statements(statements) {
                        Err(e) => break 'parse e.to_string(),
                        Ok(f) => f,
                    };
                    let pos = match fig.order.solve() {
                        Err(e) => break 'parse e.to_string(),
                        Ok(f) => f,
                    };
                    solution.set(HashMap::from_iter(
                        fig.point_map.into_iter().map(|(point, i)| (point, pos[i])),
                    ));
                    String::default()
                })
            },
        },
        div {
            id: "gcad-display-area",
            svg {
                id: "gcad-display",
                view_box: "{min.x} {min.y} {size.x} {size.y}",
                width: size.x,
                height: size.y,
                style {
                    "circle {{
                        fill: "black";
                    }}"
                    ".label {{
                        fill: black;
                        font: italic {svg_font_size}px sans-serif;
                    }}"
                },
                for (point, pos) in solution.cloned() {
                    circle {
                        cx: pos.x,
                        cy: pos.y,
                        r: svg_font_size/6.,
                    }
                    text {
                        class: "label",
                        x: pos.x,
                        y: pos.y - svg_font_size/2.,
                        "{point}",
                    }
                },
            },
            "{err}"
        }
    }
}

fn bounding_box(mut pos: impl Iterator<Item = Vector>) -> Option<(Vector, Vector)> {
    let mut min = pos.next()?;
    let mut max = min;
    for p in pos {
        min.x = min.x.min(p.x);
        min.y = min.y.min(p.y);
        max.x = max.x.max(p.x);
        max.y = max.y.max(p.y);
    }
    let size = max - min;
    let margin = (size.x.max(size.y) * 0.25).max(3.);
    min.x -= margin;
    min.y -= margin;
    max.x += margin;
    max.y += margin;
    Some((min, max-min))
}
