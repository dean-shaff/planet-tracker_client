use std::f64::consts::FRAC_PI_2;

use leptos::*;
use logging::log;
use wasm_bindgen::JsCast;
use web_sys::MouseEvent;

use crate::{
    app::MIN_POLAR_PLOT_WIDTH,
    models::{AstronObject, AstronObjectResponse},
    AstronObjectsRw, SelectedRw,
};

#[derive(Debug, Clone)]
pub struct Tooltip {
    x: f64,
    y: f64,
    obj: Option<AstronObject>,
}

impl Default for Tooltip {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            obj: None,
        }
    }
}

type TooltipRw = RwSignal<Tooltip>;

fn transform_radius(radius: f64) -> f64 {
    radius.sqrt()
}

fn transform_az_el(az: f64, el: f64, radius: f64, center_x: f64, center_y: f64) -> (f64, f64) {
    let el_abs = el.abs();
    let rad_rel = transform_radius(1.0 - (el_abs / FRAC_PI_2));
    let cx = radius * rad_rel * (az - FRAC_PI_2).cos();
    let cy = radius * rad_rel * (az - FRAC_PI_2).sin();
    (cx + center_x, cy + center_y)
}

fn transform_az_r_rel(
    az: f64,     // azimuth angle
    r_rel: f64,  // reletive radius
    radius: f64, // screen radius
    center_x: f64,
    center_y: f64,
) -> (f64, f64) {
    let cx = radius * r_rel * (az - FRAC_PI_2).cos();
    let cy = radius * r_rel * (az - FRAC_PI_2).sin();
    (cx + center_x, cy + center_y)
}

#[component]
pub fn AstronObjectView(
    radius: f64,
    width: f64,
    center_x: f64,
    center_y: f64,
    obj: AstronObjectResponse,
    selected: SelectedRw,
    tooltip: TooltipRw,
) -> impl IntoView {
    let (obj, _) = create_signal(obj);
    let scale_factor = 1.5 * width as f64 / MIN_POLAR_PLOT_WIDTH as f64;
    let obj_size = move || 2.0 + scale_factor * (obj.get().size + 1.0).ln();

    let node_ref = create_node_ref::<leptos::svg::Circle>();

    let get_scroll = || -> (f64, f64) {
        let doc = web_sys::window().unwrap().document().unwrap();
        let body = doc.body().unwrap();
        let doc_elem = doc.document_element().unwrap();
        let (scroll_x, scroll_y) = (
            body.scroll_left() + doc_elem.scroll_left(),
            body.scroll_top() + doc_elem.scroll_top(),
        );
        (scroll_x as f64, scroll_y as f64)
    };

    create_effect(move |_| {
        if let (Some(circle), Some(current_selected)) = (node_ref.get(), selected.get()) {
            let obj = obj.get();
            if current_selected != obj.name {
                return;
            }
            let rect = circle.get_bounding_client_rect();
            let (scroll_x, scroll_y) = get_scroll();
            tooltip.set(Tooltip {
                x: rect.x() + obj_size() + scroll_x as f64,
                y: rect.y() + scroll_y as f64,
                obj: Some(obj.name.clone()),
            });
        };
    });

    let on_click = move |_| {
        let obj = obj.get();
        let circle = node_ref.get().expect("circle exists").clone();
        let rect = circle.get_bounding_client_rect();
        let (scroll_x, scroll_y) = get_scroll();

        logging::log!(
            "rect.x={}, rect.y={}, scroll_x={}, scroll_y={}",
            rect.x(),
            rect.y(),
            scroll_x,
            scroll_y
        );
        tooltip.set(Tooltip {
            x: rect.x() + obj_size() + scroll_x as f64,
            y: rect.y() + scroll_y as f64,
            obj: Some(obj.name.clone()),
        });

        if let Some(current_selected) = selected.get() {
            if current_selected != obj.name {
                selected.set(Some(obj.name.clone()));
            } else {
                selected.set(None);
            }
        } else {
            selected.set(Some(obj.name.clone()));
        }
    };

    let circle_view = move || {
        let obj = obj.get();
        let (cx, cy) = transform_az_el(obj.az, obj.el, radius, center_x, center_y);
        let astron_obj = obj.name.clone();
        let fill = astron_obj.get_color();
        view! {
            <circle
                node_ref={node_ref}
                cx={cx}
                cy={cy}
                fill={fill}
                r={obj_size}
                on:click=on_click
            />
        }
    };
    view! { {circle_view} }
}

