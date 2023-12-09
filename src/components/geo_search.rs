use chrono::{DateTime, Utc};
use leptos::*;

use crate::{models::{SearchItem, SearchResponse, SearchQueryParams, Position}, errors::AppError, api::search};



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
    
    let set_position_time = use_context::<WriteSignal<(Position, DateTime<Utc>)>>().unwrap();

    let search_results = create_resource(query, geo_search);


    let on_change = move |evt: web_sys::Event| {
        let value = event_target_value(&evt);
        logging::log!("value={}", value);
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
                                        set_position_time.update(|pt: &mut (Position, DateTime<Utc>)| {
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
            <input 
                type="search"
                placeholder="Geolocation Search"
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


