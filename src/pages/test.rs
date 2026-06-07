use dioxus::prelude::*;
use crate::utils::title;

#[component]
pub fn Test() -> Element {
    let mut counter = use_signal(|| 0);
    let mut input_value = use_signal(String::new);
    let mut selected_option = use_signal(|| "option1".to_string());
    let mut is_checked = use_signal(|| false);
    let mut slider_value = use_signal(|| 50.0);

    title::set_page_title("测试页面 - 干徒");

    rsx! {
        div { class: "test-container",
            h1 { class: "test-title", "🧪 测试页面" }
            p { class: "test-description", "这是一个用于测试各种功能和组件的页面" }

            // 计数器测试
            div { class: "test-section",
                h2 { "计数器测试" }
                div { class: "counter-display",
                    span { "当前计数: " }
                    span { class: "counter-value", "{counter}" }
                }
                div { class: "counter-controls",
                    button {
                        class: "test-btn",
                        onclick: move |_| counter.set(counter() - 1),
                        "减少"
                    }
                    button {
                        class: "test-btn",
                        onclick: move |_| counter.set(0),
                        "重置"
                    }
                    button {
                        class: "test-btn",
                        onclick: move |_| counter.set(counter() + 1),
                        "增加"
                    }
                }
            }

            // 输入框测试
            div { class: "test-section",
                h2 { "输入框测试" }
                div { class: "input-group",
                    input {
                        class: "test-input",
                        placeholder: "请输入内容...",
                        value: "{input_value}",
                        oninput: move |evt| input_value.set(evt.data.value())
                    }
                    div { class: "input-display",
                        "输入内容: "
                        span { class: "input-value", "{input_value}" }
                    }
                }
            }

            // 选择框测试
            div { class: "test-section",
                h2 { "选择框测试" }
                div { class: "select-group",
                    select {
                        class: "test-select",
                        value: "{selected_option}",
                        onchange: move |evt| selected_option.set(evt.data.value()),
                        option { value: "option1", "选项 1" }
                        option { value: "option2", "选项 2" }
                        option { value: "option3", "选项 3" }
                    }
                    div { class: "select-display",
                        "选择的值: "
                        span { class: "select-value", "{selected_option}" }
                    }
                }
            }

            // 复选框测试
            div { class: "test-section",
                h2 { "复选框测试" }
                div { class: "checkbox-group",
                    label { class: "test-checkbox",
                        input {
                            type: "checkbox",
                            checked: is_checked(),
                            onchange: move |evt| is_checked.set(evt.data.checked())
                        }
                        span { "同意条款" }
                    }
                    div { class: "checkbox-display",
                        "复选框状态: "
                        span { class: "checkbox-value", if is_checked() { "已选中" } else { "未选中" } }
                    }
                }
            }

            // 滑块测试
            div { class: "test-section",
                h2 { "滑块测试" }
                div { class: "slider-group",
                    input {
                        type: "range",
                        class: "test-slider",
                        min: "0",
                        max: "100",
                        step: "1",
                        value: "{slider_value}",
                        oninput: move |evt| {
                            if let Ok(value) = evt.data.value().parse::<f64>() {
                                slider_value.set(value);
                            }
                        }
                    }
                    div { class: "slider-display",
                        "滑块值: "
                        span { class: "slider-value", "{slider_value}" }
                    }
                }
            }

            // 颜色主题测试
            div { class: "test-section",
                h2 { "颜色主题测试" }
                div { class: "color-test",
                    div { class: "color-box primary", "主色调" }
                    div { class: "color-box secondary", "次要色调" }
                    div { class: "color-box accent", "强调色" }
                    div { class: "color-box text", "文本色" }
                    div { class: "color-box background", "背景色" }
                }
            }

            // 动画测试
            div { class: "test-section",
                h2 { "动画测试" }
                div { class: "animation-test",
                    div { class: "animated-box bounce", "弹跳" }
                    div { class: "animated-box pulse", "脉冲" }
                    div { class: "animated-box rotate", "旋转" }
                    div { class: "animated-box slide", "滑动" }
                }
            }

            // 响应式测试
            div { class: "test-section",
                h2 { "响应式测试" }
                div { class: "responsive-grid",
                    div { class: "responsive-item", "项目 1" }
                    div { class: "responsive-item", "项目 2" }
                    div { class: "responsive-item", "项目 3" }
                    div { class: "responsive-item", "项目 4" }
                    div { class: "responsive-item", "项目 5" }
                    div { class: "responsive-item", "项目 6" }
                }
            }
        }
    }
}
