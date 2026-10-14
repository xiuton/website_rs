---
title: "css常用场景"
date: 2023-02-20 12:58:00
author: 干徒
link: https://www.cnblogs.com/ganto/articles/17136988.html
tags: ["前端"]
---
# 初始化样式

```css
body,
html,
div,
p,
span,
i,
b,
ul,
ol
li,
input,
img,
video {
  padding: 0;
  margin: 0;
  box-sizing: border-box;
}

img {
 vertical-align: bottom;
 object-fit: cover;
}

ul,
ol {
  list-style: none;
}

i {
  font-style: normal;
}

.container {
  width: 1200px;
  margin: auto;
  position: relative;
}

._row {
  white-space: nowrap;
  text-overflow: ellipsis;
  overflow: hidden;
}

```

[Github仓库](https://gitee.com/ganto/css)

# 清除浮动

**0、给父元素内部最后一个没有浮动的子标签设置clear:both**

```html
<style>
  .box{
    width: 500px;
    background-color: red;
  }
  .child1{
    width: 100px;
    height: 100px;
    background-color: green;
    float: right;
  }
  .child2{
    width: 100px;
    height: 100px;
    background-color: pink;
    float: right;
  }
  .child3{
    clear: both;
  }
</style>

<div class="box">
  <div class="child1">盒子1</div>
  <div class="child2">盒子2</div>
  <div class="child3">你好呀</div>
</div>

```

**1、给父元素内部添加一个空的子标签**

```html
<style>
  .box{
    width: 500px;
    background-color: red;
  }
  .child1{
    width: 100px;
    height: 100px;
    background-color: green;
    float: right;
  }
  .child2{
    width: 100px;
    height: 100px;
    background-color: pink;
    float: right;
  }

  .clear{
    clear: both;
  }
</style>

<div class="box">
  <div class="child1">盒子1</div>
  <div class="child2">盒子2</div>
  <div class="clear"></div>
</div>

```

**2、给父元素设置一个after伪类来代替空标签**

```html
<style>
  .box{
    width: 500px;
    background-color: red;
  }
  .child1{
    width: 100px;
    height: 100px;
    background-color: green;
    float: right;
  }
  .child2{
    width: 100px;
    height: 100px;
    background-color: pink;
    float: right;
  }

  .clearfix::after{
    content: "";
    display: block;
    clear: both;
  }
</style>

<div class="box" class="clearfix">
  <div class="child1">盒子1</div>
  <div class="child2">盒子2</div>
</div>

```

**3、给父元素设置overflow:hidden**

```html
<style>
  .box{
    width: 500px;
    background-color: red;
    overflow: hidden;
  }
  .child1{
    width: 100px;
    height: 100px;
    background-color: green;
    float: right;
  }
  .child2{
    width: 100px;
    height: 100px;
    background-color: pink;
    float: right;
  }
</style>

<div class="box">
  <div class="child1">盒子1</div>
  <div class="child2">盒子2</div>
</div>

```

BFC块级格式话上下文

overflow:hidden;属性就可以触发BFC规则去渲染页面

BFC规则有：浮动的元素也参与运算高度

# 图片不变形

```css
object-fit: contain;

```

# 固定背景图片

将背景图片在视口内固定。

```css
background-attachment: fixed;

```

# 提高权限

通过添加`!important`的方式提高权限

```css
box{
  background-color: '#ff0000 !important';
}

```

# flex布局对调

见名知意，就是flex布局左右对调。

```css
display: flex;
flex-flow: row-reverse;

```

# :last-child::after

```css
.item::after {
  content: '';
  display: inline-block;
  width: 1px;
  height: 24px;
  background-color: #fff;
  position: absolute;
  right: 0;
  top: 50%;
  margin-top: -12px;
}
.item:last-child::after{
  background-color: transparent;
}

```

# 文字省略

```css
.omit {
  white-space: nowrap;
  text-overflow: ellipsis;
  overflow: hidden;
}

```

# white-space: pre;

保留原始格式

```css
div	{
  white-space: pre;
}

```

# BFC

## BFC简介

*   BFC是Block Formatting Context(块级格式上下文)，可以理解成元素的一个“特异功能”。
*   该“特异功能”，在默认的情况下处于关闭状态；当元素满足了某些条件后，该“特异功能”被激活。
*   所谓“特异功能”，专业点说就是：该元素创建了BFC（又称：开启了BFC）

## 开启BFC能解决什么问题

*   元素开启BFC后，其子元素不会再产生margin塌陷问题
*   元素开启BFC后，自己不会被其他浮动元素所覆盖
*   元素开启BFC后，就算其子元素浮动，元素自身高度也不会塌陷

## 如何开启BFC

*   根元素（html元素天生就开启了BFC）
*   浮动的元素（设置float属性）
*   绝对定位、固定定位的元素（position属性，属性值需要是absolute、fixed）
*   行内块元素（display: inline-block）
*   表格单元格：`table`、`thead`、`tbody`、`tfoot`、`th`、`td`、`tr`、`caption`（display: table）
*   `overflow`的值不为`visible`的块元素（overflow: auto）
*   伸缩项目 （给其需要开启BFC的父元素设置，display: flex）
*   多列容器（设置column-count: 1）
*   `column-span`为`all`的元素（即使该元素没有包裹在多列容器中）
*   `display`的值，设置为`flow-root`(设置display: flow-root)

**注意：**强调为常用

子元素浮动问题，导致父元素高度坍塌，依然可以使用BFC来解决，另一种方式就是[清除浮动](#%E6%B8%85%E9%99%A4%E6%B5%AE%E5%8A%A8)

# CSS权重

> 权重计算规则  
> 第一等：代表内联样式，如: `style=""`，权值为**1000**。  
> 第二等：代表ID选择器，如：`#content`，权值为**0100**。  
> 第三等：代表类，伪类和属性选择器，如`.content`，权值为**0010**。  
> 第四等：代表类型选择器和伪元素选择器，如`div p`，权值为**0001**。  
> 通配符、子选择器、相邻选择器等的。如`*、>、+`,权值为**0000**。  
> 继承的样式没有权值。

# 选择父元素

语法：`body:has(.a) .item { color: #FF0000; }`

```html
<body>
  <div class="items">
  	<div class="item">
	  <span class="is_has">isHas</span>
	  <p>文字</p>
	</div>
	<div class="item">
	  <span class="not_has">notHas</span>
	  <p>文字</p>
	</div>
  </div>
</body>

```

```css
.items>.item:has(.is_has)>p{
  color: #FF0000;
}

```

# 相邻兄弟选择器

```html
<body>
  <span>Java</span>
  <span>JavaScript</span>
  <span>Golang</span>
  <span>C</span>
</body>

```

```css
span+span::before{
	content: "、";
}

```

以上写法的意思是选择每个 `span` 后边相邻的第一个`span` ，给其添加伪类 `before`

# 黑白

```css
filter: grayscale(1);

```

# 文字内背景

```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta http-equiv="X-UA-Compatible" content="IE=edge">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Document</title>
    <style>
        div{
            font-weight: 900;
            font-size: 100px;
            background-image: url(./1.png);
            background-clip: text;
            -webkit-background-clip: text;
            -webkit-text-fill-color: transparent;
        }
    </style>
</head>
<body>
    <div>20%</div>
</body>
</html>

```

# 字体显示两行省略

```css
p{
  overflow: hidden;
  text-overflow: ellipsis;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
}

```

# 字体背景图内嵌

```css
h1{
  	background-image: url(https://baidu.com/favicon.ico);
    background-clip: text;
    -webkit-background-clip: text;
  	color: transparent;
    -webkit-text-fill-color: transparent;
}

```

# 边框渐变

```css
div {
    width: 100px;
    height: 50px;
    border: 5px solid;
    border-image: linear-gradient(to right, red, #fff000) 2;
}

```

# 等高

给父类添加这个属性

```css
display: grid;
grid-auto-flow: column;

```

## swiper row 等高

只需要给子tiem添加`height: auto;`即可

`display: flex;`布局，如果需要子元素等高，给子元素设置`height: auto;`

确保`height`高度一致，请不要给父元素添加`align-items: flex-start;`

# meta标签重定向

```html
<meta http-equiv="refresh" content="0; url=https://figmachina.com">

```

# 动态融合

```html
<!DOCTYPE html>
<html>
<head>
    <style>
        .contantBox {
            filter: contrast(50);
            background: black;
            padding: 100px 0 0;
            display: flex;
        }
        .item {
            width: 50px;
            height: 50px;
            filter: blur(10px);
            background: white;
            animation: item 5s linear infinite;
            margin-bottom: -25px;
        }
        @keyframes item {
            0%{
                transform: rotate(0deg);
            }
            100%{
                transform: rotate(360deg);
            }
        }
    </style>
</head>
<body>
  <div class="contantBox">
      <div class="item"></div>
      <div class="item"></div>
      <div class="item"></div>
      <div class="item"></div>
      <div class="item"></div>
      <div class="item"></div>
      <div class="item"></div>
      <div class="item"></div>
      <div class="item"></div>
      <div class="item"></div>
      <div class="item"></div>
      <div class="item"></div>
      <div class="item"></div>
      <div class="item"></div>
      <div class="item"></div>
      <div class="item"></div>
      <div class="item"></div>
      <div class="item"></div>
      <div class="item"></div>
      <div class="item"></div>
  </div>
</body>
</html>

```

这里的背景颜色不能为白色，盒子不能为黑色，否则将失效

# 将a标签设置为不可点击

```css
pointer-events: none;

```

# 单词连接符

```css
.text{
    word-break: break-word;
    -webkit-hyphens: auto;
    -moz-hyphens: auto;
    hyphens:auto;
}

```

连接符要配合html属性lang一起使用，英文单词就必须设置lang为英文属性，德语单词就必须设置lang为德语的属性，否则单词连接符会有问题。

# 锚点滚动

```html
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta http-equiv="X-UA-Compatible" content="IE=edge">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>Document</title>
  <style>
    .box{
      border: 1px solid red;
      height: 100vh;
      overflow:scroll;
      scroll-behavior: smooth;
    }
    header{
      width: 100%;
      height: 500px;
    }
    main{
      width: 100%;
      height: 900px; 
    }
    footer{
      width: 100%;
      height: 500px;
    }
  </style>
</head>
<body>
  <div class="box">
    <header></header>
    <main>
      <h1 id="h1">fjeiafjieoafjoeaf <a href="#h1">#h1</a></h1>
    </main>
    <footer></footer>
  </div>
</body>
</html>

```

# 滚动吸附

容器

```css
.container{
    scroll-snap-type: x mandatory; /* 滚动方向，吸附方式 mandatory 强制吸附 */
}
.container .item{
    scroll-snap-align: start; /* 吸附方式 */
    scroll-snap-stop: always; /* 一项一项的滚动，不能跳过中间的项 */
}

```

# 遮罩层

将图片透明部分挖空

```html
<div class="container">
    <p>
        福建人道具佛饿啊哇建瓯放假哦饿啊我发金额就哦给人家而搜集哦感觉肉色机构金融二十感觉热分解二篇金佛破傲剑法静安分局
    </p>
</div>

```

```css
.container{
    -webkit-mask: url(https://jp.puprime.com/wp-content/themes/puprime_new/images/home_awards_item3.webp?v=4) repeat;
    -webkit-mask-image: linear-gradient(top, right, #000, #000 13%, transparent 13%, transparent 82%, #000 82%), url(./mask.svg);
    -webkit-mask-position: center;
    -webkit-mask-repeat: no-repeat;
}

```

# css变量

```css
:root{
    --background-color: red;
}
body{
    background-color: var(--background-color, #fff)
}

```

以上定义了一个css变量`--background-color: red;`，在`body`上通过`var(--background-color, #fff)`，var的第二个参数是默认值，如果`--background-color: red;`变量没有值，则会启动默认值。

# 反色

```css
mix-blend-mode: difference;

```

# 暗黑模式

根据系统颜色自动设置网页颜色

```css
html {
    color-scheme: light dark;
}

```

系统颜色媒体查询

```css
@media (prefers-color-scheme: dark) {
    body {
        background: #000000;
        color: #ffffff;
    }
}
@media (prefers-color-scheme: light) {
    body {
        background: #ffffff;
        color: #000000;
    }
}

```

```css
@media (prefers-color-scheme: dark) {
    :root {
        --background: #000000;
        --color: #ffffff;
    }
}
@media (prefers-color-scheme: light) {
    :root {
        --background: #ffffff;
        --color: #000000;
    }
}
body {
    background-color: var(--background);
    color: var(--color);
}

```

css可以有媒体查询检测系统颜色，那么js当然也有检测系统颜色的方式

```js
const themeMedia = window.matchMedia("(prefers-color-scheme: light)");
if (themeMedia.matches) {
    console.log("light")
} else {
    console.log("dark")
}

```

js不仅可以检测系统颜色，还可以检测系统颜色的变化

```js
const themeMedia = window.matchMedia("(prefers-color-scheme: light)");
themeMedia.addListener(e => {
    if (e.matches) {
        console.log("light");
    } else {
        console.log("dark");
    }
});

```

应用

```html
<!DOCTYPE html>
<html lang="en" data-theme="auto">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Document</title>
    <style>
        [data-theme=light] {
            --background: #ffffff;
            --color: #000000;
        }
        [data-theme=dark] {
            --background: #000000;
            --color: #ffffff;
        }
        body {
            background-color: var(--background);
            color: var(--color);
        }
    </style>
</head>
<body>
    <p>ABCabc123你好</p>
    <script>
        const initThemeMedia = window.matchMedia("(prefers-color-scheme: light)");
        if (initThemeMedia.matches) {
            console.log("light")
            document.querySelector("html").setAttribute("data-theme", "light")
        } else {
            console.log("dark")
            document.querySelector("html").setAttribute("data-theme", "dark")
        }
        const themeMedia = window.matchMedia("(prefers-color-scheme: light)");
        themeMedia.addListener(e => {
            if (e.matches) {
                console.log("light");
                document.querySelector("html").setAttribute("data-theme", "light")
            } else {
                console.log("dark");
                document.querySelector("html").setAttribute("data-theme", "dark")
            }
        });
    </script>
</body>
</html>

```

# 字体渐变

```css
p {
	background: -webkit-linear-gradient(left,#D008FF 2.56%,#05F3F3 100%);
    background: -moz-linear-gradient(left,#D008FF 2.56%,#05F3F3 100%);
    background: linear-gradient(90deg,#D008FF 2.56%,#05F3F3 100%);
    -webkit-background-clip: text;
    background-clip: text;    
    color: transparent;
    -webkit-text-fill-color: transparent;
}

```

# 字体背景图

```css
p {
    background-image: url(https://baidu.com/favicon.ico);
    background-clip: text;
    -webkit-background-clip: text;
    color: transparent;
    -webkit-text-fill-color: transparent;
}

```

# css透明图片阴影效果

```css
filter: drop-shadow(0 0 10px #000);

```

`filter: drop-shadow(0 0 10px #000);`属性是只给图片的可视区域（像素点）添加阴影效果，透明部分不添加

而`box-shadow`，会给整个盒子做阴影

# 去除移动端点击蓝色背景

```css
body{ -webkit-tap-highlight-color: transparent; }

```

# 去除safari浏览器 输入框右侧的icon

```css
input::-webkit-credentials-auto-fill-button{ display: none !important; visibility: hidden; pointer-events: none; position: absolute; right: 0; } /* 去除密码icon */
input::-webkit-contacts-auto-fill-button{ display: none !important; visibility: hidden; pointer-events: none; position: absolute; right: 0; } /* 去除用户名icon */

```

# CSS边界，碰撞检测

利用css动画的反转进行实现

唯一的缺点就是，在不同大小的窗口，无法控制速度保持一致

```css
.box {
  width: 50px;
  height: 50px;
  border-radius: 50%;
  background-color: red;
  position: absolute;
  left: 0;
  top: 0;
  animation: hor 1.6s infinite linear alternate, ver 0.5s infinite linear alternate;
}
@keyframes hor {
  to {
    left: calc(100vw - 50px);
  }
}
@keyframes ver {
  to {
    top: calc(100vh - 50px);
  }
  
}

```

# 行盒换行边框/背景被截断问题

![image-20231202151605893](images/css%E5%B8%B8%E7%94%A8%E5%9C%BA%E6%99%AF_images/image-20231202151605893.png)

```css
span {
    box-decoration-break: clone;
    -webkit-box-decoration-break: clone;
}

```

![image-20231202151548317](images/css%E5%B8%B8%E7%94%A8%E5%9C%BA%E6%99%AF_images/image-20231202151548317.png)

# 容器查询

区别于媒体查询，媒体查询是相对于浏览器视口进行工作的

而容器查询是相对于元素进行工作，区别于媒体查询，可以直接设置，而容器查询需要设置父元素的container属性，以配合@container进行使用，只有设置了container属性的子元素才能有效果。

container是整合写法，可以分开写container-type、container-name等

`container: layout / inline-size;`等同于`container-name: layout; container-type: inline-size;`

```html
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>Document</title>
  <style>
    .box {
      display: flex;
      & .item {
        flex: 1;
        container: layout / inline-size;
        &:nth-child(odd) {
          background-color: #c9c9c9;
        }
      }
    }
    @media (max-width: 768px) {
      .box {
        & .item {
          &:nth-child(odd) {
            background-color: transparent;
          }
          &:nth-child(even) {
            background-color: #c9c9c9;
          }
        }
      }
    }
    @container (max-width: 100px) {
      .box {
        border: 1px solid rgb(0, 255, 94);
        & .item {
          border: 1px solid rgb(0, 55, 255);
          & span {
            border: 1px solid red;
            color: red;
            box-decoration-break: clone;
            -webkit-box-decoration-break: clone;
          }
        }
      }
    }
  </style>
</head>
<body>
  <div class="box">
    <div class="item"><span>This is an example text. No.1</span></div>
    <div class="item"><span>This is an example text. No.2</span></div>
    <div class="item"><span>This is an example text. No.3</span></div>
    <div class="item"><span>This is an example text. No.4</span></div>
    <div class="item"><span>This is an example text. No.5</span></div>
  </div>
</body>
</html>

```

`container-type`是定义查询模式

`container-name`定义容器名，有多个容器查询时，可以用于区分

```css
div.layout_box { container-name: layout; }
div.xxx_box { container-name: xxx; }
@container layout (max-width: 100px) {...}
@container xxx (max-width: 200px) {...}

```

## 字体大小根据容器变化

```html
<!DOCTYPE html>
<html lang="en">

<head>
  <meta charset="UTF-8">
  <title>Hello React</title>
  <style>
    div {
      width: 500px;
      height: 300px;
      resize: both;
      overflow: hidden;
      background-color: rebeccapurple;
      container: xxx / inline-size;
    }
    div p {
      font-size: 5cqw;
    }
  </style>
</head>

<body>
  <div>
    <p>
      道可道，非常道。名可名，非常名。无名天地之始，有名万物之母。故常无欲以观其妙，以观其徼。此两者，同出而异名，同谓之玄。玄之又玄，众妙之门。
    </p>
  </div>
</body>

</html>

```

# 容器适应内容

```css
p {
    width: fit-content;
}

```

# css实现进度条

```html
<!DOCTYPE html>
<html lang="en">

<head>
  <meta charset="UTF-8">
  <title>Hello</title>
  <style>
    * {
      margin: 0;
      padding: 0;
      box-sizing: border-box;
    }
    body {
      position: relative;
      background-image: linear-gradient(
        to right top,
        #409eff 50%,
        #0003 50%
      );
      background-size: 100% calc(100% - 100vh + 5px);
      background-repeat: no-repeat;
      z-index: 1;
    }
    body::after {
      content: "";
      position: fixed;
      top: 5px;
      left: 0;
      right: 0;
      bottom: 0;
      background-color: #ff0000;
      z-index: -1;
    }
    .ball {
      width: 100%;
      height: 2000px;

    }
  </style>
</head>

<body>
  <div class="ball">
    123123
  </div>
</body>

</html>

```

# 背景颜色混合模式

默认css中，多背景颜色或者多背景图，只能显示一个，有个属性可以进行多个背景颜色或者背景图片混合显示

```css
div {
  background-color: #ff0000;
  background-image: url("https://files.cevno.cn/files/Sunlight_Cloud_Beach_Coast_Seashore_Hills_Iceland_5K_1920x1080.jpg");
  background-size: cover;
  background-position: center;
  background-blend-mode: darken;
}

```

# 页面滚动阻尼效果

```html
<!DOCTYPE html>
<html lang="en">

<head>
  <meta charset="UTF-8">
  <title>Hello React</title>
  <style>
    body {
      display: flex;
      flex-direction: column;
      align-items: center;
      background-color: #171717;
    }
    img {
      width: 40rem;
      margin-bottom: 5rem;
      pointer-events: none;
      user-select: none;
    }
    .scrollbox {
      position: relative;
      display: flex;
      flex-direction: column;
      align-items: center;
      width: 100%;
      flex-shrink: 0;
      transition: 1.15s ease;
    }
    .viewbox {
      position: fixed;
      top: 0;
      display: flex;
      align-items: flex-start;
      width: 100vw;
      height: 100vh;
    }
  </style>
</head>

<body>
  <div class="viewbox">
    <div class="scrollbox">
      <img src="http://files.cevno.cn/files/骰子游戏黑黄点-微距摄影壁纸壁纸_1920x1080[10wallpaper.com].jpg" alt="">
      <img src="http://files.cevno.cn/files/窗口，可爱，宠物，猫，特写，照片壁纸_1920x1080[10wallpaper.com].jpg" alt="">
      <img src="http://files.cevno.cn/files/小米，Mix_3，抽象，多彩，黑暗，桌面壁纸_1920x1080[10wallpaper.com].jpg" alt="">
      <img src="http://files.cevno.cn/files/Cheetah_Maasai_Mara_National_Reserve_Kenya_Bing_4K_1920x1080.jpg" alt="">
      <img src="http://files.cevno.cn/files/54.jpg" alt="">
      <img src="http://files.cevno.cn/files/43.jpg" alt="">
      <img src="http://files.cevno.cn/files/5.jpg" alt="">
      <img src="http://files.cevno.cn/files/12.jpg" alt="">
      <img src="http://files.cevno.cn/files/34.jpg" alt="">
      <img src="http://files.cevno.cn/files/41.jpg" alt="">
      <img src="http://files.cevno.cn/files/54.jpg" alt="">
      <img src="http://files.cevno.cn/files/64.jpg" alt="">
      <img src="http://files.cevno.cn/files/48.jpg" alt="">
      <img src="http://files.cevno.cn/files/49.jpg" alt="">
    </div>
  </div>

  <script>
    let scrollbox = document.querySelector('.scrollbox');
    function resize_body() {
      let height = scrollbox.offsetHeight;
      console.log(height)
      document.body.style.height = height + 'px';
    }
    function scroll() {
      console.log(scrollY)
      scrollbox.style.transform = 'translateY(' + -scrollY + 'px)';
    }
    window.addEventListener("scroll", scroll);
    window.addEventListener("resize", resize_body);
    window.addEventListener("load", resize_body);
  </script>
</body>

</html>

```

# 字体根据背景颜色融合

```js
p {
    mix-blend-mode: difference;
}

```

`mix-blend-mode`属性的值有很多