use std::f64::consts::PI;

use chrono::{DateTime, Utc};
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
    let futs = all::<AstronObject>().map(|obj| {
        let qp = AstronObjectQueryParams {
            name: obj,
            lon: position.lon,
            lat: position.lat,
            elevation: position.elevation,
            when: when.naive_utc(),
        };
        get_astron_object_data(qp)
    }).collect::<Vec<_>>();

    let res = join_all(futs)
        .await
        .into_iter()
        .collect();
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
    logging::log!("deg2cardinal: deg={}, i={}", deg, i);
    CardinalDirection::from(i)
}


#[component]
pub fn TextDisplay(objs: Vec<AstronObjectResponse>) -> impl IntoView 
{
    let objs_len = objs.len();
    let divider = move |idx: usize| {
        logging::log!("idx={}", idx);
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
            // <svg width="1000" height="1000">
            //     <PolarPlot radius=200 />
            // </svg>
        </div>
    }
}



#[component]
pub fn PolarPlot(radius: usize) -> impl IntoView {

    let padding = 10;

    let el_lines = 5;
    let el_increment = radius as f64 / el_lines as f64; 
    let az_lines = 10;
    let az_increment = 360.0 / az_lines as f64;

    let (center_x, center_y) = (radius + padding, radius + padding);

    let el_circles = (1..el_lines + 1).map(|idx| {
        let idx_radius = idx as f64 * el_increment;
        // log!("idx_radius={}", idx_radius);
        view! { 
            <circle cx={center_x} cy={center_y} r={idx_radius} stroke="black" stroke-width="2" fill="none" />
        }
    }).collect::<Vec<_>>();

    let az_lines = (0..az_lines).map(|idx| {
        let rotation = az_increment * idx as f64;
        let transform = format!("rotate({} {} {}) translate({}, {})", rotation, center_x, center_y, center_x, center_y);
        view! {
            <line x1=0 x2={radius} y1=0 y2=0 stroke="black" stroke-width="2" transform={transform}/>
        }  
    }).collect::<Vec<_>>();

    view! {
        { el_circles }
        { az_lines }
    }
}

#[component]
fn Divider() -> impl IntoView {
    view! { <hr/> }
}


