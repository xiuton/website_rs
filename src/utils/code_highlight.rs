const HIGHLIGHT_JS: &str = include_str!("highlight.js");

pub fn init_highlight() {
    let Some(window) = web_sys::window() else { return };
    let Some(document) = window.document() else { return };
    let Some(head) = document.head() else { return };
    let Some(body) = document.body() else { return };

    if let Ok(link) = document.create_element("link") {
        let _ = link.set_attribute("rel", "stylesheet");
        let _ = link.set_attribute("href", "/static/highlight/github-dark.min.css");
        let _ = head.append_child(&link);
    }

    if let Ok(script) = document.create_element("script") {
        let _ = script.set_attribute("src", "/static/highlight/highlight.min.js");
        let _ = script.set_attribute("async", "false");
        let _ = head.append_child(&script);
    }

    if let Ok(init_script) = document.create_element("script") {
        init_script.set_text_content(Some(HIGHLIGHT_JS));
        let _ = body.append_child(&init_script);
    }
}

pub fn reapply_highlight() {
    let Some(window) = web_sys::window() else { return };
    let Some(document) = window.document() else { return };
    let Some(body) = document.body() else { return };

    if let Ok(script) = document.create_element("script") {
        script.set_text_content(Some(r#"
        function applyHighlight() {
            if (typeof hljs !== 'undefined') {
                document.querySelectorAll('pre code').forEach((block) => {
                    const languageClass = block.className.split(' ').find(cls => cls.startsWith('language-'));
                    if (languageClass) {
                        const language = languageClass.replace('language-', '');
                        block.parentElement.setAttribute('data-lang', language);
                    }
                });
                hljs.highlightAll();
            } else {
                setTimeout(applyHighlight, 100);
            }
        }
        applyHighlight();
    "#));
        let _ = body.append_child(&script);
        let _ = body.remove_child(&script);
    }
}