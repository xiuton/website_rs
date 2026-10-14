---
title: "浅谈React中的mutation"
date: 2024-03-27 11:56:00
author: 干徒
link: https://www.cnblogs.com/ganto/articles/18098638
tags: ["JavaScript", "React", "技术"]
---
> 官方文档：[保持组件纯粹](https://zh-hans.react.dev/learn/keeping-components-pure)、[更新 state 中的对象](https://zh-hans.react.dev/learn/updating-objects-in-state)、[更新 state 中的数组](https://zh-hans.react.dev/learn/updating-arrays-in-state)

# 保持组件纯粹

这里说的纯粹也就是JS中的纯函数

那么，什么是纯函数？

纯函数 通常具有如下特征（摘自React官网）：

*   **只负责自己的任务。**它不会更改在该函数调用前就已存在的对象或变量。
*   **输入相同，则输出相同。**给定相同的输入，纯函数应总是返回相同的结果。

***只负责自己的任务*** 简单点说就是不应该有任何副作用；  
***输入相同，则输出相同*** 就是字面意思，函数输入的参数相同的情况下，输出的内容一定是相同的；  
函数满足以上两点，那么该函数就可以被成为***纯函数***。

就如以下代码（官网示例进行简化）：

```jsx
function Recipe({ drinkers }) {
  return (
    <ol>    
      <li>drinkers: {drinkers}</li>
      <li>drinkers: {drinkers}; 0.5 * drinkers: {0.5 * drinkers}</li>
      <li>0.5 * drinkers: {0.5 * drinkers}</li>
    </ol>
  );
}

export default function App() {
  return (
    <section>
      <Recipe drinkers={2} />
      <Recipe drinkers={4} />
    </section>
  );
}

```

以上代码中的`Recipe`组件就是一个纯粹的组件，因为`Recipe`函数是一个纯函数，在参数`drinkers`的值固定的情况下，那么`{drinkers}` `{0.5 * drinkers}`将永恒不变

官网有这么一段话，强调组件应当保持纯粹：React 的渲染过程必须自始至终是纯粹的。组件应该只 返回 它们的 JSX，而不 改变 在渲染前，就已存在的任何对象或变量 — 这将会使它们变得不纯粹！  
并且给出了反例：

```jsx
let guest = 0;

function Cup() {
  // Bad：正在更改预先存在的变量！
  guest = guest + 1;
  return <h2>Tea cup for guest #{guest}</h2>;
}

export default function TeaSet() {
  return (
    <>
      <Cup />
      <Cup />
      <Cup />
    </>
  );
}

```

`Cup`组件正在读写其外部声明的 guest 变量，多次调用该组件会产生不同的结果，而`Cup`组件的结果将变的无法预测

React官网将这种 `组件改变了 预先存在的 变量的值` 的现象称为 突变（mutation）

# 更新 state 中的对象

在state中可以存放JavaScript中的任何值，这些值应该是不可变的（immutable），它们不能被改变或是只读的。

如以下代码:

```jsx
const [count, setCount] = useState(0)
setCount(1)

```

count从0到1，其本质是新创建了一个数字1覆盖掉了数字0，0本身并没有改变  
如果你了解JS中的辟栈内，那么你应该了解，0和1并非是同一个变量，而是存在栈内存中的两个不相关的内存空间

如果你了解了上面的内容，你大致也能猜到JS中的对象应该如何更新状态

```jsx
const [obj, setObj] = useState({name: "张三", age: 19})
const changeObj = () => {
  obj.name = "李四"
  setObj(obj)
}

```

以上的代码对吗？很显然不对，在一开始就已经说过 在state中可以存放JavaScript中的任何值，这些值应该是不可变的（immutable），它们不能被改变或是只读的。

`obj.name = "李四"`这段代码就会制造一个**mutation**

如果运行以上代码，显然不会触发重新渲染，可以将其修改为以下代码

```jsx
const [obj, setObj] = useState({name: "张三", age: 19})
const changeObj = () => {
  setObj({...obj, name: "李四"})
}

```

现在一切完美

# 更新 state 中的数组

数组是特殊的对象，我们依然要在没有 mutation 的前提下更新数组  
我们应该遵循一个原则，不改变原数组，要利用原数组产生一个新数组，进而利用新数组去修改状态

|  | 避免使用 (会改变原始数组) | 推荐使用 (会返回一个新数组） |
| --- | --- | --- |
| 添加元素 | push，unshift | concat，\[...arr\] 展开语法（[例子](https://zh-hans.react.dev/learn/updating-arrays-in-state#adding-to-an-array)） |
| 删除元素 | pop，shift，splice | filter，slice（[例子](https://zh-hans.react.dev/learn/updating-arrays-in-state#removing-from-an-array)） |
| 替换元素 | splice，arr\[i\] = ... 赋值 | map（[例子](https://zh-hans.react.dev/learn/updating-arrays-in-state#replacing-items-in-an-array)） |
| 排序 | reverse，sort | 先将数组复制一份（[例子](https://zh-hans.react.dev/learn/updating-arrays-in-state#making-other-changes-to-an-array)） |