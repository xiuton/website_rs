---
title: "JavaScript闭包直接修改内部属性"
date: 2023-11-17 07:41:00
author: 干徒
link: https://www.cnblogs.com/ganto/articles/17837785.html
tags: ["JavaScript", "技术"]
---
## 事例代码

```js
var o = (function() {
    var obj = {
        a: 1,
        b: 2,
    }
    return {
        get: function(k) {
            return obj[k]
        }
    }
})()

```

## 说明

如上一段代码，在不改变原代码的情况下，修改obj对象中的a、b属性。

## 分析

代码使用了闭包，返回一个对象，对象中有一个get函数，这个get函数只能通过`o.get('a')`的方式查看obj中的属性  
如果想要直接修改是无法修改的

所有对象都有一个原型链，该原型链继承自Object，Object上有一个`valueOf`属性，该属性为一个函数，执行可以返回对象自身

```js
var o = (function() {
    var obj = {
        a: 1,
        b: 2,
    }
    return {
        get: function(k) {
            return obj[k]
        }
    }
})()
console.log(o.get('valueOf'))

```

如上代码，可以打印出来obj的valueOf结构了，直接执行`console.log(o.get('valueOf')())`，发现已经报错了，这里报错的原因是因为在执行valueOf的this指向错误。

如下代码，演示this指向：

```js
const obj = Object.prototype.valueOf
console.log(obj()) // 报错

const obj = Object.prototype.valueOf()
console.log(obj) // 可以获取Object对象

```

那么标志着这种方式行不通

## 突破

```js
var o = (function() {
    var obj = {
        a: 1,
        b: 2,
    }
    return {
        get: function(k) {
            return obj[k]
        }
    }
})()

Object.defineProperty(Object.prototype, 'xxx', {
    get() {
        return this
    }
})

console.log(o.get('xxx'));

```

以上代码是在Object的原型上添加了一个xxx属性的访问器，因为是在Object的原型上，所以其他属性也可以读取xxx属性  
Object原型上的xxx属性的访问器，返回了this，这里的this是只要哪个对象访问这个属性，this就指向哪个对象，这样就解决了this指向的问题

当o.get('xxx')执行的时候，会触发o对象返回的get方法中返回的obj\['xxx'\]，这里就相当于o中的obj访问了xxx属性，所以这里读取o.get('xxx')，但是obj身上没有xxx属性，就会读Object身上的xxx属性，就会触发xxx属性访问器中的get方法，那么this指向obj，也就是obj对象

在此时我们就可以修改obj属性值了

```js
var o = (function() {
    var obj = {
        a: 1,
        b: 2,
    }
    return {
        get: function(k) {
            return obj[k]
        }
    }
})()

Object.defineProperty(Object.prototype, 'xxx', {
    get() {
        return this
    }
})
o.get('xxx').a = 100
o.get('xxx').b = 200
console.log(o.get('a'), o.get('b')); // 100 200

```

## 如何避免这种情况

### 判断对象属性是否存在

可以在读取obj的属性的时候加一个判断

```js
var o = (function() {
    var obj = {
        a: 1,
        b: 2,
    }
    return {
        get: function(k) {
            if (obj.hasOwnProperty(k)) {
                return obj[k]
            }
            return undefined
        }
    }
})()

Object.defineProperty(Object.prototype, 'xxx', {
    get() {
        return this
    }
})
o.get('xxx').a = 100
o.get('xxx').b = 200
console.log(o.get('a'), o.get('b'));

```

这样处理后就无法读取xxx属性了，以为xxx属性不是obj身上存在的属性

### 将对象原型设置为null

```js
var o = (function() {
    var obj = {
        a: 1,
        b: 2,
    }
    Object.setPrototypeOf(obj, null)
    return {
        get: function(k) {
            return obj[k]
        }
    }
})()

Object.defineProperty(Object.prototype, 'xxx', {
    get() {
        return this
    }
})
o.get('xxx').a = 100
o.get('xxx').b = 200
console.log(o.get('a'), o.get('b'));

```

这样obj就不存在原型了，无法去Object身上读取xxx属性