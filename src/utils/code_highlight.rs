const HIGHLIGHT_JS: &str = include_str!("highlight.js");

pub fn init_highlight() {
    let window = web_sys::window().expect("Failed to get window");
    let document = window.document().expect("Failed to get document");

    let link = document.create_element("link").expect("Failed to create link element");
    link.set_attribute("rel", "stylesheet").expect("Failed to set rel attribute");
    link.set_attribute("href", "https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/styles/github-dark.min.css").expect("Failed to set href attribute");
    link.set_attribute("crossorigin", "anonymous").expect("Failed to set crossorigin attribute");
    let _ = document.head().expect("Failed to get head").append_child(&link);

    let script = document.create_element("script").expect("Failed to create script element");
    script.set_attribute("src", "https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/highlight.min.js").expect("Failed to set src attribute");
    script.set_attribute("async", "false").expect("Failed to set async attribute");
    script.set_attribute("crossorigin", "anonymous").expect("Failed to set crossorigin attribute");
    let _ = document.head().expect("Failed to get head").append_child(&script);

    let init_script = document.create_element("script").expect("Failed to create init script element");
    init_script.set_text_content(Some(HIGHLIGHT_JS));
    let _ = document.body().expect("Failed to get body").append_child(&init_script);
}

pub fn reapply_highlight() {
    let window = web_sys::window().expect("Failed to get window");
    let document = window.document().expect("Failed to get document");

    let script = document.create_element("script").expect("Failed to create script element");
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
    let _ = document.body().expect("Failed to get body").append_child(&script);
}