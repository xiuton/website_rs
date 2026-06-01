---
title: "ES6 标准入门 - 阮一峰"
date: 2023-02-20 12:58:00
author: 干徒
link: https://www.cnblogs.com/ganto/articles/17136991.html
---

# ES6 标准入门 - 阮一峰

# Object.assign()

```js
const a = {name: '小华', age: 19, sex: '女'}
const b = {name: '小华', age: 20}
const c = Object.assign(a, b)
console.log(a)
console.log(b)
console.log(c)
// {name: '小华', age: 20, sex: '女'}
// {name: '小华', age: 20}
// {name: '小华', age: 20, sex: '女'}

```

通过上方代码可以看出，通过`Object.assign()`方法，会修改原对象a，并且返回的c对象和a对象一样，b对象不会修改被对象，`Object.assign()`第一个对象为目标对象，后边的无论多少都是源对象，目标对象是合并后，要返回的对象，在时机使用过程中一般第一个对象写成一个空对象，这样才不会对其他对象造成影响。

```js
const a = {name: '小华', age: 19, sex: '女'}
const b = {name: '小华', age: 20}
Object.assign({}, a, b);

```

# new set()

## 去重

这里用到了Rest参数

```js
let arr = [1,1,2,2,2,2,2,3,3,3,4,4,4]
let item = [...new Set(arr)] // 这里放在数组里面使用Rest参数将去重后的数值一个一个存放到item数组中
console.log(item)
// [1,2,3,4]

```

# 新增方法

## Array.from()

将伪数组转化为真正的数组

*   转化后数组会根据伪数组中的`length: 2`属性进行，也就是如果length的值为2，只会转化伪数组的前两个值

```js
let likeArr  = {
  0:'React',
  1:'Vue',
  2:'Angular',
  3:'Node',
  4:'Vant',
  'length':2
}
let arr = Array.from(likeArr);
console.log(arr); // (2) ['React', 'Vue']

```

## Object.freeze()

```js
const foo = {name: '小明'}
foo.name = '小华'
console.log(foo); // {name: '小华'}

```

如上代码，虽然const声明的为常量，其foo指向的内存地址不可改变，但其内部的属性可以改变，也就是上述代码可以正确执行。

如果使用`Object.freeze()`方法，则会把对象冻结，则表示不仅foo指向的内存地址不可改变，其内部的属性也不会被改变。如下代码，在使用了`Object.freeze()`方法之后，更改foo的属性将不起作用。

```js
const foo = {name: '小明'}
Object.freeze(foo)
foo.name = '小华'
console.log(foo); // {name: '小明'}

```

如果在使用了`Object.freeze()`方法后，再开启严格模式`'use strict'`，代码更会报错。

```js
'use strict'
const foo = {name: '小明'}
Object.freeze(foo)
foo.name = '小华'
console.log(foo); // （报错）

```

## Object.keys()

### 冷知识

对象可以通过以下两种方式获取值。

```js
let obj = {name1: '小强', name2: '小虎', name3: '小华', name4: '小明'}
console.log(obj['name1'], obj.name1); // 小强 小强

```

```js
let obj = {name1: '小强', name2: '小虎', name3: '小华', name4: '小明'}
const objKeys = Object.keys(obj)
console.log(objKeys) // (4) ['name1', 'name2', 'name3', 'name4']

```

如上代码所示，`Object.keys`的主要作用就是将对象中的所有`key`打包成数组进行返回。然后可以通过`forEach()`对对象的值进行遍历，如下代码所示。

```js
let obj = {name1: '小强', name2: '小虎', name3: '小华', name4: '小明'}
Object.keys(obj).forEach((key, index) => {
  console.log(obj[key]) // 小强 小虎 小华 小明
})

```

**注意**：这里的取值采用`obj[key]`的方式，因为`Object.keys(obj)`返回的对象`key`为字符串，如果取值用`obj.'name1'`显然不对。

# ES6 声明变量的六种方法

1.  var **\[ES5\]**
2.  function **\[ES5\]**
3.  let **\[ES6\]**
4.  const **\[ES6\]**
5.  import **\[ES6\]**
6.  class **\[ES6\]**

# Generator

`yield`表达式只能用在Generator函数内，用在其他地方将报错。

```js
function* f() {
  let a = 0;
  while(true){
  	yield a
    a++
  }
}

const [a, b, c, d, e, ff, g] = f()
console.log(a, b, c, d, e, ff, g)

```

# 作用域

因为在foo函数作用域中，新声明了一个x变量，而y()函数中的x其实是foo函数参数那里新声明的x，x=2也是改变的foo函数参数那里新声明的x，foo函数内到的打印x，遵循就近原则，先找自己作用域下的x，也就是x=3。在下面代码中声明了三个x变量，根据不同的作用域，改变不同的x变量

```js
var x = 1;
function foo(x, y = function() { x = 2; }) {
  let x = 3;
  console.log('1',x); // 3
  y();
  console.log('2',x); // 3
}
console.log('3',x); // 1
foo()
console.log('4',x); // 1

```