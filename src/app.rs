use leptos::*;
use leptos_meta::*;


#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/tailwind.css"/>
        <Home/>
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


#[component]
fn Home() -> impl IntoView {
    view! {
        <div class="my-0 mx-auto max-w-3xl">
            <h1 class="text-4xl">"Planet Tracker"</h1>
            <Divider/>
            <svg width="1000" height="1000">
                <PolarPlot radius=200 />
            </svg>
        </div>
    }
}