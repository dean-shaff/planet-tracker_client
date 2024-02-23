use std::collections::HashMap;

use crate::{
    models::{AstronObject, AstronObjectResponse, SelectedAstronObjectResponse},
    utils::{deg2cardinal, rad2deg},
    AstronObjectsRw, SelectedRw,
};
use chrono::{DateTime, Local, TimeZone, Utc};
use leptos::logging::log;
use leptos::*;

#[component]
pub fn TextDisplayRow(obj: AstronObjectResponse, selected: SelectedRw) -> impl IntoView {
    let (obj, _) = create_signal(obj);

    let highlight = move || {
        if let Some(name) = selected.get() {
            return name == obj.get().name;
        }
        false
    };

    let az_cardinal = move || format!("{:#}", deg2cardinal(rad2deg(obj.get().az)));
    let el = move || format!("{:.2}°", rad2deg(obj.get().el));
    let name = move || obj.get().name.to_string();
    let formatter = "%H:%M";

    let setting_rising_time_view = move || {
        let obj = obj.get();
        let (setting_time, rising_time) = if obj.setting_time > obj.rising_time {
            let rising_time_local: DateTime<Local> =
                Utc.from_local_datetime(&obj.rising_time).unwrap().into();
            (
                "-".to_string(),
                rising_time_local.format(formatter).to_string(),
            )
        } else {
            let setting_time_local: DateTime<Local> =
                Utc.from_local_datetime(&obj.setting_time).unwrap().into();
            (
                setting_time_local.format(formatter).to_string(),
                "-".to_string(),
            )
        };

        view! {
            <td class="hidden sm:table-cell">
                {setting_time}
            </td>
            <td class="hidden sm:table-cell">
                {rising_time}
            </td>
        }
    };

    let magnitude = move || format!("{:.2}", obj.get().magnitude);
    let handle_click = move |_| {
        let obj = obj.get();
        selected.set(Some(obj.name));
    };

    view! {
        <tr
            on:click=handle_click
            style:background-color=move || if highlight() {"rgb(228 228 231)"} else {"white"}
        >
            <td>
                {name}
            </td>
            <td>
                {az_cardinal}
            </td>
            <td>
                {el}
            </td>
            {setting_rising_time_view}
            <td class="hidden sm:table-cell">
                {magnitude}
            </td>
        </tr>
    }
}

#[component]
pub fn TextDisplay(objs: AstronObjectsRw, selected: SelectedRw) -> impl IntoView {
    view! {
        <table class="table-auto divide-y divide-solid">
            <thead>
                <tr>
                    <th class="font-semibold text-left">"Name"</th>
                    <th class="font-semibold text-left">"Direction"</th>
                    <th class="font-semibold text-left">"Elevation"</th>
                    <th class="font-semibold text-left hidden sm:table-cell">"Setting Time"</th>
                    <th class="font-semibold text-left hidden sm:table-cell">"Rising Time"</th>
                    <th class="font-semibold text-left hidden sm:table-cell">"Apparent Magnitude"</th>
                </tr>
            </thead>
            <tbody class="divide-y divide-solid">
                <For
                    each=move || objs.get()
                    key=|obj| (obj.name.clone())
                    children=move |obj: AstronObjectResponse| {
                        view! {
                            <TextDisplayRow obj=obj selected=selected/>
                        }
                    }
                />
            </tbody>
        </table>
    }
}
