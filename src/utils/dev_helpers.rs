use dioxus::prelude::*;
use wasm_bindgen_futures::spawn_local;
use wasm_bindgen::JsCast;

use super::constants;

pub const BG_IMG_COUNT: u32 = 5;
pub const HIDE_BTN_DELAY_MS: i32 = 3000;
pub const CAROUSEL_INTERVAL_MS: i32 = 15000;

pub fn extract_filenames(text: &str) -> Vec<String> {
    if let Ok(obj) = serde_json::from_str::<serde_json::Value>(text) {
        if let Some(inner) = obj["data"].as_object() {
            if let Some(arr) = inner["data"].as_array() {
                return arr.iter().filter_map(|v| v.as_str().map(|s| s.trim().to_string())).collect();
            }
        }
    }
    Vec::new()
}

pub fn create_delayed_hide_timer(
    mut show_exit_button: Signal<bool>,
    mut hide_btn_timer: Signal<Option<i32>>,
    mut hide_cursor: Signal<bool>,
    delay_ms: i32,
) -> Option<i32> {
    let closure = wasm_bindgen::closure::Closure::once_into_js(Box::new(move || {
        show_exit_button.set(false);
        hide_cursor.set(true);
        hide_btn_timer.set(None);
    }) as Box<dyn FnOnce()>);
    web_sys::window()
        .and_then(|w| {
            w.set_timeout_with_callback_and_timeout_and_arguments_0(
                closure.unchecked_ref(),
                delay_ms,
            )
            .ok()
        })
}

pub fn create_carousel_timer(
    background_images: Signal<Vec<String>>,
    mut current_bg_index: Signal<usize>,
) -> Option<i32> {
    let closure = wasm_bindgen::closure::Closure::wrap(Box::new(move || {
        let len = background_images().len();
        if len > 0 {
            current_bg_index.set((current_bg_index() + 1) % len);
        }
    }) as Box<dyn FnMut()>);
    let handle = web_sys::window()
        .and_then(|w| {
            w.set_interval_with_callback_and_timeout_and_arguments_0(
                closure.as_ref().unchecked_ref(),
                CAROUSEL_INTERVAL_MS,
            )
            .ok()
        });
    closure.forget();
    handle
}

pub fn load_single_image(url: &str) -> std::pin::Pin<Box<dyn std::future::Future<Output = bool>>> {
    let url = url.to_string();
    Box::pin(async move {
        let img = match web_sys::HtmlImageElement::new() {
            Ok(img) => img,
            Err(_) => return false,
        };
        let (tx, rx) = futures_channel::oneshot::channel();
        let tx_success = std::sync::Arc::new(std::sync::Mutex::new(Some(tx)));
        let tx_error = tx_success.clone();
        let success_callback = wasm_bindgen::closure::Closure::once_into_js(
            Box::new(move || { if let Some(tx) = tx_success.lock().ok().and_then(|mut g| g.take()) { let _ = tx.send(true); } }) as Box<dyn FnOnce()>
        );
        let error_callback = wasm_bindgen::closure::Closure::once_into_js(
            Box::new(move || { if let Some(tx) = tx_error.lock().ok().and_then(|mut g| g.take()) { let _ = tx.send(false); } }) as Box<dyn FnOnce()>
        );
        img.set_onload(Some(success_callback.unchecked_ref()));
        img.set_onerror(Some(error_callback.unchecked_ref()));
        img.set_src(&url);
        rx.await.unwrap_or(false)
    })
}

pub fn fetch_and_set_random_image(mut img_url: Signal<Option<String>>) {
    spawn_local(async move {
        let resp = gloo_net::http::Request::get(&format!("{}1", constants::API_IMAGES_RANDOM)).send().await;
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

pub async fn load_background_wall_images(
    background_images: Signal<Vec<String>>,
    current_bg_index: Signal<usize>,
    mut bg_timer_handle: Signal<Option<i32>>,
    default_bg_image: &str,
) {
    let resp = gloo_net::http::Request::get(&format!(
        "{}{}",
        constants::API_IMAGES_RANDOM,
        BG_IMG_COUNT
    ))
    .send()
    .await;

    let filenames = match resp {
        Ok(response) => match response.text().await {
            Ok(text) => extract_filenames(&text),
            Err(_) => return,
        },
        Err(_) => return,
    };

    if filenames.is_empty() {
        return;
    }

    for url in &filenames {
        let url = url.clone();
        let mut bg_imgs = background_images;

        for _ in 0..2 {
            if load_single_image(&url).await {
                let mut imgs = bg_imgs();
                if !imgs.contains(&url) {
                    if imgs.len() == 1 && imgs[0] == default_bg_image {
                        imgs.clear();
                    }
                    let loaded_count = imgs.len();
                    imgs.push(url.clone());
                    bg_imgs.set(imgs);

                    if loaded_count == 0 {
                        if let Some(old_handle) = bg_timer_handle() {
                            if let Some(w) = web_sys::window() {
                                w.clear_interval_with_handle(old_handle);
                            }
                        }
                        let handle = create_carousel_timer(
                            bg_imgs,
                            current_bg_index,
                        );
                        bg_timer_handle.set(handle);
                    }
                }
                break;
            }
        }
    }
}