function loadHighlightLanguages() {
    if (typeof hljs === 'undefined') {
        setTimeout(loadHighlightLanguages, 100);
        return;
    }

    // hljs 11.x 没有内置这些语言名（或它们其实是别的语言的别名）。
    // 注册别名后既不会去请求不存在的文件，highlightAll 也能正常处理对应 class。
    const languageAliases = {
        html: 'xml',
        jsonc: 'json',
        vue: 'xml',
        tsx: 'typescript',
        jsx: 'javascript',
        toml: 'ini',
        csv: 'plaintext',
        text: 'plaintext',
        txt: 'plaintext',
        undefined: 'plaintext',
        wgsl: 'c',
    };
    Object.entries(languageAliases).forEach(([alias, target]) => {
        if (!hljs.getLanguage(alias) && hljs.getLanguage(target)) {
            hljs.registerAliases(alias, { languageName: target });
        }
    });

    // Scan the page for actually used language-* classes
    const used = new Set();
    document.querySelectorAll('pre code[class*="language-"]').forEach((block) => {
        const cls = block.className.split(' ').find(c => c.startsWith('language-'));
        if (cls) used.add(cls.replace('language-', ''));
    });

    if (used.size === 0) {
        applyHighlight();
        return;
    }

    // Alias some language names to their hljs filename
    const alias = {
        'js': 'javascript',
        'ts': 'typescript',
        'sh': 'bash',
        'shell': 'bash',
        'cs': 'csharp',
        'c++': 'cpp',
        'c#': 'csharp',
        'yml': 'yaml',
    };

    // Only load language files that are NOT already registered
    const needed = Array.from(used)
        .map(lang => alias[lang] || lang)
        .filter(name => !hljs.getLanguage(name));

    if (needed.length === 0) {
        applyHighlight();
        return;
    }

    // 语言文件已放到站内，避免 cdnjs 缺文件时返回 404 HTML 被浏览器 ORB 拦截
    const promises = needed.map(name => new Promise((resolve) => {
        const script = document.createElement('script');
        script.src = `/static/highlight/languages/${name}.min.js`;
        script.async = true;
        script.onload = () => resolve();
        script.onerror = () => resolve();
        document.head.appendChild(script);
    }));

    Promise.all(promises).then(() => applyHighlight());
}

function applyHighlight() {
    if (typeof hljs === 'undefined') {
        setTimeout(applyHighlight, 100);
        return;
    }
    hljs.highlightAll();
}

loadHighlightLanguages();