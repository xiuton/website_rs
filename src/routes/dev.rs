use dioxus::prelude::*;
use wasm_bindgen_futures::spawn_local;
use wasm_bindgen::JsCast;
use gloo_timers::callback::Timeout;

#[component]
pub fn Dev() -> Element {
    let img_url = use_signal(|| None::<String>);
    let mut is_background_mode = use_signal(|| false);
    let background_images = use_signal(Vec::new);
    let current_bg_index = use_signal(|| 0);
    let bg_timer_handle = use_signal(|| None::<i32>);
    let mut show_exit_button = use_signal(|| false);
    let mut hide_btn_timer = use_signal(|| None::<i32>);
    let mut hide_cursor = use_signal(|| false);
    let default_bg_image = "https://files.ganto.cn/files/default-bg.jpg";  // 添加默认背景图片URL

    // 小图片区域：点击换一张
    let fetch_random_img = {
        let img_url = img_url.clone();
        move |_evt: Event<MouseData>| {
            let mut img_url = img_url.clone();
            spawn_local(async move {
                let resp = gloo_net::http::Request::get("https://yun.ganto.cn/bgimg").send().await;
                if let Ok(response) = resp {
                    if let Ok(filename) = response.text().await {
                        let url = format!("https://files.ganto.cn/files/{}", filename.trim());
                        img_url.set(Some(url));
                    }
                }
            });
        }
    };

    // 进入背景墙时预加载多张图片
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
            
            // 初始化背景图片列表，添加默认图片
            let mut initial_images = Vec::new();
            initial_images.push(default_bg_image.to_string());
            background_images.set(initial_images);

            // 创建轮播定时器的函数
            let create_carousel_timer = move || {
                let background_images = background_images.clone();
                let mut current_bg_index = current_bg_index.clone();
                let closure = wasm_bindgen::closure::Closure::wrap(Box::new(move || {
                    let len = background_images().len();
                    if len > 0 {
                        current_bg_index.set((current_bg_index() + 1) % len);
                    }
                }) as Box<dyn FnMut()>);
                let handle = web_sys::window().unwrap().set_interval_with_callback_and_timeout_and_arguments_0(
                    closure.as_ref().unchecked_ref(),
                    15000
                ).unwrap();
                closure.forget();
                handle
            };
            
            // 如果已经有加载的图片（不只是默认图片），直接启动轮播
            if background_images().len() > 1 {
                let handle = create_carousel_timer();
                *bg_timer_handle.write() = Some(handle);
                return;
            }

            // 如果没有加载的图片（除默认外），加载新图片
            spawn_local(async move {
                let mut loaded_count = 0;
                while loaded_count < 5 {
                    let resp = gloo_net::http::Request::get("https://yun.ganto.cn/bgimg").send().await;
                    if let Ok(response) = resp {
                        if let Ok(filename) = response.text().await {
                            let url = format!("https://files.ganto.cn/files/{}", filename.trim());
                            let mut background_images = background_images.clone();
                            
                            // 尝试加载图片
                            let load_image = |url: String| -> std::pin::Pin<Box<dyn std::future::Future<Output = bool>>> {
                                Box::pin(async move {
                            let img = web_sys::HtmlImageElement::new().unwrap();
                                    let (tx, rx) = futures::channel::oneshot::channel();
                                    
                                    let tx_success = std::sync::Arc::new(std::sync::Mutex::new(Some(tx)));
                                    let tx_error = tx_success.clone();
                                    
                                    let success_callback = wasm_bindgen::closure::Closure::wrap(
                                        Box::new(move || {
                                            if let Some(tx) = tx_success.lock().unwrap().take() {
                                                let _ = tx.send(true);
                                            }
                                        }) as Box<dyn FnMut()>
                                    );
                                    
                                    let error_callback = wasm_bindgen::closure::Closure::wrap(
                                        Box::new(move || {
                                            if let Some(tx) = tx_error.lock().unwrap().take() {
                                                let _ = tx.send(false);
                                            }
                                        }) as Box<dyn FnMut()>
                                    );
                                    
                                    img.set_onload(Some(success_callback.as_ref().unchecked_ref()));
                                    img.set_onerror(Some(error_callback.as_ref().unchecked_ref()));
                                    img.set_src(&url);
                                    
                                    success_callback.forget();
                                    error_callback.forget();
                                    
                                    rx.await.unwrap_or(false)
                                })
                            };
                            
                            // 尝试加载两次
                            let mut success = false;
                            for _ in 0..2 {
                                if load_image(url.clone()).await {
                                    success = true;
                                let mut imgs = background_images();
                                    if !imgs.contains(&url) {
                                        // 如果这是第一张成功加载的图片（除了默认图片），移除默认图片
                                        if imgs.len() == 1 && imgs[0] == default_bg_image {
                                            imgs.clear();
                                        }
                                        imgs.push(url.clone());
                                background_images.set(imgs);
                                        loaded_count += 1;

                                        // 如果这是第一张成功加载的图片，启动轮播
                                        if loaded_count == 1 {
                                            // 确保没有其他定时器在运行
                                            if let Some(old_handle) = *bg_timer_handle.read() {
                                                web_sys::window().unwrap().clear_interval_with_handle(old_handle);
                                            }
                                            let handle = create_carousel_timer();
                                            *bg_timer_handle.write() = Some(handle);
                                        }
                                    }
                                    break;
                                }
                            }
                            
                            if !success {
                                continue;
                            }
                        }
                    }
                }
            });
        }
    };

    // 退出背景墙时停止轮播
    let mut exit_background_mode = {
        let mut bg_timer_handle = bg_timer_handle.clone();
        move || {
            is_background_mode.set(false);
            let handle_opt = bg_timer_handle.read().clone();
            if let Some(handle) = handle_opt {
                web_sys::window().unwrap().clear_interval_with_handle(handle);
                *bg_timer_handle.write() = None;
            }
            // 不再清空图片列表
        }
    };

    // 首次加载小图片
    use_effect(move || {
        if img_url().is_none() {
            let mut img_url = img_url.clone();
            spawn_local(async move {
                let resp = gloo_net::http::Request::get("https://yun.ganto.cn/bgimg").send().await;
                if let Ok(response) = resp {
                    if let Ok(filename) = response.text().await {
                        let url = format!("https://files.ganto.cn/files/{}", filename.trim());
                        img_url.set(Some(url));
                    }
                }
            });
        }
        ()
    });

    // 预加载背景墙图片
    use_effect(move || {
        for url in background_images().iter() {
            let img = web_sys::HtmlImageElement::new().unwrap();
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
                    style: "backdrop-filter: blur(8px); background: rgba(255, 255, 255, 0.2); border: 1px solid rgba(255, 255, 255, 0.3); padding: 8px; border-radius: 8px; transition: all 0.3s ease;",
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
                    style: "backdrop-filter: blur(8px); background: rgba(255, 255, 255, 0.2); border: 1px solid rgba(255, 255, 255, 0.3); padding: 8px; border-radius: 8px; margin-left: 8px; transition: all 0.3s ease;",
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
                        // 清除旧的定时器
                        if let Some(handle) = hide_btn_timer() {
                            web_sys::window().unwrap().clear_timeout_with_handle(handle);
                        }
                        // 新建定时器
                        let mut show_exit_button = show_exit_button.clone();
                        let mut hide_btn_timer = hide_btn_timer.clone();
                        let mut hide_cursor = hide_cursor.clone();
                        let closure = wasm_bindgen::closure::Closure::wrap(Box::new(move || {
                            show_exit_button.set(false);
                            hide_cursor.set(true);
                            hide_btn_timer.set(None);
                        }) as Box<dyn FnMut()>);
                        let handle = web_sys::window().unwrap()
                            .set_timeout_with_callback_and_timeout_and_arguments_0(closure.as_ref().unchecked_ref(), 3000)
                            .unwrap();
                        hide_btn_timer.set(Some(handle));
                        closure.forget();
                    },
                    onmouseleave: move |_| {
                        // 鼠标移出背景墙时立即隐藏按钮和光标
                        show_exit_button.set(false);
                        hide_cursor.set(true);
                        // 清除定时器
                        if let Some(handle) = hide_btn_timer() {
                            web_sys::window().unwrap().clear_timeout_with_handle(handle);
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
                                web_sys::window().unwrap().clear_timeout_with_handle(handle);
                                hide_btn_timer.set(None);
                            }
                        },
                        onmouseleave: move |_| {
                            let mut show_exit_button = show_exit_button.clone();
                            let mut hide_btn_timer = hide_btn_timer.clone();
                            let mut hide_cursor = hide_cursor.clone();
                            let closure = wasm_bindgen::closure::Closure::wrap(Box::new(move || {
                                show_exit_button.set(false);
                                hide_cursor.set(true);
                                hide_btn_timer.set(None);
                            }) as Box<dyn FnMut()>);
                            let handle = web_sys::window().unwrap()
                                .set_timeout_with_callback_and_timeout_and_arguments_0(closure.as_ref().unchecked_ref(), 3000)
                                .unwrap();
                            hide_btn_timer.set(Some(handle));
                            closure.forget();
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