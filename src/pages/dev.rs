use dioxus::prelude::*;
use wasm_bindgen_futures::spawn_local;
use crate::utils::title;
use crate::utils::constants;
use crate::utils::dev_helpers::{
    create_delayed_hide_timer, create_carousel_timer,
    fetch_and_set_random_image, load_background_wall_images,
    HIDE_BTN_DELAY_MS,
};

#[component]
pub fn Dev() -> Element {
    use_effect(move || {
        title::set_page_title("开发 - 干徒");
        
    });

    let img_url = use_signal(|| None::<String>);
    let is_background_mode = use_signal(|| false);
    let background_images = use_signal(Vec::new);
    let current_bg_index = use_signal(|| 0);
    let bg_timer_handle = use_signal(|| None::<i32>);
    let mut show_exit_button = use_signal(|| false);
    let mut hide_btn_timer = use_signal(|| None::<i32>);
    let mut hide_cursor = use_signal(|| false);
    let default_bg_image = constants::DEFAULT_BG_IMAGE;

    let fetch_random_img = move |_evt: Event<MouseData>| {
        fetch_and_set_random_image(img_url);
    };

    let mut enter_background_mode = {
        let mut is_background_mode = is_background_mode;
        move || {
            is_background_mode.set(true);
            let mut background_images = background_images;
            let mut bg_timer_handle = bg_timer_handle;

            if background_images().len() > 1 {
                let handle = create_carousel_timer(background_images, current_bg_index);
                bg_timer_handle.set(Some(handle));
                return;
            }

            if background_images().is_empty() {
                background_images.set(vec![default_bg_image.to_string()]);
            }

            spawn_local(load_background_wall_images(
                background_images,
                current_bg_index,
                bg_timer_handle,
                default_bg_image,
            ));
        }
    };

    let mut exit_background_mode = {
        let mut is_background_mode = is_background_mode;
        let mut bg_timer_handle = bg_timer_handle;
        move || {
            is_background_mode.set(false);
            if let Some(handle) = bg_timer_handle() {
                web_sys::window().expect("Failed to get window").clear_interval_with_handle(handle);
                bg_timer_handle.set(None);
            }
        }
    };

    use_effect(move || {
        if img_url().is_none() {
            fetch_and_set_random_image(img_url);
        }
        
    });

    // 预加载背景墙图片
    use_effect(move || {
        for url in background_images().iter() {
            let img = web_sys::HtmlImageElement::new().expect("Failed to create HtmlImageElement");
            img.set_src(url);
        }
    });

    rsx! {
        div { class: "dev-container",
            div { class: "identification",
                div { class: "content",
                    span { class: "hole"}
                    div { class: "header" }
                    div { class: "default",
                        div { class: "before", "Ganto" }
                        div { class: "middle", "." }
                        div { class: "after", "Me" }
                }
                    div { class: "foot" }
                }
            }
            div { class: "dev-img-wrapper",
                if let Some(url) = img_url() {
                    img {
                        src: url,
                        class: "dev-img"
                    }
                }
                div {
                    class: "dev-btns",
                    button {
                        class: "img-switch-btn",
                        onclick: fetch_random_img,
                        svg {
                            xmlns: "http://www.w3.org/2000/svg",
                            view_box: "0 0 24 24",
                            width: "24",
                            height: "24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            path {
                                d: "M23 4v6h-6M1 20v-6h6M3.51 9a9 9 0 0 1 14.85-3.36L23 10M1 14l4.64 4.36A9 9 0 0 0 20.49 15"
                            }
                        }
                    }
                    button {
                        class: "background-mode-btn",
                        onclick: move |_| {
                            enter_background_mode();
                        },
                        svg {
                            xmlns: "http://www.w3.org/2000/svg",
                            view_box: "0 0 24 24",
                            width: "24",
                            height: "24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            path {
                                d: "m15.75 10.5 4.72-4.72a.75.75 0 0 1 1.28.53v11.38a.75.75 0 0 1-1.28.53l-4.72-4.72M4.5 18.75h9a2.25 2.25 0 0 0 2.25-2.25v-9a2.25 2.25 0 0 0-2.25-2.25h-9A2.25 2.25 0 0 0 2.25 7.5v9a2.25 2.25 0 0 0 2.25 2.25Z"
                            }
                        }
                    }
                }
            }
            // 最上层的背景墙元素
            if is_background_mode() {
                div { 
                    class: if hide_cursor() { "background-wall hide-cursor" } else { "background-wall" },
                    onmousemove: move |_| {
                        show_exit_button.set(true);
                        hide_cursor.set(false);
                        if let Some(handle) = hide_btn_timer() {
                            web_sys::window().expect("Failed to get window").clear_timeout_with_handle(handle);
                        }
                        let handle = create_delayed_hide_timer(
                            show_exit_button,
                            hide_btn_timer,
                            hide_cursor,
                            HIDE_BTN_DELAY_MS,
                        );
                        hide_btn_timer.set(Some(handle));
                    },
                    onmouseleave: move |_| {
                        show_exit_button.set(false);
                        hide_cursor.set(true);
                        if let Some(handle) = hide_btn_timer() {
                            web_sys::window().expect("Failed to get window").clear_timeout_with_handle(handle);
                            hide_btn_timer.set(None);
                        }
                    },
                    {background_images().iter().enumerate().map(|(index, url)| {
                        rsx! {
                            img {
                                key: "{url}",
                                src: url.clone(),
                                class: format_args!("background-wall-img {}", if index == current_bg_index() { "active" } else { "" })
                            }
                        }
                    })}
                    div { 
                        class: "exit-button-container",
                        style: "position: fixed; top: 20px; right: 20px; z-index: 1000;",
                        onmouseenter: move |_| {
                            show_exit_button.set(true);
                            hide_cursor.set(false);
                            if let Some(handle) = hide_btn_timer() {
                                web_sys::window().expect("Failed to get window").clear_timeout_with_handle(handle);
                                hide_btn_timer.set(None);
                            }
                        },
                        onmouseleave: move |_| {
                            let handle = create_delayed_hide_timer(
                                show_exit_button,
                                hide_btn_timer,
                                hide_cursor,
                                HIDE_BTN_DELAY_MS,
                            );
                            hide_btn_timer.set(Some(handle));
                        },
                        button {
                            class: "exit-background-btn",
                            onclick: move |_| {
                                exit_background_mode();
                            },
                            svg {
                                xmlns: "http://www.w3.org/2000/svg",
                                view_box: "0 0 24 24",
                                width: "24",
                                height: "24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "2",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                path {
                                    d: "M6 6l12 12M6 18L18 6"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::dev_helpers::extract_filenames;

    #[test]
    fn test_extract_filenames_empty() {
        let result = extract_filenames("");
        assert!(result.is_empty());
    }

    #[test]
    fn test_extract_filenames_invalid_json() {
        let result = extract_filenames("not json");
        assert!(result.is_empty());
    }

    #[test]
    fn test_extract_filenames_valid() {
        let json = r#"{"code":200,"message":"success","data":{"data":["https://yun.ganto.cn/f/img1.jpg","https://yun.ganto.cn/f/img2.jpg"],"total":2}}"#;
        let result = extract_filenames(json);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], "https://yun.ganto.cn/f/img1.jpg");
        assert_eq!(result[1], "https://yun.ganto.cn/f/img2.jpg");
    }

    #[test]
    fn test_extract_filenames_single() {
        let json = r#"{"code":200,"message":"success","data":{"data":["https://yun.ganto.cn/f/img.jpg"],"total":1}}"#;
        let result = extract_filenames(json);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], "https://yun.ganto.cn/f/img.jpg");
    }

    #[test]
    fn test_extract_filenames_empty_array() {
        let json = r#"{"code":200,"message":"success","data":{"data":[],"total":0}}"#;
        let result = extract_filenames(json);
        assert!(result.is_empty());
    }

    #[test]
    fn test_extract_filenames_missing_data_field() {
        let json = r#"{"code":200,"message":"success"}"#;
        let result = extract_filenames(json);
        assert!(result.is_empty());
    }

    #[test]
    fn test_extract_filenames_trim_whitespace() {
        let json = r#"{"data":{"data":["  https://yun.ganto.cn/f/img.jpg  "]}}"#;
        let result = extract_filenames(json);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], "https://yun.ganto.cn/f/img.jpg");
    }
}