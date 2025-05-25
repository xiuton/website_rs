---
title: "JavaScript中的事件冒泡与事件捕获"
date: "2023-01-19 13:29:10"
author: "干徒"
tags: ["JavaScript"]
---

## 前言
Capture：事件捕获
Bubble：事件冒泡

![](/static/blog-images/JavaScript中的事件冒泡与事件捕获_images/2229842-20241220165118171-411205481.png)
如图所示，大致展示了事件冒泡和事件捕获的流程

可能还有点迷惑，无妨，继续往下看

## 事件冒泡展示

我们先创建一个嵌套的HTML结构
```html
<div class="div1">
    <p>div1</p>
    <div class="div2">
        <p>div2</p>
        <button>CLICK ME!</button>
    </div>
</div>
```
并且为他们各自添加点击事件，并给window和document也添加上点击事件

```js
window.addEventListener("click", () => {
  console.log('Window');
});
document.addEventListener("click", () => {
  console.log('Document');
});
document.querySelector(".div1").addEventListener("click", () => {
  console.log('DIV 1');
});
document.querySelector(".div2").addEventListener("click", () => { 
  console.log('DIV 2');
});
document.querySelector("button").addEventListener("click", () => {
  console.log('CLICK ME!');
});
```

完整代码
```html
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>Document</title>
</head>
<body>
  <div class="div1">
    <p>div1</p>
    <div class="div2">
        <p>div2</p>
        <button>CLICK ME!</button>
    </div>
  </div>
  <script>
    window.addEventListener("click", () => {
      console.log('Window');
    });
    document.addEventListener("click", () => {
      console.log('Document');
    });
    document.querySelector(".div1").addEventListener("click", () => {
      console.log('DIV 1');
    });
    document.querySelector(".div2").addEventListener("click", () => { 
      console.log('DIV 2');
    });
    document.querySelector("button").addEventListener("click", () => {
      console.log('CLICK ME!');
    });
  </script>
</body>
</html>
```

在点击`CLICK ME!`按钮的时候，可以在控制台看到打印结果
```
CLICK ME!
DIV 2
DIV 1
Document
Window
```
以上代码所绑定的所有点击事件，默认都是事件冒泡，所以事件触发会由内往外扩展

## 事件捕获展示
如上代码可知，默认绑定的事件会默认开始事件冒泡，如果想要开始事件捕获，可以通过事件绑定的第三个参数进行设置

```js
document.querySelector("button").addEventListener("click", () => {
  console.log('CLICK ME!');
}, true);
```

因为第三个参数如果不进行传入，则默认为false，也就是事件冒泡模式

现在我们重新修改代码

```js
window.addEventListener("click", () => {
  console.log('Window');
}, true);
document.addEventListener("click", () => {
  console.log('Document');
}, true);
document.querySelector(".div1").addEventListener("click", () => {
  console.log('DIV 1');
}, true);
document.querySelector(".div2").addEventListener("click", () => { 
  console.log('DIV 2');
}, true);
document.querySelector("button").addEventListener("click", () => {
  console.log('CLICK ME!');
}, true);
```

完整代码
```html
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>Document</title>
</head>
<body>
  <div class="div1">
    <p>div1</p>
    <div class="div2">
        <p>div2</p>
        <button>CLICK ME!</button>
    </div>
  </div>
  <script>
    window.addEventListener("click", () => {
      console.log('Window');
    }, true);
    document.addEventListener("click", () => {
      console.log('Document');
    }, true);
    document.querySelector(".div1").addEventListener("click", () => {
      console.log('DIV 1');
    }, true);
    document.querySelector(".div2").addEventListener("click", () => { 
      console.log('DIV 2');
    }, true);
    document.querySelector("button").addEventListener("click", () => {
      console.log('CLICK ME!');
    }, true);
  </script>
</body>
</html>
```
打印结果
```
Window
Document
DIV 1
DIV 2
CLICK ME!
```

这时再次触发`CLICK ME!`按钮的点击事件，发现打印结果反过来了

因为这时已经是通过事件捕获进行了依次触发


## 事件冒泡和事件捕获混用
如果将两者混用会发生什么？
依然会遵循两者的执行顺序

```js
window.addEventListener("click", () => {
  console.log('Window');
}, true);
document.addEventListener("click", () => {
  console.log('Document');
}, true);
document.querySelector(".div1").addEventListener("click", () => {
  console.log('DIV 1');
}, false);
document.querySelector(".div2").addEventListener("click", () => { 
  console.log('DIV 2');
}, false);
document.querySelector("button").addEventListener("click", () => {
  console.log('CLICK ME!');
}, true);
```

完整代码
```html
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>Document</title>
</head>
<body>
  <div class="div1">
    <p>div1</p>
    <div class="div2">
        <p>div2</p>
        <button>CLICK ME!</button>
    </div>
  </div>
  <script>
    window.addEventListener("click", () => {
      console.log('Window');
    }, true);
    document.addEventListener("click", () => {
      console.log('Document');
    }, true);
    document.querySelector(".div1").addEventListener("click", () => {
      console.log('DIV 1');
    }, false);
    document.querySelector(".div2").addEventListener("click", () => { 
      console.log('DIV 2');
    }, false);
    document.querySelector("button").addEventListener("click", () => {
      console.log('CLICK ME!');
    }, true);
  </script>
</body>
</html>
```

打印结果
```
Window
Document
CLICK ME!
DIV 2
DIV 1
```

优先依次执行事件捕获，再从最内部开始依次事件冒泡