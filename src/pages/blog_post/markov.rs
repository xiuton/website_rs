use dioxus::prelude::*;

/// 马尔可夫链 AI 续写组件
///
/// `markov.json` 是 MB 级资源（内含全部文章的 trigram 表），**不能**在组件挂载时下载。
/// 这里改成「首次点击才加载 + 生成」：不点按钮的读者不会为这个彩蛋付出任何流量。
#[component]
pub(super) fn MarkovContinuation(slug: String) -> Element {
    let generator = use_signal(|| None::<crate::utils::markov::MarkovGenerator>);
    let loading = use_signal(|| false);
    let output = use_signal(String::new);

    let mut gen_signal = generator;
    let mut loading_signal = loading;
    let mut output_signal = output;

    let handle_generate = move |_| {
        if *loading_signal.peek() {
            return;
        }

        // 已经加载过：直接生成，不再发请求
        let cached = gen_signal.peek().as_ref().map(|gen| gen.generate(&slug, Some(""), 300));
        if let Some(text) = cached {
            output_signal.set(text);
            return;
        }

        loading_signal.set(true);
        let slug = slug.clone();
        spawn(async move {
            match crate::utils::markov::load_markov().await {
                Ok(data) => {
                    if data.contains_key(&slug) {
                        let gen = crate::utils::markov::MarkovGenerator::new(data);
                        let text = gen.generate(&slug, Some(""), 300);
                        gen_signal.set(Some(gen));
                        output_signal.set(text);
                    } else {
                        output_signal.set("暂无该文章的续写模型。".into());
                    }
                }
                Err(e) => {
                    web_sys::console::error_1(&format!("加载 markov.json 失败: {:?}", e).into());
                    output_signal.set("加载续写模型失败，请稍后重试。".into());
                }
            }
            loading_signal.set(false);
        });
    };

    rsx! {
        div { class: "markov-section",
            div { class: "markov-header",
                svg {
                    xmlns: "http://www.w3.org/2000/svg",
                    view_box: "0 0 24 24",
                    width: "16", height: "16",
                    fill: "none", stroke: "currentColor",
                    stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
                    path { d: "M12 2a10 10 0 1 0 10 10" }
                    polyline { points: "12 12 18 12 18 6" }
                }
                span { "AI 续写" }
                span { class: "markov-badge", "马尔可夫链" }
            }
            p { class: "markov-desc", "基于本文内容训练的 tri-gram 语言模型，自动生成下文片段。" }
            div { class: "markov-actions",
                button {
                    class: "markov-btn",
                    disabled: *loading.read(),
                    onclick: handle_generate,
                    if *loading.read() { "加载模型…" } else { "生成续写" }
                }
            }
            if !output.read().is_empty() {
                div { class: "markov-output",
                    {output.read().as_str()}
                }
            }
        }
    }
}
