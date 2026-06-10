---
title: "TypeScript学习笔记"
date: 2023-02-20 13:03:00
author: 干徒
link: https://www.cnblogs.com/ganto/articles/17136967.html
---
# TypeScript

## 准备

安装node.js

## 安装

```shell
npm install -g typescript

```

## 查看版本

```shell
tsc -v

```

在编写ts代码时，有时虽然没有写错，但是依然有报错信息 如：`无法重新声明块范围变量`，只需要执行`tsc --init`生成tsconfig.json文件即可解决

# 第一个TS程序

***app.ts***

```typescript
var message:string = "Hello TypeScript!"
console.log(message)

```

# TypeScript指令

使用tsc命令，将app.ts文件编译成为app.js文件

![](https://www.runoob.com/wp-content/uploads/2019/01/typescript_compiler.png)

```shell
tsc app.ts

```

tsc指令可以同时编译多个ts文件

```shell
tsc app.ts app1.ts app2.ts

```

\--declaration会再生成一个`.d.ts`文件

```shell
tsc app.ts --declaration

```

利用node命令，执行app.js文件

```shell
node app.js
Hello TypeScript!

```

自动热部署命令

首先生成tsconfig.json文件

```sh
tsc --init

```

tsconfig.json

```json
{
  "compilerOptions": {
    "module": "ES2015",
    "target": "ES2015",
    "strict": true, // 严格模式
    "outDir": "./dist", // 打包的位置
    "noEmitOnError": true // 有错误语法将不进行编译
  },
  "include": [
    "./src/**/*" // 编译哪里的ts文件
  ],
  "exclude": [
    "node_modules" // 忽略的文件
  ]
}

```

然后执行命令，就会自动热部署，不用每次修改代码都要手动部署

```sh
tsc -w

```

# tsc 常用编译参数如下表所示：

| 序号 | 编译参数说明 |
| --- | --- |
| 1. | **\--help**显示帮助信息 |
| 2. | **\--module**载入扩展模块 |
| 3. | **\--target**设置 ECMA 版本 |
| 4. | **\--declaration**额外生成一个 .d.ts 扩展名的文件。`tsc ts-hw.ts --declaration`以上命令会生成 ts-hw.d.ts、ts-hw.js 两个文件。 |
| 5. | **\--removeComments**删除文件的注释 |
| 6. | **\--out**编译多个文件并合并到一个输出的文件 |
| 7. | **\--sourcemap**生成一个 sourcemap (.map) 文件。sourcemap 是一个存储源代码与编译代码对应位置映射的信息文件。 |
| 8. | **\--module noImplicitAny**在表达式和声明上有隐含的 any 类型时报错 |
| 9. | **\--watch**在监视模式下运行编译器。会监视输出文件，在它们改变时重新编译。 |

# TypeScript 保留关键字

TypeScript 保留关键字如下表所示：

| break | as | catch | switch |
| --- | --- | --- | --- |
| case | if | throw | else |
| var | number | string | get |
| module | type | instanceof | typeof |
| public | private | enum | export |
| finally | for | while | void |
| null | super | this | new |
| in | return | true | false |
| any | extends | static | let |
| package | implements | interface | function |
| do | try | yield | const |
| continue |  |  |  |

## 空白和换行

TypeScript 会忽略程序中出现的空格、制表符和换行符。

空格、制表符通常用来缩进代码，使代码易于阅读和理解。

## TypeScript 区分大小写

TypeScript 区分大写和小写字符。

## 分号是可选的

每行指令都是一段语句，你可以使用分号或不使用， 分号在 TypeScript 中是可选的，建议使用。

以下代码都是合法的：

```typescript
console.log("Runoob")
console.log("Google");

```

如果语句写在同一行则一定需要使用分号来分隔，否则会报错，如：

```typescript
console.log("Runoob");console.log("Google");

```

## TypeScript 注释

和JavaScript一样

```typescript
// 单行注释

/* 
多行注释
多行注释
多行注释
*/

```

# TypeScript面向对象

TypeScript 面向对象编程实例：

```typescript
class Site {
  name():void {
    console.log("Runoob")
  }
}
var obj = new Site();
obj.name();

```

以上实例定义了一个类 Site，该类有一个方法 name()，该方法在终端上输出字符串 Runoob。

new 关键字创建类的对象，该对象调用方法 name()。

编译后生成的 JavaScript 代码如下：

```typescript
var Site = /** @class */ (function () {
  function Site() {
  }
  Site.prototype.name = function () {
    console.log("Runoob");
  };
  return Site;
}());
var obj = new Site();
obj.name();

```

执行以上 JavaScript 代码，输出结果如下:

```undefined
Runoob

```

## 定义类

```typescript
// 定义一个Person类
class Person {
  // 实例属性，必须实例化访问
  myName:string = "ganto"
  // 静态属性，可以直接访问
  static myAge:number = 19
  // 定义只读的属性
  readonly mySex:string = "男"
  // 定义静态只读的属性
  static readonly mySchool:string = "B站"
  // 实例方法
  sayMe():void{
    console.log(this.myName, Person.myAge)
  }
  // 静态方法
  static lookMe():void{
    console.log(this.myAge)
  }
}

const obj = new Person() // 实例化对象
console.log(obj.myName) // 访问实例属性
obj.sayMe(); // 访问实例方法

console.log(Person.myAge) // 访问静态属性
Person.lookMe() // 访问静态方法

```

## 构造方法

```typescript
class Person {
  myName:string 
  myAge:number
  constructor(myName:string, myAge:number) {
    console.log(this)
    this.myName = myName
    this.myAge = myAge
  }
  bark():void{
    console.log(this.myName, "，",this.myAge,"岁了，汪汪汪！")
  }
}

const person = new Person("旺财", 2)
person.bark()
const person1 = new Person("小明", 5)
person1.bark()

```

## 封装

封装的好处就是，类中的属性定义成私有的，通过特有的getter、setter方法进行暴露和操作，可以在setter方法中控制属性

```typescript
class Person {
  private name:string
  private age:number
  constructor(name:string, age:number) {
    this.name = name
    this.age = age
  }
  getName() {
    return this.name
  }
  setName(name:string) {
    if(name.length == 2 || name.length == 3) {
      this.name = name
    }else{
      console.log("name不合法")
    }
  }
  getAge() {
    return this.age
  }
  setAge(age:number) {
    if(age > 0 && age < 150) {
      this.age = age
    }else{
      console.log("age不合法")
    }
  }
}

const person = new Person("猪八戒",19)
console.log(person)

// person.name = "孙悟空" // 属性“name”为私有属性，只能在类“Person”中访问。
// person.age = -18 // 属性“age”为私有属性，只能在类“Person”中访问。

person.setName("孙悟空1")
person.setAge(-18)

console.log(person.getName(), person.getAge())

```

## 继承

```typescript
class Person {
  myName:string 
  myAge:number
  constructor(myName:string, myAge:number) {
    this.myName = myName
    this.myAge = myAge
  }
  say():void{
    console.log(this.myName, "",this.myAge,)
  }
}

class XiaoHua extends Person {

}

class XiaoMing extends Person {
  mySex:string
  constructor(myName:string,myAge:number,mySex:string) {
    super(myName,myAge)
    this.mySex = mySex
  }
  say(): void {
    console.log(this.myName, "", this.myAge, "", this.mySex)
  }
}

const xiaohua = new XiaoHua("小花", 18)
xiaohua.say()

const xiaoming = new XiaoMing("小明", 20, "女")
xiaoming.say()

```

super(myName, myAge)意思就是将值传递给父类，super.say()就是调用父类中的say方法

## 抽象类

抽象类只能被子类继承，不可以实例化

```typescript
// 抽象类
abstract class Animal {
  name:string 
  constructor(name:string){
    this.name = name
  }
  sayHello():void {
    console.log("动物在叫")
  }
}

class Dog extends Animal{
  sayHello(): void {
    console.log("汪汪汪汪")
  }
}
const dog = new Dog("旺财")
dog.sayHello()

// const she = new Animal("蛇") // 抽象类只能被继承，不可以实例化

```

抽象类中可以有普通方法，也可以有抽象方法，抽象方法必须被子类实现

```typescript
// 抽象类
abstract class Animal {
  name:string 
  constructor(name:string){
    this.name = name
  }
  sayHello():void {
    console.log("动物在叫")
  }
  // 抽象方法
  abstract look():void 
}

class Dog extends Animal{
  sayHello(): void {
    console.log("汪汪汪汪")
  }
  look(): void {
    console.log("狗子在看")
  }
}
const dog = new Dog("旺财")
dog.sayHello()
dog.look()

// const she = new Animal("蛇") // 抽象类只能被继承，不可以实例化

```

## 接口

接口可以当成类型声明去使用，类型声明不可重复，但接口可以重复，重复的接口会进行合并

```typescript
// 描述一个对象的类型
type myType = {
  name: string,
  age: number,
  [propname:string]:any
}
const obj: myType = {
  name: 'sss',
  age: 11
}

// 接口和上方 定义一个对象的类型 很像，接口用来 定义一个类的结构（用来包含哪些属性和方法）
interface myInterface {
  name: string,
  age: number
}
interface myInterface {
  gender: string
}
const obj1: myInterface = {
  name: 'sss',
  age: 11,
  gender: "男",
}

```

接口中定义的属性，不能有实际值

```typescript
interface myInterface {
  name: string,
  age: number
}
interface myInterface {
  gender: string
}
const obj1: myInterface = {
  name: 'sss',
  age: 11,
  gender: "男",
}

```

接口中只能有抽象方法，不能有普通方法，在接口中定义抽象方法一般省略`abstract`关键字，没有方法体

```typescript
interface myInterface {
  sayHello():void
}
const obj1: myInterface = {
  sayHello() {
    
  }
}

```

### 类实现接口

```typescript
interface myInterface {
  name: string,
  sayHello():void
}

class MyClass implements myInterface {
  name:string
  constructor(name:string) {
    this.name = name
  }
  sayHello():void{
    console.log("sayHello")
  }
}

```

### 接口可以继承接口

```typescript
interface baseInter {
  sex:string
}

interface myInterface extends baseInter {
  name: string,
  sayHello():void
}

class MyClass implements myInterface {
  name:string
  sex:string
  constructor(name:string, sex:string) {
    this.name = name
    this.sex = sex
  }
  sayHello():void{
    console.log("sayHello")
  }
}

```

# TypeScript 基础类型

## 基础类型

| 数据类型 | 关键字 | 描述 |
| --- | --- | --- |
| 任意类型 | any | 声明为any的变量可以赋予任意类型的值。 |
| 字符串 | string | 一个字符系列，使用单引号（**'**）或双引号（**"**）来表示字符串类型。反引号（**\`**）来定义多行文本和内嵌表达式。 |
| 数字 | number | 双精度 64 位浮点值。它可以用来表示整数和分数。 |
| 布尔 | boolean | 表示逻辑值：true 和 false。 |
| 数组 |  | 声明变量为数组。 |
| 元组Tuple |  | 元组类型用来表示已知元素数量和类型的数组，各元素的类型不必相同，对应位置的类型需要相同。 |
| 枚举 | enum | 枚举类型用于定义数值集合。 |
| void | void | 用于标识方法返回值的类型，表示该方法没有返回值。 |
| null | null | 表示对象值缺失。 |
| undefined | undefined | 用于初始化变量为一个未定义的值 |
| never | never | never 是其它类型（包括 null 和 undefined）的子类型，代表从不会出现的值。 |

**注意：**TypeScript 和 JavaScript 没有整数类型。

* * *

注意：在js中 如果声明变量同时赋值，变量的类型会直接被设置为所赋值的类型。如下代码是报错的。

```typescript
/* this is a typescript code */
let a = 1 // ts中声明变量直接赋值会直接锁定变量类型
a = '2' // 这里会报错

```

## Any

任意值是 TypeScript 针对编程时类型不明确的变量使用的一种数据类型，它常用于以下三种情况。

1、变量的值会动态改变时，比如来自用户的输入，任意值类型可以让这些变量跳过编译阶段的类型检查，示例代码如下：

```typescript
let x: any = 1;    // 数字类型
x = 'I am who I am';    // 字符串类型
x = false;    // 布尔类型

```

改写现有代码时，任意值允许在编译时可选择地包含或移除类型检查，示例代码如下：

```typescript
let x: any = 4;
x.ifItExists();    // 正确，ifItExists方法在运行时可能存在，但这里并不会检查
x.toFixed();    // 正确

```

定义存储各种类型数据的数组时，示例代码如下：

```typescript
let arrayList: any[] = [1, false, 'fine'];
arrayList[1] = 100;
console.log(arrayList)

```

任意类型的数组，有点像元组，但是元组声明了固定类型后就无法修改类型，任意类型的数组要比元组更加随意。

## unknown

下方的代码都不会报错，那么这么看unknown和any一样？但其实有区别

```typescript
let e: unknown
e = 10
e = 'hello'
e = true

```

unknown和any的区别：

```typescript
let a:any
a = true
let b: string
b = 'hello'
b = a // 这时b的类型将会被设置为boolean

```

```typescript
let a: unknown
a = 'string'
let b: string 
b = 'hello'
b = a // 这里将会报错，因为a的类型为unknown，无法赋值给其他类型

```

*   any类型可以给任意类型赋值，赋值过后的变量也会被改为any所属的类型 （真就any霍霍所有类型）
*   unknown类型不可以给其他类型赋值
*   unknown和any之间可以相互赋值
*   其他类型可以给unknown和any赋值

## 字符串

```typescript
let myName: string = 'xiaoming'

```

## 数字

```typescript
let age: number = 12

let binaryLiteral: number = 0b1010; // 二进制
let octalLiteral: number = 0o744;    // 八进制
let decLiteral: number = 6;    // 十进制
let hexLiteral: number = 0xf00d;    // 十六进制

```

### 字面量赋值

赋值过后无法修改，类似于常量

```typescript
let a: 10
a = 10
a = 11 // 这里的代码会报错，字面量a为10常量，a将无法修改为11

```

### 联合类型

```typescript
let b: "male" | "female"
b = "male"
b = "female"
b = "hello" // 这里的代码会报错，因为b只能为male或者female

let c: boolean | string
c = true
c = 'hello'
c = 1 // 这里的代码会报错，因为c的类型只能为boolean或者string

```

## 布尔

```typescript
let flag: boolean = true
let unflag: boolean = false

```

## 数组

```typescript
// 在元素类型后面加上[]
let arr: number[] = [1, 2];

// 或者使用数组泛型
let arr: Array<number> = [1, 2];

```

## 元组

固定长度，单独固定每项类型的数组

```typescript
let x: [string, number];
x = ['Runoob', 1];    // 运行正常
x = [1, 'Runoob'];    // 报错
console.log(x[0]);    // 输出 Runoob

```

## 枚举

```typescript
enum Color {Red, Green, Blue};
let c: Color = Color.Blue;
console.log(c);    // 输出 2

```

```typescript
enum Gender{
  Male,
  Female
}
let i: {name: string, gender: Gender}

i = {
  name: '孙悟空',
  gender: Gender.Male
}

console.log(i.gender === 1) // true
console.log(i.gender === Gender.Male) // true

```

## void

void 用来表示空，以函数为例，就表示没有返回值的函数

```typescript
function hello(): void {
  alert("Hello Runoob");
}

```

## never 类型

never 是其它类型（包括 null 和 undefined）的子类型，代表从不会出现的值。这意味着声明为 never 类型的变量只能被 never 类型所赋值，在函数中它通常表现为抛出异常或无法执行到终止点（例如无限循环），示例代码如下：

```typescript
let x: never;
let y: number;

// 编译错误，数字类型不能转为 never 类型
x = 123;

// 运行正确，never 类型可以赋值给 never类型
x = (()=>{ throw new Error('exception')})();

// 运行正确，never 类型可以赋值给 数字类型
y = (()=>{ throw new Error('exception')})();

// 返回值为 never 的函数可以是抛出异常的情况
function error(message: string): never {
  throw new Error(message);
}

// 返回值为 never 的函数可以是无法被执行到的终止点的情况
function loop(): never {
  while (true) {}
}

```

## 类型断言

类型断言有两种形式。 其一是“尖括号”语法：

```typescript
let someValue: any = "this is a string";
let strLength: number = (<string>someValue).length;

```

另一个为`as`语法：

```typescript
let someValue: any = "this is a string";
let strLength: number = (someValue as string).length;

```

两种形式是等价的。 至于使用哪个大多数情况下是凭个人喜好；然而，当你在TypeScript里使用JSX时，只有`as`语法断言是被允许的。

例子：

```typescript
let a:unknown
a = 'hello'
let b: string
b = 'b'
b = a as string // 这里使用类型断言后，不再报错
console.log(b, typeof b);

```

如下代码，使用类型断言可以欺骗编译器，可以将b的类型修改成为number

```typescript
let a:unknown
a = 1
let b: string
b = 'b'
b = a as string
console.log(b, typeof b); // 1 number

```

## 对象Object

```typescript
let a: object;
a = {}
a = function() {}

let b: {name: string};
b = {name: '喜喜'}

```

因为在js中很多东西都是对象，直接声明`let a: object`，会有一些不好。可以用第二种方式声明，明确那种object

如果一个对象必须转递一个name属性，其他属性可以随意增删，属性类型也任意，可以如下例子：

```typescript
let a: {name: string, [propName: string]: any}
a = {name: '哈哈', age: 10, sex: '女'}

```

其中`[propName: string]: any`中的前边部分`[propName: string]`表示属性名为字符串，`any`表示属性的值为任意值。

```typescript
let d: (a: number, b: number) => number;
d = function(n1, n2): number{
	return n1+n2
}

```

## 与& 或|

```typescript
let j: {name: string} & {age: number}

let x: string | number

let k: 1 | 2 | 3 | 4 | 5

```

## 类型别名

```typescript
type myType = string;
let a: myType;

```

```typescript
type myType1 = 1 | 2 | 3 | 4 | 5;
let b: myType1

```

```typescript
type Cls = {
  a: boolean,
  b: boolean
}
const cls: Cls = {
  a: true,
  b: true
}

```

# TypeScript条件语句

*   **if 语句** - 只有当指定条件为 true 时，使用该语句来执行代码
*   **if...else 语句** - 当条件为 true 时执行代码，当条件为 false 时执行其他代码
*   **if...else if....else 语句**\- 使用该语句来选择多个代码块之一来执行
*   **switch 语句** - 使用该语句来选择多个代码块之一来执行

## if

## if...else

## if...else if...else

## switch…case

# TypeScript循环

## for

## for...in

## for...of

## forEach

## every

## while

## do...while

# TypeScript函数

## 函数返回值

```typescript
function function_name():return_type { 
  // 语句
  return value; 
}

```

*   return\_type 是返回值的类型。
*   return 关键词后跟着要返回的结果。
*   一般情况下，一个函数只有一个 return 语句。
*   返回值的类型需要与函数定义的返回类型(return\_type)一致。

### 案例

```typescript
// 函数定义
function greet():string { // 返回一个字符串
  return "Hello World" 
} 
 
function caller() { 
  var msg = greet() // 调用 greet() 函数 
  console.log(msg) 
} 
 
// 调用函数
caller()

```

*   实例中定义了函数 *greet()*，返回值的类型为 string。
*   *greet()* 函数通过 return 语句返回给调用它的地方，即变量 msg，之后输出该返回值。

## 带参数函数

在调用函数时，您可以向其传递值，这些值被称为参数。

这些参数可以在函数中使用。

您可以向函数发送多个参数，每个参数使用逗号 **,** 分隔：

语法格式如下所示：

```typescript
function func_name( param1 [:datatype], param2 [:datatype]) {   
}

```

*   param1、param2 为参数名。
*   datatype 为参数类型。

### 实例

```typescript
function add(x: number, y: number): number {
  return x + y;
}
console.log(add(1,2))

```

*   实例中定义了函数 *add()*，返回值的类型为 number。
*   *add()* 函数中定义了两个 number 类型的参数，函数内将两个参数相加并返回。

## 可选参数和默认参数

### 可选参数

在 TypeScript 函数里，如果我们定义了参数，则我们必须传入这些参数，除非将这些参数设置为可选，可选参数使用问号标识 ？。

**实例**

```typescript
function buildName(firstName: string, lastName: string) {
  return firstName + " " + lastName;
}
 
let result1 = buildName("Bob");                  // 错误，缺少参数
let result2 = buildName("Bob", "Adams", "Sr.");  // 错误，参数太多了
let result3 = buildName("Bob", "Adams");         // 正确

```

以下实例，我们将 lastName 设置为可选参数：

```typescript
function buildName(firstName: string, lastName?: string) {
  if (lastName)
    return firstName + " " + lastName;
  else
    return firstName;
}
 
let result1 = buildName("Bob");  // 正确
let result2 = buildName("Bob", "Adams", "Sr.");  // 错误，参数太多了
let result3 = buildName("Bob", "Adams");  // 正确

```

可选参数必须跟在必需参数后面。 如果上例我们想让 firstName 是可选的，lastName 必选，那么就要调整它们的位置，把 firstName 放在后面。

如果都是可选参数就没关系。

### 默认参数

我们也可以设置参数的默认值，这样在调用函数的时候，如果不传入该参数的值，则使用默认参数，语法格式为：

```delphi
function function_name(param1[:type],param2[:type] = default_value) { 
}

```

注意：参数不能同时设置为可选和默认。

**实例**

以下实例函数的参数 rate 设置了默认值为 0.50，调用该函数时如果未传入参数则使用该默认值：

```typescript
function calculate_discount(price:number,rate:number = 0.50) { 
  var discount = price * rate; 
  console.log("计算结果: ",discount); 
} 
calculate_discount(1000) // 500
calculate_discount(1000,0.30) // 300

```

输出结果为：

```makefile
计算结果:  500
计算结果:  300

```

## 剩余参数

有一种情况，我们不知道要向函数传入多少个参数，这时候我们就可以使用剩余参数来定义。

剩余参数语法允许我们将一个不确定数量的参数作为一个数组传入。

```typescript
function buildName(firstName: string, ...restOfName: string[]) {
  return firstName + " " + restOfName.join(" ");
}
  
let employeeName = buildName("Joseph", "Samuel", "Lucas", "MacKinzie");

```

函数的最后一个命名参数 restOfName 以 ... 为前缀，它将成为一个由剩余参数组成的数组，索引值从0（包括）到 restOfName.length（不包括）。

```typescript
function addNumbers(...nums:number[]) {  
  var i;   
  var sum:number = 0; 

  for(i = 0;i<nums.length;i++) { 
    sum = sum + nums[i]; 
  } 
  console.log("和为：",sum) 
 } 
 addNumbers(1,2,3) 
 addNumbers(10,10,10,10,10)

```

输出结果为：

```undefined
和为： 6
和为： 50

```

# new Map()

```typescript
let nameSiteMapping = new Map();

// 设置 Map 对象
nameSiteMapping.set("Google", 1);
nameSiteMapping.set("Runoob", 2);
nameSiteMapping.set("Taobao", 3);

// 获取键对应的值
console.log(nameSiteMapping.get("Runoob"));     // 2

// 判断 Map 中是否包含键对应的值
console.log(nameSiteMapping.has("Taobao"));       // true
console.log(nameSiteMapping.has("Zhihu"));        // false

// 返回 Map 对象键/值对的数量
console.log(nameSiteMapping.size);                // 3

// 删除 Runoob
console.log(nameSiteMapping.delete("Runoob"));    // true
console.log(nameSiteMapping);
// 移除 Map 对象的所有键/值对
nameSiteMapping.clear();             // 清除 Map
console.log(nameSiteMapping);

/**
 * new Map() // 实例化对象
 * set() // 设置键值对
 * get() // 获取键对应的值
 * has() // 获取是否有键
 * delete() // 删除该键值对
 * clear() // 清空
 * size // 返回长度
 */

```

# 接口

例子

```typescript
function printLabel(labelledObj: { label: string }) {
  console.log(labelledObj.label);
}

let myObj = { size: 10, label: "Size 10 Object" };
printLabel(myObj);

```

利用接口重写

```typescript
interface LabelledValue {
  label: string;
}

function printLabel(labelledObj: LabelledValue) {
  console.log(labelledObj.label);
}

let myObj = {size: 10, label: "Size 10 Object"};
printLabel(myObj);

```

哈哈，暂时没get到接口这样写的意义

## 可选属性

```typescript
interface SquareConfig {
  color?: string;
  width?: number;
}

function createSquare(config: SquareConfig): {color: string; area: number} {
  let newSquare = {color: "white", area: 100};
  if (config.color) {
    newSquare.color = config.color;
  }
  if (config.width) {
    newSquare.area = config.width * config.width;
  }
  return newSquare;
}

let mySquare = createSquare({});
console.log(mySquare)

```

带有可选属性的接口与普通的接口定义差不多，只是在可选属性名字定义的后面加一个`?`符号。

接口里的属性添加了`?`符号后，在调用函数的时候，就可以不用传递该属性。如上方代码，是完全正确的。

## 只读属性

一些对象属性只能在对象刚刚创建的时候修改其值。 你可以在属性名前用 `readonly`来指定只读属性:

```typescript
interface Point {
  readonly x: number;
  readonly y: number;
}

```

你可以通过赋值一个对象字面量来构造一个`Point`。 赋值后， `x`和`y`再也不能被改变了。

```typescript
let p1: Point = { x: 10, y: 20 };
p1.x = 5; // error!

```

TypeScript具有`ReadonlyArray<T>`类型，它与`Array<T>`相似，只是把所有可变方法去掉了，因此可以确保数组创建后再也不能被修改：

```typescript
let a: number[] = [1, 2, 3, 4];
let ro: ReadonlyArray<number> = a;
ro[0] = 12; // error!
ro.push(5); // error!
ro.length = 100; // error!
a = ro; // error!

```

上面代码的最后一行，可以看到就算把整个`ReadonlyArray`赋值到一个普通数组也是不可以的。 但是你可以用类型断言重写：

```typescript
a = ro as number[];

```

## `readonly` vs `const`

最简单判断该用`readonly`还是`const`的方法是看要把它做为变量使用还是做为一个属性。 做为变量使用的话用`const`，若做为属性则使用`readonly`。

`const`一般用在变量声明：`const num = 1`

`readonly`一般用在属性中：`{ readonly num1: number }`

# JSX

非官网教程，写的还可以：[https://www.w3cschool.cn/typescript/typescript-jsx.html](https://www.w3cschool.cn/typescript/typescript-jsx.html)

# 三斜线指令

```typescript
/// <reference path="react.d.ts" />

```

# 编译选项tsconfig.json

通过以下命令监听app.ts的变化，一旦变化则会重新编译。-w 即 watch

```sh
tsc app.ts -w

```

通过tsc --init生成tsconfig.json文件，然后执行tsc命令会将项目中的ts文件全部编译成js文件，必须要生成tsconfig.json文件，只有tsconfig.json文件存在后，`tsc`和`tsc -w`命令才可以生效。

```sh
tsc --init

tsc # 编译全部ts文件

tsc -w # 监听全部ts文件的变化

```

## tsconfig.json

*   include: 用来指定哪些ts文件需要被编译
    
    *   \*\* 表示任意目录
    *   \* 表示任意文件
    
    ```json
    {
      "include": [
        "./src/**/*" /* src目录下的任意目录下的任意文件 */
      ]
    }
    
    ```
    
*   exclude: 用来指定不需要被编译的路径
    
    *   默认值：\["node\_modules", "bower\_components", "jspm\_packages"\]
    
    ```json
    {
      "exclude": [
        "./src/hello/**/*"
      ]
    }
    
    ```
    
*   extends: 定义被继承的配置文件
    
    ```json
    {
      "extends": "./configs/base"
    }
    
    ```
    
*   files: 指定被编译文件的列表，只有需要编译的文件少时才会用到
    
    ```json
    {
      "files": [
        "core.ts",
        "sys.ts",
        ...
      ]
    }
    
    ```
    
*   compilerOptions: 编译选项是配置文件中非常重要也比较复杂的配置选项
    
    *   target: 用来指定ts被编译成的ES版本
        
        ```json
        {
          "compilerOptions": {
            "target": "ES6"
          }
        }
        
        ```
        
    *   module: 指定要使用的模块化的规范
        
        ```json
        {
          "compilerOptions": {
            "module": "ES6"
          }
        }
        
        ```
        
    *   lib: 用来指定项目中要使用的库
        
        ```json
        {
          "compilerOptions": {
            "lib": [
              "dom"
            ]
          }
        }
        
        ```
        
        如上，dom为需要使用获取dom的包，如document.getElementById('box')
        
    *   outDir: 指定编译后文件所在的目录
        
        ```json
        {
          "compilerOptions": {
            "outDir": "./dist"
          }
        }
        
        ```
        
    *   outFile: 将多个ts编译成的js文件合并成一个文件
        
        ```json
        {
          "compilerOptions": {
        "outFile": "./dist/app.js"
          }
        }
        
        ```
        
        设置outFile后，所有的全局作用域中的代码会合并到同一个文件中
        
    *   allowJs: 是否对js文件进行编译，默认是false
        
        ```json
        {
          "compilerOptions": {
            "allowJs": true
          }
        }
        
        ```
        
    *   checkJs: 是否检查js代码是否符合语法规范，默认是false
        
        ```json
        {
          "compilerOptions": {
            "checkJs": true
          }
        }
        
        ```
        
    *   removeComments: 是否移除注释，默认是false
        
        ```json
        {
          "compilerOptions": {
            "removeComments": true
          }
        }
        
        ```
        
    *   noEmit: 不生成编译后的js文件，默认值false
        
        ```json
        {
          "compilerOptions": {
            "noEmit": true
          }
        }
        
        ```
        
    *   noEmitOnError: 当有错误时，不生成就编译后的js文件，默认值false
        
        ```json
        {
          "compilerOptions": {
            "noEmitOnError": true
          }
        }
        
        ```
        
    
    语法检查相关的配置
    
    *   alwaysStrict: 开始严格模式，默认值false
        
        ```json
        {
          "compilerOptions": {
            "alwaysStrict": true
          }
        }
        
        ```
        
    *   noImplicitAny: 不允许隐式any类型，默认值false
        
        ```json
        {
          "compilerOptions": {
            "noImplicitAny": true
          }
        }
        
        ```
        
    *   noImplicitThis: 不允许不明确的类型的this，默认值false
        
        ```json
        {
          "compilerOptions": {
            "noImplicitThis": true
          }
        }
        
        ```
        
    *   strictNullChecks: 严格的检查空值，默认值false
        
        ```json
        {
          "compilerOptions": {
            "strictNullChecks": true
          }
        }
        
        ```
        
    *   strict: 所有严格模式的总开关，默认值false
        
        ```json
        {
          "compilerOptions": {
            "strict": true
          }
        }
        
        ```
        
        如果没有单独设置其他的属性，在设置strict后，会将其他语法检查配置也和strict一直，除非单独设置其他配置
        

# 使用Webpack打包Ts代码

## 初始化package.json

通过npm初始化，生成package.json文件，管理项目

```sh
npm init -y

```

## 安装依赖

```sh
npm i -D webpack webpack-cli typescript ts-loader

```

项目根目录创建webpack.config.js配置文件，跟package.json同级

## webpack.config.js

```js
// 引入一个包
const path = require('path')
// webpack中的所有的配置信息都应该写在module.exports中
module.exports = {
  // 指定入口文件
  entry: './src/index.ts',
  
  // 指定打包文件所在的目录
  output: {
  	// 指定打包文件的目录
    path: path.resolve(__dirname, 'dist'),
    // 打包后文件的名字
    filename: 'bundle.js'
  },
  
  // 指定webpack打包时要使用的模块
  module: {
    // 指定要加载的规则
    rules: [
      {
        // test指定的是规则生效的文件
        test: /\.ts$/,
        // 要使用的loader
        use: 'ts-loader',
        // 要排除的文件
        exclude: /node-modules/
      }
    ]
  }
}

```

## tsconfig.json

使用指令创建tsconfig.json文件

```sh
tsc --init

```

```json
/* tsconfig.json */
{
  "compilerOptions": {
    "module": "ES2015",
    "target": "ES2015",
    "strict": true
  }
}

```

## package.json

在package.json中添加这样的配置

```json
/* package.json */
{
  "scripts": {
    "build": "webpack"
  }
}

```

## npm run build

上面配置完成后，通过`npm run build`进行打包

## 安装html-webpack-plugin

该插件在执行npm run build打包时，会在/dist/文件夹下创建index.html文件，并且可以自动引入js文件和配置一些内容。

```sh
npm i -D html-webpack-plugin

```

```js
/* webpack.config.js */

// 在webpack中引入插件
const HTMLWebpackPlugin = require('html-webpack-plugin')

// 配置Webpack插件
plugins: [
  new HTMLWebpackPlugin({
    // index.html中的title
    title: '自定义title',
    // 会将src下的index.html打包到dist文件夹下
    template: './src/index.html'
  }),
]

```

## Webpack开发服务器webpack-dev-server

webpack-dev-server支持热重载

安装

```sh
cnpm i -D webpack-dev-server
cnpm i -D clean-webpack-plugin

```

clean-webpack-plugin 是在打包前将旧的打包生成的文件先删除。

```js
/* webpack.config.js */

// 在webpack中引入插件
const {CleanWebpackPlugin} = require('clean-webpack-plugin');

// 配置Webpack插件
plugins: [
  new HTMLWebpackPlugin({
    // index.html中的title
    title: '自定义title',
    // 会将src下的index.html打包到dist文件夹下
    template: './src/index.html'
  }),
  new CleanWebpackPlugin(),
]

```

在package.json中进行配置

```json
/* package.json */
{
  "scripts": {
    "build": "webpack",
    "start": "webpack serve --open chrome.exe"
  }
}

```

配置webpack.config.js使其支持ts和js扩展名模块的导入

```js
{
  resolve: {
		extensions: ['.ts', '.js']
  }
}

```

安装babel

```sh
cnpm i -D @babel/core @babel/preset-env babel-loader core-js

```

webpack.config.js

```js
// 引入一个包
const path = require('path')
// webpack中的所有的配置信息都应该写在module.exports中
module.exports = {
  // 指定入口文件
  entry: './src/index.ts',
  
  // 指定打包文件所在的目录
  output: {
  	// 指定打包文件的目录
    path: path.resolve(__dirname, 'dist'),
    // 打包后文件的名字
    filename: 'bundle.js',
    // 告诉webpack不使用箭头函数
    environment: {
      arrowFunction: false
    }
  },
  
  // 指定webpack打包时要使用的模块
  module: {
    // 指定要加载的规则
    rules: [
      {
        // test指定的是规则生效的文件
        test: /\.ts$/,
        // 要使用的loader
        use: [
          // 配置babel
          {
            // 指定加载器
            loader: "babel-loader",
          	// 设置babel
            options: {
              // 设置预定义的环境
              presets:[
                [
                  // 指定环境的插件
                  "@babel/preset-env",
                  {
                    // 要兼容的目标浏览器
                    targets: {
                      "chrome":"58",
                      "ie":"11"
                    },
                    // 指定corejs的版本
                    "corejs": "3",
                    // 使用corejs的方法 "usage":表示按需加载
                    "useBuiltIns": "usage"
                  }
                ]
              ]
            }
          },
          'ts-loader'
        ],
        // 要排除的文件
        exclude: /node-modules/
      }
    ]
  }
}

```

# 面向对象

## 类

语法

```typescript
class 类名{
	属性名: 类型;
  constructor(参数: 类型) {
    this.属性名 = 参数;
  }
  方法名() {
    ...
  }
}

```

实例

```typescript
class Person{
  // 实例属性 通过实例化对象访问
  name: String;
  readonly age: number;

  // 静态属性 可以直接通过类名访问
  static sex: String = "男";

  // 构造器
  constructor(name: String, age: number) {
    this.name = name;
    this.age = age;
  }

  // 定义奔跑方法
  run (): String {
    console.log(this.name, this.age);
    return "奔跑中";
  }
}

const per = new Person("张三", 18);
// per.age = 20; // 只读属性 无法修改
console.log(per.run(), Person.sex);

```

## 继承

继承后可以将父类中的方法和属性完全继承过来，也可以重写父类中的方法

```typescript
class Animal{
  name: String;
  age: number;


  constructor(name: String, age: number) {
    this.name = name;
    this.age = age;
  }

  run(): void {
    console.log("动物在运动");
  }
}

class Dog extends Animal{
  run(): void {
    console.log(this.name + "" + this.age + "岁，在运动");
  }
}

const dog = new Dog("小黑", 5);
dog.run();

```

如果需要在子类中添加新的属性，然后需要在子类中写构造器的话，需要手动调用父类中的构造器。

```js
class Animal{
  name: String;
  age: number;

  constructor(name: String, age: number) {
    this.name = name;
    this.age = age;
  }

  run(): void {
    console.log("动物在运动");
  }
}

class Dog extends Animal{
  sex: String;

  constructor(name: String, age: number, sex: String) {
    super(name, age);
    this.sex = sex;
  }

  run(): void {
    console.log(this.name + "" + this.age + "岁，" + this.sex + "，在运动");
  }
}

const dog = new Dog("小黑", 5, '公');
dog.run();

```

子类中的构造器需要也把父类中的属性也传入，因为子类的构造器相当于一个中转，将参数中传给父类中的构造器。

## 抽象类

通过`abstract`关键字定义的类，就是抽象类，抽象类不能实例化对象，只能被子类继承。

抽象类中存在抽象方法，抽象方法也是通过`abstract`关键字定义的，并且抽象方法只能定义在抽象类中，子类必须对父类中的抽象方法进行重写。抽象方法没有方法体。，基本上和java中的抽象方法保持一致。

```typescript
abstract class Animal{
  name: String;

  constructor(name: String) {
    this.name = name;
  }

  abstract run(): void;
}

class Dog extends Animal{
  age: number;
  
  constructor(name: String, age: number) {
    super(name);
    this.age = age;
  }

  run(): void {
    console.log(this.name + "" + this.age + "岁，在运动");
  }
}

const dog = new Dog("小黑", 5);
dog.run();

```

## 接口

例子

```typescript
type myType = {
  name: String,
  age: number
}

const obj: myType = {
  name: 'sss',
  age: 10
}

```

使用接口实现

```typescript
interface myInterface {
  name: String;
  age: number;
}

const obj: myInterface = {
  name: 'sss',
  age: 10
}

```

接口是用来定义一个类的结构，用来定义一个类中应该包含哪些属性和方法，同时接口也可以当成类型声明去使用。

### 多接口同名合并

type不允许定义同名的类型声明，而接口可以定义多个同名的接口，只是定义多个相同的接口，会将多个同名接口融合一起。

```typescript
interface myInterface {
  name: String;
  age: number;
}

interface myInterface {
  sex: String;
}

const obj: myInterface = {
  name: "小户",
  age: 10,
  sex: "男"
}

```

### 接口中的属性和方法

接口中的属性和方法都是只定义，方法只能为抽象方法，属性只定义不赋值。

```typescript
interface myInterface {
  name: String;
  sayHello(): void;
}

```

### 实现接口

类实现接口，需要将接口中的抽象方法全部重写，然后将接口中的参数利用构造方法进行赋值操作。

```typescript
interface myInterface {
  name: String;
  sayHello(): void;
}

class MyClass implements myInterface {
  name: String;
  constructor(name: string) {
    this.name = name;
  }
  sayHello(): void {
    console.log("注意看，这个男人他叫" + this.name)
  }
}

const myClass = new MyClass("小帅");
myClass.sayHello();

```

## 属性的封装

> public 公共
> 
> private 私有
> 
> protected 受保护

TS中的属性和方法关键字和Java中有所不同，TS中没有省略的，只有public、private、protected，这三个修饰符的作用和java中的一模一样。

如果省略则表示public公共的。

私有变量依然可以定义getter、setter方法。

```typescript
;(function(){
  // 定义一个表示人的类
  class Person{
    private name: String;
    private age: number;
    constructor(name: String, age: number) {
      this.name = name;
      this.age = age;
    }
    public setName(name: String): void {
      this.name = name;
    }
    public getName(): String {
      return this.name;
    }
    public setAge(age: number): void {
      this.age = age;
    }
    public getAge(): number {
      return this.age;
    }
  }

  const per = new Person("孙悟空", 18);
  per.setName("猪八戒");
  per.setAge(-100);
  console.log(per.getName(), per.getAge());
})();

```

getter、setter方法的好处就是，让本类中的变量不会被随意修改，可以在setter方法中进行控制。

```typescript
;(function(){
  // 定义一个表示人的类
  class Person{
    private name: String;
    private age: number;
    constructor(name: String, age: number) {
      this.name = name;
      this.age = age;
    }
    public setName(name: String): void {
      this.name = name;
    }
    public getName(): String {
      return this.name;
    }
    public setAge(age: number): void {
      if(age >= 0) {
        this.age = age;
      }
    }
    public getAge(): number {
      return this.age;
    }
  }

  const per = new Person("孙悟空", 18);
  per.setName("猪八戒");
  per.setAge(-100);
  console.log(per.getName(), per.getAge());
})();

```

TtypScript中的get、set方法的另一种写法

```typescript
// 定义一个表示人的类
class Person{
  private _name: String;
  private _age: number;
  constructor(name: String, age: number) {
    this._name = name;
    this._age = age;
  }
  get name() {
    return this._name;
  }
  set name(name: String) {
    this._name = name;
  }
  get age() {
    return this._age;
  }
  set age(age: number) {
    if(age >= 0) {
      this._age = age;
    }
  }
}

const per = new Person("孙悟空", 18);
per.name = "猪八戒";
per.age = -100;
console.log(per.name, per.age);

```

## 构造器和属性结合

以下两种写法完全一样。

```typescript
// 定义一个表示人的类
class Person{
  public name: String;
  constructor(name: String) {
    this.name = name;
  }
}

const per = new Person("红孩儿");
console.log(per.name);


```

```typescript
// 定义一个表示人的类
class Person{
  constructor(public name: String) {}
}

const per = new Person("红孩儿");
console.log(per.name);

```

# 泛型

在定义函数或者类时，如果遇到类型不明确就可以使用泛型。

```typescript
// function fn(num: ?): ? {
// 	return num;
// }

function fn<T>(num: T): T{
  return num;
}

console.log(fn("你好，泛型。")); // 不指定泛型类型，TS可以自动对类型进行推断
console.log(fn<String>("你好，泛型。")); // 指定泛型类型
console.log(fn<number>(24)); // 指定泛型类型

```

泛型可以指定多个

```typescript
function fn<T, K>(num: T, num1: K): T{
  console.log(num1);
  return num;
}

console.log(fn('123', 123));
console.log(fn<String, Number>('123', 123));

```

限制泛型的类型

```typescript
interface Inter{
	length: number;
}

function fn<T extends Inter>(a: T): number{
  return a.length;
}

fn("123"); // 字符串有.length属性
fn(123); // 报错，因为数字没有.length属性
fn({length: 1}); // 自定义对象内有.length属性，所以不会报错

```

以上泛型`T`是实现了接口`Inter`，但是这里使用的是`extends`关键字，而不是`implements`，需要特别注意。

类中的泛型

```typescript
class MyClass<T> {
	name: T;
  constructor(name: T) {
    this.name = name;
  }
}

const mc = new MyClass<String>("孙悟空");
const mc1 = new MyClass<Number>(123);
console.log(mc,mc1);

```