//! 圆形生成器的结果/尺寸变化弹窗。

use super::Circle;
use dioxus::prelude::*;

#[component]
pub(super) fn ResultModal(selected: Option<(usize, Circle)>, on_close: EventHandler<()>) -> Element {
    rsx! {
        div { class: "cg-modal-overlay",
            div { class: "cg-modal",
                div { class: "cg-modal-header",
                    span { "选取结果" }
                    button {
                        class: "cg-modal-close",
                        onclick: move |_| on_close.call(()),
                        "\u{2716}"
                    }
                }
                div { class: "cg-modal-body",
                    {selected.map(|(idx, circle)| {
                        rsx! {
                            div { class: "cg-result-info",
                                div { class: "cg-result-row",
                                    span { class: "cg-result-label", "编号" }
                                    span { class: "cg-result-value", "{idx + 1}" }
                                }
                                div { class: "cg-result-row",
                                    span { class: "cg-result-label", "X 坐标" }
                                    span { class: "cg-result-value", "{circle.x:.1}" }
                                }
                                div { class: "cg-result-row",
                                    span { class: "cg-result-label", "Y 坐标" }
                                    span { class: "cg-result-value", "{circle.y:.1}" }
                                }
                                div { class: "cg-result-row",
                                    span { class: "cg-result-label", "半径" }
                                    span { class: "cg-result-value", "{circle.radius:.1}" }
                                }
                            }
                        }
                    })}
                }
                div { class: "cg-modal-footer",
                    button {
                        class: "cg-btn",
                        onclick: move |_| on_close.call(()),
                        "关闭"
                    }
                }
            }
        }
    }
}

#[component]
pub(super) fn ResizeModal(on_close: EventHandler<()>, on_regenerate: EventHandler<()>) -> Element {
    rsx! {
        div { class: "cg-modal-overlay",
            div { class: "cg-modal cg-resize-modal",
                div { class: "cg-modal-header",
                    span { "窗口大小已变化" }
                    button {
                        class: "cg-modal-close",
                        onclick: move |_| on_close.call(()),
                        "\u{2716}"
                    }
                }
                div { class: "cg-modal-body",
                    div { class: "cg-resize-info",
                        span { "检测到浏览器窗口大小已变化，是否重新生成圆形以适配新尺寸？" }
                    }
                }
                div { class: "cg-modal-footer",
                    button {
                        class: "cg-btn",
                        onclick: move |_| on_close.call(()),
                        "取消"
                    }
                    button {
                        class: "cg-btn cg-btn-primary",
                        onclick: move |_| on_regenerate.call(()),
                        "重新生成"
                    }
                }
            }
        }
    }
}
