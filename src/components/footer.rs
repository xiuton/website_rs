use dioxus::prelude::*;

#[component]
pub fn Footer() -> Element {
    let is_ganto_domain = use_signal(|| {
        let host = web_sys::window()
            .and_then(|window| window.location().host().ok())
            .unwrap_or_default();
        host.contains("ganto.cn")
    });

    rsx! {
        div { class: "footer-content",
            div { class: "copyright",
                span { "2019-2025 " }
                svg {
                    xmlns: "http://www.w3.org/2000/svg",
                    width: "14",
                    height: "14",
                    view_box: "0 0 512 512",
                    fill: "rgb(161, 98, 7)",
                    style: "margin: 0 2px; position: relative; top: 2px;",
                    path {
                        d: "M256 48a208 208 0 1 1 0 416 208 208 0 1 1 0-416zm0 464A256 256 0 1 0 256 0a256 256 0 1 0 0 512zM199.4 312.6c-31.2-31.2-31.2-81.9 0-113.1s81.9-31.2 113.1 0c9.4 9.4 24.6 9.4 33.9 0s9.4-24.6 0-33.9c-50-50-131-50-181 0s-50 131 0 181s131 50 181 0c9.4-9.4 9.4-24.6 0-33.9s-24.6-9.4-33.9 0c-31.2 31.2-81.9 31.2-113.1 0z"
                    }
                }
                span { " 干徒 / Ganto" }
            }
            if *is_ganto_domain.peek() {
                div { class: "icp-info",
                    a {
                        href: "https://beian.miit.gov.cn",
                        target: "_blank",
                        rel: "noopener noreferrer",
                        "豫ICP备2024049779号"
                    }
                }
            }
        }
    }
}