---
title: "TypeScript类型取反"
date: 2024-03-13 23:01:00
author: 干徒
link: https://www.cnblogs.com/ganto/articles/18071774
tags: ["TypeScript", "技术"]
---

# TypeScript类型取反

读码见意

```ts
function func(arg: number) {}
func(1)
func("zs") // ERROR Argument of type 'string' is not assignable to parameter of type 'number'.

```

如想要将函数参数定义为非number的其他类型，则可以这样

```ts
function func<T>(arg: T extends number ? never : T) {}
func(1) // ERROR Argument of type 'number' is not assignable to parameter of type 'never'.
func("zs")
func(true)
func([])
func({})
func(() => {})

```

写成一个通用的type类型

```ts
type negationType<T, U> = T extends U ? never : T
function func<T>(arg: negationType<T, number>) {}
func(1) // ERROR Argument of type 'number' is not assignable to parameter of type 'never'.
func("zs")
func(true)
func([])
func({})
func(() => {})

```