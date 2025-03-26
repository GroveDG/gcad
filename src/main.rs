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
    let mut err: Signal<Option<(String, String)>> = use_signal(|| None);

    let (min, size) = bounding_box(solution.read().values().copied()).unwrap_or_default();
    let svg_font_size = size.y / 30.;

    rsx! {
        div {
            id: "gcad-document-area",
            textarea {
                id: "gcad-document",
                value: "{doc}",
                oninput: move |event| {
                    doc.set(event.value());
    
                    err.set('parse: {
                        let statements = match parse::parse(doc.read().as_ref()) {
                            Err(e) => {
                                let pos = unsafe { e.1.offset_from(doc.read().as_ptr()) };
                                break 'parse Some((e.0.to_string(), pos as usize))
                            },
                            Ok(s) => s,
                        };
                        let new_hash = {
                            let mut hasher = DefaultHasher::new();
                            statements.hash(&mut hasher);
                            hasher.finish()
                        };
                        if hash == new_hash {
                            break 'parse None;
                        }
                        hash = new_hash;
                        let fig = match Figure::from_statements(statements) {
                            Err(e) => break 'parse Some((e.to_string(), 0)),
                            Ok(f) => f,
                        };
                        let pos = match fig.order.solve() {
                            Err(e) => break 'parse Some((e.to_string(), 0)),
                            Ok(f) => f,
                        };
                        solution.set(HashMap::from_iter(
                            fig.point_map.into_iter().map(|(point, i)| (point, pos[i])),
                        ));
                        None
                    }.map(|(e, pos)| {
                        (e, doc.read()[..pos].chars().map(|c| if c.is_whitespace() {
                            c
                        } else {
                            ' '
                        }).collect())
                    }));
                },
            },
            if let Some((err, err_spacing)) = err.cloned() {
                textarea {
                    id: "gcad-error",
                    value: "\n{err_spacing}^{err}"
                }
            }
        },
        div {
            id: "gcad-display-area",
            svg {
                id: "gcad-display",
                view_box: "{min.x} {min.y} {size.x} {size.y}",
                width: size.x,
                height: size.y,
                defs {
                    marker {
                        id: "triangle",
                        view_box: "0 0 10 10",
                        ref_x: "10",
                        ref_y: "5",
                        marker_units: "strokeWidth",
                        marker_width: "10",
                        marker_height: "10",
                        orient: "auto",
                        path {
                            d: "M 0 0 L 10 5 L 0 10 z",
                            fill: "gray",
                        }
                    }
                }
                style {
                    "circle {{
                        fill: black;
                    }}"
                    "line {{
                        stroke: gray;
                        stroke-width: {svg_font_size/24.}px;
                    }}"
                    "text {{
                        fill: black;
                        font: italic {svg_font_size}px sans-serif;
                    }}"
                },
                line {
                    x1: 0,
                    y1: min.y,
                    x2: 0,
                    y2: min.y + size.y,
                    marker_end: "url(#triangle)",
                },
                line {
                    x1: min.x,
                    y1: 0,
                    x2: min.x + size.x,
                    y2: 0,
                    marker_end: "url(#triangle)",
                },
                for (point, pos) in solution.cloned() {
                    circle {
                        cx: pos.x,
                        cy: pos.y,
                        r: svg_font_size/6.,
                    },
                    text {
                        x: pos.x,
                        y: pos.y - svg_font_size/2.,
                        "{point}",
                    },
                },
            },
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
    Some((min, max - min))
}
