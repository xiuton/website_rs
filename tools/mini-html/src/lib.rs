//! 把 comrak 渲染出的 HTML 转换成小程序可用的标签结构。
//!
//! 以前这部分是 20 多条正则替换（`<div` → `<view`、`<a>` → `<navigator>` 等），
//! 遇到嵌套标签、属性值里含 `>`、大小写不一致时并不可靠。
//! 这里改用 [`lol_html`]：真正的流式 HTML 解析器 + CSS 选择器改写。
//!
//! 输出目标（与旧实现保持一致的语义）：
//! - 删除脚本、样式、iframe、svg、canvas、音视频、frame 等不安全/不支持的标签
//! - `div` → `view`，`span` → `text`
//! - `a[href]` → `navigator[url]`，无 href 的 `a` → `text`
//! - `img` 加 `mode="widthFix"`，没有 src 的图片直接删除
//! - 粗体/斜体/删除线/代码/引用/表格内联样式
//! - 去掉脚注相关的 class、`aria-hidden`、`data-footnote-ref` 与 `user-content-` 前缀

use std::cell::RefCell;
use std::rc::Rc;

use lol_html::html_content::{ContentType, Element};
use lol_html::{element, HtmlRewriter, Settings};

/// 需要整段删除的标签（连同内容）。
const DROP_TAGS: &str =
    "script, style, iframe, svg, canvas, video, audio, embed, object, param, applet, frame, frameset";

