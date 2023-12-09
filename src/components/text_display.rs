use leptos::*;

use crate::{models::AstronObjectResponse, utils::{rad2deg, deg2cardinal}};


#[component]
pub fn TextDisplay(objs: Vec<AstronObjectResponse>) -> impl IntoView 
{

    let rows = move || {
        objs
            .iter()
            .map(|obj| {
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