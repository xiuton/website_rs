use dioxus::prelude::*;

// Import the CSS file
static STYLE: &str = include_str!("../assets/playground.css");

#[component]
pub fn Playground() -> Element {
    // Initialize CSS on component mount
    use_effect(move || {
        if let Some(window) = web_sys::window() {
            let document = window.document().unwrap();
            if let Some(head) = document.head() {
                let style = document.create_element("style").unwrap();
                style.set_text_content(Some(STYLE));
                let _ = head.append_child(&style);
            }
        }
    });

    rsx! {
        div { 
            class: "playground-page",
            style: "background-image: url('/static/images/img.png');",

            // Glass gallery container
            div { 
                class: "glass-gallery",
                
                // Bouncing glass element
                div { 
                    class: "glass-container bounce",
                    div { class: "glass-filter" }
                    div { class: "glass-specular" }
                }

                // Floating glass element
                div { 
                    class: "glass-container float",
                    div { class: "glass-filter" }
                    div { class: "glass-specular" }
                }

                // Pulsing glass element
                div { 
                    class: "glass-container pulse",
                    div { class: "glass-filter" }
                    div { class: "glass-specular" }
                }

                // Rotating glass element
                div { 
                    class: "glass-container rotate",
                    div { class: "glass-filter" }
                    div { class: "glass-specular" }
                }

                // Wobble glass element
                div { 
                    class: "glass-container wobble",
                    div { class: "glass-filter" }
                    div { class: "glass-specular" }
                }

                // Jelly glass element
                div { 
                    class: "glass-container jelly",
                    div { class: "glass-filter" }
                    div { class: "glass-specular" }
                }

                // New: Morph glass element
                div { 
                    class: "glass-container morph",
                    div { class: "glass-filter" }
                    div { class: "glass-specular" }
                }

                // New: Splash glass element
                div { 
                    class: "glass-container splash",
                    div { class: "glass-filter" }
                    div { class: "glass-specular" }
                }

                // New: Wave glass element
                div { 
                    class: "glass-container wave",
                    div { class: "glass-filter" }
                    div { class: "glass-specular" }
                }

                // New: Twist glass element
                div { 
                    class: "glass-container twist",
                    div { class: "glass-filter" }
                    div { class: "glass-specular" }
                }

                // New: Bubble glass element
                div { 
                    class: "glass-container bubble",
                    div { class: "glass-filter" }
                    div { class: "glass-specular" }
                }

                // New: Shatter glass element
                div { 
                    class: "glass-container shatter",
                    div { class: "glass-filter" }
                    div { class: "glass-specular" }
                }

                // Realistic glass element
                div { 
                    class: "glass-container realistic",
                    div { class: "glass-filter" }
                    div { class: "glass-distortion" }
                    div { class: "glass-specular" }
                    div { class: "glass-reflection" }
                    div { class: "glass-refraction" }
                    div { class: "glass-highlight" }
                }
            }

            // SVG filter definitions
            svg {
                style: "display: none",
                // Original liquid distortion filter
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

                // Strong wobble filter
                filter {
                    id: "lg-wobble",
                    height: "100%",
                    width: "100%",
                    x: "0%",
                    y: "0%",
                    feTurbulence {
                        base_frequency: "0.015 0.015",
                        num_octaves: "3",
                        result: "noise",
                        seed: "10",
                        "type": "fractalNoise"
                    }
                    feGaussianBlur {
                        "in": "noise",
                        result: "blurred",
                        std_deviation: "3"
                    }
                    feDisplacementMap {
                        "in": "SourceGraphic",
                        in2: "blurred",
                        scale: "100",
                        x_channel_selector: "R",
                        y_channel_selector: "G"
                    }
                }

                // Gentle ripple filter
                filter {
                    id: "lg-ripple",
                    height: "100%",
                    width: "100%",
                    x: "0%",
                    y: "0%",
                    feTurbulence {
                        base_frequency: "0.005 0.005",
                        num_octaves: "2",
                        result: "noise",
                        seed: "30",
                        "type": "fractalNoise"
                    }
                    feGaussianBlur {
                        "in": "noise",
                        result: "blurred",
                        std_deviation: "1"
                    }
                    feDisplacementMap {
                        "in": "SourceGraphic",
                        in2: "blurred",
                        scale: "40",
                        x_channel_selector: "R",
                        y_channel_selector: "G"
                    }
                }

                // New: Wave distortion filter
                filter {
                    id: "lg-wave",
                    height: "100%",
                    width: "100%",
                    x: "0%",
                    y: "0%",
                    feTurbulence {
                        base_frequency: "0.01 0.003",
                        num_octaves: "1",
                        result: "noise",
                        seed: "5",
                        "type": "fractalNoise"
                    }
                    feGaussianBlur {
                        "in": "noise",
                        result: "blurred",
                        std_deviation: "1.5"
                    }
                    feDisplacementMap {
                        "in": "SourceGraphic",
                        in2: "blurred",
                        scale: "50",
                        x_channel_selector: "R",
                        y_channel_selector: "G"
                    }
                }

                // New: Morph distortion filter
                filter {
                    id: "lg-morph",
                    height: "100%",
                    width: "100%",
                    x: "0%",
                    y: "0%",
                    feTurbulence {
                        base_frequency: "0.02 0.02",
                        num_octaves: "4",
                        result: "noise",
                        seed: "15",
                        "type": "fractalNoise"
                    }
                    feGaussianBlur {
                        "in": "noise",
                        result: "blurred",
                        std_deviation: "2.5"
                    }
                    feDisplacementMap {
                        "in": "SourceGraphic",
                        in2: "blurred",
                        scale: "85",
                        x_channel_selector: "R",
                        y_channel_selector: "G"
                    }
                }

                // New: Splash distortion filter
                filter {
                    id: "lg-splash",
                    height: "100%",
                    width: "100%",
                    x: "0%",
                    y: "0%",
                    feTurbulence {
                        base_frequency: "0.04 0.04",
                        num_octaves: "1",
                        result: "noise",
                        seed: "25",
                        "type": "turbulence"
                    }
                    feGaussianBlur {
                        "in": "noise",
                        result: "blurred",
                        std_deviation: "1"
                    }
                    feDisplacementMap {
                        "in": "SourceGraphic",
                        in2: "blurred",
                        scale: "60",
                        x_channel_selector: "R",
                        y_channel_selector: "G"
                    }
                }

                // Liquid Glass Filter - balanced distortion
                filter { 
                    id: "lg-realistic-distortion",
                    height: "200%",
                    width: "200%",
                    x: "-50%",
                    y: "-50%",
                    // Liquid turbulence
                    feTurbulence {
                        base_frequency: "0.02 0.02",
                        num_octaves: "3",
                        result: "turbulence",
                        seed: "42",
                        "type": "fractalNoise"
                    }
                    // Smooth blur
                    feGaussianBlur {
                        "in": "turbulence",
                        result: "blurred-turbulence",
                        std_deviation: "3"
                    }
                    // Glass displacement - moderate distortion
                    feDisplacementMap {
                        "in": "SourceGraphic",
                        in2: "blurred-turbulence",
                        scale: "25",
                        x_channel_selector: "R",
                        y_channel_selector: "G",
                        result: "displacement"
                    }
                    // Subtle color enhancement
                    feColorMatrix {
                        "type": "matrix",
                        "in": "displacement",
                        result: "enhanced",
                        values: "1.05 0 0 0 0
                                0 1.05 0 0 0
                                0 0 1.08 0 0
                                0 0 0 1 0"
                    }
                    // Glass specular lighting
                    feSpecularLighting {
                        "in": "enhanced",
                        result: "specular",
                        lighting_color: "#ffffff",
                        surface_scale: "4",
                        specular_constant: "1.5",
                        specular_exponent: "30",
                        fePointLight {
                            x: "100",
                            y: "100",
                            z: "200"
                        }
                    }
                    // Composite specular
                    feComposite {
                        "in": "specular",
                        in2: "enhanced",
                        operator: "arithmetic",
                        k1: "0",
                        k2: "1",
                        k3: "1",
                        k4: "0",
                        result: "composite"
                    }
                    // Final glass blur
                    feGaussianBlur {
                        "in": "composite",
                        result: "final",
                        std_deviation: "1.5"
                    }
                }

                // Original realistic glass filter (keeping for reference)
                filter {
                    id: "lg-realistic",
                    height: "200%",
                    width: "200%",
                    x: "-50%",
                    y: "-50%",
                    // Gaussian blur for overall softness
                    feGaussianBlur {
                        "in": "SourceGraphic",
                        result: "blur",
                        std_deviation: "2"
                    }
                    // Color matrix for glass tint
                    feColorMatrix {
                        "type": "matrix",
                        "in": "blur",
                        result: "tint",
                        values: "1.1 0 0 0 0
                                0 1.1 0 0 0
                                0 0 1.2 0 0
                                0 0 0 1 0"
                    }
                    // Specular highlight
                    feSpecularLighting {
                        "in": "tint",
                        result: "specular",
                        lighting_color: "#ffffff",
                        surface_scale: "2",
                        specular_constant: "1",
                        specular_exponent: "20",
                        fePointLight {
                            x: "100",
                            y: "100",
                            z: "100"
                        }
                    }
                    // Composite specular over tint
                    feComposite {
                        "in": "specular",
                        in2: "tint",
                        operator: "arithmetic",
                        k1: "0",
                        k2: "1",
                        k3: "1",
                        k4: "0",
                        result: "composite"
                    }
                    // Add subtle distortion
                    feTurbulence {
                        "type": "fractalNoise",
                        base_frequency: "0.01",
                        num_octaves: "2",
                        result: "noise"
                    }
                    feDisplacementMap {
                        "in": "composite",
                        in2: "noise",
                        scale: "10",
                        x_channel_selector: "R",
                        y_channel_selector: "G",
                        result: "displacement"
                    }
                }
            }
        }
    }
} 