/// 把文章 HTML 转换为小程序标签结构。
pub fn sanitize_for_mini_program(html: &str) -> String {
    let output = Rc::new(RefCell::new(Vec::<u8>::new()));
    let sink = {
        let output = Rc::clone(&output);
        move |chunk: &[u8]| output.borrow_mut().extend_from_slice(chunk)
    };

    let settings = Settings::new()
        .append_element_content_handler(element!(DROP_TAGS, |el: &mut Element| {
            el.remove();
            Ok(())
        }))
        // 通用属性清理：脚注锚点、无障碍属性、comrak 的 id 前缀
        .append_element_content_handler(element!("*", |el: &mut Element| {
            el.remove_attribute("aria-hidden");
            el.remove_attribute("data-footnote-ref");
            if let Some(class) = el.get_attribute("class") {
                let drop = class == "anchor"
                    || class.starts_with("footnote-")
                    || class == "footnote-definition"
                    || class == "footnote-body";
                if drop {
                    el.remove_attribute("class");
                }
            }
            if let Some(id) = el.get_attribute("id") {
                if let Some(stripped) = id.strip_prefix("user-content-") {
                    el.set_attribute("id", stripped)?;
                }
            }
            Ok(())
        }))
        .append_element_content_handler(element!("a", |el: &mut Element| {
            let href = el.get_attribute("href");
            let name = el.get_attribute("name");
            match (href, name) {
                // 无 href 的命名锚点：旧实现会整段删除
                (None, Some(_)) => el.remove(),
                (Some(href), _) => {
                    el.remove_attribute("href");
                    el.set_attribute("url", &href)?;
                    el.set_tag_name("navigator")?;
                }
                (None, None) => {
                    el.set_tag_name("text")?;
                }
            }
            Ok(())
        }))
        .append_element_content_handler(element!("div", |el: &mut Element| {
            el.set_tag_name("view")?;
            Ok(())
        }))
        .append_element_content_handler(element!("span", |el: &mut Element| {
            el.set_tag_name("text")?;
            Ok(())
        }))
        .append_element_content_handler(element!("strong, b", |el: &mut Element| {
            el.set_tag_name("text")?;
            el.set_attribute("style", "font-weight:bold;")?;
            Ok(())
        }))
        .append_element_content_handler(element!("em, i", |el: &mut Element| {
            el.set_tag_name("text")?;
            el.set_attribute("style", "font-style:italic;")?;
            Ok(())
        }))
        .append_element_content_handler(element!("del, s", |el: &mut Element| {
            el.set_tag_name("text")?;
            el.set_attribute("style", "text-decoration:line-through;")?;
            Ok(())
        }))
        .append_element_content_handler(element!("code", |el: &mut Element| {
            el.set_tag_name("text")?;
            el.set_attribute(
                "style",
                "font-family:monospace;background:#f4f4f4;padding:2px 4px;border-radius:3px;",
            )?;
            Ok(())
        }))
        .append_element_content_handler(element!("pre", |el: &mut Element| {
            el.set_tag_name("view")?;
            el.set_attribute(
                "style",
                "font-family:monospace;background:#f4f4f4;padding:12px;border-radius:8px;overflow-x:auto;",
            )?;
            Ok(())
        }))
        .append_element_content_handler(element!("blockquote", |el: &mut Element| {
            el.set_tag_name("view")?;
            el.set_attribute(
                "style",
                "border-left:4px solid #6a9b50;padding-left:16px;background:#f5f5f5;padding:12px 16px;margin:16px 0;border-radius:0 8px 8px 0;",
            )?;
            Ok(())
        }))
        .append_element_content_handler(element!("table", |el: &mut Element| {
            el.set_attribute("style", "width:100%;border-collapse:collapse;")?;
            Ok(())
        }))
        .append_element_content_handler(element!("td", |el: &mut Element| {
            el.set_attribute(
                "style",
                "border:1px solid #e0e0e0;padding:8px;box-sizing:border-box;",
            )?;
            Ok(())
        }))
        .append_element_content_handler(element!("th", |el: &mut Element| {
            el.set_attribute(
                "style",
                "border:1px solid #e0e0e0;padding:8px;box-sizing:border-box;background:#f5f5f5;",
            )?;
            Ok(())
        }))
        .append_element_content_handler(element!("img", |el: &mut Element| {
            if el.get_attribute("src").is_none() {
                el.remove();
            } else {
                el.set_attribute("mode", "widthFix")?;
            }
            Ok(())
        }))
        .append_element_content_handler(element!("hr", |el: &mut Element| {
            el.replace("<hr/>", ContentType::Html);
            Ok(())
        }))
        .append_element_content_handler(element!("br", |el: &mut Element| {
            el.replace("<br/>", ContentType::Html);
            Ok(())
        }));

    let mut rewriter = HtmlRewriter::new(settings, sink);

    if rewriter.write(html.as_bytes()).is_err() {
        return String::new();
    }
    if rewriter.end().is_err() {
        return String::new();
    }

    let bytes = output.borrow().clone();
    String::from_utf8(bytes).unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::sanitize_for_mini_program;

    #[test]
    fn drops_unsafe_tags_and_their_content() {
        let html = r#"<div>keep<script>alert(1)</script><style>p{}</style><iframe src="x"></iframe></div>"#;
        let out = sanitize_for_mini_program(html);
        assert!(!out.contains("<script"), "{out}");
        assert!(!out.contains("alert(1)"), "{out}");
        assert!(!out.contains("<style"), "{out}");
        assert!(!out.contains("<iframe"), "{out}");
        assert!(out.contains("keep"), "{out}");
    }

    #[test]
    fn maps_basic_tags() {
        let html = r#"<div><span>hi</span><strong>bold</strong><em>italic</em><del>gone</del></div>"#;
        let out = sanitize_for_mini_program(html);
        assert!(out.contains("<view>"), "{out}");
        assert!(out.contains("<text>hi</text>"), "{out}");
        assert!(out.contains("font-weight:bold;"), "{out}");
        assert!(out.contains("font-style:italic;"), "{out}");
        assert!(out.contains("text-decoration:line-through;"), "{out}");
    }

    #[test]
    fn rewrites_links_and_images() {
        let html = r#"<p><a href="/post/x">link</a><a name="fn"></a><img src="a.png" alt="a"><img></p>"#;
        let out = sanitize_for_mini_program(html);
        assert!(out.contains("<navigator url=\"/post/x\">link</navigator>"), "{out}");
        assert!(!out.contains("name=\"fn\""), "{out}");
        assert!(out.contains("mode=\"widthFix\""), "{out}");
        // 没有 src 的图片被删除，只剩一张
        assert_eq!(out.matches("<img").count(), 1, "{out}");
    }

    #[test]
    fn strips_footnote_attributes_and_id_prefix() {
        let html = r#"<p id="user-content-x"><sup class="footnote-ref" aria-hidden="true">1</sup><span class="anchor">#</span></p>"#;
        let out = sanitize_for_mini_program(html);
        assert!(out.contains("id=\"x\""), "{out}");
        assert!(!out.contains("footnote-ref"), "{out}");
        assert!(!out.contains("aria-hidden"), "{out}");
        assert!(!out.contains("class=\"anchor\""), "{out}");
    }

    #[test]
    fn keeps_pre_code_structure() {
        let html = r#"<pre><code class="language-rust">fn main() {}</code></pre>"#;
        let out = sanitize_for_mini_program(html);
        assert!(out.contains("<view"), "{out}");
        assert!(out.contains("language-rust"), "{out}");
        assert!(out.contains("fn main() {}"), "{out}");
        assert!(!out.contains("<pre"), "{out}");
        assert!(!out.contains("<code"), "{out}");
    }
}
