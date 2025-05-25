---
title: "JavaScript 高阶函数之 reduce"
date: "2023-01-16 23:00:20"
author: "干徒"
tags: ["JavaScript"]
---

> MDN：https://developer.mozilla.org/zh-CN/docs/Web/JavaScript/Reference/Global_Objects/Array/reduce



关于reduce函数，我们再熟悉不过了，我们通常用他对数据进行求和，也被称为**累加器**

reduce常见用例如下：

```js
let goods = [
  { name: "T恤", price: 99, count: 2 },
  { name: "polo衫", price: 120, count: 1 },
  { name: "牛仔裤", price: 197, count: 11 }
]

const goodsSum = goods.reduce((sum, item) => {
  return sum + item.price * item.count;
}, initSum = 0)

console.log(goodsSum) // 2485
```

以上代码，我相信对于你那真实信手捏来...

但是还是需要解释一下，reduce就是对goods数组中的每项的价格进行相加

reduce传递两个参数，第一个参数是一个回调函数，第二个参数是一个用于统计累加结果的初始值

reduce函数的第一个回调函数又有两个参数，第一个参数`sum`的初始状态等于reduce第二个参数`initSum`，在reduce执行完一次，第一个参数`sum`会变成reduce执行完返回的内容

reduce函数的第一个回调函数的第二个参数也就是goods数组中的每项



reduce执行次数与数组的长度有关





在阅读vue官方文档的时候，发现了一段有趣的代码

```js
// plugins/i18n.js
export default {
  install: (app, options) => {
    // 注入一个全局可用的 $translate() 方法
    app.config.globalProperties.$translate = (key) => {
      // 获取 `options` 对象的深层属性
      // 使用 `key` 作为索引
      return key.split('.').reduce((o, i) => {
        if (o) return o[i]
      }, options)
    }
  }
}
```

以上代码是vue中定义一个插件的代码，有趣的是其中的reduce函数

我们将代码进行精简

```js
let obj = {
  greetings: {
    hello: "你好"
  }
}

const ret = "greetings.hello".split('.').reduce((o, i) => {
  if (o) {
    return o[i]
  }
}, obj)

console.log(ret)
```

以上代码是通过"greetings.hello"这段字符串，进而获取obj对象中的值



以上代码`"greetings.hello".split('.')` 就是数组 `["greetings", "hello"]`

reduce函数中的第一个回调函数中的第一个参数也就是`o`等于`obj`对象，第二个参数`i`变成了 `["greetings", "hello"]` 数组中的每项

第一次执行 `o 是 {greetings: {hello: "你好"}}` `i 是 "greetings"` ，返回 `o[i] 也就是 {hello: "你好"}` 此时 `o 变成 {hello: "你好"}`

进入下一次reduce回调

第二次执行 `o 是 {hello: "你好"}` `i 是 "hello"` ，返回 `o[i] 也就是 "你好"`

结束reduce

返回 `"你好"`



啊？reduce函数还能这么用！