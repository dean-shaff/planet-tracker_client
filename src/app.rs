use std::f64::consts::{PI, FRAC_PI_2, TAU};

use chrono::{DateTime, Utc, Duration};
use leptos::*;
use leptos_meta::*;
use enum_iterator::all;
use futures::future::join_all;

use crate::{
    api::get_astron_object_data, 
    models::{AstronObjectResponse, AstronObject, AstronObjectQueryParams, CardinalDirection}, errors::AppError
};


#[derive(Debug, PartialEq, Clone)]
struct Position {
    lat: f64,
    lon: f64,
    elevation: f64
}


async fn get_all_astron_object_data(position_time: (Position, DateTime<Utc>)) -> Result<Vec<AstronObjectResponse>, AppError>
{
    logging::log!("get_all_astron_object_data: position_time={:?}", position_time);
    let (position, when) = position_time;
    // let futs = all::<AstronObject>().map(|obj| {
    //     let qp = AstronObjectQueryParams {
    //         name: obj,
    //         lon: position.lon,
    //         lat: position.lat,
    //         elevation: position.elevation,
    //         when: when.naive_utc(),
    //     };
    //     get_astron_object_data(qp)
    // }).collect::<Vec<_>>();

    // let res = join_all(futs)
    //     .await
    //     .into_iter()
    //     .collect();
    let res = Ok(vec![
        AstronObjectResponse { 
            name: AstronObject::Jupiter, 
            magnitude: 0.0, 
            size: 10.0, 
            az: 5.06, 
            el: 0.34, 
            ra: 0.0, 
            dec: 0.0, 
            setting_time: when.naive_utc() + Duration::hours(2), 
            rising_time: when.naive_utc() + Duration::hours(5), 
            when: when.naive_utc()
        },
        AstronObjectResponse { 
            name: AstronObject::Mars, 
            magnitude: 0.0, 
            size: 4.0, 
            az: 1.74, 
            el: 1.13, 
            ra: 0.0, 
            dec: 0.0, 
            setting_time: when.naive_utc() + Duration::hours(2), 
            rising_time: when.naive_utc() + Duration::hours(5), 
            when: when.naive_utc()
        }
    ]);

    res
}

fn rad2deg(rad: f64) -> f64 
{
    (rad * 180.0) / PI
}


fn deg2cardinal(deg: f64) -> CardinalDirection
{
    let mut shifted = deg + 22.5;
    if shifted > 360.0 {
        shifted = shifted - 360.0; 
    }
    let i = (shifted / 45.0).ceil() as u8;
    CardinalDirection::from(i)
}


#[component]
pub fn TextDisplay(objs: Vec<AstronObjectResponse>) -> impl IntoView 
{
    let objs_len = objs.len();
    let divider = move |idx: usize| {
        if idx == objs_len - 1 {
            view! {}.into_view()
        } else {
            view! { <Divider/> }.into_view()
        }
    };

    let rows = move || {
        objs
            .iter()
            .enumerate()
            .map(|(idx, obj)| {
                let az = format!("{:.2}°", rad2deg(obj.az));
                let az_cardinal = format!("{:#}", deg2cardinal(rad2deg(obj.az)));
                let el = format!("{:.2}°", rad2deg(obj.el));
                let formatter = "%H:%M";
                
                let (setting_time, rising_time) = if obj.setting_time > obj.rising_time {
                    ("-".to_string(), obj.rising_time.format(formatter).to_string())
                } else {
                    (obj.setting_time.format(formatter).to_string(), "-".to_string())
                };
                
                view! {
                    <div class="flex py-2">
                        <div class="flex-1 pr-2">
                            {obj.name.to_string()}
                        </div>
                        <div class="flex-1 pr-2">
                            {az_cardinal}
                        </div>
                        <div class="flex-1 pr-2">
                            {az}/{el}
                        </div>
                        <div class="flex-1 pr-2">
                            {setting_time}
                        </div>
                        <div class="flex-1">
                            {rising_time}
                        </div>
                    </div>
                    { divider(idx) }
                }
            })
            .collect_view()
    };

    view! {
        
        <div class="flex flex-col w-screen">
            <div class="flex py-2">
                <div class="font-semibold flex-1 pr-2">"Name"</div>
                <div class="font-semibold flex-1 pr-2">"Cardinal Direction"</div>
                <div class="font-semibold flex-1 pr-2">"Azimuth/Elevation"</div>
                <div class="font-semibold flex-1 pr-2">"Setting Time"</div>
                <div class="font-semibold flex-1">"Rising Time"</div>
            </div>
            <Divider/>
            {rows}
        </div>
    }
}



