use dioxus::prelude::*;

// Import the CSS file
static STYLE: &str = include_str!("../assets/playground.css");

#[component]
pub fn Playground() -> Element {
    let mut glass_container = use_signal(|| None::<web_sys::Element>);
    let mut playground_page = use_signal(|| None::<web_sys::Element>);
    
    // Initialize mouse tracking and CSS on component mount
    use_effect(move || {
        if let Some(window) = web_sys::window() {
            let document = window.document().unwrap();
            
            // Add CSS to document head
            if let Some(head) = document.head() {
                let style = document.create_element("style").unwrap();
                style.set_text_content(Some(STYLE));
                let _ = head.append_child(&style);
            }
            
            // Initialize element references
            if let Ok(container) = document.query_selector(".glass-container") {
                if let Some(element) = container {
                    glass_container.set(Some(element));
                }
            }
            if let Ok(page) = document.query_selector(".playground-page") {
                if let Some(element) = page {
                    playground_page.set(Some(element));
                }
            }
        }
    });

    // Mouse move handler
    let onmousemove = move |e: Event<MouseData>| {
        if let Some(container) = glass_container() {
            if let Some(page) = playground_page() {
                // Get page element's bounding rectangle
                let rect = page.get_bounding_client_rect();
                // Calculate mouse position relative to the playground-page element
                let x = e.client_coordinates().x - rect.left();
                let y = e.client_coordinates().y - rect.top();
                let _ = container.set_attribute("style", &format!("transform: translate(-50%, -50%) translate({}px, {}px);", x, y));
            }
        }
    };

    rsx! {
        div { 
            class: "playground-page",
            onmousemove: onmousemove,
            style: "height: 100vh; background: url('https://files.ganto.cn/files/123.jpg') center no-repeat; background-size: cover; margin: 0; position: relative;",

            // Glass container
            div { 
                class: "glass-container",
                // Glass filter with liquid effect
                div { class: "glass-filter" }
                // Edge specular effect
                div { class: "glass-specular" }
            }

            // SVG filter definition
            svg {
                style: "display: none",
                filter {
                    id: "lg-dist",
                    height: "100%",
                    width: "100%",
                    x: "0%",
                    y: "0%",
                    feTurbulence {
                        base_frequency: "0.008 0.008",
                        num_octaves: "2",
                        result: "noise",
                        seed: "92",
                        "type": "fractalNoise"
                    }
                    feGaussianBlur {
                        "in": "noise",
                        result: "blurred",
                        std_deviation: "2"
                    }
                    feDisplacementMap {
                        "in": "SourceGraphic",
                        in2: "blurred",
                        scale: "70",
                        x_channel_selector: "R",
                        y_channel_selector: "G"
                    }
                }
            }
        }
    }
} 