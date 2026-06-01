---
title: "React中使用Immer编写简洁的更新逻辑"
date: 2024-03-28 09:08:00
author: 干徒
link: https://www.cnblogs.com/ganto/articles/18100748
tags: ["JavaScript", "React", "前端", "技术"]
---

# React中使用Immer编写简洁的更新逻辑

在 [浅谈React中的mutation](https://www.cnblogs.com/ganto/articles/18098638) 一文中，突出表达了对象和数组类型的state，不宜直接修改原对象和数组，直接修改会制造一个mutation，并且无法触发React的重新渲染

在官方文档中提到一个库 [Immer](https://github.com/immerjs/use-immer) ，Immer 可以让我们直接修改对象和数组。

## useImmer

需要先安装use-immer库

```sh
pnpm add use-immer -D

```

```jsx
import { useImmer } from 'use-immer';

export default function App() {
  const [state, setState] = useImmer({
    count: 0,
    name: 'Joe'
  })

  const handleClick = () => {
    setState(draft => {
      draft.count += 1
    })
    console.log(state)
  }

  return (
    <div className="app">
      <h1>{state.count}</h1>
      <h2>{state.name}</h2>
      <button onClick={handleClick}>CLICK ME</button>
    </div>
  )
}

```

## useState

```jsx
import { useState } from 'react';

export default function App() {
  const [state, setState] = useState({
    count: 0,
    name: 'Joe'
  })

  const handleClick = () => {
    setState(obj => {
      obj.count += 1
      return obj // setState的参数-匿名函数需要有返回值
    })
    console.log(state) // 此处的打印结果并非最新的state，但是依然能反应出state已被修改
  }

  return (
    <div className="app">
      <h1>{state.count}</h1>
      <h2>{state.name}</h2>
      <button onClick={handleClick}>CLICK ME</button>
    </div>
  )
}

```