use dioxus::prelude::*;
use dioxus_router::prelude::Link;
use crate::models::BlogPost;
use crate::routes::Route;
use crate::BLOG_POSTS;
use crate::utils::title;

/// 系列（多章节文档）目录页
/// 路由：/series/:slug —— slug 为系列的目录标识（front matter 的 catalog 字段），
/// 未配置 catalog 时回退为入口章（order 最小的一章）的 slug
#[component]
pub fn SeriesView(slug: String) -> Element {
    // 通过入口章或目录标识反查系列信息
    let entry_post = BLOG_POSTS
        .iter()
        .find(|p| p.slug == slug && !p.series.is_empty())
        .or_else(|| BLOG_POSTS.iter().find(|p| p.catalog == slug && !p.series.is_empty()));

    let page_title = entry_post
        .map(|e| format!("{} - 干徒", e.series))
        .unwrap_or_else(|| "系列文档 - 干徒".to_string());
    title::set_page_title(&page_title);

    let info: Option<(String, Vec<&'static BlogPost>, &'static BlogPost)> =
        entry_post.map(|entry| {
            let series = entry.series.to_string();
            let mut chapters: Vec<&BlogPost> = BLOG_POSTS
                .iter()
                .filter(|p| p.series == entry.series)
                .collect();
            chapters.sort_by(|a, b| a.order.cmp(&b.order).then_with(|| b.date.cmp(&a.date)));
            let intro = chapters.first().copied().unwrap_or(entry);
            (series, chapters, intro)
        });

    rsx! {
        div { class: "series-container",
            {match info {
                None => rsx! {
                    div { class: "series-not-found",
                        h2 { "未找到该系列" }
                        p { "系列不存在或已移除，请返回首页浏览。" }
                        Link { to: Route::Home, class: "series-back-home", "← 返回首页" }
                    }
                },
                Some((series, chapters, intro)) => {
                    let total = chapters.len();
                    let last_date = chapters.iter().map(|c| c.date).max().unwrap_or(intro.date);
                    let first_slug = chapters.first().map(|c| c.slug.to_string()).unwrap_or_default();
                    rsx! {
                        div { class: "series-hero",
                            div { class: "series-hero-tag", "系列文档" }
                            h1 { class: "series-hero-title", "{series}" }
                            if !intro.summary.is_empty() {
                                p { class: "series-hero-desc", "{intro.summary}" }
                            }
                            div { class: "series-hero-meta",
                                span { "共 {total} 章" }
                                span { "作者：{intro.author}" }
                                span { "最后更新：{last_date}" }
                            }
                            Link {
                                to: Route::BlogPostView { slug: first_slug },
                                class: "series-start-btn",
                                "开始阅读 →"
                            }
                        }
                        div { class: "series-chapter-list",
                            {chapters.iter().enumerate().map(|(i, ch)| {
                                let to = Route::BlogPostView { slug: ch.slug.to_string() };
                                rsx! {
                                    Link {
                                        to,
                                        class: "series-chapter-card",
                                        span { class: "series-chapter-index", "{i + 1}" }
                                        div { class: "series-chapter-body",
                                            h3 { class: "series-chapter-title", "{ch.title}" }
                                            if !ch.summary.is_empty() {
                                                p { class: "series-chapter-summary", "{ch.summary}" }
                                            }
                                            span { class: "series-chapter-date", "{ch.date}" }
                                        }
                                    }
                                }
                            })}
                        }
                    }
                }
            }}
        }
    }
}
