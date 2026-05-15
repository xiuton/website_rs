function loadHighlightLanguages() {
    if (typeof hljs === 'undefined') {
        setTimeout(loadHighlightLanguages, 100);
        return;
    }

    const languages = [
        { name: "rust", file: "rust.min.js" },
        { name: "javascript", file: "javascript.min.js" },
        { name: "typescript", file: "typescript.min.js" },
        { name: "python", file: "python.min.js" },
        { name: "go", file: "go.min.js" },
        { name: "java", file: "java.min.js" },
        { name: "cpp", file: "cpp.min.js" },
        { name: "csharp", file: "csharp.min.js" },
        { name: "php", file: "php.min.js" },
        { name: "ruby", file: "ruby.min.js" },
        { name: "swift", file: "swift.min.js" },
        { name: "kotlin", file: "kotlin.min.js" },
        { name: "scala", file: "scala.min.js" },
        { name: "bash", file: "bash.min.js" },
        { name: "shell", file: "shell.min.js" },
        { name: "sql", file: "sql.min.js" },
        { name: "xml", file: "xml.min.js" },
        { name: "yaml", file: "yaml.min.js" },
        { name: "json", file: "json.min.js" },
        { name: "markdown", file: "markdown.min.js" },
        { name: "html", file: "xml.min.js" }
    ];

    const loadPromises = languages.map(lang => {
        return new Promise((resolve) => {
            const script = document.createElement('script');
            script.src = `https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/languages/${lang.file}`;
            script.async = true;
            script.onload = () => resolve();
            document.head.appendChild(script);
        });
    });

    Promise.all(loadPromises).then(() => {
        applyCodeHighlight();
    });
}

function applyCodeHighlight() {
    if (typeof hljs === 'undefined') {
        setTimeout(applyCodeHighlight, 100);
        return;
    }

    document.querySelectorAll('pre code').forEach((block) => {
        const languageClass = block.className.split(' ').find(cls => cls.startsWith('language-'));
        if (languageClass) {
            const language = languageClass.replace('language-', '');
            block.parentElement.setAttribute('data-lang', language);
        }
    });

    hljs.highlightAll();
}

loadHighlightLanguages();