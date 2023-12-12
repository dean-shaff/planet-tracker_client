use std::slice::RSplit;

use chrono::{DateTime, Utc, Local, TimeZone};
use leptos::*;

use crate::{models::{SearchItem, SearchResponse, SearchQueryParams, Position, AstronObjectResponse, AstronObject}, errors::AppError, api::search};

async fn geo_search(query: Option<String>) -> Result<Option<SearchResponse>, AppError>
{
    if let Some(query) = query {
        let query = SearchQueryParams { q: query, max_results: 5, fuzzy: 0.8 };
        search(query).await.map(|r| Some(r))
    } else {
        Ok(None)
    }
}


#[component]
pub fn GeoSearch() -> impl IntoView
{
    let (hidden, set_hidden) = create_signal(true);
    let (query, set_query) = create_signal::<Option<String>>(None);
    
    let position_time_rw = use_context::<RwSignal<(Position, DateTime<Utc>)>>().unwrap();

    let search_results = create_resource(query, geo_search);

    let on_change = move |evt: web_sys::Event| {
        let value = event_target_value(&evt);
        if value != "" {
            set_query.set(Some(value));
        } else {
            set_query.set(None);
            set_hidden.set(true);
        }
    };

    let success_view = move || {
        search_results.and_then(|data| {
            match data {
                None => {
                    set_hidden.set(true);
                    view! { <></> }.into_view()
                },
                Some(data) => {
                    set_hidden.set(false);
                    let items = data.items
                        .clone()
                        .into_iter()
                        .map(|i| {
                            let name = i.name.clone();
                            let paranthetical_text = format!(" ({}, {})", i.country, i.sub_division);
                            view! {
                                <div
                                    class="flex hover:bg-gray-200 w-full py-1 px-2"
                                    on:click=move |_| { 
                                        position_time_rw.update(|pt: &mut (Position, DateTime<Utc>)| {
                                            let new_pos = Position::from(i.clone());
                                            *pt = (new_pos, pt.1);
                                        });
                                        // set_geo_location.set(Some(i.clone()));
                                        set_hidden.set(true);
                                    }
                                >
                                    <span class="font-medium">{name.clone()}</span><span class="whitespace-pre">{paranthetical_text}</span>
                                </div>
                            }
                        }).collect_view();
                    items.into_view()
                }
            }
        })
    };

    view! {
        <div>
            <label class="font-bold">"Geolocation Search"</label>
            <input 
                type="search"
                placeholder="Try \"Berlin\" or \"Mumbai\""
                on:input=on_change
                class=("search-input-active", move || ! hidden.get())
                class=("search-input-inactive", move || hidden.get())
                class="py-1 px-2 border-gray-300 hover:border-gray-500 focus:border-gray-700 focus:outline-none w-full"
            />
            <Transition fallback=move || { view! {<div>"Loading..."</div>}}>
                <ErrorBoundary fallback=move |_| { view! {<div>"Error!"</div>}}>
                    <div
                        class:hidden={move || hidden.get()}
                        class="flex flex-col divide-y divide-solid rounded-b-md border border-solid border-1 border-gray-300"
                    >
                        { success_view }
                    </div>  
                </ErrorBoundary>
            </Transition>
        </div>
    }
}


#[component]
pub fn DateTimeSearch(sun: AstronObjectResponse) -> impl IntoView
{
    let position_time_rw = use_context::<RwSignal<(Position, DateTime<Utc>)>>().unwrap();
    let now = Utc::now();
    
    let when = Utc.from_local_datetime(&sun.when).unwrap();

    let setting_time = Utc.from_local_datetime(&sun.setting_time).unwrap();
    let rising_time = Utc.from_local_datetime(&sun.rising_time).unwrap();

    logging::log!("when={:?}, now={:?}, setting_time={:?}, rising_time={:?}", when, now, setting_time, rising_time);

    let on_click_factory = move |dt: DateTime<Utc>| {
        move |_evt: web_sys::MouseEvent| {
            position_time_rw.update(|pt| {
                *pt = (pt.0.clone(), dt)
            })
        }
    };

    if when > now {
        return view! {
            <label class="font-bold">"Date/time selector"</label>
            <button class="
                rounded-md
                border
                border-solid
                py-1
                px-2
                border-gray-300
                hover:border-gray-500
                focus:border-gray-700
                focus:outline-none
                w-full"
                on:click=on_click_factory(now)
            >"Now"</button>        
        }
    }

    let options =  if rising_time > setting_time {
        view! {
            <option on:click=on_click_factory(setting_time)>"Today at sunset"</option>
            <option on:click=on_click_factory(rising_time)>"Tomorrow at dawn"</option>
        }.into_view()
    } else {
        view! {
            <option on:click=on_click_factory(rising_time)>"Tomorrow at dawn"</option>
            <option on:click=on_click_factory(setting_time)>"Tomorrow at sunset"</option>
        }.into_view()
    };

    view! {
        <label class="font-bold">"Date/time selector"</label>
        <select class="rounded-md border border-solid py-1 px-2 border-gray-300 hover:border-gray-500 focus:border-gray-700 focus:outline-none w-full">
            <option value="now" on:click=on_click_factory(now)>"Now"</option>
            {options}
        </select>
    }
}


#[component]
pub fn GeoDateTimeSearch(objs: Vec<AstronObjectResponse>) -> impl IntoView
{
    let sun = objs
        .iter()
        .find(|obj| obj.name == AstronObject::Sun)
        .unwrap()
        .clone();
    let position_time_rw = use_context::<RwSignal<(Position, DateTime<Utc>)>>().unwrap();
    let text_display = move || {
        
        let (position, time) = position_time_rw.get();
        let time_local = DateTime::<Local>::from(time); 

        view! {
            "Showing Ephemerides for "
            <span class="font-bold">{format!("{:.2}", position.lat)}"°N, "{format!("{:.2}", position.lon)}"°E"</span>
            " at "
            <span class="font-bold">{time_local.format("%H:%M:%S").to_string()}</span>
            " on "
            <span class="font-bold">{time_local.format("%A, %e %B %Y").to_string()}</span>    
        }
    };

    view! { 
        <div class="flex flex-col space-y-2 sm:flex sm:flex-row sm:space-x-2">
            <div class="sm:flex-1">
                <GeoSearch/>
                <DateTimeSearch sun=sun/>
            </div>
            <div class="sm:flex-1">
                {text_display}
            </div>
        </div>
    }

}