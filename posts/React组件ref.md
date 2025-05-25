---
title: "React组件ref"
date: "2023-01-17 10:35:00"
author: "干徒"
tags: ["JavaScript", "React"]
---

# React中forwardRef的用法

> 官网文档：https://zh-hans.react.dev/reference/react/forwardRef



今天我们就一起学习一下React中的ref怎么获取组件实例，只讨论函数式组件中的ref

如下代码，我们直接在App根组件中，意图只使用ref就获取Footer组件实例

细心的同学可能已经发现控制台报错了，提示我们应该使用forwardRef，并且编辑器也提示`ref={footRef}`类型不对

这是因为React不会默认暴露组件中的东西，如果开发者想要从组件中暴露出来东西，需要手动挡进行操作，这也是为了组件数据安全的考量

App.tsx

```tsx
import { useRef } from 'react'
import { Footer } from '@/components/footer'

const App = () => {
  const footRef = useRef(null)
  const getFootRef = () => {
    console.log(footRef.current)
  }
  return (
    <div className='app'>
      <h1>App</h1>
      <button onClick={getFootRef}>Show footRef</button>
      <Footer ref={footRef}></Footer>
    </div>
  )
}

export default App
```

components/Footer/index.tsx

```tsx
import { useState } from 'react'

export const Footer = () => {
  const [count, setCount] = useState(0)
  return (
    <div className="footer">
      <h1>Footer</h1>
      <p>{ count }</p>
      <button onClick={() => setCount(count + 1)}>Click Me</button>
    </div>
  )
}
```



那么我们如何获取组件实例呢

直接请出forwardRef来对Footer组件进行改造

App.tsx 不变

Footer组件改造如下

components/Footer/index.tsx

```tsx
import { useState, forwardRef } from 'react'

export const Footer = forwardRef<HTMLDivElement>((props, ref) => {
  console.log(props)
  const [count, setCount] = useState(0)
  return (
    <div className="footer">
      <h1>Footer</h1>
      <p ref={ref}>{ count }</p>
      <button onClick={() => setCount(count + 1)}>Click Me</button>
    </div>
  )
})
```

可以看出已经能打印出来p标签了，等等...为什么ref绑定在了p标签上，是的没错，想将哪个标签暴露给父组件，就绑定给谁身上

forwardRef的用法就是直接套在组件函数外部，此时组件函数入参的第一个参数依然是props，第二个参数就是ref

此时更应该注意的是，forwardRef需要编写泛型，该泛型就是需要暴露出去的东东的类型，如上代码，我们将p标签暴露出去了

如想将组件状态数据暴露出去，需要使用一个Hook，那就是`useImperativeHandle`

使用方法见以下代码

App.tsx不变

Footer组件改造为以下

components/Footer/index.tsx

```tsx
import { useState, useRef, forwardRef, useImperativeHandle } from 'react'

export const Footer = forwardRef<object>((props, ref) => {
  console.log(props)
  const [count, setCount] = useState(0)
  const pRef = useRef<HTMLParagraphElement>(null)

  useImperativeHandle(ref, () => {
    return {
      count,
      pRef
    }
  })

  return (
    <div className="footer">
      <h1>Footer</h1>
      <p ref={pRef}>{ count }</p>
      <button onClick={() => setCount(count + 1)}>Click Me</button>
    </div>
  )
})
```

很明显的问题，p标签如要一同暴露，不能直接使用`ref={ref}`进行暴露，需要将p标签设置一个useRef，然后将其他同其他状态数据一起暴露

