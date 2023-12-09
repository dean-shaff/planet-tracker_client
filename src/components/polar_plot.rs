use std::f64::consts::FRAC_PI_2;

use leptos::*;
use logging::log;
use wasm_bindgen::JsCast;
use web_sys::MouseEvent;

use crate::{models::{AstronObject, AstronObjectResponse}, app::MIN_POLAR_PLOT_WIDTH};



#[derive(Debug, Clone)]
struct Tooltip {
    x: f64, 
    y: f64,
    obj: Option<AstronObject>,
    visible: bool
}

impl Default for Tooltip {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            obj: None,
            visible: false
        }
    }
}


#[component]
pub fn PolarPlot(
    width: usize,
    height: usize,
    radius: usize,
    objs: Vec<AstronObjectResponse>
) -> impl IntoView {

    log!("PolarPlot: width={}, height={}, radius={}", width, height, radius);

    let radius_f64 = radius as f64;
    fn transform_radius(radius: f64) -> f64 {
        radius.sqrt()
    }

    fn transform_az_el(
        az: f64, 
        el: f64,
        radius: f64,
        center_x: f64,
        center_y: f64
    ) -> (f64, f64)
    {
        let el_abs = el.abs();
        let rad_rel = transform_radius(1.0 - (el_abs / FRAC_PI_2));
        let cx = radius * rad_rel * (az - FRAC_PI_2).cos();
        let cy = radius * rad_rel * (az - FRAC_PI_2).sin();
        (cx + center_x, cy + center_y)
    }

    fn transform_az_r_rel(
        az: f64, // azimuth angle 
        r_rel: f64, // reletive radius
        radius: f64, // screen radius 
        center_x: f64,
        center_y: f64
    ) -> (f64, f64)
    {
        let cx = radius * r_rel * (az - FRAC_PI_2).cos();
        let cy = radius * r_rel * (az - FRAC_PI_2).sin();
        (cx + center_x, cy + center_y)
    }

    let padding = width / 2 - radius;

    let el_increment = 15;
    let el_lines: Vec<f64> = (0..90/el_increment).map(|val| (val*el_increment) as f64).collect();
    let az_increment = 30;
    let az_lines: Vec<f64> = (0..360/az_increment).map(|val| (val*az_increment) as f64).collect();
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
                let (x, y) = transform_az_r_rel((az_line - 1.0).to_radians(), 1.1, radius as f64, center_x as f64, center_y as f64);
                (format!("rotate({} {} {})", az_line - 270.0, x, y), (x, y))
            };
            view! {
                <line x1=0 x2={radius} y1=0 y2=0 stroke="#1f2937" stroke-width="1" transform={transform}/>
                <text x={x} y={y} font-family="serif" font-size="10" fill="#1f2937" transform={text_transform}>{text}</text>
            }  
        })
        .collect::<Vec<_>>();
    
    let (tooltip, set_tooltip) = create_signal(Tooltip::default());

    let obj_views = objs
        .into_iter()
        .map(|resp| {
            let (cx, cy) = transform_az_el(resp.az, resp.el, radius_f64, center_x as f64, center_y as f64); 
            let obj = resp.name.clone();
            let fill = obj.get_color();
            let scale_factor = 1.5 * width as f64 / MIN_POLAR_PLOT_WIDTH as f64;
            let obj_size = 2.0 + scale_factor * (resp.size + 1.0).ln();
            // logging::log!("scale_factor={}, obj={}, obj_size={}", scale_factor, obj, obj_size);
        
            let on_click = move |evt: MouseEvent| {
                let tooltip_val = tooltip.get();
                if let Some(current_obj) = tooltip_val.obj.clone() {
                    if current_obj == obj {
                        set_tooltip(Tooltip { visible: !tooltip_val.visible, ..tooltip_val});
                        return
                    }
                }
                logging::log!("x={}, y={}, offset_x={}, offset_y={} cx={}, cy={}", evt.x(), evt.y(), evt.offset_x(), evt.offset_y(), cx, cy);
                let target = evt.target().expect("target exists");
                let div: web_sys::Element = target.dyn_into().unwrap();
                let rect = div.get_bounding_client_rect();

                let doc = web_sys::window().unwrap().document().unwrap();
                let body = doc.body().unwrap();
                let doc_elem = doc.document_element().unwrap();
                let (scroll_x, scroll_y) = (
                    body.scroll_left() + doc_elem.scroll_left(), 
                    body.scroll_top() + doc_elem.scroll_top()
                );

                logging::log!("rect.x={}, rect.y={}, scroll_x={}, scroll_y={}", rect.x(), rect.y(), scroll_x, scroll_y);
                set_tooltip(
                    Tooltip { 
                        x: rect.x() + obj_size + scroll_x as f64, 
                        y: rect.y() + scroll_y as f64, 
                        obj: Some(obj.clone()),
                        visible: true 
                    }
                );
            };
            
            view! {
                <circle 
                    cx={cx} 
                    cy={cy} 
                    fill={fill} 
                    r={obj_size}
                    on:click=on_click
                />
            }
            
        })
        .collect::<Vec<_>>();

    let tooltip_style = move || {
        let tooltip_val = tooltip.get();
        logging::log!("tooltip_val={:?}", tooltip_val);
        if tooltip_val.visible {
            format!("position: absolute; left: {}px; top: {}px; transform: translate(-50%, -110%);", tooltip_val.x, tooltip_val.y)
        } else {
            "position: absolute; display: none".to_string()
        }
    };

    view! {
        <div class="content-center justify-center">
            <svg width={width} height={height} style="display: block; margin: auto;">
                { el_circles }
                { az_lines }
                { obj_views }   
            </svg>
             <div
                style={tooltip_style} 
                class="text-zinc-50 rounded-md bg-zinc-500 py-1 px-2 opacity-80" 
            >
                {move || tooltip.get().obj.map(|o| o.to_string()).unwrap_or("".to_string())}
            </div>
        </div>
    }
}


