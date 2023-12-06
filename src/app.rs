use std::{f64::consts::{PI, FRAC_PI_2, TAU}, cmp, thread::current};

use chrono::{DateTime, Utc, Duration};
use leptos::{*, leptos_dom::Element};
use leptos_meta::*;
use enum_iterator::all;
use futures::future::join_all;
use wasm_bindgen::JsCast;
use web_sys::{MouseEvent};

use crate::{
    api::{get_astron_object_data, search}, 
    models::{AstronObjectResponse, AstronObject, AstronObjectQueryParams, CardinalDirection, SearchResponse, SearchQueryParams, SearchItem}, errors::AppError
};


#[derive(Debug, PartialEq, Clone)]
struct Position {
    lat: f64,
    lon: f64,
    elevation: f64
}


async fn geo_search(query: Option<String>) -> Result<Option<SearchResponse>, AppError>
{
    if let Some(query) = query {
        let query = SearchQueryParams { q: query, max_results: 5 };
        search(query).await.map(|r| Some(r))
    } else {
        Ok(None)
    }

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
        },
        // AstronObjectResponse { 
        //     name: AstronObject::Moon, 
        //     magnitude: 0.0, 
        //     size: 4.0, 
        //     az: 1.74, 
        //     el: -1.13, 
        //     ra: 0.0, 
        //     dec: 0.0, 
        //     setting_time: when.naive_utc() + Duration::hours(2), 
        //     rising_time: when.naive_utc() + Duration::hours(5), 
        //     when: when.naive_utc()
        // }
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
                    // { divider(idx) }
                }
            })
            .collect_view()
    };

    view! {
        
        <div class="flex flex-col w-full divide-y divide-solid">
            <div class="flex py-2">
                <div class="font-semibold flex-1 pr-2">"Name"</div>
                <div class="font-semibold flex-1 pr-2">"Cardinal Direction"</div>
                <div class="font-semibold flex-1 pr-2">"Azimuth/Elevation"</div>
                <div class="font-semibold flex-1 pr-2">"Setting Time"</div>
                <div class="font-semibold flex-1">"Rising Time"</div>
            </div>
            {rows}
        </div>
    }
}



