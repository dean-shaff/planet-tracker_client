use chrono::{DateTime, Utc, Duration};
use leptos::{*, html::Div};
use leptos_use::{use_element_size, UseElementSizeReturn, use_resize_observer};
use logging::log;
use leptos_meta::*;
use enum_iterator::all;
use futures::future::join_all;

use crate::{
    api::get_astron_object_data, 
    models::{
        AstronObjectResponse,
        AstronObject,
        AstronObjectQueryParams, Position,
    }, 
    errors::AppError, 
    components::{
        GeoSearch,
        TextDisplay, 
        PolarPlot
    }
};


pub const MIN_POLAR_PLOT_WIDTH: usize = 300;


async fn get_all_astron_object_data_dummy(position_time: (Position, DateTime<Utc>)) -> Result<Vec<AstronObjectResponse>, AppError>
{
    let (_, when) = position_time;
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
    ]);
    res
}

async fn get_all_astron_object_data(position_time: (Position, DateTime<Utc>)) -> Result<Vec<AstronObjectResponse>, AppError>
{
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

#[component]
pub fn AppInner(geo_position: Position) -> impl IntoView {
    let (position_time, set_position_time) = create_signal::<(Position, DateTime<Utc>)>((geo_position, Utc::now()));

    provide_context(set_position_time);
    
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

    let el = create_node_ref::<Div>();

    let (width, set_width) = create_signal(MIN_POLAR_PLOT_WIDTH);

    use_resize_observer(el, move |entries, _| {
        let rect = entries[0].content_rect();
        let rect_width = rect.width() as usize;
        if rect_width != width.get_untracked() {
            log!("callback: width={}", rect_width);
            set_width.set(rect_width);
        }
    });

    
    let radius = Signal::derive(move || 2 * width.get() / 5);

    let success_view = move || {

        let info_text = move || {
            let (position, time) = position_time.get();
            view! {
                <div class="border border-1 rounded-md py-1 px-2 border-gray-300">
                    "Showing Ephemerides for "
                    <span class="font-bold">{format!("{:.2}", position.lat)}"°N, "</span>
                    <span class="font-bold">{format!("{:.2}", position.lon)}"°E"</span>
                    " at "
                    <span class="font-bold">{time.format("%H:%M:%S").to_string()}</span>
                    " on "
                    <span class="font-bold">{time.format("%A, %e %B %Y").to_string()}</span>
                </div>
            }
        };

        astron_objs.and_then(|data| {
            view! {
                <>
                    <GeoSearch/>
                    {info_text}
                    <TextDisplay objs={data.clone()}/>
                    <div>
                        <PolarPlot width={width.get()} height={width.get()} radius={radius.get()} objs={data.clone()}/>
                    </div>
                </>
            }
        })
    };

    view! {
        <>
            <h1 class="text-4xl my-2 mx-2">"Planet Tracker"</h1>
            <Transition fallback=move || { view! {<div>"Loading..."</div>}}>
                <ErrorBoundary fallback>
                <div node_ref=el class="flex flex-col content-center justify-center space-y-1 mx-2 sm:mx-0">
                    { success_view }
                </div>  
                </ErrorBoundary>
            </Transition>
        </>
    }
}


#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    let geo_position = Position::from_browser().expect("Can get geolocation");

    create_effect(move |_| {
        log!("geo_position={:?}", geo_position.get());
    });


    view! {
        <Stylesheet id="leptos" href="/pkg/tailwind.css"/>
        <div class="my-0 mx-auto max-w-3xl">
            <Show 
                when=move || geo_position.get().is_some()
                fallback=move || view ! { <div>"Waiting for geo location"</div>}
            > 
                <AppInner geo_position={geo_position.get().unwrap()}/>
            </Show>
        </div>

    }
}
