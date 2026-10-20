// 导航条鼠标拖拽滚动。
// 只处理鼠标；触摸端继续使用 overflow-x 的原生横向滚动。
// 通过 dataset 防止重复绑定；拖动超过阈值时在捕获阶段吞掉本次 click，避免误触链接。
(() => {
    const init = () => {
        // 只绑定 SPA 挂载后的导航，避免绑定到 index.html 的骨架屏
        const el = document.querySelector('#main .navbar-links');
        if (!el) return false;
        if (el.dataset.dragBound === '1') return true;
        el.dataset.dragBound = '1';

        let active = false;
        let captured = false;
        let startX = 0;
        let startLeft = 0;
        let moved = false;
        let suppressClick = false;

        // 链接默认 draggable=true，会触发浏览器原生“拖出链接”，必须禁掉
        el.addEventListener(
            'dragstart',
            (event) => {
                event.preventDefault();
                event.stopPropagation();
            },
            true,
        );
        el.querySelectorAll('a').forEach((link) => link.setAttribute('draggable', 'false'));

        el.addEventListener('pointerdown', (event) => {
            if (event.pointerType !== 'mouse' || event.button !== 0) return;
            active = true;
            moved = false;
            startX = event.clientX;
            startLeft = el.scrollLeft;
            el.classList.add('dragging');
            // 这里不能立即 setPointerCapture：会把 click 重定向到容器，链接点不动
        });

        el.addEventListener('pointermove', (event) => {
            if (!active) return;
            const dx = event.clientX - startX;
            if (Math.abs(dx) <= 4) return;
            moved = true;
            if (!captured && el.setPointerCapture) {
                el.setPointerCapture(event.pointerId);
                captured = true;
            }
            el.scrollLeft = startLeft - dx;
            event.preventDefault();
        });

        const finish = (event) => {
            if (!active) return;
            active = false;
            el.classList.remove('dragging');
            if (captured && el.hasPointerCapture && el.hasPointerCapture(event.pointerId)) {
                el.releasePointerCapture(event.pointerId);
            }
            captured = false;
            if (moved) {
                suppressClick = true;
                // click 在 pointerup 之后同步触发，下一轮事件循环再恢复
                setTimeout(() => {
                    suppressClick = false;
                }, 0);
            }
        };

        el.addEventListener('pointerup', finish);
        el.addEventListener('pointercancel', finish);
        el.addEventListener(
            'click',
            (event) => {
                if (!suppressClick) return;
                event.preventDefault();
                event.stopPropagation();
            },
            true,
        );
        return true;
    };

    init();
    // Dioxus 可能在挂载过程中替换节点，持续观察并给新节点补绑定
    if (!window.__navbarDragObserverStarted) {
        window.__navbarDragObserverStarted = true;
        const observer = new MutationObserver(() => {
            init();
        });
        observer.observe(document.documentElement, { childList: true, subtree: true });
    }
})();
