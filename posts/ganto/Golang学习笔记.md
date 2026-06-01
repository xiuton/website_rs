---
title: "Golang学习笔记"
date: 2023-08-16 17:20:00
author: 干徒
link: https://www.cnblogs.com/ganto/articles/17635693.html
---

# Golang学习笔记

# Hello Golang

```go
// Hello Golang
package main

import "fmt"

func main() {
  	fmt.Println("Hello Golang")
}

```

Gogs是unknwon无闻主导开发的git管理平台，类似于gitlab，可以本地部署，采用Go语言开发，要比gitlab更加轻量。

[Gogs官网](//gogs.io)

Gitea是Gogs的克隆项目，是属于Gogs的一个分支项目。

gitea.com gitea.io

# Golang

天生支持高并发

```go
package main

import (
	"fmt"
  "time"
)

func goFunc(i int) {
  fmt.Println("goroutine ", i, " ...")
}

func main() {
  for i := 0; i < 10000; i++ {
    go goFunc(i) // 开启一个并发协程
  }
  
  time.Sleep(time.Second)
}

```

# Windows - Golang开发环境的搭建

## 安装配置SDK（软件开发工具包）

go官网: [https://go.dev](https://go.dev)

将官网下载的SDK直接解压到本地目录D:\\Program Files\\go

在D:\\Program Files\\go\\bin目录cmd，执行go version指令查看版本，如能正常打印版本，则表示安装成功。

```sh
D:\Program Files\go\bin>go versioin
go version go1.19.1 windows/amd64

```

## 环境变量的配置

| key | value |
| --- | --- |
| GOROOT | D:\\Program Files\\go |
| Path | %GOROOT%\\bin |
| GOPATH | E:\\Development\\GolangLearn |

*   GOROOT: 指定SDK安装的路径
    
*   Path: 引用上方的GOROOT环境变量然后进入bin目录下
    
*   GOPATH: 工作目录，项目的开发目录
    

配置好之后，就可以全局使用go命令

# \*基础

# 第一个go程序

## 目录

```sh
 GoProject
 └── src
     └── go_code
         ├── project01
         |   ├── main
         |   |   ├── hello.exe
         |   |   └── hello.go
         |   └── package
         └── project02

```

## 程序

```go
// hello.go

package main
import "fmt"
func main() {
	fmt.Println("hello go")
}

```

```sh
go build hello.go # 执行go build 生成.exe文件
hello.exe # hello go

```

```sh
go run hello.go # go run可以直接将.go文件转换成.exe文件，然后运行

```

打包指定文件名

```sh
go build -o myhello.exe hello.go

```

## 注意

*   在Go语言中，每个文件都必须归属一个包
*   定义的包或变量、导入的包，必须使用，否则报错
*   代码结尾不需要分号，多条语句不要写在同一行
*   严格区分大小写

# 转义字符

| 符号 | 含义 |
| --- | --- |
| `\t` | 一个制表符，实现对齐的功能 |
| `\n` | 换行符 |
| `\\` | 一个`\` |
| `\"` | 一个`"` |
| `\r` | 一个回车 fmt.Println("Hello \\r Golang") |

# 数据类型

先背诵Java的数据类型：

| 类型名称 | 字节空间 | 应用场景 |
| --- | --- | --- |
| byte | 1Byte | 字节数据 |
| short | 2Byte | 短整数 |
| int | 4Byte | 普通整数 |
| long | 8Byte | 长整数 |
| float | 4Byte | 浮点数 |
| double | 8Byte | 双精度浮点数 |
| boolean | 1Byte | 逻辑变量(true,flase) |
| char | 2Byte | 一个字符 |

Go的数据类型：

| 类型 | 描述 |
| --- | --- |
| 布尔型 | 布尔型的值只可以是常量 true 或者 false。一个简单的例子：var b bool = true。 |
| 数字类型 | 整型 int 和浮点型 float32、float64，Go 语言支持整型和浮点型数字，并且支持复数，其中位的运算采用补码。 |
| 字符串类型: | 字符串就是一串固定长度的字符连接起来的字符序列。Go 的字符串是由单个字节连接起来的。Go 语言的字符串的字节使用 UTF-8 编码标识 Unicode 文本。 |
| 派生类型: | 包括：  
(a) 指针类型（Pointer）  
(b) 数组类型  
(c) 结构化类型(struct)  
(d) Channel 类型  
(e) 函数类型  
(f) 切片类型  
(g) 接口类型（interface）  
(h) Map 类型 |

# 变量声明

## 四种变量的声明方式

```go
package main

import (
  "fmt"
)

// 声明全局变量
var num1 int = 100
var num2 = 200

func main() {
  // 方法一：声明一个变量 int类型变量的默认值是0
  var a int
  fmt.Println("a =", a)
  fmt.Printf("type of a = %T\n", a)
  
  // 方法而：声明一个变量，初始化一个值
  var b int = 100
  fmt.Println("b =", b)
  fmt.Printf("type of b = %T\n", b)

  var b1 string = "hello go"
  fmt.Printf("b1 = '%s', type of b1 = %T\n", b1, b1)
  
  // 方法三：在初始化的时候，可以省去数据类型，通过值自动匹配当前的变量的数据类型
  var c = 100
  fmt.Println("c =", c)
  fmt.Printf("type of c = %T\n", c)

  var c1 = "hello go"
  fmt.Printf("c1 = '%s', type of c1 = %T\n", c1, c1)

  // 方法四：常用 省去var关键字，直接自动匹配
  e := 100
  fmt.Println("e =", e)
  fmt.Printf("type of e = %T\n", e)
  

  // 全局变量打印
  fmt.Println(num1, num2)
}

```

方法一、方法二、方法三是可以声明全局变量的

## 声明多个变量

```go
package main

import (
  "fmt"
)

func main() {
  var x, y int = 234, 293
  fmt.Println("x =", x, "y =", y)

  var a, b = 100, "b"
  fmt.Println(a, b)

  var(
    xx int = 1
    yy bool = true
  )
  fmt.Println(xx, yy)
}

```

## 变量的内置 pair 结构详细说明

# 常量声明

```go
package main

import (
  "fmt"
)

func main() {
  const MAX int = 10
  fmt.Println(MAX)
}

```

## IOTA

iota只能出现在const定义的变量中

```go
package main

import (
  "fmt"
)

// const 定义枚举类型
const (
  BEIJING = 0
  SHANGHAI = 1
  SHENZHEN = 2
  ZHENGZHOU = 3
)

// 添加一个关键字iota，每行的iota都会累加1，第一行的iota默认值是0
const (
  A = iota // iota = 0
  B // iota = 1
  C // iota = 2
  D // iota = 3
)

const (
  NUM1, NUM2 = iota+1, iota+2 // iota = 0; iota + 1 = 1, iota + 2 = 2
  NUM3, NUM4 // iota = 1; iota + 1 = 2, iota + 2 = 3
  NUM5, NUM6 // iota = 2; iota + 1 = 3, iota + 2 = 4
  NUM7, NUM8 // iota = 3; iota + 1 = 4, iota + 2 = 5
)

func main() {
  const MAX int = 10
  fmt.Println(MAX)

  fmt.Println(BEIJING, SHANGHAI, SHENZHEN, ZHENGZHOU)
  
  fmt.Println(A, B, C, D)

  fmt.Println("\n", NUM1, NUM2, "\n", NUM3, NUM4, "\n", NUM5, NUM6, "\n", NUM7, NUM8)
}

```

# 位运算符

<< 左移，>> 右移

```go
package main

import "fmt"

func main() {
	const (
		_  = iota
		KB = 1 << (10 * iota) // 十进制1转换成二进制0001，往左位移10位，变成0001 0000 0000，十进制为1024
		MB = 1 << (10 * iota)
		GB = 1 << (10 * iota)
		TB = 1 << (10 * iota)
		PB = 1 << (10 * iota)
	)
	fmt.Println(KB) // 1024
	fmt.Println(MB) // 1048576
	fmt.Println(GB) // 1073741824
	fmt.Println(TB) // 1099511627776
	fmt.Println(PB) // 1125899906842624
}

```

# 函数

## 多返回值

```go
package main

import (
  "fmt"
)

func foo1(a string, b int) int {
  fmt.Println("a =", a)
  fmt.Println("b =", b)
  
  c := 10
  return c
}

// 返回多个返回值，匿名的
func foo2(a string, b int) (int, int) {
  fmt.Println("a =", a)
  fmt.Println("b =", b)
  return 666, 999
}

// 返回多个返回值，有形参的
func foo3(x int, y int) (r1 int, r2 int) { // 这里返回的r1, r2 为int类型，所以默认值为0
  r1 = x
  r2 = y
  return

  // return x, y // 和上方代码效果一样
}

func foo4(a int, b int) (r1, r2 int) {
  r1 = a
  r2 = b
  return
}

func main() {
  c := foo1("haha", 11)
  fmt.Println("return =", c)

  return1, return2 := foo2("xixi", 99)
  fmt.Println(return1, return2)

  r11, r22 := foo3(1, 2)
  fmt.Println(r11, r22)

  rx, ry := foo4(3, 4)
  fmt.Println(rx, ry)
}

```

# import导包路径问题与init方法调用流程

init函数一般先于main函数前执行，可以在init中进行初始化操作

```go
package main

import "lib1" // 注意：这里是找 D:\Program Files\go\src\ 下的lib1
import "lib2"

func main() {
	lib1.Lib1Test()
	lib2.Lib2Test()
}

```

# import匿名及别名导包方式

*   `_`为匿名别名，意思为导入了包，并且起了匿名别名，但是不使用也不会报错，会调用匿名导入包的init函数
    
*   `mylib2`为别名，可以使用别名`mylib2.Lib2Test()`调用包中的方法
    
*   `.`为将包中的所有方法导入到当前包的作用中，可以直接调用`Lib3Test()`（少用，不同包中同名函数，会造成歧义）
    

```go
package main
import (
	_ "GolangStudy/5-init/lib1"
  mylib2 "GolangStudy/5-init/lib2"
  . "GolangStudy/5-init/lib3"
)

func main() {
  // lib1.Lib1Test()
  
  // lib2.Lib2Test()
  mylib2.Lib2Test()
  
  Lib3Test()
}

```

另一种写法，和上方一样

```go
package main

import _ "GolangStudy/5-init/lib1"
import mylib2 "GolangStudy/5-init/lib2"
import . "GolangStudy/5-init/lib3"

func main() {
  // lib1.Lib1Test()
  
  // lib2.Lib2Test()
  mylib2.Lib2Test()
  
  Lib3Test()
}

```

# 指针

引子

```go
package main

import "fmt"

func changeValue(p int) {
	p = 10
}

func main() {
	var a int = 1
	changeValue(a)
	fmt.Println("a =", a) // a = 1
}

```

以上代码会打印出`a = 1`，这里是值传递，所以p和a没有关系

可以通过引用传递的方式，将两者建立关系

```go
package main

import "fmt"

func changeValue(p *int) {
	*p = 10 // 修改p指向的那个内存地址的值，也就是a的值
}

func main() {
	var a int = 1
	changeValue(&a) // 将a的引用（内存地址）传递过去
	fmt.Println("a =", a) // a = 10
}

```

指针例子：

```go
package main

import "fmt"

func swap(a int, b int) {
  var temp int
  temp = a
  a = b
  b = temp
}

func main() {
  var a int = 10
  var b int = 20
  
  // swap
  
  fmt.Println("a =", a, " b =", b)
}

```

以上，只是值传递，并不会改变main函数中的a,b变量

```go
package main

import "fmt"

func swap(a *int, b *int) {
  var temp int
  temp = *a // 把a的内存地址赋值给temp
  *a = *b // 把b的内存地址赋值给a的内存地址
  *b = temp // 把temp的值赋值给b的内存地址
}

func main() {
  var a int = 10
  var b int = 20
  
  swap(&a, &b)
  
  fmt.Println("a =", a, " b =", b)
}

```

## 二级指针

```go
package main

import "fmt"

func main() {
	var a int = 10

  var aa *int // 一级指针
	aa = &a
	fmt.Println(&a, " ", aa)

	var aaa **int // 二级指针
	aaa = &aa

	fmt.Println(&aa, " ", aaa)

	**aaa = 22

	fmt.Println(a) // 22
}

```

# defer语句调用顺序

defer定义的语句放在函数结束前进行触发，比如go语言中打开一个文件，需要最后关闭，可以直接添加defer语句，进行关闭

类似于Java中try catch finally 中的finally，Java中的finally无论程序进入到try还是catch中，都最后执行finall中的语句

```go
package main

import "fmt"

func main() {

	// 写入defer关键字
	defer fmt.Println("main -------------> end1")
  defer fmt.Println("main -------------> end2")

	fmt.Println("main::hello go 1")
	fmt.Println("main::hello go 2")
	/*
		main::hello go 1
		main::hello go 2
		main -------------> end2
		main -------------> end1
	*/
}

```

如果存在多个defer定义的语句，那么类似于栈的原理，先进后出，main end1先进，所以后出

defer 和 return 执行顺序对比

```go
package main

import "fmt"

func deferFunc() int {
	fmt.Println("defer func called...")
	return 0
}

func returnFunc() int {
	fmt.Println("return func called...")
	return 0
}

func returnAndDefer() int {
	defer deferFunc()
	return returnFunc()
}

func main() {
	returnAndDefer()
}

```

可以看到结果，defer还是在最后执行

# 数组与动态数组的区别

## 定义数组语法

```go
package main

import "fmt"

func main() {
	// 固定长度的数组
	var myArray [10]int
	var i int 
	for i = 0; i < len(myArray); i++ {
		fmt.Println(myArray[i], " ", i)
	}

	// 另一种定义固定长度数组的方式
	myArray1 := [6]int{1,2,3,4}
	// 另一种for循环，for range
	for index, value := range myArray1 {
		fmt.Println(value, " ", index)
	}

	// 查看数组的数据类型
	fmt.Printf("myArray type = %T\n", myArray) // myArray type = [10]int
	fmt.Printf("myArray1 type = %T\n", myArray1) // myArray1 type = [6]int
}

```

## 值传递

```go
package main

import "fmt"

func printArray(myArray [4]int) {
  for index, value := range myArray {
    fmt.Println("value =", value)
  }
  myArray[0] = 100
}

func main() {
  myArray := [4]int{1,2,3,4}
  printArray(myArray)
  
  for index, value := range myArray {
    fmt.Println("value =", value)
  }
}

```

## 引用传递

```go
package main

import "fmt"

func printArray(myArray []int) {
  for index, value := range myArray {
    fmt.Println("value =", value)
  }
  myArray[0] = 100
}

func main() {
  myArray := []int{1,2,3,4}
  printArray(myArray)
  
  for index, value := range myArray {
    fmt.Println("value =", value)
  }
}

```

引用传递在Golang中被称为切片slice

动态数组在传递上是引用传递，而且不同元素长度的动态数组他们的形参是一致

## 数组切割

```go
package main

import "fmt"

func main() {
	//定义一个数组
	a1 := []int{1, 2, 3, 4, 5, 6, 7, 8, 9}
	s2 := a1[0:4] //基于一个数组切割  [0:4]为起始、结束下标，左包含 右不包含  即为[1,2,3,4]
	fmt.Println(s2) // [1 2 3 4]
}

```

# 切片

切片的初始化方式有以下几种

```go
var nums []int // 值
nums := []int{1, 2, 3} // 值
nums := make([]int, 0, 0) // 值
nums := new([]int) // 指针

```

## slice切片的4种声明定义方式

Go 语言切片是对数组的抽象。

Go 数组的长度不可改变，在特定场景中这样的集合就不太适用，Go 中提供了一种灵活，功能强悍的内置类型切片("动态数组")，与数组相比切片的长度是不固定的，可以追加元素，在追加时可能使切片的容量增大。

### 第一种

声明slice1是一个切片，并且初始化默认值是：1，2，3。切片长度len是3

```go
package main

import "fmt"

func main() {
	slice1 := []int{1,2,3}
	fmt.Printf("len = %d, slice1 = %v\n", len(slice1), slice1) // len = 3, slice1 = [1 2 3]
}

```

其中`%d`为切片长度的占位符，`%v`则是打印切片的详细信息

### 第二种

声明slice1是一个切片，但是并没有给slice1分配空间

```go
package main

import "fmt"

func main() {
	var slice1 []int
  // 因为没有分配空间，所以以下代码会报错
  slice1[0] = 10 // 报错
	fmt.Printf("len = %d, slice1 = %v\n", len(slice1), slice1) // len = 0, slice1 = []
  
  // 判断一个slice是否为0
  if slice1 == nil {
    fmt.Println("slice1是一个空切片")
  } else {
    fmt.Println("slice1是有空间的")
  }
}

```

通过make()开辟空间，默认值为0

```go
package main

import "fmt"

func main() {
	slice1 := []int{1,2,3}
	slice1 = make([]int, 3)
  slice1[0] = 10
	fmt.Printf("len = %d, slice1 = %v\n", len(slice1), slice1) // len = 3, slice1 = [0 0 0]
}

```

### 第三种

声明slice1是一个切片，同时给slice1分配3个空间，初始化值为0

```go
package main

import "fmt"

func main() {
	var slice1 []int = make([]int, 3)
	fmt.Printf("len = %d, slice1 = %v\n", len(slice1), slice1) // len = 3, slice1 = [0 0 0]
}

```

第三种的简写形式

```go
package main

import "fmt"

func main() {
	slice1 := make([]int, 3)
	fmt.Printf("len = %d, slice1 = %v\n", len(slice1), slice1) // len = 3, slice1 = [0 0 0]
}

```

make() 其实可以传递三个参数：

*   第一个参数是切片的 `类型`
*   第二个参数是切片的 `长度`
*   第三个参数是切片的 `容量`

```go
package main

import "fmt"

func main() {
	nums := make([]int, 1, 2)
	fmt.Println(nums, len(nums), cap(nums)) // [0] 1 2
}

```

其中 **长度** 永远 *少于等于* **容量**

### 第四种

第四种是指针

```go
package main

import "fmt"

func main() {
	nums := new([]int) // 指针
	fmt.Println(nums) // &[]
}

```

用法暂不可知

## slice切片容量追加与截取

### 切片追加

```go
package main

import "fmt"

func main() {
  //                       len, cap
	var numbers = make([]int, 3, 5)

	// len = 3, cap = 5, numbers = [0 0 0] 其中长度3，容量5
	fmt.Printf("len = %d, cap = %d, numbers = %v\n", len(numbers), cap(numbers), numbers)

	// 向numbers切片追加一个元素1， numbers len = 4, [0,0,0,1], cap = 5
	numbers = append(numbers, 11)
	// len = 4, cap = 5, numbers = [0 0 0 11]
	fmt.Printf("len = %d, cap = %d, numbers = %v\n", len(numbers), cap(numbers), numbers)
	
	numbers = append(numbers, 23)
	// len = 5, cap = 5, numbers = [0 0 0 11 23]
	fmt.Printf("len = %d, cap = %d, numbers = %v\n", len(numbers), cap(numbers), numbers)
	
	// 此时容量已经满了，在追加，会自动在开辟一个cap大小的容量，扩容到原容量的2倍
	numbers = append(numbers, 41)
	// len = 6, cap = 10, numbers = [0 0 0 11 23 41]
	fmt.Printf("len = %d, cap = %d, numbers = %v\n", len(numbers), cap(numbers), numbers)
	
	var numbers2 = make([]int, 3)
	// len = 3, cap = 3, numbers2 = [0 0 0]
	fmt.Printf("len = %d, cap = %d, numbers2 = %v\n", len(numbers2), cap(numbers2), numbers2)
	numbers2 = append(numbers2, 23)
	// len = 4, cap = 6, numbers2 = [0 0 0 23]
	fmt.Printf("len = %d, cap = %d, numbers2 = %v\n", len(numbers2), cap(numbers2), numbers2)
}

```

### 截取

```go
package main

import "fmt"

func main() {
	s := []int{2,4,1} // len = 3, cap = 3

	// [0, 2)
	s1 := s[0:2]
	fmt.Println(s1) // [2, 4]
}

```

| \_ | 说明 |
| --- | --- |
| \[:\] | 从第0位截取到末尾，也就是截取全部 |
| \[2:\] | 从第2位截取到末尾 |
| \[:3\] | 从第0位截取到第3位，但不包含第3位 |
| \[0:2\] | 从第0为截取到第2位，但不包含第2位 |

### copy

以下类似于js中的深拷贝/浅拷贝

```go
package main

import "fmt"

func main() {
	s := []int{2,4,1,23,5,6,223,1231}

	s1 := s[:2]
	fmt.Println(s1)
	s1[0] = 123 // 改变s1的第0位值，s也会改变
	fmt.Println(s1)
	fmt.Println(s)
}

```

使用copy 可以将底层数组的slice一起进行拷贝

```go
package main

import "fmt"

func main() {
	s := []int{1,2,3}

	s1 := make([]int, 3)

	// 将s中的值，一次拷贝到s1中
	copy(s1, s)
	s1[0] = 123

	fmt.Println(s1) // [123 2 3]
	fmt.Println(s) // [1 2 3]
}

```

以上代码s1和s没有关联

# Map 集合

Map 是一种无序的键值对的集合。

## 语法

```go
package main

import "fmt"

func main() {
	// 声明myMap1是一种map类型 key是string类型，value是string类型
	var myMap1 map[string]string
	if myMap1 == nil {
		fmt.Println("myMap1 是一个空map")
	} else {
		fmt.Println("myMap1 不是一个空map")
	}
}

```

make开辟空间

第一种方式

```go
package main

import "fmt"

func main() {
	// 声明myMap1是一种mao类型 key是string类型，value是string类型
	var myMap1 map[string]string

	// 在使用map前，需要先用make给map分配数据空间
	myMap1 = make(map[string]string)

	myMap1["id"] = "001"
	myMap1["name"] = "Golang"

	fmt.Println(myMap1)
}

```

第二种方式使用make开辟空间，是上方代码的简写形式

```go
package main

import "fmt"

func main() {
	// 声明myMap1是一种map类型 key是int / string类型，value是string类型；并开辟容量为1的空间
	myMap1 := make(map[int]string, 1)
	// myMap1 := make(map[string]string)
  
  // 创建一个初始容量为 10 的 Map
	// m := make(map[string]int, 10)

	myMap1[1] = "001"
	myMap1[2] = "Golang"

	fmt.Println(myMap1)
}

```

第三种声明方式

使用字面量创建 Map

```go
package main

import "fmt"

func main() {
	myMap1 := map[string] string {
		"one": "Golang",
		"two": "Java",
		"three": "C",
	}

	fmt.Println(myMap1)
}

```

## 操作map

### 获取元素

```go
package main

import "fmt"

func main() {
	// 使用字面量创建 Map
	m := map[string]string{
		"apple": "apple",
		"banana": "banana",
		"orange": "orange",
	}
	// 获取键值对
	v1 := m["apple"]
	v2, ok := m["pear"]  // 如果键不存在，ok 的值为 false，v2 的值为该类型的零值
	fmt.Println("v1:", v1, ", v2:", v2, ", ok:", ok, ".")
  // v1: apple , v2:  , ok: false .
}

```

### 修改元素

```go
package main

import "fmt"

func main() {
	// 使用字面量创建 Map
	m := map[string]string{
		"apple": "apple",
		"banana": "banana",
		"orange": "orange",
	}
	m["apple"] = "xiaomi"
  fmt.Println(m["apple"])
  // xiaomi
}

```

### 获取 Map 的长度

```go
package main

import "fmt"

func main() {
	// 使用字面量创建 Map
	m := map[string]string{
		"apple": "apple",
		"banana": "banana",
		"orange": "orange",
	}
  len := len(m)
  fmt.Println(len)
  // 3
}

```

### 遍历 Map

```go
package main

import "fmt"

func main() {
	// 使用字面量创建 Map
	m := map[string]string{
		"apple": "apple",
		"banana": "banana",
		"orange": "orange",
	}
	for key, value := range m {
		fmt.Println(key, value)
	}
	/*
		apple apple
		banana banana
		orange orange
	*/
}

```

### 删除元素

```go
package main

import "fmt"

func main() {
	// 使用字面量创建 Map
	m := map[string]string{
		"apple": "apple",
		"banana": "banana",
		"orange": "orange",
	}
	// 删除键值对
	delete(m, "banana")
	for key, value := range m {
		fmt.Println(key, value)
	}
	/*
		apple apple
		orange orange
	*/
}

```

## map的基本使用方式

```go
package main

import "fmt"

func main() {
	// 定义map
	cityMap := make(map[string]string)

	// 添加
	cityMap["china"] = "zhengzhou"
	cityMap["usa"] = "riben"

	// 遍历
	for key, value := range cityMap {
		fmt.Println(key, ":", value)
	}

	// 删除
	delete(cityMap, "usa")

	fmt.Println("--------------")

	// 修改
	cityMap["china"] = "shangqiu"

	// 遍历
	for key, value := range cityMap {
		fmt.Println(key, ":", value)
	}
}

```

引用传递

```go
package main

import "fmt"

// 这里是引用传递，传递是指针，也就是传递过来的是cityMap的内存地址
func printMap(cityMap map[string]string) {
	cityMap["usa"] = "dog"
	// 遍历
	for key, value := range cityMap {
		fmt.Println(key, ":", value)
	}
}

func main() {
	// 定义map
	cityMap := make(map[string]string)

	// 添加
	cityMap["china"] = "zhengzhou"
	cityMap["usa"] = "riben"

	// 遍历
	printMap(cityMap)

	fmt.Println("--------------")

	// 修改
	cityMap["china"] = "shangqiu"

	// 遍历
	for key, value := range cityMap {
		fmt.Println(key, ":", value)
	}
}

```

## 查询map种是否存在某个元素

```go
package main

import "fmt"

func main() {
	// 使用字面量创建 Map
	m := map[string]string{
		"apple": "apple",
		"banana": "banana",
		"orange": "orange",
	}
	value, ok := m["ganto"]
	if (ok) {
		fmt.Println("存在键为：", value, "的值。")
	} else {
		fmt.Println("不存在该键")
	}
  // 不存在该键
}

```

# struct 结构体

基本定义与使用

```go
package main

import "fmt"

// 声明一种新的数据类型myint，是int的一个别名
type myint int

func main() {
	var a myint = 10
	fmt.Println("a =", a) // a = 10
	fmt.Printf("type of a = %T\n", a) // type of a = main.myint
}

```

默认值传递，可以通过指针修改为引用传递

```go
package main

import "fmt"

// 定义一个结构体
type Book struct {
	title string
	auth string
}

func changeBook(book Book) {
	// 传递一个book的副本
	book.auth = "李四"
}

func changeBook1(book *Book) {
	// 传递一个book的副本
	book.auth = "李四"
}


func main() {
	var book1 Book
	book1.title = "Golang"
	book1.auth = "张三"

	fmt.Printf("%v\n", book1) // {Golang 张三}

	changeBook(book1)

	fmt.Printf("%v\n", book1) // {Golang 张三}
	
	changeBook1(&book1)
	
	fmt.Printf("%v\n", book1) // {Golang 李四}
}

```

```go
package main

import "fmt"

type Books struct {
   title string
   author string
   subject string
   book_id int
}


func main() {

    // 创建一个新的结构体
    fmt.Println(Books{"Go 语言", "www.runoob.com", "Go 语言教程", 6495407})

    // 也可以使用 key => value 格式
    fmt.Println(Books{title: "Go 语言", author: "www.runoob.com", subject: "Go 语言教程", book_id: 6495407})

    // 忽略的字段为 0 或 空
   fmt.Println(Books{title: "Go 语言", author: "www.runoob.com"})
}

/*
  {Go 语言 www.runoob.com Go 语言教程 6495407}
  {Go 语言 www.runoob.com Go 语言教程 6495407}
  {Go 语言 www.runoob.com  0}
*/

```

# 面向对象

## 类的表示

go 语言中，类的表示是通过结构体进行的

```go
type Hero struct {
	Name string
	Ad int
	Level int
}

```

## 封装

```go
package main

import "fmt"

// 定义一个结构体
type Hero struct {
	Name string
	Ad int
	Level int
}

func (this Hero) Show() {
	fmt.Println("hero = ", this)
	fmt.Println("Name = ", this.Name)
	fmt.Println("Ad = ", this.Ad)
	fmt.Println("Level = ", this.Level)
}

func (this Hero) GetName() string {
	return this.Name
}

func (this Hero) SetName(Name string) {
	this.Name = Name
}

func main() {
	// 创建一个对象
	hero := Hero{Name: "张三", Ad: 100, Level: 1}

	hero.Show()
  // hero =  {张三 100 1}
	// Name =  张三
	// Ad =  100
	// Level =  1

	hero.SetName("李四")

	hero.Show()
  // hero =  {张三 100 1}
	// Name =  张三
 	// Ad =  100
	// Level =  1
}

```

以上写法，虽然没有语法错误，但是有功能上的问题，就是调用 `SetName()` 方法的时候，无法修改值，因为 `(this Hero)` 是值传递，非引用传递。

修改为指针引用传递

```go
package main

import "fmt"

// 定义一个结构体
type Hero struct {
	Name string
	Ad int
	Level int
}

func (this *Hero) Show() {
	fmt.Println("hero = ", this)
	fmt.Println("Name = ", this.Name)
	fmt.Println("Ad = ", this.Ad)
	fmt.Println("Level = ", this.Level)
}

func (this *Hero) GetName() string {
	return this.Name
}

func (this *Hero) SetName(Name string) {
	this.Name = Name
}

func main() {
	// 创建一个对象
	hero := Hero{Name: "张三", Ad: 100, Level: 1}

	hero.Show()
  // hero =  &{张三 100 1}
	// Name =  张三
	// Ad =  100
	// Level =  1

	hero.SetName("李四")

	hero.Show()
  // hero =  &{李四 100 1}
	// Name =  李四
	// Ad =  100
	// Level =  1

}

```

通过 `(this *Hero)` 修改成为引用传递后，再次调用 `SetName()` 方法，则可以修改值。

golang中的 变量名、类名、属性名 的首字母大小写，不仅代表着不同的变量，还表示变量是否公开。如果首字母 `大写` 则表示是一个 `公开` 的变量；如果首字母 `小写` 则表示一个 `私有` 的变量。

```go
type Hero struct {
	name string // 私有属性
	Ad int // 公开属性
	Level int // 公开属性
}

```

```go
func (this *Hero) GetName() string { // 公开方法
	return this.Name
}

func (this *Hero) getName() string { // 私有方法
	return this.Name
}

```

```go
type Hero struct { // 公开类
	...
}

type hero struct { // 私有类
	...
}

```

类名、属性名、方法名 首字母大写表示对外（其它包）可以访问，否则只能够在本包内访问。

## 继承

go中的类的继承是在结构体中写入需要继承的类名即可

```go
// 父类
type Human struct {
  ...
}

type SuperMan struct {
	// SuperMan类继承了Human类的方法
	Human

	level int
}

```

实例化对象的时候，需要嵌套父类的实例化

```go
s := SuperMan{Human{"李4", "男"}, 18}

```

另有一种实例化的写法，这样就不用嵌套父类的实例化，更加的直观

```go
var s SuperMan
s.name = "李4"
s.sex = "男"
s.level = 18

```

以下是完整例子

```go
package main

import "fmt"

type Human struct {
	name string
	sex string
}

func (this *Human) Eat() {
	fmt.Println("Human.Eat()...", this)
}

func (this *Human) Walk() {
	fmt.Println("Human.Walk()...")
}

type SuperMan struct {
	// SuperMan类继承了Human类的方法
	Human

	level int
}

// 重定义父类的 Eat() 方法
func (this *SuperMan) Eat() {
	fmt.Println("SuperMan.Eat()...", this)
}

// SuperMan类中的新方法
func (this *SuperMan) Fly() {
	fmt.Println("SuperMan.Fly()...")
}

func main() {
	h := Human{name: "张三", sex: "女"}
	// h := Human{"张三", "女"} // 简写，需要按照顺序

	h.Eat()
	h.Walk()

	s := SuperMan{Human{"李4", "男"}, 18}

	s.Eat()
  s.Walk() // 因为Walk()方法，没有重写，所以调用的是父类的Walk()方法
	s.Fly()
}
/*
Human.Eat()... &{张三 女}
Human.Walk()...
SuperMan.Eat()... &{{李4 男} 18}
Human.Walk()...
SuperMan.Fly()...
*/

```

## 多态

### 接口的定义

```go
// 本质是一个指针
type AnimalIF interface {
	Sleep()
	GetColor() string // 获取动物的颜色
	GetType() string // 获取动物的种类
}

```

### 多态

一个类实现一个接口，必须将接口中的方法都实现

Golang 中，类实现了接口中的所有方法，就实现了这个接口。

```go
package main

import "fmt"

// 本质是一个指针
type AnimalIF interface {
	Sleep()
	GetColor() string // 获取动物的颜色
	GetType() string // 获取动物的种类
}

// 具体的类
type Cat struct {
	color string // 猫的颜色
}
// 实现接口中的方法
func (this *Cat) Sleep() {
	fmt.Println("Cat.Sleep()...")
}
func (this *Cat) GetColor() string {
	fmt.Println("Cat.GetColor()...")
	return this.color
}
func (this *Cat) GetType() string {
	fmt.Println("Cat.GetType()...")
	return "Cat"
}

// 具体的类
type Dog struct {
	color string
}
// 实现接口中的方法
func (this *Dog) Sleep() {
	fmt.Println("Dog.Sleep()...")
}
func (this *Dog) GetColor() string {
	fmt.Println("Dog.GetColor()...")
	return this.color
}
func (this *Dog) GetType() string {
	fmt.Println("Dog.GetType()...")
	return "Dog"
}

func showAnimal(animal AnimalIF) {
	animal.Sleep() // 多态
	fmt.Println("color =", animal.GetColor())
	fmt.Println("kind =", animal.GetType())
}

func main() {
	var animal AnimalIF // 父类型引用指向子类型对象
	animal = &Dog{"红色"} // 多态
	animal.Sleep()

	animal = &Cat{"黑色"} // 多态
	animal.Sleep()

	cat := Cat{"绿色"}
	dog := Dog{"白色"}
	showAnimal(&cat)
	showAnimal(&dog)
}

```

以上就通过接口，实现了go语言中的多态。父类型引用指向子类型对象。

这些都和Java中的多态机制的概念和作用基本保持一致，只是实现方式有了区别。

### interface 空接口通用万能类型

空接口：`interface{}`

int、string、float32、float64、struct... 都实现了 `interface{}`，根据多态机制，`interface{}` 类型 引用 任意的数据类型。（这就类似于Java中的 Object 对象）

```go
package main

import "fmt"

// interface{} 是万能数据类型
func myFunc(arg interface{}) {
	fmt.Println("myFunc is called...")
	fmt.Println(arg)

	// interface{} 提供 “类型断言” 机制
	value, ok := arg.(string)
	if !ok {
		fmt.Println("arg is not string type")
		fmt.Println("============\n")
	} else {
		fmt.Println("arg is string type, value =", value)
		fmt.Printf("arg type is %T\n", arg)
		fmt.Printf("value type is %T\n", value)
		fmt.Println("============\n")
	}
}

type Book struct {
	auth string
}

func main() {
	book := Book{"Golang"}
	myFunc(book)
	myFunc(100)
	myFunc(3.14)
	myFunc("gantoho")
}

```

### interface 类型断言机制

```go
package main

import "fmt"

func Func(x interface{}) {
	value, ok := x.(string)
	fmt.Println(value)
	fmt.Println(ok)
}

func main() {
	Func("123")
}

```

以上代码只有当 ok 为 true 的时候，value 才能有值。

## 反射

### 变量结构

变量是由 type 和 value 组成，type 和 value 统称为 pair，type又有 static type（静态类型：int、string...） 和 concrete type（具体类型）。

```go
package main

import "fmt"

func main() {
	var a string
	// pair<statictype: string, value: "abc">
	a = "abc"

	// pair<type: string, value: "abc">
	var allType interface{}
	allType = a
	str, _ := allType.(string)
	fmt.Printf("%s %T", str, str) // abc string
}

```

例子：

```go
package main

import (
	"fmt"
)

type Reader interface {
	ReadBook()
}

type Writer interface {
	WriteBook()
}

type Book struct {
}

func (this Book) ReadBook() {
	fmt.Println("Read a Book")
}

func (this Book) WriteBook() {
	fmt.Println("Write a Book")
}

func main() {
	b := &Book{}

	var r Reader
	r = b
	r.ReadBook()

	var w Writer
	w = r.(Writer)
	w.WriteBook()
}

```

### reflect包

*   `func ValueOf(i interface{}) Value {...}` ValueOf用来获取输入参数接口中的数据的值，如果接口为空则返回0
*   `func TypeOf(i interface{}) Type {...}` TypeOf 用来动态获取输入参数接口中的值的类型，如果接口为空则返回nil

```go
package main

import (
	"fmt"
	"reflect"
)

func reflectNum(arg interface{}) {
	fmt.Println("Type: ", reflect.TypeOf(arg))
	fmt.Println("Value: ", reflect.ValueOf(arg))
}

func main() {
	var a float32 = 100.1
	reflectNum(a)
	// Type:  float32
	// Value:  100.1
}


```

如果传入指针

```go
package main

import (
	"fmt"
	"reflect"
)

func reflectNum(arg interface{}) {
	fmt.Println("Type: ", reflect.TypeOf(arg))
	fmt.Println("Value: ", reflect.ValueOf(arg))
}

func main() {
	var a float32 = 100.1
	reflectNum(&a)
	// Type:  *float32
	// Value:  0xc00001c098
}

```

实例：

```go

package main

import (
    "fmt"
    "reflect"
)

func main() {
    var num float64 = 1.2345

    pointer := reflect.ValueOf(&num)
    value := reflect.ValueOf(num)

    // 可以理解为“强制转换”，但是需要注意的时候，转换的时候，如果转换的类型不完全符合，则直接panic
    // Golang 对类型要求非常严格，类型一定要完全符合
    // 如下两个，一个是*float64，一个是float64，如果弄混，则会panic
    convertPointer := pointer.Interface().(*float64)
    convertValue := value.Interface().(float64)

    fmt.Println(convertPointer)
    fmt.Println(convertValue)
}

运行结果：
0xc42000e238
1.2345

```

`reflect.TypeOf()` 有两个方法，分别是：`.Name()`获取类型名称（可以是自定义类型名称） 和 `.Kind()`获取真实类型名称。

```go
package main

import (
	"fmt"
	"reflect"
)

type User struct {
	Id int
}

func main() {
	user := User{1}
	LookType(user)
}

func LookType(input interface{}) {
	inputType := reflect.TypeOf(input)
	fmt.Println(inputType.Name(), inputType.Kind()) // User struct
}

```

完整案例

```go
package main

import (
	"fmt"
	"reflect"
)

type User struct {
	Id int
	Name string
	Age int
}

func (this User) Call() int {
	fmt.Println(this)
	return 0
}

func main() {
	user := User{1, "ToHo", 20}
	LookType(user)
}

func LookType(input interface{}) {
	// 获取input的type
	inputType := reflect.TypeOf(input)
	fmt.Println(inputType, inputType.Name(), inputType.Kind()) // main.User User struct

	// 获取input的value
	inputValue := reflect.ValueOf(input)
	inputValueAll := reflect.ValueOf(input).Interface()
	fmt.Println(inputValue, inputValueAll) // {1 ToHo 20} {1 ToHo 20}

	// 通过type获取里面的字段
	// 1.获取interface的reflect.Type，通过Type得到NumField，进行遍历
	// 2.得到每个field，数据类型
	// 3.通过field的一个Interface()方法得到对应的value
	fmt.Println(inputType.NumField()) // 3 类中的字段总数
	for i := 0; i < inputType.NumField(); i++ {
		field := inputType.Field(i)
		value := inputValue.Field(i).Interface()
		value1 := inputValue.Field(i) // 加不加Interface()好像效果一样。还不知区别。
		fmt.Println(field, field.Name, field.Type, value, value1)
		// 类中的字段详情     字段名称    字段类型     字段值      字段值
		// {Id  int  0 [0] false} Id int 1 1
		// {Name  string  8 [1] false} Name string ToHo ToHo
		// {Age  int  24 [2] false} Age int 20 20
	}

	// 通过type获取里面的方法, 调用
	fmt.Println(inputType.NumMethod()) // 1 类中的方法总数
	for i := 0; i < inputType.NumMethod(); i++ {
		m := inputType.Method(i)
		fmt.Println(m.Name, m.Type)
		// 类中的方法名称     类中的方法类型
		// Call func(main.User) int
	}
}

```

数组和切片，亦有区别

```go
package main

import (
	"fmt"
	"reflect"
)

func main() {
	a := []int{1,2,3,4,5}
	aa := reflect.TypeOf(a)
	b := [5]int{1,2,3,4,5}
	bb := reflect.TypeOf(b)
	fmt.Println(aa, aa.Kind(), bb, bb.Kind()) // []int slice [5]int array
	fmt.Printf("%T %T", a, b) // []int [5]int
}

```

可以看出通过Printf打印出来的变量类型是有局限性的，最重要的是，Printf只能用来打印，如果想要在语句中用到变量类型，还是得用reflect包进行反射。

### 解析结构体标签Tag

```go
package main

import (
	"fmt"
	"reflect"
)

type resume struct {
	Name string `json:"name" doc:"我的名字" age:"age" val:"30"`
}

func findDoc(stru interface{}) (map[string]string, map[string]string) {
	t := reflect.TypeOf(stru).Elem()
	doc := make(map[string]string)
	age := make(map[string]string)
	for i := 0; i < t.NumField(); i++ {
		doc[t.Field(i).Tag.Get("json")] = t.Field(i).Tag.Get("doc")
		age[t.Field(i).Tag.Get("age")] = t.Field(i).Tag.Get("val")
	}

	return doc, age

}

func main() {
	var stru resume
	doc, age := findDoc(&stru)
	fmt.Printf("name字段为：%s，age字段为：%s\n", doc["name"], age["age"])
  // name字段为：我的名字，age字段为：30
}

```

结构体标签关键函数 `.ELem()` 和 `.Tag.Get("")`。

`.Elem()`函数，是通过 `TypeOf()` 函数获取结构体标签内容。

然后通过for循环，将数据循环出来，通过 `Tag.Get("")` 获取某个值，再组合成 `map`。

```go
package main

import (
	"fmt"
	"reflect"
)

type resume struct {
	Name string `json:"name" doc:"我的名字" age:"age" val:"30"`
	Sex string `json:"sex" doc:"男"`
}

func findTag(stru interface{}) (map[string]string) {
	t := reflect.TypeOf(stru).Elem()
	doc := make(map[string]string)
	fmt.Println(t.NumField()) // 2 字段数量
	for i := 0; i < t.NumField(); i++ {
		doc[t.Field(i).Tag.Get("json")] = t.Field(i).Tag.Get("doc")
		doc[t.Field(i).Tag.Get("age")] = t.Field(i).Tag.Get("val")
	}

	return doc

}

func main() {
	var stru resume
	doc := findTag(&stru)
	fmt.Printf("name字段为：%s，age字段为：%s\n", doc["name"], doc["age"])
	fmt.Printf("sex字段为：%s\n", doc["sex"])
	// name字段为：我的名字，age字段为：30
	// sex字段为：男
}

```

### 结构体标签在json中的应用

```go
package main

import (
	"fmt"
	"encoding/json"
)

type Movie struct {
	Title string `json:"title"`
	Year int `json:"year"`
	Price int `json:"rmb"`
	Actors []string `json:"actors"`
}

func main() {
	movie := Movie{"喜剧之王", 2000, 10, []string{"星爷", "张柏芝"}}

	// 编码的过程 结构体 转换成 json
	jsonStr, err := json.Marshal(movie)
	if err != nil {
		fmt.Println("json marshal error", err)
		return
	}
	fmt.Printf("jsonStr = %s\n", jsonStr)
	// jsonStr = {"title":"喜剧之王","year":2000,"rmb":10,"actors":["星爷","张柏芝"]}

	// 解码的过程 json 转换成 结构体
	myMovie := Movie{}
	err = json.Unmarshal(jsonStr, &myMovie)
	if err != nil {
		fmt.Println("json unmarshal error", err)
		return
	}
	fmt.Printf("%v\n", myMovie)
	// {喜剧之王 2000 10 [星爷 张柏芝]}
}

```

如果不写结构体标签，则会将成员变量名当作json数据的key

```go
package main

import (
	"fmt"
	"encoding/json"
)

type Movie struct {
	Title string
	Year int
	Price int
	Actors []string
}

func main() {
	movie := Movie{"喜剧之王", 2000, 10, []string{"星爷", "张柏芝"}}

	// 编码的过程 结构体 转换成 json
	jsonStr, err := json.Marshal(movie)
	if err != nil {
		fmt.Println("json marshal error", err)
		return
	}
	fmt.Printf("jsonStr = %s\n", jsonStr)
	// jsonStr = {"Title":"喜剧之王","Year":2000,"Price":10,"Actors":["星爷","张柏芝"]}
}

```

# Range 范围

Go 语言中 range 关键字用于 for 循环中迭代数组(array)、切片(slice)、通道(channel)或集合(map)的元素。在数组和切片中它返回元素的索引和索引对应的值，在集合中返回 key-value 对。

for 循环的 range 格式可以对 slice、map、数组、字符串等进行迭代循环。格式如下：

```go
arr := []string{"aaa", "bbb", "ccc"}
for key, value := range arr {
  fmt.Println("key：", key, "，value：", value, "。")
}

```

```go
for key, value := range oldMap {
    newMap[key] = value
}

```

以上代码中的 key 和 value 是可以省略。

如果只想读取 key，格式如下：

```go
for key := range oldMap
// 或者
for key, _ := range oldMap

```

只想读取value，格式如下：

```go
for _, value := range oldMap

```

因为value是第二个参数，所以第一个参数不能省略

例子：

```go
package main

import "fmt"

var pow = []int{1, 2, 4, 8, 16, 32, 64, 128}

func main() {
  for i, v := range pow {
    fmt.Printf("2^%d : %d\n", i, v)
  }
}

```

for 循环的 range 格式可以省略 key 和 value，如下实例：

```go
for key, value := range map1 {
  fmt.Printf("key is: %d - value is: %f\n", key, value)
}

// 读取 key
for key := range map1 {
  fmt.Printf("key is: %d\n", key)
}

// 读取 key
for key, _ := range map1 {
  fmt.Printf("key is: %d\n", key)
}

// 读取 value
for _, value := range map1 {
  fmt.Printf("value is: %f\n", value)
}


```

range 遍历其他数据结构：

```go
package main
import "fmt"
func main() {
    //这是我们使用 range 去求一个 slice 的和。使用数组跟这个很类似
    nums := []int{2, 3, 4}
    sum := 0
    for _, num := range nums {
        sum += num
    }
    fmt.Println("sum:", sum)
    //在数组上使用 range 将传入索引和值两个变量。上面那个例子我们不需要使用该元素的序号，所以我们使用空白符"_"省略了。有时侯我们确实需要知道它的索引。
    for i, num := range nums {
        if num == 3 {
            fmt.Println("index:", i)
        }
    }
    //range 也可以用在 map 的键值对上。
    kvs := map[string]string{"a": "apple", "b": "banana"}
    for k, v := range kvs {
        fmt.Printf("%s -> %s\n", k, v)
    }

    //range也可以用来枚举 Unicode 字符串。第一个参数是字符的索引，第二个是字符（Unicode的值）本身。
    for i, c := range "go" {
        fmt.Println(i, c)
    }
}

/*
sum: 9
index: 1
a -> apple
b -> banana
0 103
1 111
*/

```

## range 与 通道

```go
package main

import (
	"fmt"
)

func main() {
	ch := make(chan int)

	go func() {
		for i := 0; i < 4; i++ {
			ch <- i
		}
		close(ch)
	}()

	// for {
	// 	if data, ok := <-ch; ok {
	// 		fmt.Println(data)
	// 	} else {
	// 		break
	// 	}
	// }

	// 和上方 for if 一样的效果
	for data := range ch {
		fmt.Println(data)
	}

	fmt.Println("Main Finished..")
}

```

# 递归函数

递归操作就是自己调用自己，但是一定要设置退出条件，否则将会一直执行，导致死循环。

## 阶乘（例子）

```go
package main

import "fmt"

func Factorial(n uint64)(result uint64) {
	if (n > 0) {
		result := n * Factorial(n - 1)
		return result
	}
	return 1
}

func main() {
	i := 3
	fmt.Println(Factorial(uint64(i)))
}

```

## 斐波那契数列

```go
package main

import "fmt"

func fibonacci(n int) int {
  if n < 2 {
   return n
  }
  return fibonacci(n-2) + fibonacci(n-1)
}

func main() {
    var i int
    for i = 0; i < 10; i++ {
       fmt.Printf("%d\t", fibonacci(i))
    }
}

```

# 类型转换

## 数值类型转换

```go
var a int = 10
var b float64 = float64(a)

```

将 `int` 类型转换成 `uint64` 类型

```go
package main

import "fmt"

func main() {
	i := 100
	fmt.Printf("%T %d\n", i, i)
	ii := uint64(i)
	fmt.Printf("%T %d", ii, ii)
}

```

将整型 `int` 转化为浮点型 `float32`，并计算结果，将结果赋值给浮点型变量：

```go
package main

import "fmt"

func main() {
   var sum int = 17
   var count int = 5
   var mean float32
   
   mean = float32(sum)/float32(count)
   fmt.Printf("mean 的值为: %f\n",mean)
}

```

## 字符串类型转换

### 字符串转换为整型

`strconv.Atoi` 函数可以将整数形式的string字符串，转换成int类型。

`strconv.Atoi` 函数返回两个参数，第一个参数为转换后的数据，第二个参数为可能发生错误的错误信息，我们可以使用空白标识符 **\_** 来忽略这个错误。

```go
package main

import (
	"fmt"
	"strconv"
)

func main() {
	str := "123"
	num, err := strconv.Atoi(str)
	if err != nil {
		fmt.Println("转换错误:", err)
	} else {
		fmt.Printf("字符串 '%s' 转换为整数为：%d %T\n", str, num, num)
    // 字符串 '123' 转换为整数为：123 int
	}
}

```

### 整型转换为字符串

`strconv.Itoa` 函数可以将int类型的整型数字，转换成string类型的字符串。

`strconv.Itoa` 函数返回一个参数，参数为转换后的数据。

```go
package main

import (
	"fmt"
	"strconv"
)

func main() {
	i := 123
	num := strconv.Itoa(i)
	fmt.Printf("i: %d %T, num: %s %T。", i, i, num, num)
}

```

### 字符串转换为浮点型

`strconv.ParseFloat` 函数返回两个参数，第一个参数为转换后的数据，第二个参数为可能发生错误的错误信息，我们可以使用空白标识符 **\_** 来忽略这个错误。

```go
package main

import (
    "fmt"
    "strconv"
)

func main() {
    str := "3.14"
    num, err := strconv.ParseFloat(str, 64)
    if err != nil {
        fmt.Println("转换错误:", err)
    } else {
        fmt.Printf("字符串 '%s' 转为浮点型为：%f\n", str, num)
    }
}

```

### 浮点型转换为字符串

```go
package main

import (
    "fmt"
    "strconv"
)

func main() {
    num := 3.14
    str := strconv.FormatFloat(num, 'f', 2, 64)
    fmt.Printf("浮点数 %f 转为字符串为：'%s'\n", num, str)
}

```

## 接口类型转换

接口类型转换有两种情况：`类型断言`和`类型转换`。

```go
value.(type) 
或者 
value.(T)

```

其中 value 是接口类型的变量，type 或 T 是要转换成的类型。

如果类型断言成功，它将返回转换后的值和一个布尔值，表示转换是否成功。

```go
package main

import "fmt"

func main() {
    var i interface{} = "Hello, World"
    str, ok := i.(string)
    if ok {
        fmt.Printf("'%s' is a string\n", str)
    } else {
        fmt.Println("conversion failed")
    }
}

```

# 接口

实例

```go
package main

import (
	"fmt"
)

// 定义一个 DivideError 结构
type DivideError struct {
	dividee int
	divider int
}

// 实现 `error` 接口
func (de *DivideError) Error() string {
	strFormat := `
    Cannot proceed, the divider is zero.
    dividee: %d
    divider: 0
`
	return fmt.Sprintf(strFormat, de.dividee)
}

// 定义 `int` 类型除法运算的函数
func Divide(varDividee int, varDivider int) (result int, errorMsg string) {
	if varDivider == 0 {
		dData := DivideError{
			dividee: varDividee,
			divider: varDivider,
		}
		errorMsg = dData.Error()
		return
	} else {
		return varDividee / varDivider, ""
	}

}

func main() {

	// 正常情况
	if result, errorMsg := Divide(100, 10); errorMsg == "" {
		fmt.Println("100/10 = ", result)
	}
	// 当除数为零的时候会返回错误信息
	if _, errorMsg := Divide(100, 0); errorMsg != "" {
		fmt.Println("errorMsg is: ", errorMsg)
	}

}

```

输出

```go
100/10 =  10

errorMsg is:                            
    Cannot proceed, the divider is zero.
    dividee: 100
    divider: 0

```

golang中的接口，可以用于实现多态

# 错误处理

例子

```go
package main

import (
    "fmt"
)

// 定义一个 DivideError 结构
type DivideError struct {
    dividee int
    divider int
}

// 实现 `error` 接口
func (de *DivideError) Error() string {
    strFormat := `
    Cannot proceed, the divider is zero.
    dividee: %d
    divider: 0
`
    return fmt.Sprintf(strFormat, de.dividee)
}

// 定义 `int` 类型除法运算的函数
func Divide(varDividee int, varDivider int) (result int, errorMsg string) {
    if varDivider == 0 {
            dData := DivideError{
                    dividee: varDividee,
                    divider: varDivider,
            }
            errorMsg = dData.Error()
            return
    } else {
            return varDividee / varDivider, ""
    }

}

func main() {

    // 正常情况
    if result, errorMsg := Divide(100, 10); errorMsg == "" {
            fmt.Println("100/10 = ", result)
    }
    // 当除数为零的时候会返回错误信息
    if _, errorMsg := Divide(100, 0); errorMsg != "" {
            fmt.Println("errorMsg is: ", errorMsg)
    }

}

```

# \*高阶

# 并发（go）

## 基本使用

golang天然支持高并发，我们只需要通过go关键字来开启 `goroutine` 即可。

```go
package main

import (
    "fmt"
    "time"
)

func say(s string) {
    for i := 0; i < 5; i++ {
        time.Sleep(100 * time.Millisecond)
        fmt.Println(s)
    }
}

func main() {
    go say("world")
    say("hello")
}

```

运行对比以下两个例子即可看出区别，例子中只有使用go关键字和没有使用go关键字的区别：

使用go关键字

```go
package main
 
import (
    "fmt"
    "time"
)
 
func newTask() {
    i := 0
    for {
        i++
        fmt.Printf("new goroutine: i = %d\n", i)
        time.Sleep(1*time.Second) //延时1s
    }
}
 
func main() {
    //创建一个 goroutine，启动另外一个任务
    go newTask()
    i := 0
    //main goroutine 循环打印
    for {
        i++
        fmt.Printf("main goroutine: i = %d\n", i)
        time.Sleep(1 * time.Second) //延时1s
    }
}

```

没有使用go关键字

```go
package main

import (
    "fmt"
    "time"
)
// 子goroutine
func newTask() {
    i := 0
    for {
        i++
        fmt.Printf("new goroutine: i = %d\n", i)
        time.Sleep(1*time.Second) //延时1s
    }
}
// 主goroutine
func main() {
    //创建一个 goroutine，启动另外一个任务
    newTask()
    i := 0
    //main goroutine 循环打印
    for {
        i++
        fmt.Printf("main goroutine: i = %d\n", i)
        time.Sleep(1 * time.Second) //延时1s
    }
}

```

只有主goroutine一直存在，子goroutine才能存在，否则子goroutine直接结束

```go
package main

import (
    "fmt"
    "time"
)
// 子goroutine
func newTask() {
    i := 0
    for {
        i++
        fmt.Printf("new goroutine: i = %d\n", i)
        time.Sleep(1*time.Second) //延时1s
    }
}
// 主goroutine
func main() {
    //创建一个 goroutine，启动另外一个任务
    go newTask()

	fmt.Println("main goroutine exit")

    // i := 0
    // //main goroutine 循环打印
    // for {
    //     i++
    //     fmt.Printf("main goroutine: i = %d\n", i)
    //     time.Sleep(1 * time.Second) //延时1s
    // }
}

```

可以创建匿名goroutine

```go
package main

import (
  "fmt"
	"runtime"
	"time"
)

func main() {
	// 用go创建承载一个形参为空，返回值为空的一个函数
    go func() {
		defer fmt.Println("A.defer")

		func() {
			defer fmt.Println("B.defer")
			// 退出当前goroutine
			runtime.Goexit()
			fmt.Println("B")
		}()

		fmt.Println("A")
	}()

	// 死循环
	for {
		time.Sleep(1 * time.Second)
	}
}

```

其中`func() {}()`是立即执行函数。

`runtime.Goexit()` 可以将goroutine退出

也可以声明带参数的匿名函数

```go
package main

import (
    "fmt"
    "time"
)

func main() {
    go func(x int, y int) bool {
        fmt.Println("x", x, "y", y)
        return true
    }(10, 20)

    // 死循环
    for {
      	time.Sleep(1 * time.Second)
    }
}

```

在以上代码中，匿名函数返回了布尔类型的返回值，但是无法通过普通的语法进行获取，需要使用通道来进行协程之间的正常通信，因为协程是异步的，无法同步赋值。

## time包

在以上的案例中可以看到导入的time包

`time.Sleep` 是延迟函数，内部传入的参数 `time.Second` 反射出来时 `int64` 类型，值是 `1s`，time.Sleep 函数中好像必须传入 time.Second 。

```go
time.Sleep(1 * time.Second)

```

`time.Now()` 是获取当前时间的函数，在此函数下，还包含：`.Year()`，`.Month()`，`.Day()`，`.Hour()`，`.Minute()`，`.Second()` 等函数，他们是分别获取 年、月、日、时、分、秒 的函数。

```go
now := time.Now()
fmt.Println(now)
fmt.Println(now.Year())
fmt.Println(now.Month())
fmt.Println(now.Day())
fmt.Println(now.Hour())
fmt.Println(now.Minute())
fmt.Println(now.Second())

```

# 通道（channel）

## 基本

通道（channel）是用来传递数据的一个数据结构。

通道可用于两个 `goroutine` 之间通过传递一个指定类型的值来同步运行和通讯。操作符 `<-` 用于指定通道的方向，发送或接收。如果未指定方向，则为双向通道。

```go
ch <- v    // 把 v 发送到通道 ch
v := <-ch  // 从 ch 接收数据
           // 并把值赋给 v

```

声明一个通道很简单，我们使用chan关键字即可，通道在使用前必须先创建：

```go
ch := make(chan int)

```

## 无通道缓冲区

**注意**：默认情况下，通道是不带缓冲区的。发送端发送数据，同时必须有接收端相应的接收数据。

以下实例通过两个 goroutine 来计算数字之和，在 goroutine 完成计算后，它会计算两个结果的和：

```go
package main

import "fmt"

func sum(s []int, c chan int) {
        sum := 0
        for _, v := range s {
                sum += v
        }
        c <- sum // 把 sum 发送到通道 c
}

func main() {
        s := []int{7, 2, 8, -9, 4, 0}

        c := make(chan int)
      	b := make(chan int)
        go sum(s[:len(s)/2], c)  // 数组下标 开始 - 2
        go sum(s[len(s)/2:], b) // 数组下标 3 - 末尾
        x, y := <-c, <-b // 从通道 c 中接收

        fmt.Println(x, y, x+y) // 17 -5 12
}

```

## 通道缓冲区

通道可以设置缓冲区，通过 make 的第二个参数指定缓冲区大小：

```go
ch := make(chan int, 100)

```

通过 `len()` 和 `cap()` 获取通道的 `长度` 和 `容量`

```go
package main

import "fmt"

func main() {
	ch := make(chan int, 5)
	ch <- 199
	fmt.Println(len(ch), cap(ch))
}

```

带缓冲区的通道允许发送端的数据发送和接收端的数据获取处于异步状态，就是说发送端发送的数据可以放在缓冲区里面，可以等待接收端去获取数据，而不是立刻需要接收端去获取数据。

不过由于缓冲区的大小是有限的，所以还是必须有接收端来接收数据的，否则缓冲区一满，数据发送端就无法再发送数据了。

**注意**：如果通道不带缓冲，发送方会阻塞直到接收方从通道中接收了值。如果通道带缓冲，发送方则会阻塞直到发送的值被拷贝到缓冲区内；如果缓冲区已满，则意味着需要等待直到某个接收方获取到一个值。接收方在有值可以接收之前会一直阻塞。

***意思就是：有缓冲区***

如下代码，设置缓存区为3的通道，将三个数存放到通道中，正好存满，延时两秒进行取通道中的数据。

```go
package main

import (
	"fmt"
	"time"
)

func main() {
	ch := make(chan int, 3)
	fmt.Println("len(ch) = ", len(ch), ", cap(ch) = ", cap(ch))

	go func() {
		defer fmt.Println("子go程结束")

		for i := 0; i < 3; i++ {
			ch <- i
			fmt.Println("子go程正在运行，发送的元素 = ", i, " len(ch) = ", len(ch), " cap(ch) = ", cap(ch))
		}
	}()

	time.Sleep(2 * time.Second)

	for i := 0; i < 3; i++ {
		num := <-ch // 从通道ch中国呢取数据，并赋值给num
		fmt.Println("num = ", num)
	}

	fmt.Println("main结束")
}

```

打印结果

```go
len(ch) =  0 , cap(ch) =  3
子go程正在运行，发送的元素 =  0  len(ch) =  1  cap(ch) =  3
子go程正在运行，发送的元素 =  1  len(ch) =  2  cap(ch) =  3
子go程正在运行，发送的元素 =  2  len(ch) =  3  cap(ch) =  3
子go程结束
num =  0
num =  1
num =  2
main结束

```

如果存进去的数据大于通道ch的容量，则会发生阻塞，当通道的容量被腾出来，才能再存。

```go
package main

import (
	"fmt"
	"time"
)

func main() {
	ch := make(chan int, 3)
	fmt.Println("len(ch) = ", len(ch), ", cap(ch) = ", cap(ch))

	go func() {
		defer fmt.Println("子go程结束")

		for i := 0; i < 4; i++ {
			ch <- i
			fmt.Println("子go程正在运行，发送的元素 = ", i, " len(ch) = ", len(ch), " cap(ch) = ", cap(ch))
		}
	}()

	time.Sleep(2 * time.Second)

	for i := 0; i < 4; i++ {
		num := <-ch // 从通道ch中国呢取数据，并赋值给num
		fmt.Println("num = ", num)
	}

	fmt.Println("main结束")
	time.Sleep(2 * time.Second)
}

```

打印结果：

```go
len(ch) =  0 , cap(ch) =  3
子go程正在运行，发送的元素 =  0  len(ch) =  1  cap(ch) =  3
子go程正在运行，发送的元素 =  1  len(ch) =  2  cap(ch) =  3
子go程正在运行，发送的元素 =  2  len(ch) =  3  cap(ch) =  3
num =  0
num =  1
num =  2
num =  3
main结束
子go程正在运行，发送的元素 =  3  len(ch) =  0  cap(ch) =  3
子go程结束

```

## 关闭通道

关闭通道是通过 `close(ch)` 关键字进行关闭的

```go
package main

import (
	"fmt"
)

func main() {
	ch := make(chan int)

	go func() {
		for i := 0; i < 4; i++ {
			ch <- i
		}
		// close关闭一个通道channel
		close(ch)
	}()

	// 死循环
	for {
		// if语句缩写，表示ok代表if的判断条件。如果ok为true，则表示通道还未关闭；如果ok为false，则表示通道已经关闭。
		if data, ok := <-ch; ok {
			fmt.Println(data)
		} else {
			break
		}
	}

	fmt.Println("Main Finished..")
}

```

```css
0
1
2
3
Main Finished..

```

如果以上代码没有关闭通道，则会出现死锁，然后报错停止运行

```go
package main

import (
	"fmt"
)

func main() {
	ch := make(chan int)

	go func() {
		for i := 0; i < 4; i++ {
			ch <- i
		}
		// close关闭一个通道channel
		// close(ch)
	}()

	// 死循环
	for {
		// if语句缩写，表示ok代表if的判断条件。如果ok为true，则表示通道还未关闭；如果ok为false，则表示通道已经关闭。
		if data, ok := <-ch; ok {
			fmt.Println(data)
		} else {
			break
		}
	}

	fmt.Println("Main Finished..")
}

```

```less
0
1
2
3
fatal error: all goroutines are asleep - deadlock!

goroutine 1 [chan receive]:
main.main()
        /Volumes/MacData/golang/main.go:21 +0xbf
exit status 2

```

如果向一个关闭的通道发送数据，也会报错

```go
package main

import (
	"fmt"
)

func main() {
	ch := make(chan int)

	go func() {
		for i := 0; i < 4; i++ {
			ch <- i
			// close关闭一个通道channel
			close(ch) // 发送第一个参数后，直接关闭通道
		}
	}()

	// 死循环
	for {
		// if语句缩写，表示ok代表if的判断条件。如果ok为true，则表示通道还未关闭；如果ok为false，则表示通道已经关闭。
		if data, ok := <-ch; ok {
			fmt.Println(data)
		} else {
			break
		}
	}

	fmt.Println("Main Finished..")
}

```

```less
0
Main Finished..
panic: send on closed channel

goroutine 18 [running]:
main.main.func1()
        /Volumes/MacData/golang/main.go:12 +0x32
created by main.main
        /Volumes/MacData/golang/main.go:10 +0x6a
exit status 2

```

## 通道 与 range

\[range 与 通道\](#range 与 通道)

## 通道 与 select

单流程下，一个go只能监控一个通道的状态，select可以完成监控多个channel的状态

```go
package main

import (
	"fmt"
)

func fibonacii(ch, quit chan int) {
	x, y := 1, 1
	
	for {
		select {
		case ch <- x:
			x, y = y, x+y
		case <-quit:
			fmt.Println("quit")
			return
		}
	}
}

func main() {
	ch := make(chan int)
	quit := make(chan int)

	go func() {
		for i := 0; i < 10; i++ {
			fmt.Println(<-ch)
		}
		
		quit <- 1
	}()

	fibonacii(ch, quit)
}

```

```undefined
1
1
2
3
5
8
13
21
34
55
quit

```

以上代码，main函数中，定义了两个通道，一个用于存放数字，一个用于结束程序。

当main中的 for循环中 想要读取`<-ch`通道中的数据，但如果没有读到就会发生阻塞，等待fibonacii函数中，将数据存放到通道`ch`中即可读到，这个过程执行10次后，跳出main函数中的for循环，然后将一个数存到`quit`通道内，这个数字可以任意，因为fibonacii中`case <-quit:`只是检测到`quit`通道有数据就会`return`。`case ch <- x:` 的意思是检测ch通道是否可以存放数据，如果可以存放就将`x`存放到`ch`通道内，并进入case内，如果不能则阻塞。

select具备多路channel的监控状态功能

# 骚的语法

```go
package main

import "fmt"

func main() {
        s := []int{7, 2, 8, -9, 4, 0}

        fmt.Println(s[:len(s)/2], s[len(s)/2:]) // [7 2 8] [-9 4 0]
}

```

`s[:index]` `s[index:]`index为数组下标，冒号在前，表示截取0到中间位置，不包含下标处的值；冒号在后，表示截取中介位置到末尾，包含下标处的值。

# GOPATH工作模式的弊端

GOPATH的弊端：

*   无版本控制概念
*   无法同步一致第三方版本号
*   无法指定当前项目引用的第三方版本号

正是因为GOPATH有诸多弊端，才有了Go Modules来取代GOPATH

# \*进阶

# Go Modules

## go mod命令

执行一下命令查看go mod支持哪些命令

```sh
go mod help

```

| 命令 | 作用 |
| --- | --- |
| go mod init | 生成 go.mod 文件 |
| go mod download | 下载 go.mod 文件中指明的所有依赖 |
| go mod tidy | 整理现有的依赖 |
| go mod graph | 查看现有的依赖结构 |
| go mod edit | 编辑 go.mod 文件 |
| go mod vendor | 导出项目所有的依赖到vendor目录 |
| go mod verify | 校验一个模块是否被篡改过 |
| go mod why | 查看为什么需要依赖某模块 |

查看go mod环境变量

```sh
go env

```

可以通过命令，将GO111MODULE改为on，默认auto，建议设置为`on`

```sh
go env -w GO111MODULE=on

```

## GOPROXY

这个环境变量主要是用于设置Go模块代理（Go module proxy），其作用是用于使Go在后续拉取模块版本时直接通过镜像站点来快速拉取。

GOPROXY的默认地址为： [https://proxy.golang.org](https://proxy.golang.org) ,direct

建议改成国内镜像源

*   阿里云：[https://mirrors.aliyun.com/goproxy](https://mirrors.aliyun.com/goproxy)
*   七牛云： [https://goproxy.cn](https://goproxy.cn) ,direct

```bash
go env -w GOPROXY= https://goproxy.cn ,direct

```

可以设置多个镜像源，以逗号`,`分割开即可

```sh
go env -w GOPROXY= https://goproxy.cn,https://mirrors.aliyun.com/goproxy ,direct

```

查看是否设置成功

```sh
go env
# 或
go env | grep GOPROXY

```

## GOSUMDB

用来校验拉取的第三方库是否是完整的

GOSUMDB的默认值为：sum.golang.org，在国内也是无法访问的，但是GOSUMDB可以被Go模块代理所代理（详见：Proxying a Checksum Database）。

如果设置了GOPROXY，这个就不用设置了

通过一下命令查看 GOSUMDB

```sh
go env | grep GOSUMDB

```

可以通过以下命令设置GOSUMDB

```sh
go env -w GOSUMDB=off

```

一般不建议设置关闭，因为关闭之后无法对拉取的代码进行校验。

## GoModules初始化项目

### 开启Go Modules

```sh
go env -w GO111MODULE=on

```

查看

```sh
go env | grep GO111MODULE

```

也可以通过直接设置系统环境变量（写入对应的～/.bash\_profile文件即可）来实现这个目的：

```sh
vim ~/.bashrc

```

填入以下内容，保存退出后

```sh
export GO111MODULE=on

```

接着执行

```sh
source ~/.bashrc

```

### 初始化项目

创建项目文件夹

```sh
mkdir -p ganto/modules_test
# 进入到项目文件夹内
cd ganto/modules_test

```

执行初始化modules命令

```sh
go mod init github.com/gantoho/modules_test

```

创建main.go文件

```go
package main

import (
	"fmt"
	"github.com/aceld/zinx/ziface"
	"github.com/aceld/zinx/znet"
)

// ping test 自定义路由
type PingRouter struct {
	znet.BaseRouter
}

// Ping Handle
func (this *PingRouter) Handle(request ziface.IRequest) {
	// 先读取客户端的数据
	fmt.Println("recv from client : msgId=", request.GetMsgID(), ", data=", string(request.GetData()))

	// 再回写ping...ping...ping
	err := request.GetConnection().SendBuffMsg(0, []byte("ping...ping...ping"))
	if err != nil {
		fmt.Println(err)
	}
}

func main() {
	// 1 创建一个server句柄
	s := znet.NewServer()

	// 2 配置路由
	s.AddRouter(0, &PingRouter{})

	// 3 开启服务
	s.Serve()
}

```

运行前，先进行下载导入的包

```sh
go get github.com/aceld/zinx/ziface
go get github.com/aceld/zinx/znet

```

## 改变模块依赖关系

```sh
go mod edit -replace=zinx@v0.0.2=zinx@v0.0.1

```

## go mod的基本使用

先初始化mod

```sh
go mod init example.com/m/v2

```

然后安装gin

```sh
go get -u github.com/gin-gonic/gin

```

在go程序中，要先引用gin才能使用

```go
import (
	"github.com/gin-gonic/gin"
)

```

简单例子：

```go
package main

import (
	"fmt"
	"github.com/gin-gonic/gin"
	"net/http"
	"reflect"
)

func main() {
	r := gin.Default()
	r.GET("/api", func(c *gin.Context) {
		msg := c.Query("msg")
		fmt.Println(len(msg))
		if len(msg) == 0 {
			msg = "默认值"
		}
		fmt.Println("接收的参数msg：", msg, reflect.TypeOf(msg))
		data := map[string]interface{}{
			"code": "ok",
			"data": map[string]string{"name": "张三", "age": msg},
		}
		c.AsciiJSON(http.StatusOK, data)
	})
	r.Run(":8989")
}

```

例子调试：[http://localhost:8989/api?msg=iamziyou](http://localhost:8989/api?msg=iamziyou)

## 有趣的例子

```go
package main

import (
	"fmt"
	"github.com/gin-gonic/gin"
	"net/http"
)

type Num struct {
	value string
}

func main() {
	num := Num{}
	r := gin.Default()
	r.GET("/setNum", func(c *gin.Context) {
		passWord := c.Query("password")
		data := map[string]interface{}{}
		if passWord == "ganto" {
			num.value = c.Query("num")
			fmt.Printf("num.value的类型：%T", num.value)
			data = map[string]interface{}{
				"code": "设置成功",
			}
		} else {
			if passWord == "" {
				data = map[string]interface{}{
					"code": "请输入密码",
				}
			} else {
				data = map[string]interface{}{
					"code": "密码错误",
				}
			}
		}
		c.JSON(http.StatusOK, data)
	})
	r.GET("/getNum", func(c *gin.Context) {
		num1 := c.Query("num")
		data := map[string]interface{}{}
		if num1 == num.value {
			data = map[string]interface{}{
				"code": "正确",
			}
		} else {
			data = map[string]interface{}{
				"code": "失败",
			}
		}
		c.JSON(http.StatusOK, data)
	})
	r.Run(":8989")
}

```

> 通过setNum接口，携带num和password参数，对数据进行设置；
> 
> 通过getNum接口，携带num参数，对数据进行验证。

# 使用Go Modules初始化项目

[GoModules初始化项目](#GoModules%E5%88%9D%E5%A7%8B%E5%8C%96%E9%A1%B9%E7%9B%AE)

# 连接数据库MySQL

创建数据库test

```sql
SET NAMES utf8mb4;
SET FOREIGN_KEY_CHECKS = 0;

-- ----------------------------
-- Table structure for user
-- ----------------------------
DROP TABLE IF EXISTS `user`;
CREATE TABLE `user`  (
  `id` varchar(20) CHARACTER SET utf8mb4 COLLATE utf8mb4_bin NOT NULL,
  `name` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_bin NULL DEFAULT NULL,
  `age` tinyint(0) NULL DEFAULT NULL,
  `address` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_bin NULL DEFAULT NULL,
  PRIMARY KEY (`id`) USING BTREE
) ENGINE = InnoDB CHARACTER SET = utf8mb4 COLLATE = utf8mb4_bin ROW_FORMAT = Dynamic;

-- ----------------------------
-- Records of user
-- ----------------------------
INSERT INTO `user` VALUES ('12132', '张三', 35, '北京市');
INSERT INTO `user` VALUES ('16162', '王五', 22, '上海市');

SET FOREIGN_KEY_CHECKS = 1;

```

安装依赖

```sh
github.com/go-sql-driver/mysql
github.com/jmoiron/sqlx

```

查询

main.go

```go
package main

import (
    "fmt"
	"github.com/jmoiron/sqlx"
	_ "github.com/go-sql-driver/mysql"
)

var db *sqlx.DB

type Person struct {
    UserId   string `db:"id"`
    Username string `db:"name"`
    Age      int    `db:"age"`
    Address  string `db:"address"`
}

func init() {
    conn, err := sqlx.Open("mysql", "root:admin@tcp(127.0.0.1)/test")
    if err != nil {
        fmt.Println("Open mysql failed", err)
        return
    }

    db = conn
}

func main() {
	query()
	list()
    defer db.Close()
}

func query() {
    var person Person
    //查询一个是Get，多个是Select
    err := db.Get(&person, "select * from user where id = ?", "12132")
    if err != nil {
        fmt.Println("query failed:", err)
        return
    }
    fmt.Printf("query succ:%+v", person)
}

func list() {
	var perons []Person
	err := db.Select(&perons, "select * from user")
	if err != nil {
		fmt.Println("list err", err)
		return
	}
	fmt.Printf("list succ,%+v", perons)
}

```

新增

```go
func insert() {
   result, err := db.Exec("insert into user value (?,?,?,?)", "120230", "李四", 12, "广州市")
   if err != nil {
      fmt.Println("insert err:", err)
      return
   }
   id, err := result.LastInsertId()
   if err != nil {
      fmt.Println("insert err:", err)
      return
   }
   fmt.Println("insert succ:", id)
}

```

更新

```go
func update() {
   res, err := db.Exec("update user set name = ? where id = ?", "赵六", "120230")
   if err != nil {
      fmt.Println("update err:", err)
      return
   }
   eff, err := res.RowsAffected()
   if err != nil || eff == 0 {
      fmt.Println("update err:", err)
      return
   }
   fmt.Println("Update succ")
}

```

删除

```go
func delete() {
   res, err := db.Exec("delete from user where id = ?", "120230")
   if err != nil {
      fmt.Println("delete err:", err)
      return
   }
   eff, err := res.RowsAffected()
   if err != nil || eff == 0 {
      fmt.Println("delete err:", err)
      return
   }
   fmt.Println("delete succ")
}

```

# 即时通信系统/项目

## 即时通信系统/v0.1基础server构建

server.go

```go
package main

import (
	"fmt"
	"net"
)

type Server struct {
	Ip   string
	Port int
}

// 创建一个server的接口
func NewServer(ip string, port int) *Server {
	server := &Server{
		Ip:   ip,
		Port: port,
	}
	return server
}

func (this *Server) Handler(conn net.Conn) {
	// ...当前链接的业务
	fmt.Println("链接建立成功", conn)
}

// 启动服务器的接口
func (this *Server) Start() {
	// socket listen
	listener, err := net.Listen("tcp", fmt.Sprintf("%s:%d", this.Ip, this.Port))
	if err != nil {
		fmt.Println("net.Listen err:", err)
		return
	}

	// close listen socket
	defer listener.Close()

	for {
		// accept
		conn, err := listener.Accept()
		if err != nil {
			fmt.Println("listener accept err:", err)
			continue
		}
		// do handler
		go this.Handler(conn)
	}
}

```

main.go

```go
package main

func main() {
	server := NewServer("127.0.0.1", 9999)
	server.Start()
}

```

编译命令

```sh
go build -o server main.go server.go

```

编译完成会多一个server文件

执行

```sh
./server

```

可以在浏览器访问`127.0.0.1:9999`，或者终端输入命令`nc 127.0.0.1:9999`，进行验证服务是否开启成功。

## 即时通信系统/v0.2用户上线及广播功能

server.go

```go
package main

import (
	"fmt"
	"net"
	"sync"
)

type Server struct {
	Ip   string
	Port int

	// 在线用户的列表
	OnlineMap map[string]*User
	mapLock   sync.RWMutex

	// 消息广播的channel
	Message chan string
}

// 创建一个server的接口
func NewServer(ip string, port int) *Server {
	server := &Server{
		Ip:        ip,
		Port:      port,
		OnlineMap: make(map[string]*User),
		Message:   make(chan string),
	}
	return server
}

// 监听Message广播消息channel的goroutine，一旦有消息就发送给全部的在线User
func (this *Server) ListenMessager() {
	for {
		msg := <-this.Message

		//	将msg发送给全部的在线User
		this.mapLock.Lock()
		for _, cli := range this.OnlineMap {
			cli.C <- msg
		}
		this.mapLock.Unlock()
	}
}

// 广播消息的方法
func (this *Server) BroadCast(user *User, msg string) {
	sendMsg := "[" + user.Addr + "]" + user.Name + ":" + msg
	this.Message <- sendMsg
}

func (this *Server) Handler(conn net.Conn) {
	// ...当前链接的业务
	//fmt.Println("链接建立成功", conn)
	user := NewUser(conn)

	// 用户上线了，将用户加入到onlineMap中
	this.mapLock.Lock()
	this.OnlineMap[user.Name] = user
	this.mapLock.Unlock()

	// 广播当前用户上线消息
	this.BroadCast(user, "已上线")

	//	当前handler阻塞
	select {}
}

// 启动服务器的接口
func (this *Server) Start() {
	// socket listen
	listener, err := net.Listen("tcp", fmt.Sprintf("%s:%d", this.Ip, this.Port))
	if err != nil {
		fmt.Println("net.Listen err:", err)
		return
	}

	// close listen socket
	defer listener.Close()

	//启动监听Message的goroutine
	go this.ListenMessager()

	for {
		// accept
		conn, err := listener.Accept()
		if err != nil {
			fmt.Println("listener accept err:", err)
			continue
		}
		// do handler
		go this.Handler(conn)
	}
}

```

user.go

```go
package main

import "net"

type User struct {
	Name string
	Addr string
	C    chan string
	conn net.Conn
}

// 创建一个用户的API
func NewUser(conn net.Conn) *User {
	userAddr := conn.RemoteAddr().String()
	user := &User{
		Name: userAddr,
		Addr: userAddr,
		C:    make(chan string),
		conn: conn,
	}

	// 启动监听当前User channel消息的goroutine
	go user.ListenMessage()

	return user
}

// 监听当前User channel的方法，一旦有消息，就直接发送给对端客户端
func (this *User) ListenMessage() {
	for {
		msg := <-this.C
		this.conn.Write([]byte(msg + "\n"))
	}
}

```

main.go

```go
package main

func main() {
	server := NewServer("127.0.0.1", 9999)
	server.Start()
}

```

编译、执行

```sh
go build -o server main.go server.go user.go

./server

```