#[component]
pub fn PolarPlot(
    width: usize,
    height: usize,
    radius: usize,
    objs: AstronObjectsRw,
    selected: SelectedRw,
) -> impl IntoView {
    log!(
        "PolarPlot: width={}, height={}, radius={}",
        width,
        height,
        radius
    );

    let padding = width / 2 - radius;
    log!("PolarPlot: padding={}", padding);

    let el_increment = 15;
    let el_lines: Vec<f64> = (0..90 / el_increment)
        .map(|val| (val * el_increment) as f64)
        .collect();
    let az_increment = 30;
    let az_lines: Vec<f64> = (0..360 / az_increment)
        .map(|val| (val * az_increment) as f64)
        .collect();
    let (center_x, center_y) = (radius + padding, radius + padding / 2);

    let el_circles = el_lines
        .iter()
        .map(|r_line| {
            let rad_norm = 1.0 - (r_line / 90.0);
            let rad_transformed = transform_radius(rad_norm);
            let r = radius as f64 * rad_transformed;
            let (x, y) = if *r_line == 0.0 {
                transform_az_r_rel(11.0_f64.to_radians(), 1.01, radius as f64, center_x as f64, center_y as f64)
            } else {
                transform_az_el(10.0_f64.to_radians(), (r_line - 1.0).to_radians(), radius as f64, center_x as f64, center_y as f64)
            };
            let text = format!("{:.0}°", r_line);
            let transform = format!("rotate(10 {} {})", x, y);
            view! {
                <circle cx={center_x} cy={center_y} r={r} stroke="#1f2937" stroke-width="1" fill="none"/>
                <text x={x} y={y} font-family="serif" font-size="10" fill="#1f2937" transform={transform}>{text}</text>
            }
        })
        .collect::<Vec<_>>();

    let az_lines = az_lines
        .iter()
        .map(|az_line| {
            let transform = format!("rotate({} {} {}) translate({}, {})", az_line, center_x, center_y, center_x, center_y);
            let text = format!("{:.0}°", az_line);
            let (text_transform, (x, y)) = if *az_line <= 180.0 {
                let (x, y) = transform_az_r_rel((az_line + 1.0).to_radians(), 1.01, radius as f64, center_x as f64, center_y as f64);
                (format!("rotate({} {} {})", az_line - 90.0, x, y), (x, y))
            } else {
                let transform = |width: f64| -> f64
                {
                    let (x0, y0) = (360.0, 1.14);
                    let (x1, y1) = (768.0, 1.07);
                    let x_diff = x1 - x0;
                    let y_diff = y1 - y0;
                    let m = y_diff / x_diff;
                    let b = (x1*y0 - x0*y1) / x_diff;
                    width * m + b
                };
                let (x, y) = transform_az_r_rel((az_line - 1.0).to_radians(), transform(width as f64), radius as f64, center_x as f64, center_y as f64);
                (format!("rotate({} {} {})", az_line - 270.0, x, y), (x, y))
            };
            view! {
                <line x1=0 x2={radius} y1=0 y2=0 stroke="#1f2937" stroke-width="1" transform={transform}/>
                <text x={x} y={y} font-family="serif" font-size="10" fill="#1f2937" transform={text_transform}>{text}</text>
            }
        })
        .collect::<Vec<_>>();

    let tooltip = create_rw_signal(Tooltip::default());

    let tooltip_style = move || {
        let tooltip_val = tooltip.get();
        if let Some(current_selected) = selected.get() {
            format!(
                "position: absolute; left: {}px; top: {}px; transform: translate(-50%, -110%);",
                tooltip_val.x, tooltip_val.y
            )
        } else {
            "position: absolute; display: none".to_string()
        }
    };

    let tooltip_text = move || {
        let tooltip = tooltip.get();
        tooltip.obj.map(|o| o.to_string()).unwrap_or("".to_string())
    };

    view! {
        <div class="content-center justify-center">
            <svg width={width} height={height} style="display: block; margin: auto;">
                { el_circles }
                { az_lines }
                <For
                    each=move || objs.get()
                    key=|obj| obj.name.clone()
                    children=move |obj: AstronObjectResponse| {
                        view! {
                            <AstronObjectView
                                obj=obj
                                selected=selected
                                tooltip=tooltip
                                radius={radius as f64}
                                center_x={center_x as f64}
                                center_y={center_y as f64}
                                width={width as f64}
                            />
                        }
                    }
                />
            </svg>
            <div
                style={tooltip_style}
                class="text-zinc-50 rounded-md bg-zinc-500 py-1 px-2 opacity-80"
            >
                {tooltip_text}
            </div>
        </div>
    }
}

// #[component]
// pub fn PolarPlot(
//     width: usize,
//     height: usize,
//     radius: usize,
//     objs: AstronObjectsRw,
// ) -> impl IntoView {
//     view! {
//         <div>"Placeholder"</div>
//     }
// }
