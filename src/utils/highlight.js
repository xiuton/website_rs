function loadHighlightLanguages() {
    if (typeof hljs === 'undefined') {
        setTimeout(loadHighlightLanguages, 100);
        return;
    }

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

    // Alias some language names to their CDN filename
    const alias = {
        'html': 'xml',
        'js': 'javascript',
        'ts': 'typescript',
        'sh': 'bash',
        'cs': 'csharp',
        'c++': 'cpp',
        'c#': 'csharp',
        'yml': 'yaml',
    };

    // Only load language files that are NOT already in the common bundle
    const needed = Array.from(used)
        .map(lang => alias[lang] || lang)
        .filter(name => !hljs.getLanguage(name));

    if (needed.length === 0) {
        applyHighlight();
        return;
    }

    const promises = needed.map(name => new Promise((resolve) => {
        const script = document.createElement('script');
        script.src = `https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/languages/${name}.min.js`;
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