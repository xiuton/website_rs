---
title: "防抖函数debounce"
date: 2023-02-20 13:04:00
author: 干徒
link: https://www.cnblogs.com/ganto/p/17136969.html
---
# 防抖函数

防抖函数

## 封装

```js
// 防抖函数的封装
export default function debounce(func, delay){
  let timer = null
  return function () {
    if (timer) clearTimeout(timer)
    timer = setTimeout(() => {
      func.apply()
    }, delay)
  }
}

```

## 使用

```jsx
getHomeDataList = debounce(() => {
  getHomeData()
    .then(res => {
      console.log(res)
    })
    .catch(err => {
      console.error(err)
    })
}, 500)

```