#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    let (position_time, set_position_time) = create_signal((Position {
        lon:13.4,
        lat:52.5,
        elevation:0.0,
    }, Utc::now()));
    
    let astron_objs = create_resource(position_time, get_all_astron_object_data);

    let fallback = move |errors: RwSignal<Errors>| {
        logging::log!("error fallback");
        let error_list = move || {
            errors.with(|errors| {
                errors
                    .iter()
                    .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                    .collect_view()
            })
        };

        view! {
            <div class="error">
                <h2>"Error"</h2>
                <ul>{error_list}</ul>
            </div>
        }
    };

    let success_view = move || {
        astron_objs.and_then(|data| {
            view! {
                <div class="flex justify-center">
                    <div class="flex">
                        <svg width="500" height="500">
                            <PolarPlot radius=200 objs={data.clone()}/>
                        </svg>
                    </div>
                    <TextDisplay objs={data.clone()}/>
                </div>
            }
        })
    };

    view! {
        <Stylesheet id="leptos" href="/pkg/tailwind.css"/>
        <div class="my-0 mx-auto max-w-3xl">
            <h1 class="text-4xl">"Planet Tracker"</h1>
            <Divider/>
            <Transition fallback=move || { view! {<div>"Loading..."</div>}}>
                <ErrorBoundary fallback>
                <div>
                    { success_view }
                </div>  
                </ErrorBoundary>
            </Transition>
        </div>
    }
}




#[component]
pub fn PolarPlot(
    radius: usize,
    objs: Vec<AstronObjectResponse>
) -> impl IntoView {

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


    let padding = 20;

    let el_increment = 15;
    let el_lines: Vec<f64> = (0..90/el_increment).map(|val| (val*el_increment) as f64).collect();
    let az_increment = 30;
    let az_lines: Vec<f64> = (0..360/az_increment).map(|val| (val*az_increment) as f64).collect();
    let (center_x, center_y) = (radius + padding, radius + padding);

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
                <circle cx={center_x} cy={center_y} r={r} stroke="#1e293b" stroke-width="1" fill="none"/>
                <text x={x} y={y} font-family="serif" font-size="10" fill="#1e293b" transform={transform}>{text}</text>
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
                <line x1=0 x2={radius} y1=0 y2=0 stroke="#1e293b" stroke-width="1" transform={transform}/>
                <text x={x} y={y} font-family="serif" font-size="10" fill="#1e293b" transform={text_transform}>{text}</text>
            }  
        })
        .collect::<Vec<_>>();


    let obj_views = objs
        .iter()
        .map(|obj| {
            let (cx, cy) = transform_az_el(obj.az, obj.el, radius as f64, center_x as f64, center_y as f64); 
            view! {
                <circle cx={cx} cy={cy} fill="black" r={obj.size}/>
                <text x={cx} y={cy} font-family="serif" font-size="10" fill="#1e293b">{obj.name.to_string()}</text>
            }
        })
        .collect::<Vec<_>>();


    view! {
        { el_circles }
        { az_lines }
        { obj_views }   
    }
}

#[component]
fn Divider() -> impl IntoView {
    view! { <hr/> }
}


