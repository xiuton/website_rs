---
title: "使用class实现评分"
date: 2023-11-17 02:07:00
author: 干徒
link: https://www.cnblogs.com/ganto/articles/17837740.html
---

# 使用class实现评分

# 创建Star类

```js
class Star {
  constructor(will, score, dom) {
    this.will = will;
    this.score = score;
    this.dom = dom;
  }
  init() {
    const that = this;
    this.dom.forEach(element => {
      element.onmouseover = e => {
        that.point(Number(e.target.innerText));
      }
      element.onmouseout = () => {
        that.point();
      }
      element.onclick = function() {
        if (that.score === this.innerText) {
          that.score = 0;
        } else {
          that.score = Number(this.innerText);
        }
      }
    })
  }
  point(parm) {
    this.will = parm || this.score;
    this.dom.forEach((element, i) => {
      element.className = i < this.will ? "on" : "";
    });
  }
}

```

# 使用Star类

创建Star实例时需要输入成员变量  
will 为 mouseover 的时候保存的星星数，用于鼠标经过星星时进行星星点亮的数据依据  
score 为 click 的时候确定选择的星星数  
dom 为 每组星星的 dom 数组

```js
const star_1 = new Star(0, 0, document.querySelectorAll(".star_1 ul li"));
star_1.init();

```

# 完整代码

```html
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>Document</title>
  <style>
    * {
      margin: 0;
      padding: 0;
      box-sizing: border-box;
    }
    html, body {
      background-color: #000;
      color: #fff;
    }
    body {
      height: 100vh;
      display: flex;
      justify-content: center;
      align-items: center;
    }
    main {
      .star {
        width: 200px;
        height: 40px;
        & ul {
          display: flex;
          & li {
            list-style: none;
            width: 40px;
            height: 40px;
            border-radius: 50%;
            background-color: yellowgreen;
            display: flex;
            justify-content: center;
            align-items: center;
            cursor: pointer;
          }
          & li.on {
            background-color: red;
          }
        }
      }
      .btn {
        text-align: center;
        margin-top: 10px;
        #submit {
          display: inline-block;
        }
      }
    }
  </style>
</head>
<body>
  <main>
    <div class="star star_1">
      <ul>
        <li>1</li>
        <li>2</li>
        <li>3</li>
        <li>4</li>
        <li>5</li>
      </ul>
    </div>
    <div class="star star_2">
      <ul>
        <li>1</li>
        <li>2</li>
        <li>3</li>
        <li>4</li>
        <li>5</li>
      </ul>
    </div>
    <div class="star star_3">
      <ul>
        <li>1</li>
        <li>2</li>
        <li>3</li>
        <li>4</li>
        <li>5</li>
      </ul>
    </div>
    <div class="btn">
      <button id="submit">提交</button>
    </div>
  </main>

  <script>
    window.onload = function() {
      class Star {
        constructor(will, score, dom) {
          this.will = will;
          this.score = score;
          this.dom = dom;
        }
        init() {
          const that = this;
          this.dom.forEach(element => {
            element.onmouseover = e => {
              that.point(Number(e.target.innerText));
            }
            element.onmouseout = () => {
              that.point();
            }
            element.onclick = function() {
              if (that.score === this.innerText) {
                that.score = 0;
              } else {
                that.score = Number(this.innerText);
              }
            }
          })
        }
        point(parm) {
          this.will = parm || this.score;
          this.dom.forEach((element, i) => {
            element.className = i < this.will ? "on" : "";
          });
        }
      }
      const star_1 = new Star(0, 0, document.querySelectorAll(".star_1 ul li"));
      const star_2 = new Star(0, 0, document.querySelectorAll(".star_2 ul li"));
      const star_3 = new Star(0, 0, document.querySelectorAll(".star_3 ul li"));
      console.log(star_1, star_2, star_3)
      star_1.init();
      star_2.init();
      star_3.init();

      document.querySelector("#submit").onclick = () => {
        alert(`${star_1.score}, ${star_2.score}, ${star_3.score}`);
      }
    }
  </script>
</body>
</html>

```