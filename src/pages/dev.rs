use dioxus::prelude::*;
use wasm_bindgen_futures::spawn_local;
use wasm_bindgen::JsCast;
use crate::utils::title;

const BG_IMG_COUNT: u32 = 5;
const HIDE_BTN_DELAY_MS: i32 = 3000;
const CAROUSEL_INTERVAL_MS: i32 = 15000;

fn extract_filenames(text: &str) -> Vec<String> {
    if let Ok(obj) = serde_json::from_str::<serde_json::Value>(text) {
        if let Some(inner) = obj["data"].as_object() {
            if let Some(arr) = inner["data"].as_array() {
                return arr.iter().filter_map(|v| v.as_str().map(|s| s.trim().to_string())).collect();
            }
        }
    }
    Vec::new()
}

fn create_delayed_hide_timer(
    mut show_exit_button: Signal<bool>,
    mut hide_btn_timer: Signal<Option<i32>>,
    mut hide_cursor: Signal<bool>,
    delay_ms: i32,
) -> i32 {
    let closure = wasm_bindgen::closure::Closure::wrap(Box::new(move || {
        show_exit_button.set(false);
        hide_cursor.set(true);
        hide_btn_timer.set(None);
    }) as Box<dyn FnMut()>);
    let handle = web_sys::window().expect("Failed to get window")
        .set_timeout_with_callback_and_timeout_and_arguments_0(
            closure.as_ref().unchecked_ref(),
            delay_ms,
        )
        .expect("Failed to set timeout");
    closure.forget();
    handle
}

fn create_carousel_timer(
    background_images: Signal<Vec<String>>,
    mut current_bg_index: Signal<usize>,
) -> i32 {
    let closure = wasm_bindgen::closure::Closure::wrap(Box::new(move || {
        let len = background_images().len();
        if len > 0 {
            current_bg_index.set((current_bg_index() + 1) % len);
        }
    }) as Box<dyn FnMut()>);
    let handle = web_sys::window().expect("Failed to get window")
        .set_interval_with_callback_and_timeout_and_arguments_0(
            closure.as_ref().unchecked_ref(),
            CAROUSEL_INTERVAL_MS,
        )
        .expect("Failed to set interval");
    closure.forget();
    handle
}

fn load_single_image(url: &str) -> std::pin::Pin<Box<dyn std::future::Future<Output = bool>>> {
    let url = url.to_string();
    Box::pin(async move {
        let img = web_sys::HtmlImageElement::new().expect("Failed to create HtmlImageElement");
        let (tx, rx) = futures_channel::oneshot::channel();
        let tx_success = std::sync::Arc::new(std::sync::Mutex::new(Some(tx)));
        let tx_error = tx_success.clone();
        let success_callback = wasm_bindgen::closure::Closure::wrap(
            Box::new(move || { if let Some(tx) = tx_success.lock().expect("Failed to lock mutex").take() { let _ = tx.send(true); } }) as Box<dyn FnMut()>
        );
        let error_callback = wasm_bindgen::closure::Closure::wrap(
            Box::new(move || { if let Some(tx) = tx_error.lock().expect("Failed to lock mutex").take() { let _ = tx.send(false); } }) as Box<dyn FnMut()>
        );
        img.set_onload(Some(success_callback.as_ref().unchecked_ref()));
        img.set_onerror(Some(error_callback.as_ref().unchecked_ref()));
        img.set_src(&url);
        success_callback.forget();
        error_callback.forget();
        rx.await.unwrap_or(false)
    })
}

fn fetch_and_set_random_image(mut img_url: Signal<Option<String>>) {
    spawn_local(async move {
        let resp = gloo_net::http::Request::get("https://yun.ganto.cn/api/v1/images/random/1").send().await;
        if let Ok(response) = resp {
            if let Ok(text) = response.text().await {
                let filenames = extract_filenames(&text);
                if let Some(url) = filenames.first() {
                    img_url.set(Some(url.clone()));
                }
            }
        }
    });
}

#[component]
pub fn Dev() -> Element {
    use_effect(move || {
        title::set_page_title("开发 - 干徒");
        ()
    });

    let img_url = use_signal(|| None::<String>);
    let is_background_mode = use_signal(|| false);
    let background_images = use_signal(Vec::new);
    let current_bg_index = use_signal(|| 0);
    let bg_timer_handle = use_signal(|| None::<i32>);
    let mut show_exit_button = use_signal(|| false);
    let mut hide_btn_timer = use_signal(|| None::<i32>);
    let mut hide_cursor = use_signal(|| false);
    let default_bg_image = "https://yun.ganto.cn/f/default-bg.jpg";

    let fetch_random_img = {
        let img_url = img_url.clone();
        move |_evt: Event<MouseData>| {
            fetch_and_set_random_image(img_url.clone());
        }
    };

    let mut enter_background_mode = {
        let mut is_background_mode = is_background_mode.clone();
        let background_images = background_images.clone();
        let current_bg_index = current_bg_index.clone();
        let bg_timer_handle = bg_timer_handle.clone();
        move || {
            is_background_mode.set(true);
            let mut background_images = background_images.clone();
            let current_bg_index = current_bg_index.clone();
            let mut bg_timer_handle = bg_timer_handle.clone();

            if background_images().len() > 1 {
                let handle = create_carousel_timer(background_images.clone(), current_bg_index.clone());
                bg_timer_handle.set(Some(handle));
                return;
            }

            if background_images().is_empty() {
                background_images.set(vec![default_bg_image.to_string()]);
            }

            spawn_local(async move {
                let resp = gloo_net::http::Request::get(&format!("https://yun.ganto.cn/api/v1/images/random/{}", BG_IMG_COUNT)).send().await;
                let filenames = match resp {
                    Ok(response) => {
                        match response.text().await {
                            Ok(text) => extract_filenames(&text),
                            Err(_) => return,
                        }
                    }
                    Err(_) => return,
                };

                if filenames.is_empty() {
                    return;
                }

                for url in &filenames {
                    let url = url.clone();
                    let mut background_images = background_images.clone();

                    for _ in 0..2 {
                        if load_single_image(&url).await {
                            let mut imgs = background_images();
                            if !imgs.contains(&url) {
                                if imgs.len() == 1 && imgs[0] == default_bg_image {
                                    imgs.clear();
                                }
                                let loaded_count = imgs.len();
                                imgs.push(url.clone());
                                background_images.set(imgs);

                                if loaded_count == 0 {
                                    if let Some(old_handle) = bg_timer_handle() {
                                        web_sys::window().expect("Failed to get window").clear_interval_with_handle(old_handle);
                                    }
                                    let handle = create_carousel_timer(background_images.clone(), current_bg_index.clone());
                                    bg_timer_handle.set(Some(handle));
                                }
                            }
                            break;
                        }
                    }
                }
            });
        }
    };

    let mut exit_background_mode = {
        let mut is_background_mode = is_background_mode.clone();
        let mut bg_timer_handle = bg_timer_handle.clone();
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
            fetch_and_set_random_image(img_url.clone());
        }
        ()
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
                            show_exit_button.clone(),
                            hide_btn_timer.clone(),
                            hide_cursor.clone(),
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
                                show_exit_button.clone(),
                                hide_btn_timer.clone(),
                                hide_cursor.clone(),
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
    use super::*;

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