#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    let (width, set_width) = create_signal(300_usize);
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

    let svg_parent: NodeRef<leptos::html::Div> = create_node_ref();

    create_effect(move |_| {
        let elem = svg_parent.get().expect("<div> to exist");
        let width = elem.offset_width();
        logging::log!("width={}", width);
        if width > 0 {
            set_width.set(cmp::min(width as usize, 500));
        }
    });


    let success_view = move || {
        astron_objs.and_then(|data| {
            view! {
                <div _ref={svg_parent} class="flex flex-col content-center justify-center space-y-1">
                    <GeoSearch/>
                    <TextDisplay objs={data.clone()}/>
                    <div>
                        <PolarPlot width={width.get()} height={width.get()} radius={2 * width.get() / 5} objs={data.clone()}/>
                    </div>
                </div>
            }
        })
    };

    view! {
        <Stylesheet id="leptos" href="/pkg/tailwind.css"/>
        <div class="my-0 mx-auto max-w-3xl">
            <h1 class="text-4xl my-2">"Planet Tracker"</h1>
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
pub fn GeoSearch() -> impl IntoView
{
    let (hidden, set_hidden) = create_signal(false);
    let (query, set_query) = create_signal::<Option<String>>(None);
    let (geo_location, set_geo_location) = create_signal::<Option<SearchItem>>(None);
    let search_results = create_resource(query, geo_search);

    let on_change = move |evt: web_sys::Event| {
        let value = event_target_value(&evt);
        if value != "" {
            set_query.set(Some(value));
            set_hidden.set(false);
        } else {
            set_query.set(None);
            set_hidden.set(true);
        }
    };

    let success_view = move || {
        logging::log!("success_view!");
        search_results.and_then(|data| {
            match data {
                None => view! { <div>"None"</div> }.into_view(),
                Some(data) => {
                    let items = data.items
                        .clone()
                        .into_iter()
                        .map(|i| {
                            let name = i.name.clone();
                            view! {
                                <div
                                    class="flex hover:bg-gray-200 w-full"
                                    on:click=move |_| { 
                                        set_geo_location.set(Some(i.clone()));
                                        set_hidden.set(true);
                                    }
                                >
                                    {name.clone()}
                                </div>
                            }
                        }).collect_view();
                    items.into_view()
                }
            }
        })
    };

    view! {
        <input type="search" placeholder="Search" on:input=on_change class="rounded-md py-1 px-2 border border-solid border-1 border-gray-300"/>
        <Transition fallback=move || { view! {<div>"Loading..."</div>}}>
            <ErrorBoundary fallback=move |_| { view! {<div>"Error!"</div>}}>
                <div
                    class:hidden={move || hidden.get()}
                    class="flex flex-col divide-y divide-solid"
                >
                    { success_view }
                </div>  
            </ErrorBoundary>
        </Transition>
    }
}


// #[component]
// pub fn GeoSearch() -> impl IntoView
// {
//     let (hide_search, set_hide_search) = create_signal(false);
//     let (geo_location, set_geo_location) = create_signal::<Option<SearchItem>>(None);
//     let (query, set_query) = create_signal::<Option<String>>(None);
//     let search_results = create_resource(query, geo_search);

//     create_effect(move |_| {
//         logging::log!("hide_search={}", hide_search.get());
//     });

//     let on_change = move |evt: web_sys::Event| 
//     {
//         let value = event_target_value(&evt);
//         if value != "" {
//             set_query.set(Some(value))
//         }
//     };

//     let fallback = move |errors: RwSignal<Errors>| {
//         let error_strs: Vec<String> = errors
//             .get()
//             .iter()
//             .map(|(_, e)| e.to_string())
//             .collect();
//         logging::log!("error_strs={:?}", error_strs);
//     };

//     let success_view = move || {
//         view! {<></>}
//         // search_results.and_then(|data| {
//         //     logging::log!("data={:?}", data);
//         //     match data {
//         //         None => view! {<></>}.into_view(),
//         //         Some(data) => {
//         //             set_hide_search.set(false);
//         //             let items = data.items.clone()
//         //                 .into_iter()
//         //                 .map(|i| {
//         //                     let name = i.name.clone();
//         //                     view! {
//         //                         <div
//         //                             class="flex hover:bg-gray-200 w-full"
//         //                             on:click=move |_| { 
//         //                                 set_geo_location.set(Some(i.clone()));
//         //                                 set_hide_search.set(true)
//         //                             }
//         //                         >
//         //                             {name.clone()}
//         //                         </div>
//         //                     }
//         //                 }).collect_view();
//         //             items.into_view()
//         //         }
//         //     }
//         // })
//     };


//     view! {
//         <></>
//         // <input placeholder="Search" on:change=on_change class="rounded-md py-1 px-2 border border-solid border-1 border-gray-300"/>
//         // <Transition fallback=move || view! {<div>"Loading..."</div>}>
//         //     <ErrorBoundary fallback>
//         //         <div
//         //             // class:hidden={move || hide_search.get()} 
//         //             class="flex flex-col divide-y divide-solid"
//         //         >
//         //             { success_view }
//         //         </div>
//         //     </ErrorBoundary>
//         // </Transition>
//     }
// }



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
            // let obj_name = obj.to_string();
            let obj_size = resp.size;
            // let tooltip_height = radius_f64 / 8.0;
            // let tooltip_width = (radius_f64 / 5.0) + (obj_name.len() as f64 - 4.0) * radius_f64 / 30.0;
            // let y_offset = radius_f64 / 8.0 + obj.size;
            // let x_offset = tooltip_width / 2.0;
            // let node_ref = create_node_ref();
            let on_click = move |evt: MouseEvent| {
                let tooltip_val = tooltip.get();
                if let Some(current_obj) = tooltip_val.obj.clone() {
                    if current_obj == obj {
                        set_tooltip(Tooltip { visible: !tooltip_val.visible, ..tooltip_val});
                        return
                    }
                }


                logging::log!("x={}, y={}, offset_x={}, offset_y={} cx={}, cy={}", evt.x(), evt.y(), evt.offset_x(), evt.offset_y(), cx, cy);
                // let html_elem = node_ref.get().expect("<circle> exists");
                // let elem = html_elem.as_ref().get_bounding_client_rect();
                let target = evt.target().expect("target exists");
                let div: web_sys::Element = target.dyn_into().unwrap();
                let rect = div.get_bounding_client_rect();
                logging::log!("rect.x={}, rect.y={}", rect.x(), rect.y());
                set_tooltip(Tooltip { x: rect.x() + obj_size, y: rect.y(), obj: Some(obj.clone()), visible: true })
            };
            
            view! {
                <circle 
                    // node_ref={node_ref}
                    cx={cx} 
                    cy={cy} 
                    fill={fill} 
                    r={obj_size}
                    on:click=on_click
                />
                // <rect 
                //     x={cx - x_offset} 
                //     y={cy - y_offset}
                //     width={tooltip_width}
                //     height={tooltip_height}
                //     rx="5"
                //     ry="5"
                //     fill="#1f2937"
                //     opacity="0.7"
                // />
                // <text 
                //     x={cx - x_offset + tooltip_width / 2.0} 
                //     y={cy - y_offset + tooltip_height / 2.0} 
                //     font-size={tooltip_height / 2.0} 
                //     font-family="monospace"
                //     fill="#f8fafc" // slate-50
                //     dominant-baseline="middle"
                //     text-anchor="middle"
                // >
                //     {obj_name}
                // </text>
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
        <div>
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

#[component]
fn Divider() -> impl IntoView {
    view! { <hr/> }
}


