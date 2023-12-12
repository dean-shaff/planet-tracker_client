use chrono::{Utc, TimeZone, Local, DateTime};
use leptos::*;

use crate::{models::AstronObjectResponse, utils::{rad2deg, deg2cardinal}};


#[component]
pub fn TextDisplayRow(obj: AstronObjectResponse) -> impl IntoView 
{
    // let az = format!("{:.2}°", rad2deg(obj.az));
    let az_cardinal = format!("{:#}", deg2cardinal(rad2deg(obj.az)));
    let el = format!("{:.2}°", rad2deg(obj.el));
    let formatter = "%H:%M";

    let (setting_time, rising_time) = if obj.setting_time > obj.rising_time {
        let rising_time_local: DateTime<Local> = Utc.from_local_datetime(&obj.rising_time).unwrap().into();
        ("-".to_string(), rising_time_local.format(formatter).to_string())
    } else {
        let setting_time_local: DateTime<Local> = Utc.from_local_datetime(&obj.setting_time).unwrap().into();
        (setting_time_local.format(formatter).to_string(), "-".to_string())
    };

    let magnitude = format!("{:.2}", obj.magnitude);
    
    view! {
        <tr>
            <td>
                {obj.name.to_string()}
            </td>
            <td>
                {az_cardinal}
            </td>
            <td>
                {el.clone()}
            </td>
            <td class="hidden sm:table-cell">
                {setting_time}
            </td>
            <td class="hidden sm:table-cell">
                {rising_time}
            </td>
            <td class="hidden sm:table-cell">
                {magnitude}
            </td>
        </tr>
    }
}


#[component]
pub fn TextDisplay(objs: Vec<AstronObjectResponse>) -> impl IntoView 
{

    let rows = move || {
        objs
            .iter()
            .map(|obj| {
                view! {
                    <TextDisplayRow obj=obj.clone()/>
                }
            })
            .collect_view()
    };

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
                {rows}
            </tbody>
        </table>
    }
}