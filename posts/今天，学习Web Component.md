---
title: "Web Component"
date: "2023-04-12 15:35:44"
author: "干徒"
tags: ["JavaScript", "Web Component"]
---

[Web Component MDN](https://developer.mozilla.org/zh-CN/docs/Web/API/Web_components)

## 例子1：
```html
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>Document</title>
</head>
<body>
  <popup-info text="web components" style="background-color: 50px;"/>

  <script>
    // Create a class for the element
    class PopUpInfo extends HTMLElement {
      constructor() {
        // Always call super first in constructor
        super();

        // Create a shadow root
        var shadow = this.attachShadow({ mode: "open" });

        // Create spans
        var wrapper = document.createElement("span");
        wrapper.setAttribute("class", "wrapper");
        var icon = document.createElement("span");
        icon.setAttribute("class", "icon");
        icon.setAttribute("tabindex", "0");
        var info = document.createElement("span");
        info.setAttribute("class", "info");

        // Take attribute content and put it inside the info span
        var text = this.getAttribute("text");
        info.textContent = text;

        // Insert icon
        var imgUrl;
        if (this.hasAttribute("img")) {
          imgUrl = this.getAttribute("img");
        } else {
          imgUrl = "https://baidu.com/favicon.ico";
        }
        var img = document.createElement("img");
        img.style = `
          width: 50px;
          height: 50px;
          border: 2px solid #f0f;
          border-radius: 10px;
        `
        img.src = imgUrl;
        icon.appendChild(img);

        // Create some CSS to apply to the shadow dom
        var style = document.createElement("style");

        style.textContent = `
          .wrapper {
            position: relative;
          }
          
          .info {
            font-size: 0.8rem;
            width: 200px;
            display: inline-block;
            border: 1px solid black;
            padding: 10px;
            background: white;
            border-radius: 10px;
            opacity: 0;
            transition: 0.6s all;
            position: absolute;
            top: 20px;
            left: 10px;
            z-index: 3;
          }
          
          img {
            width: 20.2rem;
          }
          
          .icon:hover + .info, .icon:focus + .info {
            opacity: 1;
          }
        `;

        // attach the created elements to the shadow dom

        shadow.appendChild(style);
        shadow.appendChild(wrapper);
        wrapper.appendChild(icon);
        wrapper.appendChild(info);
      }
    }

    customElements.define("popup-info", PopUpInfo);
  </script>
</body>
</html>
```

## 例子2：
这个例子使用了template标签的形式描述html结构，而不是如例子1那样，用纯js来描述html结构
index.html
```html
<!DOCTYPE html>
<html>

<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width">
  <title>JS Bin</title>
</head>

<body>
  <user-card image="https://semantic-ui.com/images/avatar2/large/kristy.png" name="User Name"
    email="yourmail@some-email.com"></user-card>

  <template id="userCardTemplate">
    <style>
      :host {
        display: flex;
        align-items: center;
        width: 450px;
        height: 180px;
        background-color: #d4d4d4;
        border: 1px solid #d5d5d5;
        box-shadow: 1px 1px 5px rgba(0, 0, 0, 0.1);
        border-radius: 3px;
        overflow: hidden;
        padding: 10px;
        box-sizing: border-box;
        font-family: 'Poppins', sans-serif;
      }

      .image {
        flex: 0 0 auto;
        width: 160px;
        height: 160px;
        vertical-align: middle;
        border-radius: 5px;
      }

      .container {
        box-sizing: border-box;
        padding: 20px;
        height: 160px;
      }

      .container>.name {
        font-size: 20px;
        font-weight: 600;
        line-height: 1;
        margin: 0;
        margin-bottom: 5px;
      }

      .container>.email {
        font-size: 12px;
        opacity: 0.75;
        line-height: 1;
        margin: 0;
        margin-bottom: 15px;
      }

      .container>.button {
        padding: 10px 25px;
        font-size: 12px;
        border-radius: 5px;
        text-transform: uppercase;
      }
    </style>

    <img class="image">
    <div class="container">
      <p class="name"></p>
      <p class="email"></p>
      <button class="button">Follow John</button>
    </div>
  </template>

  <script src="./index.js"></script>
</body>

</html>
```
index.js
```js
class UserCard extends HTMLElement {
  constructor() {
    super();
    var shadow = this.attachShadow( { mode: 'closed' } );
    
    var templateElem = document.getElementById('userCardTemplate');
    var content = templateElem.content.cloneNode(true); // 这里克隆一个新的结构，而不是在原template节点上操作，因为确保原template不会变化，从而可以复用
    content.querySelector('img').setAttribute('src', this.getAttribute('image'));
    content.querySelector('.container>.name').innerText = this.getAttribute('name');
    content.querySelector('.container>.email').innerText = this.getAttribute('email');

    shadow.appendChild(content);
  }
}
window.customElements.define('user-card', UserCard);
```

## 说明
`this.attachShadow( { mode: 'closed' } );` 这段代码是创建一个自定义元素
`mode: 'closed'`表示自定义组件对外部不可见，即不能通过JavaScript访问，也不能通过外部CSS样式表来直接修改其内部元素的样式
可选项为 `open`、`closed`，mode属性必须写

## 生命周期
| 函数                     | 说明                     |
| ------------------------ | ------------------------ |
| connectedCallback        | 自定义元素添加至页面。   |
| disconnectedCallback     | 自定义元素从页面中移除。 |
| adoptedCallback          | 自定义元素移动至新页面。 |
| attributeChangedCallback | 属性改变                 |
```html
<!DOCTYPE html>
<html>

<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width">
  <title>Web Component</title>
</head>

<body>
  <iframe id="iframe" style="display: none;" frameborder="0"></iframe>

  <my-com class="my-com" data="123"></my-com>
  <button onclick="document.querySelector('.my-com').remove()">移除组件</button>
  <button onclick="move()">移动到新页面</button>
  <button onclick="document.querySelector('.my-com').setAttribute('data', '456')">变更属性</button>

  <script>
    function move() {
      const iframeNode = document.querySelector('#iframe').contentDocument
      console.dir(iframeNode, '===');
      const myComNode = document.querySelector('.my-com')
      iframeNode.body.appendChild(myComNode.cloneNode(true))

      // document.querySelector('.my-com').remove()
    }
  </script>
  <script src="./MyCom.js"></script>
</body>

</html>
```

```js
class MyCom extends HTMLElement {
  constructor() {
    super();
    var shadow = this.attachShadow({ mode: "open" });

    const div = document.createElement("div")
    div.innerHTML = "123"

    shadow.appendChild(div)
  }

  static get observedAttributes() {
    return ['data'];
  }

  connectedCallback() {
    console.log("自定义元素添加至页面。");
  }

  disconnectedCallback() {
    console.log("自定义元素从页面中移除。");
  }

  adoptedCallback() {
    console.log("自定义元素移动至新页面。");
  }

  attributeChangedCallback(name, oldValue, newValue) {
    console.log(`属性 ${name} 已变更。 变更前 ${oldValue}, 变更后 ${newValue} 。`);
  }
}

customElements.define("my-com", MyCom);
```