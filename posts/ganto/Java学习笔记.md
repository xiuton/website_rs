---
title: "Java学习笔记"
date: 2023-03-01 01:21:00
author: 干徒
link: https://www.cnblogs.com/ganto/articles/17166660.html
---
# 基础👇

# 简单Java类

### 第一种开发要求

*   类名称必须存在有意义，例如：Book、Emp；
*   类之中所有的属性必须private封装，封装后的属性必须提供有setter、getter方法；
*   类之中可以提供有任意多个构造方法，但是必须保留有一个无参构造方法；
*   类之中不允许出现任何的输出语句，所有的信息输出必须交给被调用处输出；
*   类之中需要提供有一个取得对象完整信息的方法，暂定为：getInfo()，而且返回String型数据；

### 范例

```java
class Emp{ // 定义一个有意义的类

    // 用private封装
    private int empno;
    private String ename;
    private String job;
    private double sal;
    private double comm;

    public Emp(){} // 无参构造方法
    public Emp(int empno,String ename,String job,double sal,double comm){ // 有参构造方法
        empno = empno;
        ename = ename;
        job = job;
        sal = sal;
        comm = comm;
    }

    // setter方法
    public void setEmpno(int e){
        empno = e;
    }
    public void setEname(String e){
        ename = e;
    }
    public void setJob(String j){
        job = j;
    }
    public void setSal(double s){
        sal = s;
    }
    public void setComm(double c){
        comm = c;
    }

    // getter方法
    public int getEmpno(){
        return empno;
    }
    public String getEname(){
        return ename;
    }
    public String getJob(){
        return job;
    }
    public double getSal(){
        return sal;
    }
    public double getComm(){
        return comm;
    }
    
    public String getInfo(){ // 取得对象完整信息的方法
        return "雇员编号：" + empno + "\n" + 
               "雇员姓名：" + ename + "\n" + 
               "雇员职位：" + job + "\n" + 
               "基本工资：" + sal + "\n" + 
               "佣    金：" + comm;
    }
}
public class Demo{
    public static void main(String args[]){
        Emp e = new Emp(7369,"SMITH","CLERK",800.0,1.0);
        e.setEname("ALLEN");
        System.out.println(e.getInfo());
        System.out.println("姓名：" + e.getEname());
    }
}

```

# 数组的定义与使用

### 数组的基本概念

*   声明并开辟数组：
    
    数据类型 数组名称\[\] = new 数据类型\[长度\];
    
    数据类型\[\] 数组名称 = new 数据类型\[长度\];
    
*   分布完成：
    
    *   声明数组：数据类型 数组名称\[\] = null;
        
    *   开辟数组：数组名称 = new 数据类型\[长度\];
        

### 定义数组

```java
public class Demo{
    public static void main(String args[]){
        // 声明并开辟了一个三个长度的数组
        int data[] = new int[3];

        // 给数组赋值，int类型的数组默认值为0
        data[0] = 0;
        data[1] = 1;
        data[2] = 2;

        System.out.println(data[0]); // 获取数组下标为0的数据

        System.out.println(data.length); // 获取数组长度

        for(int i = 0; i < data.length; i++){ // 利用for循环，将数组中的所有数据输出
            System.out.println(data[i]);
        }
    }
}

```

上方为数组为动态初始化

下方为数组静态初始化

*   格式一：简化格式
    *   数据类型 数组名称\[\] = {值,值,值,...};
*   格式二：完整格式
    *   数据类型 数组名称\[\] = new 数据类型\[\] {值,值,值,...};

### 定义数组

```java
public class Demo{
    public static void main(String args[]){
        int data[] = new int[]{1,2,3,4,5};
        for(int i = 0; i < data.length; i++){ 
            System.out.println(data[i]);
        }
    }
}

```

# 二维数组

### 语法

```java
动态初始化：数据类型 数组名称[][] = new 数据类型[行的个数][列的个数];
静态初始化：数据类型 数组名称[][] = new 数据类型[][]{{值,值,值,...},{值,值,值,...}};

```

### 范例

```java
public class Demo{
    public static void main(String args[]){
        int data[][] = new int[][]{
            {1,2,3},
            {4,5,6},
            {7,8,9}
        };
        for(int i = 0; i < data.length; i++){
            for(int j = 0; j < data[i].length; j++){
                System.out.print(data[i][j] + "\t");
            }
            System.out.println();
        }
    }
}

```

# 数组与方法参数的传递

### 范例：一个数组传递的程序

```java
public class Demo{
    public static void main(String args[]){
        int data[] = new int[]{1,2,3};
        change(data);
        for(int i = 0; i < data.length; i++){
            System.out.println(data[i]);
        }
    }
    public static void change(int temp[]){
        for(int i = 0; i < temp.length; i++){
            temp[i] *= 2;
        }
    }
}

```

### 实现一个数组排序

```java
public class Demo{
    public static void main(String args[]){
        int data[] = new int[]{2,1,9,0,5,4,3,7,6,8};
        print(data);
    }
    public static void print(int temp[]){
        for(int i = 0; i < temp.length; i++){
            System.out.print(temp[i] + "、");
        }
        System.out.println();
    }
}

```

# 方法

## 重载

方法重载又被称为：overload

在同一个类中，方法名相同，参数列表（参数数量、参数类型、参数顺序）不同，就是方法重载。

```java
// 参数数量不同
public class Hello
{
  public static void main(String[] args) {
    sum(1);
		sum(1, 2);
  }
  
  public static void sum(int num1) {
		System.out.print("方发一");
  }
  public static void sum(int num2, int num1) {
		System.out.print("方发二");
  }
}

```

```java
// 参数类型不同
public class Hello
{
  public static void main(String[] args) {
    System.out.print(sum(1, 2));
    System.out.print(sum("百度", "一下"));
    System.out.print(sum(1.2, 2.3));
  }
  
  public static int sum(int num1, int num2) {
		return num1 + num2;
  }
  
  public static String sum(String str1, String str2) {
    return str1 + str2;
  }
  
  public static double sum(double num1, double num2) {
    return num1 + num2;
  }
}

```

```java
// 参数顺序不同
public class Hello
{
  public static void main(String[] args) {
    sum(1, "123");
		sum("123", 1);
  }
  
  public static void sum(int num1, String num2) {
		System.out.print("方发一");
  }
  public static void sum(String num2, int num1) {
		System.out.print("方发二");
  }
}

```

## 封装

*   所有属性私有化，使用`private`关键字斤西瓜修饰，private表示私有的，修饰的所有数据只能在本类中访问。
*   对外提供简单的操作入口，也就是说以后外部程序想要访问age属性，必须通过这些简单的入口进行访问。

```java
package com.ganto.www;

public class User {
  private int age;

  public int getAge() {
    return age;
  }

  public void setAge(int age) {
    if(age <= 0 || age >= 150) {
      return;
    }
    this.age = age;
  }
}

```

> setter、getter方法没有static关键字
> 
> 有static关键字修饰的方法通过`类名.方法名(实参)`调用
> 
> 没有static关键字修饰的方法通过`引用.方法名(实参)`调用
> 
> static关键字，其实就是将变量或者方法暴漏在全局变量中，可以直接通过`类名.`的形式，去操作

```java
package com.ganto.www;

public class Main {
  public static void main(String[] args) {

    // 实例化对象
    User u = new User();

    u.setAge(2);

    System.out.println(u.getAge());
  }
}

```

### 构造方法

*   构造方法又被称作构造函数、构造器、Constructor
*   构造方法的语法结构：`修饰符列表 构造方法名(形参列表){构造方法体};`
*   构造方法的方法名就是本类的类名
*   构造方法的作用：
    *   构造方法存在的意义是，通过构造方法的调用，可以创建对象
    *   创建对象的同时，初始化实例变量的内存空间（将没有赋值的实例变量进行初始化）
*   构造方法怎么调用：`User u = new User();`，可以看出构造方法调用就是实例化对象
*   构造方法如果在类中没有定义的话，会默认创建一个没有参数的构造方法
*   构造方法可以重载

```java
package com.ganto.www;

public class User {
  public User() {

  }
  public User(int i) {
    
  }
}

```

### this关键字

实例化对象的对象中才有this：`User u = new User();`

### static关键字

通过static关键字声明的方法，通过`类名.方法名`进行调用

没有通过static关键字声明的方法，通过`引用.方法名`进行调用

## 继承

(1) Java是一门面向对象的编程语言，有三大特性：封装、继承、多态

(2) 继承基本的作用是：代码复用。但是继承最重要的作用是：有了继承才有了以后方法的覆盖和多态机制

(3) 继承的语法格式：

```scala
[修饰符列表] class 类名 extends 父类名 {	
	类体 = 属性 + 方法
}

```

(4) java语言当中的继承只支持单继承，一个类不能同时继承很多类，只能继承一个类。在C++中支持多继承。

(5) 关于继承中的一些术语：

 B类继承A类，其中：

 A类称为：父类、基类、超类、superclass

 B类称为：子类、派生类、subclass

(6) 在java语言中子类继承父类：

\- 私有的不支持继承  
\- 构造方法不支持继承  
\- 其他数据都可以被继承

(7) 虽然java语言中当中支持单继承，但是一个类也可以间接继承其他类，例如：

 C extends B {}

 B extends A {}

 A extends T {}

其中C类直接继承B类，但是C类间接继承B类、T类

(8) java语言中假设一个类没有显示继承任何自定义类，则该类默认继承JavaSE库中提供的java.lang.Object类。

### 方法重写

当父类中的方法已经不能满足子类中的需求了，就需要在子类中将继承过来的方法进行重写/覆盖。

## 多态

关于多态中涉及到的几个概念：

*   向上转型（upcasting）
    
    子类型 =》父类型（自动类型转换）
    
*   向下转型（downcasting）
    
    父类型 =》子类型（强制类型转换）
    
*   无论是向上转型还是向下转型，梁总类型之间必须要有继承关系。没有继承关系，程序无法编译通过。
    

例子：

Animal.java

```java
package com.ganto.www;

// 动物类
public class Animal {
    public void move() {
        System.out.println("动物在移动");
    }
}

```

Cat.java

```java
package com.ganto.www;

// 猫类
public class Cat extends Animal{
    // 重写Animal中继承过来的方法
    public void move() {
        System.out.println("猫在奔跑");
    }

    // 不是从父类中继承过来的方法，这个方法是猫类中特有的行为
    public void catchM() {
        System.out.println("猫抓老鼠");
    }
}

```

Bird.java

```java
package com.ganto.www;

// 鸟儿类
public class Bird extends Animal{
    public void move() {
        System.out.println("鸟儿在飞翔");
    }
}

```

Test.java

```java
package com.ganto.www;

// 测试类
public class Test {
    public static void main(String[] args) {
        Animal a = new Cat();
        a.move(); // 正常 运行Cat()类中的方法
      	// a.catchM(); // 报错
    }
}

```

以上代码属于向上类型转换，a.move()先根据Animal类型进行编译，然后运行时是根据实例Cat进行运行，所以拥有多种形态，被称为多态。

如果a.catchM()将会报错，因为在第一阶段编译都没有通过，编译会先查找Animal中的catchM()方法， 然后没有找到就会报错，编译没有找到，更别说运行了。

如果就是想让a.catchM()可以运行，需要将a转换成Cat类型`(Cat) a`，这是向下装换。

Test.java

```java
package com.ganto.www;

public class Test {
    public static void main(String[] args) {
        Animal a = new Cat();
        a.move();
      	
        ((Cat) a).catchM();
      	
      	// Cat c = (Cat)a; // 这俩行代码行方法代码相同
      	// c.catchM();
    }
}

```

向下类型转换出现的异常情况

```java
package com.ganto.www;

public class Test {
    public static void main(String[] args) {
        Animal a = new Bird();
      	Cat c = (Cat)a;
    }
}

```

以上代码虽然编译不会报错，但是运行会报强制类型转换的异常，详情请看[#java.lang.ClassCastException](#java.lang.ClassCastException)

解决办法如下，先通过`instanceof`判断a的指向类型，再进行安全的强制类型转换

```java
package com.ganto.www;

public class Test {
    public static void main(String[] args) {
        Animal a = new Bird();
        if(a instanceof Cat){
            Cat c = (Cat) a;
            c.catchM();
        }else if(a instanceof Bird) {
            Bird b = (Bird) a;
            b.eat();
        }
    }
}

```

### 多态作用

多态的作用：面向抽象编程，尽量不要面向具体编程。

> 降低程序的耦合度，提高程序的扩展力。
> 
> 能使用多态尽量使用多态。
> 
> 父类型引用指向子类型对象。

宠物类 Pet.java

```java
package com.ganto.www;

/**
 * 宠物类
 */
public class Pet {
    public void eat() {
        System.out.println("吃东西");
    }
}

```

猫类 Cat.java

```java
package com.ganto.www;

/**
 * 宠物猫
 */
public class Cat extends Pet{
    public void eat() {
        System.out.println("小猫正在吃鱼！");
    }
}

```

小狗类 Dog.java

```java
package com.ganto.www;

/**
 * 小狗类
 */
public class Dog extends Pet{
    public void eat() {
        System.out.println("小狗在啃骨头！");
    }
}

```

主人类 Ren.java

```java
package com.ganto.www;

/**
 * 主人类
 */
public class Ren {
    // 喂养宠物
    public void feed(Pet p) {
        p.eat();
    }
}

```

测试类 Test.java

```java
package com.ganto.www;

/**
 * 测试类
 */
public class Test {
    public static void main(String[] args) {
        Ren r = new Ren();
        r.feed(new Cat());
        r.feed(new Dog());
    }
}

```

## final关键字

1、final是一个关键字，表示最终的，不可变的

2、final修饰的类无法被继承

3、final修饰的方法无法被覆盖

4、final修饰的变量一旦赋值之后，不可重新赋值

5、final修饰的实例变量必须手动赋值，或者在构造方法中赋值，两者本质都是需要手动赋值，不能使用默认值

6、final修饰的引用，一旦指向某个对象后，不能重新指向别的对象地址，那么被指向的对象无法被垃圾回收器回收

7、final修饰的实例变量，一般和static联合使用，被称为常量

常量的定义语法格式：`public static final 类型 常量名 = 值`

Java规范中要求所有常量的名字全部大写，每个单词之间使用下划线连接。

```java
package com.ganto.www;
/**
 * 测试类
 */
public class Test {
    public static void main(String[] args) {
        System.out.println(Chinese.GUO_JI);
        System.out.println("圆周率：" + Math.PI);
    }
}
class Math{
    public static final double PI = 3.1415926;
}
class Chinese{
    public static final String GUO_JI = "中国";
}

```

# package和import

关于java语言中的包机制：

1、包又称为package，java中引入package这种语法机制只要是为了方便程序的管理。不同功能的类被分门别类放到不同的软件包当中，查找比较方便，管理比较方便，以维护。

2、怎么定义package呢？

*   在java源程序的第一行上编写package语句。
*   package只能编写一个语句。
*   语法架构：
    *   package 包名;

3、报名的命名规范：

*   公司域名倒序 + 项目名 + 模块名 + 功能名；
*   采用这种方式重名的几率较低。因为公司域名具有全球唯一性。

4、包名要求全部小写，包名也是标识符，必须遵循标识符的命名规则。

5、一个包将来对应的是一个目录。目录之间使用`.`隔开

6、使用了package机制之后，类名将要加上完整的包名才是完整的类名

`com.ganto.www.Test`

两个类在同一个软件包中，可以只写类名，可以省略包名。如果不再同一个软件包中，可以使用import将软件包引入后，也可以只写类名，也可以省略包名。

com/ganto/www/demo/Demo.java

```java
package com.ganto.www.demo;

public class Demo {
    public void console() {
        System.out.println("com.ganto.www.demo.Demo");
    }
}

```

com/ganto/www/Test.java

```java
package com.ganto.www
  
import com.ganto.www.demo.Demo;
// import com.ganto.www.demo.*;

public class Test {
    public static void main(String[] args) {
        Demo d = new Demo();
        d.console();
    }
}

```

import需要写在package之下class之上。

# 访问控制权限

1、访问控制权限修饰符来控制元素的访问范围。

2、访问控制权限修饰符包括：

| 修饰符 | 说明 |
| --- | --- |
| public | 表示公开，在任何位置都可以访问 |
| protected | 表示受保护的，本类、同包下、子类可以访问 |
| 缺省 | 表示默认，本类、同包可以访问 |
| private | 表示私有的，只能在本类中访问 |

protected和缺省：同一个包下都可以访问；不同的包且没有继承关系都不可以访问；不同的包必须继承后protected才能访问，缺省不能访问。

3、访问控制权限修饰符可以修饰类、变量、方法...

4、当某个数据希望子类使用，使用protected进行修饰。

5、修饰符的权限关系：`private < 缺省 < protected < public`

6、类、接口只能采用public和缺省(default)的修饰符进行修饰。（内部类除外）

# 基础👆

# \-----------------------------------

# 进阶👇

汽车例子：

```java
package com.ganto.www;

public class Test {
    public static void main(String[] args) {
        // 通过无参数构造方法创造对象
        Vehicle v = new Vehicle();
        v.setSpeed(0);
        v.setSize(5);
        // 通过偶有参数的构造方法创建对象
        // Vehicle v1 = new Vehicle(70, 5);

        v.move();

        System.out.println(v.getSpeed());
        System.out.println(v.getSize());

        // 调用加速方法
        v.speedUp(10);
        System.out.println(v.getSpeed());

        // 调用减速方法
        v.speedDown(10);
        System.out.println(v.getSpeed());
    }
}

/**
 * 交通工具
 */
class Vehicle {
    // 速度
    private int speed;
    // 体积
    private int size;

    public Vehicle() {

    }

    public Vehicle(int speed, int size) {
        this.speed = speed;
        this.size = size;
    }

    public int getSpeed() {
        return speed;
    }

    public void setSpeed(int speed) {
        this.speed = speed;
    }

    public int getSize() {
        return size;
    }

    public void setSize(int size) {
        this.size = size;
    }

    /**
     * 交通工具的移动方法
     */
    public void move() {
        System.out.println("汽车开始行驶了！");
    }

    /**
     * 加速方法
     */
    public void speedUp(int addSpeed) {
        // speed++;
        this.setSpeed(this.getSpeed() + addSpeed);
    }

    /**
     * 减速方法
     */
    public void speedDown(int subSpeed) {
        // speed++;
        if(this.getSpeed()<=0) {
            return;
        }
        this.setSpeed(this.getSpeed() - subSpeed);
    }
}

```

# 抽象类

> 类到对象是实例化；对象到类是抽象。
> 
> 抽象类：
> 
> 1、什么是抽象类？
> 
>  类和类之间具有共同特征，将这些共同特征提取出来，形成的就是抽象类。
> 
>  类本身是不存在的，所以抽象类无法创建对象（无法实例化）。
> 
> 2、抽象类属于什么类型？
> 
>  抽象类也属于引用数据类型。
> 
> 3、抽象类怎么定义？
> 
>  语法：`[修饰符列表] abstract class 类名 { 类体; }`
> 
> 4、抽象类是无法实例化的，无法创建对象的，所以抽象类是用来被子类继承的。
> 
> 5、final和abstract不能联合使用，这两个关键字是对立的。
> 
> 6、抽象类的子类还可以继续抽象。
> 
> 7、抽象类虽然无法实例化，但是抽象类有构造方法，这个构造方法是供子类使用的。
> 
> 8、抽象类关联到一个概念：抽象方法：抽象方法表示没有实现的方法，没有方法体的方法。
> 
>  `public abstract void doSome ();`
> 
>  （抽象方法所在的类必须是抽象类，子类继承了该抽象类，那么子类也必须是抽象类，因为子类继承了抽象类中的抽象方法）
> 
>  抽象方法特点是：
> 
>  特点1：没有方法体，以分号结尾；
> 
>  特点2：前面修饰符列表中有abstract关键字。
> 
> 9、抽象类中不一定有抽象方法，抽象方法必须出现在抽象类中。
> 
> 10、重要结论：一个非抽象的类继承抽象类，必须将抽象类中的抽象方法实现了。这是java语法强行规定的，不然编译器会报错。这里的覆盖/重写，也可以叫做实现（对抽象的实现）。

```java
package com.ganto.www;

public class Test {
    public static void main(String[] args) {
        Account c = new CreditAccount(); // 这就是面向抽象编程
        c.doSome();
    }
}
abstract class Account{
    public abstract void doSome();
}
class CreditAccount extends Account{
    public void doSome() {
        System.out.println("子类在飞翔");
    }
}

```

# 接口

## 接口中的基础语法

> 1、接口也是一种引用数据类型。编译之后也是一个class字节码文件。
> 
> 2、接口是完全抽象的。抽象类是半抽象的。或者说接口是特殊的抽象类。
> 
> 3、接口怎么定义，语法：`[修饰符列表] interface 接口名 { }`
> 
> 4、接口可以继承，并且接口支持多继承，一个接口可以继承多个接口。`interface C extends A, B{ }`
> 
> 5、接口中只包含两部分内容：一部分是常量、一部分是抽象方法。接口中没有其他内容了。
> 
> 6、接口中所有的元素都是`public`修饰的。（都是公开的）
> 
> 7、接口中的抽象方法定义时，因为接口中只有抽象方法，并且接口中的所有元素都是`public`修饰的，那么接口中的方法`public abstract`修饰符可以省略。
> 
> ```java
> interface MyMath{
>   	// 抽象方法
>     // public abstract int sum(int num1, int num2);
>     int sum(int num1, int num2); // 两种抽象方法写法都正确
> }
> 
> ```
> 
> 8、接口中的方法都是抽象方法，所以接口中的方法不能有方法体。
> 
> 9、接口中定义常量可以省略`public static final`修饰符，因为接口中定义的变量为常量，常量一旦赋值无法修改。
> 
> ```java
> package com.ganto.www;
> 
> public class Test {
>     public static void main(String[] args) {
>         System.out.println(MyMath.PI);
>         // MyMath.PI = 3.14; // 因为接口中定义的变量为常量，常量一旦赋值无法修改。
>     }
> }
> interface MyMath {
>     // 常量
>     // public static final double PI = 3.1415926;
>     double PI = 3.1415926; // 两种常量写法都正确
> }
> 
> ```

> 类和类之间叫`继承`，类和接口之间叫做`实现`。
> 
> 仍然可以将其看作`继承`。
> 
> `继承`使用`extends`关键字完成。
> 
> `实现`使用`implements`关键字完成。
> 
> 当一个非抽象的类实现接口的话，必须将接口中的所有抽象方法全部实现（覆盖、重写）。
> 
> ```java
> package com.ganto.www;
> 
> public class Test {
>        public static void main(String[] args) {
>            MyMath m = new MyMathImpl();
>            System.out.println(m.PI);
>            System.out.println(m.sum(5, 2));
>            System.out.println(m.sub(5, 2));
>        }
> }
> 
> // 接口
> interface MyMath {
>        // 常量
>        double PI = 3.1415926;
>        // 抽象方法
>        int sum(int a, int b);
>        int sub(int a, int b);
> }
> // 类实现接口
> class MyMathImpl implements MyMath{
>        // 实现抽象方法
>        public int sum(int a, int b) {
>            return a + b;
>        }
>        public int sub(int a, int b) {
>            return a - b;
>        }
> }
> 
> ```
> 
> 子类实现接口中的方法时，不能省略`public`关键字，因为接口中的抽象方法修饰符只能为`public`，而子类对接口中的抽象方法进行重写时，重写的方法的修饰符不能比接口中的抽象方法修饰符权限低。

类和接口可以多实现

接口和接口支持多继承，那么类和接口也可以多实现。

```java
package com.ganto.www;

public class Test {
    public static void main(String[] args) {
        MyMath m = new MyMathImpl();
        System.out.println(m.PI);
        System.out.println(m.sum(5, 2));
        System.out.println(m.sub(5, 2));
        A a = new MyMathImpl();
        System.out.println(a.add(5));
    }
}

// 接口
interface MyMath {
    // 常量
    double PI = 3.1415926;
    // 抽象方法
    int sum(int a, int b);
    int sub(int a, int b);
}

interface A {
    int add(int anum);
}

// 类实现接口
class MyMathImpl implements MyMath, A{
    // 实现抽象方法
    public int sum(int a, int b) {
        return a + b;
    }
    public int sub(int a, int b) {
        return a - b;
    }
    public int add(int anum) {
        return anum;
    }
}

```

继承和实现同时存在，继承extends在前，实现implements在后

```java
package com.ganto.www;

public class Test {
    public static void main(String[] args) {
        Flyable f = new Cat();
        f.fly();
    }
}
class Animal{

}
interface Flyable{
    void fly();
}
class Cat extends Animal implements Flyable{
    public void fly() {
        System.out.println("猫有了翅膀");
    }
}

```

## 接口在开发中的作用

> 接口在开发中的作用，类似于多态在开发中的作用。

经典例子：厨师、菜单、顾客

菜单 FoodMenu.java

```java
package com.ganto.www;
// 菜单接口
interface FoodMenu {
    // 西红柿炒鸡蛋
    void xihongshichaojidan();
    // 鱼香肉丝
    void yuxiangrousi();
}

```

中餐厨师 ChinaCooker.java

```java
package com.ganto.www;
// 中餐厨师类
public class ChinaCooker implements FoodMenu{
  	// 实现接口中的抽象方法
    @Override
    public void xihongshichaojidan() {
        System.out.println("中餐师傅的西红柿炒鸡蛋做好了");
    }

    @Override
    public void yuxiangrousi() {
        System.out.println("中餐师傅的鱼香肉丝做好了");
    }
}

```

西餐厨师 AmericCooker.java

```java
package com.ganto.www;
// 西餐厨师类
public class AmericCooker implements FoodMenu{
  	// 实现接口中的抽象方法
    @Override
    public void xihongshichaojidan() {
        System.out.println("西餐师傅的西红柿炒鸡蛋做好了");
    }

    @Override
    public void yuxiangrousi() {
        System.out.println("西餐师傅的鱼香肉丝做好了");
    }
}

```

顾客 Customer.java

```java
package com.ganto.www;
// 顾客类
public class Customer {
		// 定义了一个私有的FoodMenu类型的引用
    private FoodMenu menu;
		
  	// 无参构造方法
    public Customer() { }
		// 有参构造方法
    public Customer(FoodMenu menu) {
        this.menu = menu;
    }
		
  	// getter setter方法
    public FoodMenu getMenu() {
        return menu;
    }

    public void setMenu(FoodMenu menu) {
        this.menu = menu;
    }
	
  	// 点菜
    public void order() {
        menu.xihongshichaojidan();
        menu.yuxiangrousi();
    }
}

```

测试类 Test.java

```java
package com.ganto.www;
// 测试类
public class Test {
    public static void main(String[] args) {
        // 实例化厨师
        FoodMenu f = new ChinaCooker();
        // 实例化顾客
        Customer zhangsan = new Customer(f);
				// 点菜
        zhangsan.order();
    }
}

```

# is a、has a、like a类型与类型之间的关系

## is a

Cat is a Animal（猫是一个动物），凡是能满足is a的，表示“继承关系”

```css
A extends B

```

## has a

I has a Pen（我有一支笔），凡是能满足has a关系的，表示“关联关系”，关联关系通常以“属性”的形式存在。

```css
A{
	B b;
}

```

## like a

Cooker like a FoodMenu（厨师像一个菜单一样），凡是能满足like a关系的，表示“实现关系”，实现关系通常是：类实现接口。

```css
A implements B

```

# 抽象类和接口有什么区别？

在这里我们只说一下抽象类和接口在语法上的区别。

至于以后抽象类和接口应该怎么进行选择，通过后面的项目去体会/学习。

> 抽象类是半抽象的。
> 
> 接口是完全抽象的。

> 抽象类中有构造方法。
> 
> 接口中没有构造方法。

> 接口和接口之间支持多继承。
> 
> 类和类之间只能单继承。

> 一个类可以同时实现多个接口。
> 
> 一个抽象类只能继承一个类（单继承）。

> 接口中只允许出现常量和抽象方法。

以后接口使用的比抽象类多。一般抽象类使用的比较少。

接口一般都是对“行为”的抽象。

# Scanner

输入内容

```java
java.util.*;
public class Test {
  public static void main(String[] args) {
    Scanner s = new Scanner(System.in);
    String str = s.next();
    System.out.println("输入的内容：" + str);
  }
}

```

`java.lang.*`这个包是Java默认自动导入的，如String、System等 都是java.lang包下的模块。

# Object类

JDK类库中的根类：Object

1、这个老祖宗类中的方法我们需要先研究一下，因为这些方法都是所有子类通用的。任何一个类默认继承Object。就算没有直接继承，最终也会间接继承。

2、Object类中有哪些常用的方法？

 我们去哪里找这些方法呢？

 第一种：去源码中找。但是这种方式比较麻烦，源代码比较难懂。

 第二种：去查阅java的类库的帮助文档。

目前为止我们只需要知道这几个方法即可：

*   protected Object clone() // 负责对象克隆的
*   int hashCode() // 获取对象哈希值的一个方法
*   boolean equals(Object obj) // 判断两个对象是否相等
*   String toString() // 将对象转换成字符串形式
*   protected void finalize() // 垃圾回收器负责调用的方法

## 关于Object类中的toString()方法

1、源代码

```java
public String toString() {
  	return getClass().getName() + "@" + Integer.toHexString(hashCode());
}

```

源代码上toString()方法的默认实现是：类名@对象的内存地址转换为十六进制的形式。

toString方法需要自己重写。

```java
package com.ganto.www;

public class Test {
    public static void main(String[] args) {
        MyTime time = new MyTime(1949, 10, 01);
        System.out.println(time.toString());
        System.out.println(time); // 会自动调用toString方法，和上方代码一样的效果
    }
}
class MyTime {
    int year;
    int month;
    int day;
    public MyTime() {}
    public MyTime(int year, int month, int day) {
        this.year = year;
        this.month = month;
        this.day = day;
    }

    // 重写toString()方法
    public String toString() {
        return this.year+"年"+this.month+"月"+this.day+"日";
    }

}

```

如上代码，如果直接打印对象，会自动调用实例化对象类中的toString()方法。

## 关于Object类中的equals()方法

1、源代码

```java
public boolean equals(Object obj) {
	return(this == obj);
}

```

判断基本数据类型是否相等，可以使用`==`，因为基本数据类型使用`==`是比较的两个变量的值。

判断两个java对象是否相等不能使用`==`，因为java中`==`是比较两个对象的内存地址。

所以源码使用了`==`，所以不能用源码中的equals()方法进行比较对象是否相等。所以需要我们重写equals()方法。

相同的返回值类型、相同的方法名、相同的形式参数列表。

```java
package com.ganto.www;

import java.util.Objects;

public class Test {
    public static void main(String[] args) {
        MyTime time1 = new MyTime(1949, 10, 1);
        MyTime time2 = new MyTime(1949, 10, 1);
        System.out.println(time1.equals(time2));
    }
}
class MyTime {
    int year;
    int month;
    int day;
    public MyTime() {}
    public MyTime(int year, int month, int day) {
        this.year = year;
        this.month = month;
        this.day = day;
    }

    @Override
    public boolean equals(Object o) {
        /**
         * 这里重写的equals方法中进行了三次判断
         * 1、粗略判断两个对象内存地址是否相等，如果相等则直接可以确定两个对象相等，如果不相等则走第2步
         * 2、判断传过来的对象是否是MyTime类型，如果不是则直接false，只有两者都是MyTime对象才有比较的意义，第2步是为第3步做准备
         * 3、判断两个对象中的参数的值是否相等，如果全部相等则才能证明两个对象相等。
         */
        if (this == o) return true;
        if (!(o instanceof MyTime)) return false;
        MyTime myTime = (MyTime) o;
        // 当年、月、日 都相等时，表示两个日期相等，即两个对象相等。
        return year == myTime.year && month == myTime.month && day == myTime.day;
    }
}

```

以上的equals()方法是IDEA生成的，感觉写的很好。

## java.lang.String源码中重写了toString、equals方法

java中的字符串java.lang.String类中也重写了toString方法和equals方法。

```java
package com.ganto.www;

public class Test {
    public static void main(String[] args) {
        // 大部分情况下，我们采用s1、s2这两个变量的形式来进行创建字符串对象
        String s1 = "hello";
        String s2 = "hello";
        System.out.println(s1 == s2); // true

        // 实际上String也是一个类，不属于基本数据类型
        // 既然String是一个类，那么一定存在构造方法
        String s3 = new String("hello");
        String s4 = new String("hello");
        System.out.println(s3 == s4); // false
        System.out.println(s3.equals(s4)); // true
    }
}


```

所以如果方法中存在String类型变量，重写equals方法时判断String类型的变量要使用String方法中的equals方法进行判断比较好一些。

> 总结：
> 
> *   java中的基本数据类型比较使用`==`。
> *   java中的所有的引用数据类型统一使用`equals()`方法来判断是否相等。

## finalize()方法 （看看了解就好，新版本被弃用了）

1、这个方法是protected修饰的，在Object类中的源代码：

```java
protected void finalize() throws Throwable{ }

```

2、这个方法只有一个方法体，里面没有代码，而且这个方法是protected修饰的。

3、这个方法不需要程序员手动调用，JVM的垃圾回收器负责调用这个方法。

4、finalize()方法的执行时机：当一个java对象即将被垃圾回收器回收的时候，垃圾回收器负责调用finalize()方法。

5、finalize()方法实际上是SUN公司为java程序员准备的一个时机，垃圾销毁时机。如果希望在对象销毁时机执行一段代码的话，这段代码就要写在finalize()方法当中。

6、静态代码块：静态代码块是在类加载时刻执行，并且只执行一次。

```java
// 静态代码块在类加载时执行，并且只执行一次。
static{
  	...
}

```

这就是一个SUN公司准备的时机。

finalize()方法同样也是SUN公司为程序员准备的一个时机。这个时机是垃圾回收时机。

```java
package com.ganto.www;

public class Test {
    public static void main(String[] args) {
        for(int i = 0; i < 10000000; i++){ // 多制造点垃圾，垃圾回收器才能启动
            Person p = new Person();
            p = null;
        }
    }
}
class Person {
    // 重写finalize方法
    protected void finalize() throws Throwable {
        System.out.println("即将销毁");
    }
}

```

System.gc();建议垃圾回收器启动。

```java
package com.ganto.www;

public class Test {
    public static void main(String[] args) {
            Person p = new Person();
            p = null;
            System.gc();
    }
}
class Person {
    // 重写finalize方法
    protected void finalize() throws Throwable {
        System.out.println("即将销毁");
    }
}

```

## hashCode方法

在Object中的hashCode方法是`public native int hashCode();`，这个方法不是抽象方法，带有native关键字，底层调用C++程序。

hashCode()方法返回的是哈希码/哈希值：实际上就是一个java对象的内存地址，经过哈希算法，得出的一个值。所以hashCode()方法的执行结果可以等同看做一个java对象的内存地址。

```java
package org.example;

public class Main {
    public static void main(String[] args) {
        Object o = new Object();
        int hashCodeValue = o.hashCode();
        System.out.println(hashCodeValue); // 1163157884

        myClass mc = new myClass();
        System.out.println(mc.hashCode()); // 1956725890
    }
}
class myClass{

}

```

# 内部类

1、内部类：在类的内部又定义了一个新的类。被称为内部类。

2、内部类的分类：

*   实例内部类：类似于实例变量
*   静态内部类：类似于静态变量
*   局部内部类：类似于局部变量
    *   匿名内部类：是局部内部类中的一种，因为这个类没有名字，所以叫匿名内部类

```java
class Test01{
  // 该类在类的内部，所以称为内部类
  // 由于前面有static，所以称为“静态内部类”
  static class Inner1{
    
  }
  
  // 该类在类的内部，所以称为内部类
  // 没有static叫做实例内部类
  class Inner2{
    
  }
  
  public void doSome() {
    // 局部变量
    int i = 100;
    // 该类在类的内部，所以称为内部类
    // 局部内部类
    class Inner3{
      
    }
  }
}

```

例子：

```java
package org.example;

public class Main {
    public static void main(String[] args) {
        MyMath mm = new MyMath();
        mm.mySum(new ComputeImpl(),12,12);
    }
}

// 负责计算的接口
interface Compute {
    int sum(int a, int b);
}

// Compute接口的实现类
class ComputeImpl implements Compute {
    public int sum(int a, int b) {
        return a + b;
    }
}

// 数学类
class MyMath {
    public void mySum(Compute c, int x, int y) {
        System.out.println(c.sum(x,y));
    }
}

```

匿名内部类对其改造：

```java
package org.example;

public class Main {
    public static void main(String[] args) {
        MyMath mm = new MyMath();
        mm.mySum(new Compute() {
            public int sum(int a, int b) {
                return a + b;
            }
        },12,12);
    }
}

// 负责计算的接口
interface Compute {
    int sum(int a, int b);
}

// 数学类
class MyMath {
    public void mySum(Compute c, int x, int y) {
        System.out.println(c.sum(x,y));
    }
}

```

# 数组

1、Java中的数组是一种引用数据类型，不属于基本数据类型。数组的父类是Object。

2、数组实际上是一个容器，可以同时容纳多个元素。

3、数组当中可以存储“基本数据类型”的数据，也可以存储“引用数据类型”的数据。

4、数组因为是引用类型，所以数组对象是堆内存当中的。（数组是存储在堆当中的）

5、数组当中入股存储的是“Java对象”的话，实际上存储的是对象的“引用（内存地址）”，数组中不能直接存储Java对象。

6、数组一旦创建，在Java中规定，长度不可变。（数组长度不可变）

7、数组的分类：一维数组、二维数组、三维数组、多维数组...

8、所有的数组对象都有length属性（Java自带的属性），用来获取数组中元素的个数。

9、Java中的数组要求数组中元素的类型统一。比如int类型数组只能存储int类型，Person类型数组只能存储Person类型。

10、数组在内存方面存储的时候，数组中的元素内存地址（存储的每一个元素都是有规则的挨着排列的）是连续的。内存地址连续这是数组存储元素的特点/特色。数组实际上是一种简单的数据结构。

11、所有的数组都是用第一个元素的内存地址当作整个数组对象的内存地址

12、数组中每一个元素都有下标，下标从0开始，以1递增。最后一个元素的下标为length-1

## 定义数组

```java
int[] arr1;
double[] arr2;
boolean[] arr3;
String[] arr4;
Object[] arr5;

```

## 初始化数组

### 静态初始化一维数组

```java
int[] arr = {1,2,3,4};
int[] arr1 = new int[]{1,2,3,4};

```

### 动态初始化一维数组

```java
int[] arr = new int[5]; // 这里的5表示数组的元素个数，初始化一个5个长度int类型的数组，每个元素默认为0

String[] names = new String[3]; // 初始化一个3个长度String类型的数组，每个元素默认为null

```

| 数据类型 | 默认值 |
| --- | --- |
| byte | 0 |
| short | 0 |
| int | 0 |
| long | 0L |
| float | 0.0F |
| double | 0.0 |
| boolean | false |
| char | \\u0000 |
| 引用数据类型 | null |

```java
package org.example;

public class Main {
    public static void main(String[] args) {
        int[] arr = {11,22,33,44,50};
        arr[4] = 55;
        System.out.println(arr[4]);

        String[] arr1 = new String[2];
        arr1[0] = "xixi";
        System.out.println(arr1[0]);
    }
}

```

## main方法上面的String\[\] args有什么用？

通过命令行执行java程序

```sh
javac *.java
java Main admin admin

```

```java
package org.example;

public class Main {
    public static void main(String[] args) {
        System.out.println(args.length);

        if(args.length != 2) {
            System.out.println("请输入账号密码");
            return;
        }

        String username = args[0];
        String password = args[1];
				// "admin".equals(username)这样的写法，可以避免空指针异常
        if("admin".equals(username) && "admin".equals(password)){
            System.out.println("登录成功");
        }else{
            System.out.println("登录失败");
        }
    }
}

```

IDEA通过点击run-Edit Configurations-Program arguments配置参数

## 数组中存储引用数据类型

```java
package org.example;

public class Main {
    public static void main(String[] args) {
        Animal a1 = new Animal();
        Animal a2 = new Animal();
        Animal a3 = new Cat();
        Animal[] animals = {a1, a2, a3};

        for(int i = 0; i < animals.length; i++) {
            animals[i].move();
        }
    }
}

class Animal {
    public void move() {
        System.out.println("Animal move...");
    }
}

class Cat extends Animal {
    public void move() {
        System.out.println("猫在move...");
    }
}

```

## 数组扩容

在Java开发中，数组长度一旦确定不可变，那么数组满了，就需要对数组进行扩容。Java中的数组扩容是先新建一个大容量的数组，然后将小容量数组中的数据一个一个拷贝到大数组当中。

### 数组拷贝

System.arraycopy()

```java
package org.example;

public class Main {
    public static void main(String[] args) {
        int[] a = {1,2,3,4};
        int[] b = new int[10];
        System.arraycopy(a, 1, b, 0, 2); // 5个参数：源数组、源数组开始下标、目标数组、目标数组开始下标、拷贝数组长度
        System.out.println(b[0]);
    }
}


```

## 二维数组

二维数组是特殊的一维数组，特殊在这个一维数组当中的每一个元素是一个一维数组。

### 二维数组静态初始化

```java
package org.example;

public class Main {
    public static void main(String[] args) {
        int[][] arr = {{1, 2}, {3, 4, 5, 6}, {7}};
        for(int i = 0; i < arr.length; i++) {
            for (int j = 0; j < arr[i].length; j++) {
                System.out.print(arr[i][j] + " ");
            }
            System.out.println();
        }
    }
}

```

### 二维数组动态初始化

```java
package org.example;

public class Main {
    public static void main(String[] args) {
        int[][] arr = new int[2][2];
        for(int i = 0; i < arr.length; i++) {
            for (int j = 0; j < arr[i].length; j++) {
                System.out.print(arr[i][j] + " ");
            }
            System.out.println();
        }
    }
}

```

实例：

```java
package org.example;

public class Main {
    public static void main(String[] args) {
        int[][] arr = {{1,2,3}, {4,5}, {6}};
        printArr(arr);

        int[][] arr1 = new int[3][];
        arr1[0] = new int[]{1, 2, 3};
        arr1[1] = new int[]{4, 5};
        arr1[2] = new int[]{6};
        printArr(arr1);

        printArr(new int[][]{{1,2,3}, {4,5}, {6}});
    }

    public static void printArr(int[][] array) {
        for(int i = 0; i < array.length; i++) {
            for (int j = 0; j < array[i].length; j++) {
                System.out.print(array[i][j] + " ");
            }
            System.out.println();
        }
        System.out.println("=====");
    }
}

```

## 作业

### 使用一维数组，模拟栈数据结构

**要求：**

1、这个栈可以存储Java中的任何引用类型的数据。

2、在栈中提供push方法模拟压栈。（栈满了，要有提示信息）

3、在栈中提供pop方法模拟弹栈。（栈空了，也有提示信息）

4、编写测试程序，new栈对象，调用push pop方法来模拟压栈弹栈的动作。

5、假设栈的默认初始化容量是10.（请注意无参数构造方法的编写方式。）

代码

```java
package org.example;

public class MyStack {
    private Object[] elements;

    // 栈帧，永远指向栈顶部元素
    // 默认初始值是0，最初的栈是空的，一个元素都没有
    private int index;

    public MyStack() {
        this.elements = new Object[10];
        this.index = -1;
    }

    public Object[] getElements() {
        return elements;
    }

    public void setElements(Object[] elements) {
        this.elements = elements;
    }

    public int getIndex() {
        return index;
    }

    public void setIndex(int index) {
        this.index = index;
    }

    // 压栈方法push
    public void push(Object o) {
        if(this.index >= this.elements.length-1) {
            System.out.println("栈满！压栈失败！");
            return;
        }
        this.index++;
        this.elements[this.index] = o;
        // 所有System.out.println()方法执行时，如果输出引用的话，会自动调用引用的toString()方法
        // System.out.println("压栈" + o.toString() + "元素成功，栈帧指向" + this.index);
        System.out.println("压栈" + o + "元素成功，栈帧指向" + this.index);
    }

    // 弹栈方法pop
    public Object pop() {
        if(this.index < 0) {
            System.out.println("栈空！弹栈失败！");
            return null;
        }
        System.out.print("弹栈" + this.elements[this.index] + "元素成功，");
        this.elements[this.index] = null;
        this.index--;
        System.out.println("栈帧指向" + this.index);
        return null;
    }
}

```

测试程序

```java
package org.example;

public class Main {
    public static void main(String[] args) {
        MyStack stack = new MyStack();
        stack.push(new Animal());
        stack.pop();
        stack.pop();
    }
}

class Animal {

}

```

### 酒店

酒店管理系统，模拟订房、退房、打印所有房间状态。

1、该系统的用户是：酒店前台。

2、酒店使用一个二位数组来模拟。`Room[][] romms`

3、酒店中的每一个房间应该是一个Java对象：Room

4、每一个房间Room应该有：房间编号、房间类型、房间是否空闲

5、系统应该对外提供的功能：

*   可以预定房间：用户输入房间编号，订房。
*   可以退房：用户输入房间编号，退房。
*   可以查看所有房间的状态：用户输入某个指令应该可以查看所有房间状态。

代码：

Room.java

```java
package org.example.homework;

/**
 * 房间类
 */
public class Room {
    /**
     * 房间编号
     * 1楼：101 102
     * 2楼：201 202
     * 3楼：301 302
     */
    private int no;
    /**
     * 房间类型：标准间、单人间、总统套房
     */
    private String type;
    /**
     * 房间状态
     * true 空闲
     * false 占用
     */
    private boolean status;

    public Room() {
    }

    public Room(int no, String type, boolean status) {
        this.no = no;
        this.type = type;
        this.status = status;
    }

    public int getNo() {
        return no;
    }

    public void setNo(int no) {
        this.no = no;
    }

    public String getType() {
        return type;
    }

    public void setType(String type) {
        this.type = type;
    }

    public boolean getStatus() {
        return status;
    }

    public void setStatus(boolean status) {
        this.status = status;
    }

    @Override
    public String toString() {
        return "Room{" +
                "no=" + no +
                ", type='" + type + '\'' +
                ", status=" + (status ? "'空闲'" : "'占用'") +
                '}';
    }

    @Override
    public boolean equals(Object o) {
        if (this == o) return true;
        if (!(o instanceof Room)) return false;
        Room room = (Room) o;
        return no == room.no && status == room.status && type.equals(room.type);
    }

}

```

Hotel.java

```java
package org.example.homework;

import java.util.Arrays;

/**
 * 酒店类
 */
public class Hotel {
    private Room[][] rooms;

    public Hotel() {
        this.rooms = new Room[3][10];
        for(int i = 0; i < rooms.length; i++) {
            for (int j = 0; j < rooms[i].length; j++) {
                this.rooms[i][j] = new Room();

                this.rooms[i][j].setNo(((i+1)*100)+(j+1));
                if(i == 0) {
                    this.rooms[i][j].setType("单人间");
                }else if(i == 1) {
                    this.rooms[i][j].setType("标准间");
                }else {
                    this.rooms[i][j].setType("总统套房");
                }
                this.rooms[i][j].setStatus(true);
            }
        }
    }

    public Hotel(Room[][] rooms) {
        this.rooms = rooms;
    }

    public Room[][] getRooms() {
        return rooms;
    }

    public void setRooms(Room[][] rooms) {
        this.rooms = rooms;
    }

    // 打印所有房间状态
    public void print() {
        for (int i = 0; i < this.getRooms().length; i++) {
            for (int j = 0; j < this.getRooms()[i].length; j++) {
                System.out.println(this.getRooms()[i][j]);
            }
        }
    }

    // 订房
    public void order(int roomNo) {
        for (int i = 0; i < this.getRooms().length; i++) {
            for (int j = 0; j < this.getRooms()[i].length; j++) {
                if(this.getRooms()[i][j].getNo() == roomNo) {
                    this.getRooms()[i][j].setStatus(false);
                    System.out.println(roomNo + "已订房！");
                }
            }
        }
    }

    // 退房
    public void exit(int roomNo) {
        for (int i = 0; i < this.getRooms().length; i++) {
            for (int j = 0; j < this.getRooms()[i].length; j++) {
                if(this.getRooms()[i][j].getNo() == roomNo) {
                    this.getRooms()[i][j].setStatus(true);
                    System.out.println(roomNo + "已退房！");
                }
            }
        }
    }
}

```

HotelMgtSystem.java

```java
package org.example.homework;

import java.util.Scanner;

public class HotelMgtSystem {
    public static void main(String[] args) {
//        Hotel h = new Hotel();
//        h.order(101);
//        h.print();
//        h.exit(101);
//        h.print();
        Hotel h = new Hotel();
        System.out.println("******欢迎使用！******");
        System.out.println("[1]查看房间列表；[2]订房；[3]退房；[0]退出。");
        Scanner s = new Scanner(System.in);
        while (true) {
            System.out.print("请输入功能编号：");
            int i = s.nextInt();
            switch (i) {
                case 1:
                    h.print();
                    break;
                case 2:
                    System.out.print("请输入订房房间号：");
                    int ii = s.nextInt();
                    h.order(ii);
                    break;
                case 3:
                    System.out.print("请输入退房房间号：");
                    int iii = s.nextInt();
                    h.exit(iii);
                    break;
                case 0:
                    return;
                default:
                    System.out.println("输入错误！请重新输入...");
            }
        }
    }
}

```

## 冒泡排序算法（排序算法）

```java
package org.example;

/**
 * 冒泡排序算法
 */
public class Main {
    public static void main(String[] args) {
        int[] arr = {2,5,4,3,1};

        for(int i = 0; i < arr.length - 1; i++) {
            for(int j = 0; j < arr.length - i - 1; j++) {
                if(arr[j] > arr[j+1]) {
                    int b = arr[j];
                    arr[j] = arr[j+1];
                    arr[j+1] = b;
                }
            }
        }
        for (int i = 0; i < arr.length; i++) {
            System.out.print(arr[i] + " ");
        }
    }
}

```

```java
package org.example;

/**
 * 冒泡排序算法
 */
public class Main {
    public static void main(String[] args) {
        int[] arr = {2,5,4,3,1};

        for(int i = arr.length - 1; i > 0; i--) {
            for(int j = 0; j < i; j++) {
                if(arr[j] > arr[j+1]) {
                    int b = arr[j];
                    arr[j] = arr[j+1];
                    arr[j+1] = b;
                }
            }
        }
        for (int i = 0; i < arr.length; i++) {
            System.out.print(arr[i] + " ");
        }
    }
}

```

## 选择排序算法（排序算法）

```java
package org.example;

/**
 * 选择排序算法
 */
public class Main {
    public static void main(String[] args) {
        int[] arr = {2, 5, 4, 7, 6, 1, 3};
        for(int i = 0; i < arr.length-1; i++){
            System.out.println(i);
            for(int j = i+1; j < arr.length; j++) {
                System.out.println("==>"+j);
                if(arr[i] > arr[j]) {
                    int a = arr[j];
                    arr[j] = arr[i];
                    arr[i] = a;
                }
            }
        }
        for(int i = 0; i < arr.length; i++) {
            System.out.print(arr[i] + " ");
        }
    }
}

```

## 二分法查找（查找算法）

二分法查找是建立在排序过后的数据上，才可以使用的算法。

```java
package org.example;

/**
 * 选择排序算法
 */
public class Main {
    public static void main(String[] args) {
        int[] arr = {11, 22, 55, 66, 77, 99, 111, 333};

        int a = binarySearch(arr, 333);
        System.out.println(a);
    }
    public static int binarySearch(int[] arr, int dest) {
        int begin = 0;
        int end = arr.length - 1;
        while (begin <= end) {
            int mid = (begin + end) / 2;
            if(arr[mid] == dest) {
                return mid;
            } else if(arr[mid] < dest) {
                begin = mid + 1;
            } else {
                end = mid - 1;
            }
        }
        return -1;
    }
}

```

## Arrays工具类

即：java.util.Arrays工具类

Arrays.sort()

```java
package org.example;

import java.util.Arrays;

public class Main {
    public static void main(String[] args) {
        int[] arr = {11,2,13};
        Arrays.sort(arr);
        for (int i = 0; i < arr.length; i++) {
            System.out.println(arr[i]);
        }
    }
}

```

# 常用类

## String

1、String表示字符串类型，属于引用数据类型，不属于基本数据类型

2、在Java中随便使用双引号括起来的都是String对象。

3、Java中规定，双引号括起来的字符串，是不可变的。

4、在JDK当中双引号括起来的字符串，"abc"、"你好"、"Hello World!"都是直接存储在方法区的字符串常量池当中的。而new String("xy")，是先在方法区的字符串常量池中有一份，然后堆内存中保存常量池中的内存地址，栈内存保存堆内存中的内存地址。

```java
package org.example;

public class Main {
    public static void main(String[] args) {
        String s1 = "123";
        String s2 = "123";
        System.out.println(s1 == s2); // true

        String x = new String("1");
        String y = new String("1");
        System.out.println(x == y); // false
	      System.out.println(x.equals(y)); // true

    }
}

```

> 以上代码证明，字符串之间的比较不能使用`==`，使用equals比较保险，因为String比较特殊，使用简易的方式定义 和 使用对象的方式定义 情况有所不同，简易方式定义是栈内存直接保存方法区常量池中的值的内存地址；而对象方式定义是堆内存保存方法区常量池中的值的内存地址，然后栈内存保存堆内存的内存地址。
> 
> \==String类中的equals()方法是官方已经重写过的，所以不需要再重写，可以直接使用。

### String类中的构造方法

将byte数组中的元素全部转换成字符串

将byte数组中的一部分元素转换成字符串 第二个参数是从哪个下标开始 第三个参数是元素长度

```java
package org.example;

public class Main {
    public static void main(String[] args) {
        byte[] bytes = {97,98,99}; // 97是a, 98是b，99是c
        String s1 = new String(bytes); // abc
	      String s2 = new String(bytes, 1, 2); // bc 这里的1是offset参数，2是length参数
        /**
         * 前面说过：输出一个引用的时候，会自动调用toString()方法，默认继承自Object中的toString()方法，会自动输出对象的内存地址。
         * 而以下代码会输出”abc“，则可以说明，官方也将String类中的toString()方法进行了重写。
         */
        System.out.println(s1); // abc
	      System.out.println(s1.toString()); // abc
      	System.out.println(s2); // bc
    }
}

```

将char数组中的元素全部转换成字符串

将char数组中的一部分元素转换成字符串 第二个参数是从哪个下标开始 第三个参数是元素长度

```java
package org.example;

public class Main {
    public static void main(String[] args) {
        char[] chars = {'I', '❤', 'J', 'A', 'V', 'A'};
        String s1 = new String(chars); // I❤JAVA
        String s2 = new String(chars, 2, 4); // JAVA

        System.out.println(s1); // I❤JAVA
        System.out.println(s2); // JAVA
    }
}

```

![](images%5CJava%5CString%E4%B8%AD%E7%9A%84%E6%9E%84%E9%80%A0%E6%96%B9%E6%B3%95.png)

### String类中的常用方法

#### 1、char charAt(int index)

**返回某个下标的字符**

```java
package org.example;

public class Main {
    public static void main(String[] args) {
	      // "中国人"是一个String对象，只要是对象就可以通过`.`的形式调用特有的方法。
        char c = "中国人".charAt(1);
        System.out.println(c); // 国
    }
}

```

#### 2、int compareTo(String anotherString)

**两个字符串按照字典的顺序进行比较**

```java
package org.example;

public class Main {
    public static void main(String[] args) {
        System.out.println("abc".compareTo("abc")); // 0 前后一致 10-10=0
        System.out.println("abcd".compareTo("abce")); // -1 前小后大 8-9=-1
        System.out.println("abce".compareTo("abcd")); // 1 前大后小 9-8=1
    }
}

```

> 比较规则是，相同下标的字符先进行比较，从下标零开始，如果没有比较出大小，则继续下标加一进行比较，比较出来大小后，则返回比较相同下标的两个字符的字典顺序的相减的值。

#### 3、boolean contains

**判断前面的内容是否包含后面的内容，返回Boolean值**

```java
package org.example;

public class Main {
    public static void main(String[] args) {
        System.out.println("hello java".contains("java")); // true
    }
}

```

#### 4、boolean endsWith(String suffix)

**判断当前字符串是否以某个字符串结尾**

```java
package org.example;

public class Main {
    public static void main(String[] args) {
        System.out.println("hello java".endsWith("java")); // true
    }
}

```

#### 5、boolean equals(Object anObject)

**比较两个字符串必须使用equals方法，不能使用`==`**

```java
package org.example;

public class Main {
    public static void main(String[] args) {
        System.out.println("hello java".equals("hello java")); // true
    }
}

```

#### 6、boolean equalsIgnoreCase(String anotherString)

**判断两个字符串是否相等，并且同时忽略大小写**

```java
package org.example;

public class Main {
    public static void main(String[] args) {
        System.out.println("Hello Java".equalsIgnoreCase("hello java")); // true
    }
}

```

#### 7、byte\[\] getBytes()

**将字符串对象转换成字节数组**

```java
package org.example;

public class Main {
    public static void main(String[] args) {
        byte[] bytes = "abcdef".getBytes();
        for(int i = 0; i < bytes.length; i++) {
            System.out.println(bytes[i]);
        }
    }
}

```

#### 8、int indexOf(String str)

**判断某个子字符串在当前字符串中第一次出现处的索引，如果没有该子字符串则返回`-1`**

```java
package org.example;

public class Main {
    public static void main(String[] args) {
        System.out.println("123456".indexOf("6")); // 5
        System.out.println("123456".indexOf("7")); // -1
    }
}

```

#### 9、boolean isEmpty()

**判断某个字符串是否为空**

```java
package org.example;

public class Main {
    public static void main(String[] args) {
        System.out.println("".isEmpty()); // true
        System.out.println("hello".isEmpty()); // false
    }
}

```

#### 10、int length()

**返回字符串的长度**

```java
package org.example;

public class Main {
    public static void main(String[] args) {
        System.out.println("hello java yes good java".length()); // 24
    }
}

```

#### 11、int lastIndexOf(String str)

**判断某个子字符串在当前字符串中最后一次出现的索引**

```java
package org.example;

public class Main {
    public static void main(String[] args) {
        System.out.println("hello java yes good java".lastIndexOf("java")); // 20
    }
}

```

#### 12、String replace(CharSequence target, CharSequence replacement)

**String的父接口就是CharSequence；将字符串中的某个子字符串进行替换操作**

```java
package org.example;

public class Main {
    public static void main(String[] args) {
        String newString = "http://ganto.cn".replace("http://", "https://");
        System.out.println(newString);
    }
}

```

#### 13、String\[\] split(String regex)

**以某个字符或者字符串进行拆分字符串成字符串数组**

```java
package org.example;

public class Main {
    public static void main(String[] args) {
        String[] strArr = "2023-1-3".split("-");
        for(int i = 0; i < strArr.length; i++) {
            System.out.println(strArr[i]);
        }
    }
}

```

#### 14、boolean startsWith(String prefix)

**判断某个字符串是否以某个子字符串开始**

```java
package org.example;

public class Main {
    public static void main(String[] args) {
        System.out.println("https://ganto.cn".startsWith("https://")); // true
    }
}

```

#### 15、String substring(int beginIndex)

**截取字符串，参数为起始下标（包含），截取到最后**

```java
package org.example;

public class Main {
    public static void main(String[] args) {
        System.out.println("https://ganto.cn".substring(8)); // ganto.cn
    }
}

```

#### 16、String substring(int beginIndex, int endIndex)

**截取字符串，第一个参数是起始下标（包含），第二个参数为结束下标（不包含）。包前不包后，左闭右开**

```java
package org.example;

public class Main {
    public static void main(String[] args) {
        System.out.println("https://ganto.cn".substring(8, 13)); // ganto
    }
}

```

#### 17、char\[\] toCharArray()

**将字符串转换成char数组**

```java
package org.example;

public class Main {
    public static void main(String[] args) {
        char[] chars = "你好Java".toCharArray();
        for (int i = 0; i < chars.length; i++) {
            System.out.println(chars[i]);
            /**
             * 你
             * 好
             * J
             * a
             * v
             * a
             */
        }
    }
}

```

#### 18、String toLowerCase()

**全部转换成小写**

```java
package org.example;

public class Main {
    public static void main(String[] args) {
        System.out.println("JAVA".toLowerCase()); // java
    }
}

```

#### 19、String toUpperCase()

**全部转换成大写**

```java
package org.example;

public class Main {
    public static void main(String[] args) {
        System.out.println("java".toUpperCase()); // JAVA
    }
}

```

#### 20、String trim()

**去除字符串前后空白，中间的空白不会去除**

```java
package org.example;

public class Main {
    public static void main(String[] args) {
        System.out.println("   j  a  v a    ".trim());
    }
}

```

#### 21、String.valueOf()

**String中只有一个方法是静态的，不需要new对象，将非字符串转换成字符串**

```java
package org.example;

public class Main {
    public static void main(String[] args) {
        String s = String.valueOf(100); // 数字转换成字符串
        System.out.println(s);
    }
}

```

如果传入对象，System.out.println()打印的话，会自动调用对象的toString()方法，如果对象中没有重写toString()方法，则会调用父类Object中的toString()方法

```java
package org.example;

public class Main {
    public static void main(String[] args) {
        String s = String.valueOf(new Customer());
        System.out.println(s); // Customer{}
    }
}

class Customer{
    public String toString() {
        return "Customer{}";
    }
}

```

## StringBuffer

因为方法区字符串常量池的特性是一旦创建无法修改，会导致频繁拼字符串而占用大量的常量池空间，从而导致常量池浪费。

如果以后需要进行大量的字符串拼接操作，建议使用JDK中自带的：java.lang.StringBuffer、java.lang.StringBuilder

*StringBuffer*

```java
package org.example;

public class Main {
    public static void main(String[] args) {
        /**
         * StringBuffer默认初始化容量是16个byte[]数组
         * 可以手动初始化容量 new StringBuffer(50);
         */
        StringBuffer stringBuffer = new StringBuffer();
        stringBuffer.append("j");
        stringBuffer.append("a");
        stringBuffer.append("v");
        stringBuffer.append("a");
        System.out.println(stringBuffer);
    }
}

```

*StringBuilder*

```java
package org.example;

public class Main {
    public static void main(String[] args) {
        StringBuilder stringBuilder = new StringBuilder();
        stringBuilder.append("j");
        stringBuilder.append("a");
        stringBuilder.append("v");
        stringBuilder.append("a");
        System.out.println(stringBuilder);
    }
}

```

> ***StringBuffer***和***StringBuilder***的区别：
> 
> ***StringBuffer***中的方法都有：***synchronized***关键字修饰。表示***StringBuffer***在多线程环境下运行是安全的。
> 
> ***StringBuilder***中的方法都没有：***synchronized***关键字修饰。表示***StringBuilder***在多线程环境下运行是不安全的。
> 
> ***StringBuffer***是线程安全的。
> 
> ***StringBuilder***是非线程安全的。

## 8个基本数据类型对应的8个包装类

| 数据类型 | 默认值 |
| --- | --- |
| byte | 0 |
| short | 0 |
| int | 0 |
| long | 0L |
| float | 0.0F |
| double | 0.0 |
| boolean | false |
| char | \\u0000 |
| 引用数据类型 | null |

Java中为8种基本数据类型又对应准备了8种包装类型。8装包装类属于引用数据类型，父类是Object。

引：有一种需求，如下代码，调用doSome()方法的时候需要传入一个数字进去。但是数字属于基本数据类型，而doSome()方法参数的类型是Object。可见doSome()方法无法接受基本数据类型的参数。可以传入一个基本数据类型对应的包装类进去，来解决这个问题。

Main.java

```java
package org.example;

public class Main {
    public static void main(String[] args) {
        MyInt myInt = new MyInt(100);
        doSome(myInt);
    }

    public static void doSome(Object obj) {
        System.out.println(obj);
    }
}

```

MyInt.java

```java
package org.example;

public class MyInt {
    int value;
    public MyInt() {

    }
    public MyInt(int value) {
        this.value = value;
    }

    @Override
    public String toString() {
        return String.valueOf(value);
    }
}

```

以上代码只是来通过自己写的包装类来演示这个过程。8个基本包装类并不需要我们自己手动写，因为官方已经为我们写好，我们只需要调用即可。

| 数据类型 | 默认值 | 包装类型 | 父类 |
| --- | --- | --- | --- |
| byte | 0 | java.lang.Byte | java.lang.Number |
| short | 0 | java.lang.Short | java.lang.Number |
| int | 0 | java.lang.Integer | java.lang.Number |
| long | 0L | java.lang.Long | java.lang.Number |
| float | 0.0F | java.lang.Float | java.lang.Number |
| double | 0.0 | java.lang.Double | java.lang.Number |
| boolean | false | java.lang.Boolean | java.lang.Object |
| char | \\u0000 | java.lang.Character | java.lang.Object |

以java.lang.Integer为例：

```java
package org.example;

public class Main {
    public static void main(String[] args) {
        /**
         * 123这个基本数据类型，进行构造方法的包装达到了：基本数据类型向引用数据类型的转换。
         * 基本数据类型 -（转换为）-> 引用数据类型（装箱）
         */
        Integer integer = new Integer(123);

        // 将引用数据类型 -（转换为）-> 基本数据类型（拆箱）
        float f = integer .floatValue();
        System.out.println(f);

        // 将引用数据类型 -（转换为）-> 基本数据类型（拆箱）
        int i = integer.intValue();
        System.out.println(i);
    }
}

```

Integer类、Byte类的最大值、最小值

```java
package org.example;

public class Main {
    public static void main(String[] args) {
        System.out.println(Integer.MIN_VALUE);
        System.out.println(Integer.MAX_VALUE);
        System.out.println(Byte.MIN_VALUE);
        System.out.println(Byte.MAX_VALUE);
    }
}

```

### 在JDK1.5之后，支持自动装箱和自动拆箱。

自动装箱：基本数据类型自动转换成包装类。

自动拆箱：包装类自动转换成基本数据类型。

```java
package org.example;

public class Main {
    public static void main(String[] args) {
        Integer integer = 100; // 自动装箱
        int i = integer; // 自动拆箱
        System.out.println(i);
    }
}

```

```java
package org.example;

public class Main {
    public static void main(String[] args) {
        Integer a = 128;
        Integer b = 128;
        System.out.println(a == b); // false

        Integer x = 127;
        Integer y = 127;
        System.out.println(x == y); // true
    }
}

```

以上代码的说明：

> Java中为了提高程序的执行效率，将\[-128,127)之间所有的包装对象提前创建好，
> 
> 放到了一个方法区的 “整数型常量池” 当中了，目的是只要用这个区间的数据，则不需要再new对象了，直接从 “整数型常量池” 中取出来。
> 
> 原理：x变量中保存的对象的内存地址和y变量中保存的对象的内存地址是一样的。

### Integer常用方法

#### int intValue()

**以 int 类型返回该 Integer 的值**

```java
package org.example;

public class Main {
    public static void main(String[] args) {
        Integer integer = new Integer("123");
        int i = integer.intValue();
        System.out.println(i);
    }
}

```

#### static int parseInt(String s)

**将字符串参数作为有符号的十进制整数进行解析**

```java
package org.example;

public class Main {
    public static void main(String[] args) {
        int i = Integer.parseInt("123");
        System.out.println(i + 199);

        double d = Double.parseDouble("3.1415");
        System.out.println(d + 1);

        float f = Float.parseFloat("2.0");
        System.out.println(f + 1);
    }
}

```

## String int Integer之间的互相转换

```java
package org.example;

public class Main {
    public static void main(String[] args) {
        // String => int
        String s = "123";
        int i = Integer.parseInt("123");

        // int => String
        System.out.println(123 + "");

        System.out.println(String.valueOf(123));

        // int => Integer
        Integer integer = new Integer(123);
        System.out.println(integer);

        Integer integer1 = 123;
        System.out.println(integer1);

        // Integer => int
        Integer integer2 = 123;
        int i1 = integer2;
        System.out.println(i1);

        // String => Integer
        String s1 = "123";
        Integer integer3 = Integer.valueOf(s1);

        // Integer => String
        String s2 = String.valueOf(new Integer("123"));
    }
}

```

## Java对日期的处理

java.util.Date类的toString()方法已经被官方重写了，输出的是一个日期字符串，但是不符合中国人的阅读习惯~

```java
package org.example;

import java.util.Date;

public class Main {
    public static void main(String[] args) {
        Date nowTime = new Date();
        System.out.println(nowTime);
    }
}

```

### java.text.SimpleDateFormat

### format()

Date => String

如果需要将日期格式化使用`SimpleDateFormat`

```java
package org.example;

import java.text.SimpleDateFormat;
import java.util.Date;

public class Main {
    public static void main(String[] args) {
        Date nowTime = new Date();
        System.out.println(nowTime);

        /**
         * yyyy 年 是四位
         * MM 月 是两位
         * dd 日 是两位
         * HH 时 是两位
         * mm 分 是两位
         * ss 秒 是两位
         * SSS 毫秒 是三位
         */
        SimpleDateFormat sdf = new SimpleDateFormat("yyyy-MM-dd HH:mm:ss:SSS");
        System.out.println(sdf.format(nowTime));
    }
}

```

### parse()

String => Date

日期字符串String转换成Date类型

```java
package org.example;

import java.text.SimpleDateFormat;

public class Main {
    public static void main(String[] args) throws Exception {
        String time = "2023-01-05 00:00:00:000";
        // 以下格式要与字符串日期格式一致
        SimpleDateFormat sdf = new SimpleDateFormat("yyyy-MM-dd HH:mm:ss:SSS");
        System.out.println(sdf.parse(time));
    }
}

```

## 数字格式化

java.text.DecimalFormat专门负责数字格式化

DecimalFormat df = new DecimalFormat("数字格式");

数字格式：`# 代表任意数字` `, 代表千分位` `. 小数位` `0 不够位数自动补0`

例子

```java
package org.example;

import java.text.DecimalFormat;

public class Main {
    public static void main(String[] args) throws Exception {
        DecimalFormat df = new DecimalFormat("###,###.##");
        String s = df.format(1234.567);
        System.out.println(s); // 1,234.56
    }
}

```

```java
package org.example;

import java.text.DecimalFormat;

public class Main {
    public static void main(String[] args) throws Exception {
        DecimalFormat df = new DecimalFormat("###,###.0000");
        String s = df.format(1234.567);
        System.out.println(s); // 1,234.5670
    }
}

```

## BigDecimal

java.math.BigDecimal

BigDecimal属于大数据，精度极高。不属于基本数据类型，属于java对象（引用数据类型），这是官方提供的一个类。专门用在财务软件当中。

注意：财务软件中double是不够用的。

```java
package org.example;

import java.math.BigDecimal;

public class Main {
    public static void main(String[] args) throws Exception {
        BigDecimal v1 = new BigDecimal(100);
        BigDecimal v2 = new BigDecimal(200);
        BigDecimal v3 = v1.add(v2);
        System.out.println(v3);
    }
}

```

如上例代码中所示，声明了两个精度极高的引用数据类型的数据，两者如果需要相加不能像基本数据类型那样相加，要通过.add()方法进行相加。

* * *

add() 加法

subtract() 减法

multiply() 乘法

divide() 除法

* * *

## 随机数

java.util.Random

nextInt()内不传入参数，则表示产生int类型范围内的随机数。

```java
package org.example;

import java.util.Random;

public class Main {
    public static void main(String[] args) throws Exception {
        Random random = new Random();
        int num = random.nextInt();
        System.out.println(num);
    }
}

```

nextInt(101)内传入参数，则表示产生\[0, 101)之间的随机数整数，也就是0-100之间的整数

```java
package org.example;

import java.util.Random;

public class Main {
    public static void main(String[] args) throws Exception {
        Random random = new Random();
        int num = random.nextInt(101);
        System.out.println(num);
    }
}

```

案例：产生5个不同的随机数放进数组中

```java
package org.example;

import java.util.Random;

public class Main {
    public static void main(String[] args) throws Exception {
        int[] arr = new int[5];

        Random random = new Random();
        int num;

        int count = 0;


        while (count < 5) {
            num = random.nextInt(5);
            int count1 = 0;
            if(count == 0) {
                arr[count] = num;
                count++;
            }
            for (int i = 0; i < count; i++) {
                if(num == arr[i]) {
                    count1++;
                }
            }
            if(count1 == 0) {
                arr[count] = num;
                count++;
            }
        }
        for (int i = 0; i < arr.length; i++) {
            System.out.println(arr[i]);
        }
    }
}

```

## enum枚举

> 枚举也是一种引用数据类型。
> 
> 枚举中的每一个值，可以看做是“常量”，即使用全大写。
> 
> 如果只有两种情况可以使用布尔类型，多于两种情况，可以使用枚举。

> ```java
> enum Result{
>   SUCCESS, FAIL
> }
> 
> ```

例子：

Main.java

```java
package org.example;

public class Main {
    public static void main(String[] args) throws Exception {
        Result returnNum = divide(1, 2);
        System.out.println(returnNum == Result.SUCCESS ? "计算成功" : "计算失败");
    }

    /*
    enum Result{
        SUCCESS, FAIL
    }
    */
    public static Result divide(int a, int b) {
        try {
            int c = a / b;
            return Result.SUCCESS;
        } catch (Exception e) {
            return Result.FAIL;
        }
    }
}

```

Result.java

```java
package org.example;

public enum Result {
    SUCCESS, FAIL
}


```

# 异常处理

Java中异常以类和对象形式存在

```java
package org.example;

public class Main {
    public static void main(String[] args) throws Exception {
        NumberFormatException nfe = new NumberFormatException("数字格式化异常！");
        System.out.println(nfe);

        NullPointerException npe = new NullPointerException("空指针异常！");
        System.out.println(npe);
    }
}

```

在以下代码中，如果除数为0，Java虚拟机则会自动new出来异常对象：`ArithmeticException("/ by zero")`。

代码：

```java
package org.example;

public class Main {
    public static void main(String[] args) throws Exception {
        int a = 10;
        int b = 0;
        int c = a / b; // new ArithmeticException()
        System.out.println(a + " / " + b + " = " + c);
    }
}

```

异常打印：

```x86asm
Exception in thread "main" java.lang.ArithmeticException: / by zero
	at org.example.Main.main(Main.java:7)

```

所有的异常都是发生在运行阶段的。

代码：

```java
package org.example;

public class Main {
    public static void main(String[] args) {
        // main方法中调用doSome方法
        // 因为doSome方法声明位置上有：throws ClassNotFoundException
        // 我们在调用doSome方法的时候，必须对这种异常进行预先的处理，如果不处理则编译器就报错
        // 直接调用doSome，编译器会报错：未处理 异常: java.lang.ClassNotFoundException
        doSome();
    }

    /**
     * doSome方法在方法声明的位置上使用了：throws ClassNotFoundException
     * 这个代码表示doSome方法在执行过程中，有可能会出现ClassNotFoundException异常
     * 叫做类没找到异常。这个异常直接父类是：Exception，所以ClassNotFoundException属于编译时异常。
     * @throws ClassNotFoundException
     */
    public static void doSome() throws ClassNotFoundException{
        System.out.println("doSome!!!");
    }
}

```

第一种处理方法：在方法声明的位置上继续使用`throws`，来完成异常的继续上抛，抛给调用者。

```java
package org.example;

public class Main {
    public static void main(String[] args) throws ClassNotFoundException {
        doSome();
    }

    public static void doSome() throws ClassNotFoundException{
        System.out.println("doSome!!!");
    }
}

```

第二种处理方法：try...catch进行捕捉

```java
package org.example;

public class Main {
    public static void main(String[] args) {
        try {
            doSome();
        } catch (ClassNotFoundException e) {
						// throw new RuntimeException(e);
            e.printStackTrace();
        }
    }

    public static void doSome() throws ClassNotFoundException{
        System.out.println("doSome!!!");
    }
}

```

## 上抛

处理异常的第一种方式：在方法声明的位置上使用`throws`关键字抛出，谁调用我这个方法，我就抛给谁。抛给调用者来处理。

```java
package org.example;

import java.io.FileInputStream;
import java.io.FileNotFoundException;
import java.io.IOException;

public class Main {
    public static void main(String[] args) throws IOException {
        System.out.println("main begin");
        m1();
        System.out.println("main over");
    }
    // 上抛,可以抛相同的异常,也可以抛父类异常
    public static void m1() throws IOException {
        System.out.println("m1 begin");
        m2();
        System.out.println("m1 over");
    }
    // 异常可以上抛多个
    private static void m2() throws ClassCastException, FileNotFoundException {
        System.out.println("m2 begin");
        m3();
        System.out.println("m2 over");
    }
    private static void m3() throws FileNotFoundException {
        // 调用SUN JDK中某个类的构造方法
        // 这个类暂时还没接触过，后期I/O流的时候就知道了。
        // 我们只是借助这个类学习一下异常处理机制。

        // 创建一个输入流对象，该流指向一个文件
        new FileInputStream("C:\\Users\\ganto\\Desktop\\test.txt");
    }
}

```

## 捕捉

```java
package org.example;

import java.io.FileInputStream;
import java.io.FileNotFoundException;
import java.io.IOException;

public class Main {
    public static void main(String[] args) {
        System.out.println("main begin");
        try {
            m1();
        } catch (IOException e) {
            throw new RuntimeException(e);
        }
        System.out.println("main over");
    }
    // 上抛,可以抛相同的异常,也可以抛父类异常
    public static void m1() throws IOException {
        System.out.println("m1 begin");
        m2();
        System.out.println("m1 over");
    }
    // 异常可以上抛多个
    private static void m2() throws ClassCastException, FileNotFoundException {
        System.out.println("m2 begin");
        m3();
        System.out.println("m2 over");
    }
    private static void m3() throws FileNotFoundException {
        // 调用SUN JDK中某个类的构造方法
        // 这个类暂时还没接触过，后期I/O流的时候就知道了。
        // 我们只是借助这个类学习一下异常处理机制。

        // 创建一个输入流对象，该流指向一个文件
        new FileInputStream("C:\\Users\\ganto\\Desktop\\test.txt");
    }
}

```

深入try...catch：

> 1、catch后面的小括号中的类型可以是具体的异常类型，也可以是该异常类型的夫类型。
> 
> 2、catch可以写多个。建议catch的时候，精确的一个一个处理。这样有利于程序的调试。
> 
> 3、catch写多个的时候，从上到下必须遵循从小到大的原则。

JDK8的新特性，可以用`|` (或) 的符号进行捕获多个异常

```java
try{
  ...
} catch(FileNotFoundException | ArithmeticException | NullPointerException e) {
  ...
}

```

## 异常对象的常用方法

异常对象有两个非常重要的方法：

*   获取异常简单的描述信息：String msg = exception.getMessage();
*   打印异常追踪的堆栈信息：exception.printStackTrace();

```java
package org.example;

public class Main {
    public static void main(String[] args) {
        NullPointerException e = new NullPointerException("空指针异常");
        String msg = e.getMessage();
        System.out.println(msg);

        // java后台打印异常堆栈追踪信息的时候，采用了异步线程的方式打印的。
        e.printStackTrace(); // 在实际开发中，建议使用这行代码，来打印异常信息。养成好习惯！

        System.out.println("=========");
    }
}

```

异常追踪信息的查看方式：异常追踪信息，从上往下一行一行的看；SUN公司官方代码不用看，可以直接跳过，因为官方的代码不会有问题；主要看自己编写的代码出现什么问题；看自己的代码报错的第一行，因为往往是第一个异常，导致的后续的异常。

## finally

*   在finally子句中的代码是最后执行的，并且是一定会执行的，即使try语句块中的代码出现了异常。finally子句必须和try一起出现，不能单独编写。

```java
package org.example;

import java.io.FileInputStream;
import java.io.FileNotFoundException;
import java.io.IOException;

public class Main {
    public static void main(String[] args) {
        FileInputStream fis = null;
        try {
            fis = new FileInputStream("C:\\Users\\ganto\\Desktop\\test.txt");

            String s = null;
            s.toString(); // 这里一定会出现空指针异常！

            // 流使用完需要关闭，因为流是占用资源的。
            // 即使以上代码出现了异常，流也需要被关闭！
            // 放在这里，有可能会因为上方代码异常，导致流无法被执行，从而无法关闭流。
            // 所以以下两行代码写在finally子句中比较好
            // System.out.println("关闭流");
            // fis.close();
        } catch (FileNotFoundException e) {
            e.printStackTrace();
        } catch (IOException e) {
            e.printStackTrace();
        } catch (NullPointerException e) {
            e.printStackTrace();
        } finally {
            // 这里需要判断fis不为空，避免空指针异常
            if(fis != null) {
                try {
                    System.out.println("关闭流");
                    fis.close();
                } catch (IOException e) {
                    e.printStackTrace();
                }
            }
        }

        System.out.println("Over");
    }
}

```

以下代码执行顺序是：先try然后finally最后return。所以以下代码中的finally中的语句会执行。

```java
package org.example;

public class Main {
    public static void main(String[] args) {
        try {
            System.out.println("try...");
            return;
        } finally {
            System.out.println("finally...");
        }
    }
}

```

以下代码中的finally中的语句则不会执行，因为java虚拟机退出会中断finally语句

```java
package org.example;

public class Main {
    public static void main(String[] args) {
        try {
            System.out.println("try...");
            System.exit(0);
        } finally {
            System.out.println("finally...");
        }
    }
}

```

## final finally finalize 之间的区别

### final

final是一个关键字，用于修饰常量、方法、类。[详情](#final%E5%85%B3%E9%94%AE%E5%AD%97)

```java
package org.example;

public class Main {
    public static void main(String[] args) {
        final int i = 10;
        i = 11; // 报错 final修饰的变量无法被修改
    }
}

```

### finally

finally也是一个关键字，和try联合使用，使用在异常处理机制中，在finally语句块中的代码是一定会被执行的

```java
try{
  ...
} finally {
  System.out.printlnj("一定会执行到这里");
}

```

### finalize

finalize()是Object类中的一个方法.作为方法名出现。所以finalize是标识符。

## 自定义异常

SUN官方提供的JDK内置的异常肯定是不够用的。在实际开发中，有很多业务，这些业务出现异常之后，JDK中都是没有的。和业务挂钩的异常程序员可以自己定义。

自定义异常：

*   第一步：编写一个类继承Exception或者RuntimeException
*   第二步：提供两个构造方法，一个无参数的，一个带有String参数的

```java
package org.example;

public class MyException extends Exception{ // 编译时异常
    public MyException() {

    }
    public MyException(String s) {
        super(s);
    }
}

// 或者

//public class MyException extends RuntimeException{ // 运行时异常
//    public MyException() {
//
//    }
//    public MyException(String s) {
//        super(s);
//    }
//}


```

测试自定义异常类：

```java
package org.example;

public class Main {
    public static void main(String[] args) {
        // 创建异常对象（只new了异常对象，并没有手动抛出）
        MyException e = new MyException("用户名不能为空！");
        // 打印异常堆栈信息
        e.printStackTrace();

        String msg = e.getMessage();
        System.out.println(msg);
    }
}

```

## 自定义异常在实际开发中的使用：

MyStackOperationException.java

```java
package org.example;

/**
 * 栈操作异常
 */
public class MyStackOperationException extends Exception {
    public MyStackOperationException() {
    }
    public MyStackOperationException(String s) {
        super(s);
    }
}


```

MyStack.java

```java
package org.example;

public class MyStack {
    private Object[] elements;

    // 栈帧，永远指向栈顶部元素
    // 默认初始值是0，最初的栈是空的，一个元素都没有
    private int index;

    public MyStack() {
        this.elements = new Object[10];
        this.index = -1;
    }

    public Object[] getElements() {
        return elements;
    }

    public void setElements(Object[] elements) {
        this.elements = elements;
    }

    public int getIndex() {
        return index;
    }

    public void setIndex(int index) {
        this.index = index;
    }

    // 压栈方法push
    public void push(Object o) throws MyStackOperationException {
        if(this.index >= this.elements.length-1) {
            // System.out.println("栈满！压栈失败！");
            // return;

            // 创建异常对象
            MyStackOperationException e = new MyStackOperationException("栈满！压栈失败！");
            // 手动将异常抛出去
            throw e; // 这里如果使用捕捉的话,没有实际意义 栈已满这个信息需要传递出去,也就是这里使用上抛

            // 上方两行代码可以合并成一条
            // throw new MyStackOperationException("栈满！压栈失败！");
        }
        this.index++;
        this.elements[this.index] = o;
        // 所有System.out.println()方法执行时，如果输出引用的话，会自动调用引用的toString()方法
        // System.out.println("压栈" + o.toString() + "元素成功，栈帧指向" + this.index);
        System.out.println("压栈" + o + "元素成功，栈帧指向" + this.index);
    }

    // 弹栈方法pop
    public Object pop() throws MyStackOperationException {
        if(this.index < 0) {
            // System.out.println("栈空！弹栈失败！");
            // return null;

             throw new MyStackOperationException("栈空！弹栈失败！");
        }
        System.out.print("弹栈" + this.elements[this.index] + "元素成功，");
        this.elements[this.index] = null;
        this.index--;
        System.out.println("栈帧指向" + this.index);
        return null;
    }
}

```

Main.java

```java
package org.example;

public class Main {
    public static void main(String[] args) {
        // 创建异常对象（只new了异常对象，并没有手动抛出）
        MyStack stack = new MyStack();
        try {
            stack.push(new Animal());
        } catch (MyStackOperationException e) {
            e.printStackTrace();
            System.out.println(e.getMessage());
        }

        try {
            stack.pop();
            stack.pop();
        } catch (MyStackOperationException e) {
            e.printStackTrace();
            System.out.println(e.getMessage());
        }
    }
}

class Animal{
}

```

## 异常与方法的覆盖

之前在讲解方法覆盖的时候，当时遗留了一个问题？

*   重写之后的方法*不能*比重写之前的方法抛出更多（更宽泛）的异常，可以更少（范围更小）。

## 总结

总结异常中的关键字：

 异常捕获：

 try

 catch

 finally

 throws 在方法声明位置上使用，表示上报异常信息给调用者

 throw 手动抛出异常

```java
package org.example;

class Animal {
    public void doSome() {

    }

    public void doOther() throws Exception {

    }
}

class Cat extends Animal {

    // 特殊情况，父类方法没有抛出任何异常，但是子类重写方法可以抛出RuntimeException异常
    // 编译正常
    public void doSome() throws RuntimeException {

    }

    // 重写的doSome方法抛出了更多的异常，所以会编译报错
//    public void doSome() throws Exception {
//
//    }

    // 重写的doOther方法抛出了更少的异常，所以编译正常
//    public void doOther() {
//
//    }

    // 重写的doOther方法抛出了相同的异常，所以编译正常
//    public void doOther() throws Exception {
//
//    }
}

```

在实际开发中，父类方法抛出什么异常，子类重写的方法也抛出一样的异常。

## 异常作业

IllegalNameException.java

```java
 package com.ganto.homework;
 /**
  * 自定义异常
  */
 public class IllegalNameException extends Exception {
     public IllegalNameException() {
         
     }
     public IllegalNameException(String s) {
         super(s);
     }
 }

```

UserService.java

```java
 package com.ganto.homework;
 /**
  * 用户业务类，处理用户相关的业务：例如登录、注册等功能。
  */
 public class UserService throws IllegalNameException {
     /**
      * 用户注册
      * @param username 用户名
      * @param password 密码
      * @throws IllegalNameException 当前用户名为null，或者用户名长度小于6，或者用户名长度大于14，会出现该异常！
      */
     public void register(String username, String password) {
         // 引用等于null的这个判断，最好放到所有条件的最前面。避免空指针异常的发生。
         // 尽可能将null放在前面，写成"null == username"，避免手误将其写成"username = null"，这样会将username引用赋值为空，导致程序出错误。
         if(null == username || username.length() < 6 || username.length() > 14) {
             throw new IllegalNameException("用户名不合法，长度必须在6-14位之间");
         }
         // 程序能执行到此处，说明用户名合法
         System.out.println("注册成功，欢迎["+ username +"]");
     }
 }

```

测试类Test.java

```java
 package com.ganto.homework;
 
 public class Test {
     public static void main(String[] args) {
         // 创建UserService对象
         UserService userService = new UserService();
         // 用户名和密码就不再从控制台接收了
         try {
             userService.register("gantooo", "123");
         } catch (IllegalNameException e) {
             System.out.println(e.getMessage());
         }
     }
 }

```

## 其他作业

Moveable.java

```java
package org.example.homework;

/**
 * 可移动的接口
 */
public interface Moveable {
    /**
     * 移动的方法（移动行为）
     */
    void move();
}


```

Shootable.java

```java
package org.example.homework;

/**
 * 可射击的接口
 */
public interface Shootable {
    /**
     * 射击的方法（射击行为）
     */
    void shoot();
}


```

Weapon.java

```java
package org.example.homework;

/**
 * 所有武器的父类（武器类）
 */
public class Weapon {

    public String toString() {
        return "";
    }
}

```

Tank.java

```java
package org.example.homework;

/**
 * 坦克是一个武器，可移动，可射击。
 */
public class Tank extends Weapon implements Moveable, Shootable {
    public void move() {
        System.out.println("坦克移动");
    }
    public void shoot() {
        System.out.println("坦克开炮");
    }

    @Override
    public String toString() {
        return "坦克";
    }
}

```

GaoShePao.java

```java
package org.example.homework;

/**
 * 高射炮是一个武器，不可移动，可射击。
 */
public class GaoShePao extends Weapon implements Shootable {
    public void shoot() {
        System.out.println("高射炮开炮");
    }

    @Override
    public String toString() {
        return "高射炮";
    }
}

```

Fighter.java

```java
package org.example.homework;

/**
 * 战斗机是一个武器，可移动，可射击。
 */
public class Fighter extends Weapon implements Moveable, Shootable {
    public void move() {
        System.out.println("战斗机起飞");
    }
    public void shoot() {
        System.out.println("战斗机开炮");
    }

    @Override
    public String toString() {
        return "战斗机";
    }
}

```

WuZiFeiJi.java

```java
package org.example.homework;

/**
 * 物资飞机是一个武器，可移动，不可射击。
 */
public class WuZiFeiJi extends Weapon implements Moveable {
    public void move() {
        System.out.println("物资飞机起飞");
    }

    @Override
    public String toString() {
        return "物资飞机";
    }
}

```

Army.java

```java
package org.example.homework;

/**
 * 军队
 */
public class Army {
    // 武器数组
    private Weapon[] weapons;
    /**
     * 创建军队的构造方法。
     * @param count 武器数量
     */
    public Army(int count) {
        // 动态初始化数组中的每一个元素，默认值是null
        // 武器数组是有了，但是武器数组中没有放武器
        weapons = new Weapon[count];
    }
    /**
     * 将武器加入数组
     * @param weapon
     */
    public void addWeapon(Weapon weapon) throws AddWeaponException {
        for(int i = 0; i < weapons.length; i++) {
            if(null == weapons[i]) {
                weapons[i] = weapon;
                System.out.println(weapon + "武器，添加成功！");
                return;
            }
        }
        // 程序如果执行到此处，说明武器没有添加成功
        throw new AddWeaponException("武器已经达到上限");
    }
    /**
     * 所有可攻击的武器攻击。
     */
    public void attackAll() {
        // 遍历数组
        for (int i = 0; i < weapons.length; i++) {
            if(weapons[i] instanceof Shootable) {
                // 调用子类中特有的方法，向下转型。
                Shootable shootable = (Shootable) weapons[i];
                shootable.shoot();
            }
        }
    }
    /**
     * 所有可移动的武器移动
     */
    public void moveAll() {
        // 遍历数组
        for (int i = 0; i < weapons.length; i++) {
            if(weapons[i] instanceof Moveable) {
                // 调用子类中特有的方法，向下转型。
                Moveable moveable = (Moveable) weapons[i];
                moveable.move();
            }
        }
    }
}

```

> 注意：以上代码中，在父类型转换成接口类型的时候，父类和接口不需要存在继承关系，Java语法可以直接转换成接口类型，但一定要先判断该类是该接口`instanceof`。

AddWeaponException.java

```java
package org.example.homework;

/**
 * 添加武器异常
 */
public class AddWeaponException extends Exception {
    public AddWeaponException() {

    }
    public AddWeaponException(String s) {
        super(s);
    }
}

```

测试类Test.java

```java
package org.example.homework;

/**
 * 测试程序Test
 */
public class Test {
    public static void main(String[] args) {
        // 构建一个军队
        Army army = new Army(4); // 军队只有四个武器
        // 创建武器对象
        Fighter fighter = new Fighter();
        Tank tank = new Tank();
        WuZiFeiJi wuZiFeiJi = new WuZiFeiJi();
        GaoShePao gaoShePao = new GaoShePao();
        GaoShePao gaoShePao2 = new GaoShePao();
        // 添加武器
        try {
            army.addWeapon(fighter);
            army.addWeapon(tank);
            army.addWeapon(wuZiFeiJi);
            army.addWeapon(gaoShePao);
            army.addWeapon(gaoShePao2);
        } catch (AddWeaponException e) {
            System.out.println(e.getMessage());
        }

        // 让所有可移动的移动
        army.moveAll();

        // 让所有可攻击的攻击
        army.attackAll();
    }
}


```

# 集合

## 集合概述

1.1、数组其实就是一个集合。集合实际上就是一个容器。可以来容纳其他类型的数据。

集合为什么说在开发中使用较多？

 集合是一个容器，是一个载体，可以一次容纳多个对象。

 在实际开发中，假设链接数据库，数据库当中有10条记录。

 那么假设把这10条记录查询出来，在java程序中会将10条数据封装成10个java对象，

 然后将10个java对象放到某一个集合当中，将集合传到前端，然后遍历集合，

 将数据一个一个的展示出来。

1.2、集合不能直接存储基本数据类型，另外集合也不能直接存储java对象，集合当中存储的都是java对象的内存地址。（或者说集合中存储的引用。）

 集合在java中本事是一个容器，是一个对象。

 集合中任何时候存储的都是引用。

1.3、在java中每一个不同的集合，底层会对应不同的数据结构。往不同的集合中存储元素，等于将数据放到了不同的数据结构当中。数据存储的结构就是数据结构。不同的数据结构，数据存储方式不同。

 例如：数组、二叉树、链表、哈希表...

 以上这些都是常见的数据结构。

你往集合c1中放数据，可能是放到了数组上了。

你往集合c2中放数据，可能是放到了二叉树上了。

...

使用不同的集合等同于使用了不同的数据结构。

1.4、集合在java JDK中的`java.util.*`包下。

所有的集合类的集合接口都在`java.util`包下。

1.5、集合的继承结构图

集合整个这个体系是怎样的一个结构，需要有印象。

1.6、在java中集合分为两大类：

*   一类是单个方式存储元素：单个凡是存储元素，这一类集合中超级父接口：java:util.Collection;
    
*   一类是以键值对的方式存储元素：以键值对的方式存储元素，这一类集合中超级父接口：java.util.Map;
    

## Collection

```java
package com.ganto;

import java.util.ArrayList;
import java.util.Collection;

/**
 * 关于java.util.Collection接口中常用的方法
 *
 * 1.Collection中能存放什么元素？
 * - 没有使用“泛型”之前，Collection中可以存储Object的所有子类型。
 * - 使用了“泛型”之后，Collection中只能存储某个具体的类型。
 * - 集合后期会学习“泛型”语法。目前先不用管。Collection中什么都能村。
 * - 只要是Object的子类型就行。（集合中不能直接存储基本数据类型，也不能存java对象，只能存储java对象的内存地址）
 *
 * 2.Collection中的常用方法
 * - boolean add(Object e) 向集合中添加元素
 * - int size() 获取集合中元素的个数
 * - void clear() 清空集合
 * - boolean contains(Object o) 判断当前集合中是否包含元素o，包含返回true，不包含返回false
 * - boolean remove(Object o) 删除集合中的某个元素
 * - boolean isEmpty() 判断该集合中元素的个数是否为0
 * - Object[] toArray() 调用这个方法可以把集合转换成数组
 *
 */
public class CollectionTest01 {
    public static void main(String[] args) {
        // 创建一个集合对象
        // Collection c = new Collection(); // 接口是抽象的，无法实例化对象
        // 多态
        Collection c = new ArrayList();
        // 测试Collection接口中的常用方法
        c.add(1200); // 自动装箱，实际上是放进去了一个对象的内存地址。Integer x = new Integer();
        c.add(3.14);
        c.add(new Object());
        c.add(new Stu());
        c.add(true);

        // 获取集合中元素的个数
        System.out.println("集合中元素的个数：" + c.size());

        // 清空集合
        c.clear();
        System.out.println("集合中元素的个数：" + c.size());

        // 判断集合中是否包含”绿巨人“
        c.add("绿巨人");
        boolean flag = c.contains("绿巨人");
        System.out.println(flag);
        System.out.println("集合中元素的个数：" + c.size());

        // 删除集合中某个元素
        c.remove("绿巨人");
        System.out.println("集合中元素的个数：" + c.size());

        // 判断该集合中元素的个数是否为0
        System.out.println(c.isEmpty());

        // 集合转换成数组
        c.add("嘻嘻");
        c.add("哈哈");
        c.add("嘿嘿");
        c.add("羞羞");
        c.add("呲呲");
        Object[] arr = c.toArray();
        for (int i = 0; i < arr.length; i++) {
            System.out.println(arr[i]);
        }
    }
}

class Stu {

}

```

### boolean contains(Object o)

判断当前集合中是否包含元素o，包含返回true，不包含返回false

```java
package com.ganto;

import java.util.ArrayList;
import java.util.Collection;

/**
 * 深入Collection集合的contains方法
 */
public class Main {
    public static void main(String[] args) {
        // 创建集合对象
        Collection c = new ArrayList();

        // 向集合中存储元素
        String s1 = new String("abc");
        c.add(s1);
        String s2 = new String("def");
        c.add(s2);

        // 集合中元素的个数
        System.out.println(c.size());

        String x = new String("abc");
        System.out.println(c.contains(x)); // true
    }
}

```

以上代码中的contains会返回true，因为contains源码是使用String的equals进行比较的，而String的equals是官方重写过的，只比较内容，不比较内存地址。

根据以上内容，可以得出结论：放在集合里的元素需要重写equals方法

### boolean remove(Object o)

删除集合中的某个元素

```java
package com.ganto;

import java.util.ArrayList;
import java.util.Collection;

/**
 * 深入Collection集合的contains方法
 */
public class Main {
    public static void main(String[] args) {
        // 创建集合对象
        Collection c = new ArrayList();

        // 创建字符串对象
        String s1 = new String("abc");
        // 存储s1
        c.add(s1);
        // 创建一个新的字符串对象
        String x = new String("abc");
        // 删除x
        c.remove(x);

        // 集合中元素的个数
        System.out.println(c.size()); // 0

    }
}

```

以上代码打印为0，因为remove依然是调用equals方法，而s1.equals(x)是返回true的。

## 集合遍历（迭代）

![](images%5CJava%5C%E8%BF%AD%E4%BB%A3%E5%99%A8%E6%89%A7%E8%A1%8C%E5%8E%9F%E7%90%86.png)

```java
package com.ganto;

import java.util.ArrayList;
import java.util.Collection;
import java.util.Iterator;

/**
 * 关于集合遍历（迭代）专题 ※※※※※
 */
public class Main {
    public static void main(String[] args) {
        // 注意：以下讲解的遍历方式/迭代方式，是所有Collection通用的一种方式。
        // 在Map集合中不能用。在所有的Collection以及子类中使用。

        // 创建集合对象
        Collection c = new ArrayList(); // 后面的集合无所有，主要是看前面的Collection接口怎么遍历/迭代
        // 添加元素
        c.add("abc");
        c.add("def");
        c.add(122);
        c.add(new Object());
        // 对集合Collection进行遍历/迭代
        // 第一步：获取集合对象的迭代器对象Iterator
        Iterator it = c.iterator();
        // 第二部：通过以上获取的迭代器对象开始遍历/迭代集合。
        /**
         * 以下两个方法是迭代器对象Iterator中的方法：
         * - boolean hasNext() 如果仍有元素可以迭代，则返回true
         * - Object next() 返回迭代的下一个元素
         */

//        boolean hasNext = it.hasNext();
//        if(hasNext) {
//            // 不管存进去的是什么，取出来统一都是Object
//            Object obj = it.next();
//            System.out.println(obj);
//        }

        while (it.hasNext()) {
            Object obj = it.next();
            System.out.println(obj);
        }
    }
}

```

```java
package com.ganto;

import java.util.ArrayList;
import java.util.Collection;
import java.util.Iterator;

public class Main {
    public static void main(String[] args) {
        Collection c = new ArrayList();
      
        Iterator it = c.iterator();
      
        c.add(100);
        c.add(200);
        c.add(300);
        c.add(400);
        c.add(100);

        while (it.hasNext()) {
            Object obj = it.next();
            System.out.println(obj);
        }
    }
}

```

以上代码中，如果先`Iterator it = c.iterator();`再进行`c.add(200);`，会出现异常`java.util.ConcurrentModificationException`。

注意：此时获取的迭代器，指向的是那个集合中没有元素状态下的迭代器。

一定要注意：集合结构只要发生改变，迭代器必须重新获取。

```java
package com.ganto;

import java.util.ArrayList;
import java.util.Collection;
import java.util.Iterator;

public class Main {
    public static void main(String[] args) {
        Collection c = new ArrayList();
        c.add(100);
        c.add(200);
        Iterator it = c.iterator();
        while (it.hasNext()) {
            Object obj = it.next();
            c.remove(obj);  // 出异常
            System.out.println(obj);
        }
    }
}

```

以上代码，也印证了，集合结构只要发生改变，迭代器必须重新获取。以上代码删除过程，就使集合结构发生了变化。可以通过迭代器删除，以避免出错。

```java
package com.ganto;

import java.util.ArrayList;
import java.util.Collection;
import java.util.Iterator;

public class Main {
    public static void main(String[] args) {
        Collection c = new ArrayList();
        c.add(100);
        c.add(200);
        Iterator it = c.iterator();
        while (it.hasNext()) {
            Object obj = it.next();
            it.remove(); // 关键代码（从迭代器指向的collection中移除迭代器返回的最后一个元素（可选操作））
            System.out.println(obj);
        }
        System.out.println(c.size());
    }
}

```

## List接口

List接口中常用方法：

*   List集合存储元素特点：有序可重复
    
    有序：List集合中的元素有下标
    
    从0开始，以1递增
    
    可重复：存储一个1，还可以再存储1
    
*   List既然是Collection接口的子接口，那么肯定List接口有自己“特色”的方法
    
    以下只列出List接口特有的常用的方法：
    
    *   void add(int index, Object element)
    *   Object set(int index, Object element)
    *   Object get(int index)
    *   int indexOf(Object o)
    *   int lastIndexOf(Object o)
    *   Object remove(int index)

```java
package com.ganto;

import java.util.ArrayList;
import java.util.Iterator;
import java.util.List;

public class Main {
    public static void main(String[] args) {
        // 创建List类型的集合
        List myList = new ArrayList();

        // 添加元素 默认都是向集合的末尾添加元素
        myList.add("A");
        myList.add("B");

        // 在列表的指定位置插入指定元素（第一个参数是下标）
        myList.add(1, "KING");

        // 修改指定位置的元素
        myList.set(1, "HAHA");

        // 迭代
        Iterator it = myList.iterator();
        while (it.hasNext()){
            Object obj = it.next();
            System.out.println(obj);
        }

        // 根据下标获取元素
        Object el = myList.get(1);
        System.out.println(el);

        // 因为有下标，所以List集合有自己比较特殊的遍历方法
        // 通过下标遍历。【List集合特有的方式，Set没有。】
        for (int i = 0; i < myList.size(); i++) {
            Object obj = myList.get(i);
            System.out.println(obj);
        }

        // 获取指定对象第一次出现处的索引
        System.out.println(myList.indexOf("KING"));

        // 获取指定对象最后一次出现处的索引
        System.out.println(myList.lastIndexOf("B"));

        // 删除指定下标位置的元素
        myList.remove(0);
        System.out.println(myList.size());

    }
}

```

### ArrayList

ArrayList集合底层采用了数组这种数据结构。

ArrayList集合是非线程安全的。

1、ArrayList集合默认初始化容量是10（底层先创建了一个长度为0的数组，当添加第一个元素的时候，初始化容量10）

2、ArrayList集合底层是Object类型的数组Object\[\]

3、构造方法：

*   new ArrayList();
*   new ArrayList(20);

4、ArrayList集合的扩容：

*   原容量的1.5倍
*   ArrayList集合底层是数组，尽可能少的扩容。因为数组扩容效率比较低，建议在使用ArrayList集合的时候预估元素的个数，给定一个初始化容量

5、数组优点：检索效率比较高（每个元素占用空间大小相同，内存地址是连续的，知道首元素的内存地址，然后知道下标，通过数学表达式计算出元素的内存地址，所以检索效率最高。）

6、数组缺点：随即增删元素效率比较低、数组无法存储大数据量，因为很难找到一块非常巨大的连续的内存空间。

7、向数组末尾添加元素，效率很高，不受影响

8、这么多的集合中，用ArrayList集合比较多，因为往数组末尾添加元素，效率不受影响，日常开发中往往都是向数组末尾添加元素。

9、ArrayList集合是非线程安全的。

```java
package com.ganto;

import java.util.ArrayList;
import java.util.List;

public class Main {
    public static void main(String[] args) {
        // 默认初始化容量是10
        List list1 = new ArrayList();
        list1.add(new Object());
        // 集合的size()方法是获取当前集合中元素的个数。不是获取集合的容量
        System.out.println(list1.size()); // 1
        
        // 指定初始化容量20
        List list2 = new ArrayList(20);
        // 集合的size()方法是获取当前集合中元素的个数。不是获取集合的容量
        System.out.println(list2.size()); // 0
    }
}

```

#### ArrayList的构造方法

```java
package com.ganto;

import java.util.ArrayList;
import java.util.Collection;
import java.util.HashSet;
import java.util.List;

public class Main {
    public static void main(String[] args) {
        // 默认初始化容量10
        List list1 = new ArrayList();

        // 指定初始化容量100
        List list2 = new ArrayList(20);

        // 创建一个HashSet集合
        Collection c = new HashSet();
        c.add(100);
        c.add(200);
        // 通过这个构造方法就可以将HashSet集合转换成List集合
        List list3 = new ArrayList(c);
        for (int i = 0; i < list3.size(); i++) {
            System.out.println(list3.get(i));
        }
    }
}

```

### 单项链表数据结构

单链表中的节点

节点是单向链表中基本的单元

每一个节点Node都有两个属性：

*   一个属性是存储的数据
*   另一个属性是下一个节点的内存地址

Node.java

```java
package com.ganto;

/**
 * 节点类
 */
public class Node {
    // 存储的数据
    Object element;

    // 下一个节点的内存地址
    Node next;

    public Node() {

    }

    public Node(Object element, Node next) {
        this.element = element;
        this.next = next;
    }
}

```

Link.java

```java
package com.ganto;

/**
 * 链表类
 */
public class Link {
    // 头节点
    Node header = null;

    int size = 0;

    public int size() {
        return size;
    }

    // 向链表中添加元素的方法（向末尾添加）
    public void add(Object data) {
        // 创建一个新的节点对象
        // 让之前单链表的末尾节点next指向新节点对象
        // 有可能这个元素是第一个，也可能是第二个，也可能是第三个
        if(header == null) {
            // 能进来，说明没有节点，new一个新的节点对象，作为头节点对象
            // 这个时候的头节点既是一个头节点，又是一个末尾节点
            header = new Node(data, null);
        } else {
            // 说明头节点不是空，头节点已经存在了，找出当前末尾节点，让当前末尾节点的next是新节点
            Node currentLastNode = findLast(header);
            currentLastNode.next = new Node(data, null);
        }
        size++;
    }

    // 专门查找末尾节点的方法
    private Node findLast(Node node) {
        if(node.next == null) {
            // 如果一个节点的next是null，说明这个节点就是末尾节点
            return node;
        }
        // 程序能够到这里说明：node不是末尾节点
        return findLast(node.next);
    }

    // 删除链表中某个数据的方法
    public void remove(Object obj) {

    }

    // 修改链表中某个元素的方法
    public void modify(Object newObj){

    }

    // 查找链表中某个元素的方法
    public int find(Object obj) {
        return 1;
    }
}

```

Test.java

```java
package com.ganto;

public class Test {
    public static void main(String[] args) {
        // 创建了一个集合对象
        Link link = new Link();
        // 往集合中添加元素
        link.add(100);
        link.add(200);
        link.add(300);
        // 获取元素个数
        System.out.println(link.size());
    }
}

```

链表优点：随机增删元素效率较高，因为增删元素不会涉及到大量元素位移

链表缺点：查询效率较低，每一次查找某个元素的时候都需要从头节点开始往下遍历

![](images%5CJava%5C%E5%8D%95%E5%90%91%E9%93%BE%E8%A1%A8.png)

### 双向链表数据结构

双向链表中的节点

节点是双向链表中基本的单元

每一个节点Node都有三个属性：

*   一个属性是上一个节点的内存地址
*   一个属性是存储的数据
*   一个属性是下一个节点的内存地址

![](images%5CJava%5C%E5%8F%8C%E5%90%91%E9%93%BE%E8%A1%A8.png)

#### LinkedList

```java
package com.ganto;

import java.util.LinkedList;
import java.util.List;

/**
 * 链表的优点：
 *  由于链表上的元素在空间存储上内存地址不连续
 *  所以随机增删元素的时候不会有大量元素位移，因此随机增删效率比较高
 *  在以后的开发中，如果遇到随机增删集合中元素的业务比较多时，建议使用LinkedList
 *
 * 链表的缺点：
 *  不能通过数学表达式计算被查找元素的内存地址，每一次查找都是从头节点开始遍历，知道找到为止。所以LinkedList集合检索/查找的效率较低。
 */
public class Main {
    public static void main(String[] args) {
        // LinkedList集合底层也是有下标的。
        // 注意：ArrayList之所以检索效率比较高，不是单纯因为下标的原因
        // 是因为底层数组发挥的作用。
        // LinkedList集合照样有下标，但是检索/查找某个元素的时候效率比较低，因为只能从头节点开始一个一个遍历
        List list = new LinkedList();
        list.add("a");
        list.add("b");
        list.add("c");
        for (int i = 0; i < list.size(); i++) {
            System.out.println(list.get(i));
        }
    }
}

```

LinkedList集合没有初始化容量，最初这个链表中没有任何元素。

first和last引用都是null。

不管是LinkedList还是ArrayList，以后写代码时不需要关心具体是哪个集合。因为我们要面向接口编程，调用的方法都是接口中的方法。

```java
// List list = new ArrayList(); // 这样写表示底层用了数组
List list = new LinkedList(); // 这样写表示底层用了双向链表

list.add("1");
list.add("2");
list.add("3");
for(int i = 0; i < list.size(); i++) {
  System.out.println(list.get(i));
}

```

## Vector

*   底层也是一个数组
*   初始化容量：10
*   扩容之后是当前容量的2倍
*   ArrayList集合扩容是原容量1.5倍
*   Vector中所有的方法都是线程同步的，都带有synchronized关键字，是线程安全的。效率比较低，使用的比较少。

```java
package com.ganto;

import java.util.Iterator;
import java.util.List;
import java.util.Vector;

public class Main {
    public static void main(String[] args) {
        // 创建一个Vector集合
        // Vector vector = new Vector();
        List vector = new Vector();

        // 添加元素
        // 默认容量10
        vector.add(1);
        vector.add(2);
        vector.add(3);
        vector.add(4);
        vector.add(5);
        vector.add(6);
        vector.add(7);
        vector.add(8);
        vector.add(9);
        vector.add(10);

        // 满了之后自动扩容到之前容量的2倍
        vector.add(11);

        // 迭代器
        Iterator it = vector.iterator();
        while (it.hasNext()) {
            Object obj = it.next();
            System.out.println(obj);
        }
    }
}

```

使用集合工具类：java.util.Collectioins，将线程不安全的ArrayList集合转换成线程安全的

java.util.Collection 是集合接口

java.util.Collections 是集合工具类

```java
package com.ganto;

import java.util.*;

public class Main {
    public static void main(String[] args) {
        // 以下以后要使用
        List myList = new ArrayList(); // 非线程安全的
        // 变成线程安全的 myList集合就是线程安全的了
        Collections.synchronizedList(myList);

        myList.add("111");
        myList.add("222");
        myList.add("333");
    }
}

```

## HashSet集合

*   存储时顺序和取出的顺序不同
*   不可重复
*   放到HashSet集合中的元素实际上是放到了HashMap集合的key部分了

```java
package com.ganto;

import java.util.HashSet;
import java.util.Set;

public class Main {
    public static void main(String[] args) {
        Set<String> stringSet = new HashSet<>();
        stringSet.add("a");
        stringSet.add("b");
        stringSet.add("c");
        stringSet.add("d");
        stringSet.add("d");
        for (String s: stringSet) {
            System.out.println(s);
        }
    }
}

```

## TreeSet集合

*   TreeSet集合底层实际上是一个TreeMap
*   TreeMap集合底层是一个二叉树
*   放到TreeSet集合中的元素，等同于放到TreeMap集合key部分了
*   TreeSet集合中的元素：无序不可重复，但是可以按照元素的大小顺序自动排序，称为：可排序结合

无序：指的是存进去的顺序和取出来的顺序不同，没有下标

```java
package com.ganto;

import java.util.TreeSet;

public class Main {
    public static void main(String[] args) {
        // 创建一个TreeSet<String>集合
        TreeSet<String> treeset = new TreeSet<>();
        // 添加String
        treeset.add("zhangsan");
        treeset.add("lisi");
        treeset.add("wangwu");
        treeset.add("zhaoliu");
        // 遍历
        for (String name: treeset) {
            // 按照字典顺序 升序（中文不行）
            System.out.println(name);
        }
        // 创建一个TreeSet<Integer>集合
        TreeSet<Integer> treeset1 = new TreeSet<>();
        // 添加String
        treeset1.add(123);
        treeset1.add(934);
        treeset1.add(666);
        treeset1.add(12);
        // 遍历
        for (Integer num: treeset1) {
            // 按照数字大小顺序 升序
            System.out.println(num);
        }
    }
}

```

自定义类型，TreeSet则无法排序，中文也不支持排序

需要指定规则

## Map接口

java.util.Map接口:

*   Map和Collection没有继承关系
*   Map集合以key和value的方式存储数据（键值对）
    *   key和value都是引用数据类型
    *   key和value都是存储对象的内存地址
    *   key起到主导的地位，value是key的一个附属品
*   Map接口中常用方法：
    *   V put(K key, V value) // 向Map集合中添加键值对
    *   V get(Object key) // 通过key获取value
    *   void clear() // 清空Map集合
    *   boolean containsKey(Object key) // 判断Map中是否包含某个key
    *   boolean containsValue(Object value) // 判断Map中是否包含某个value
    *   boolean isEmpty() // 判断Map集合中元素个数是否为0
    *   Set keySet() // 获取Map集合所有的key（所有的键是一个set集合）
    *   V remove(Object key) // 通过key删除键值对
    *   int size() // 获取Map集合中键值对的个数
    *   Collection values() // 获取Map集合中所有的value，返回一个Collection
    *   Set<Map.Entry<K,V>> entrySet() // 将Map集合转换成Set集合

### 常用方法

```java
package com.ganto;

import java.util.Collection;
import java.util.HashMap;
import java.util.Map;

public class Main {
    public static void main(String[] args) {
        Map<Integer, String> map = new HashMap<>();
        map.put(1, "a");
        map.put(2, "b");
        map.put(3, "c");
        map.put(4, "d");
        // 通过key删除key-value
        map.remove(2);
        // 获取map中的键值对数量
        System.out.println("map中的键值对个数：" + map.size());
        // 通过key获取value
        System.out.println("取出map中key为1的value：" + map.get(1));
        // contains方法底层调用的都是equals进行比对的，所以自定义的类型需要重写equals方法
        // 判断是否包含某个key
        System.out.println("map中是否包含key为1的键值对：" + map.containsKey(1));
        // 判断是否包含某个value
        System.out.println("map中是否包含value为b的键值对：" + map.containsValue("b"));
        // 获取所有的value
        Collection<String> values = map.values();
        for(String s:values) {
            System.out.println(s);
        }
        // 清空map集合
        map.clear();
        // 判断是否为空
        System.out.println(map.isEmpty());
    }
}

```

### Map集合的遍历

第一种方式：获取所有的key，通过遍历key，来遍历value

```java
package com.ganto;

import java.util.HashMap;
import java.util.Iterator;
import java.util.Map;
import java.util.Set;

public class Main {
    public static void main(String[] args) {
        Map<Integer, String> map = new HashMap<>();
        map.put(1, "a");
        map.put(2, "b");
        map.put(3, "c");
        map.put(4, "d");
        Set<Integer> keys = map.keySet();

        //迭代器
        Iterator<Integer> it = keys.iterator();
        while (it.hasNext()){
            Integer key = it.next();
            System.out.println(key + "=" + map.get(key));
        }

        // foreach
        for (Integer key: keys) {
            System.out.println(key + "=" + map.get(key));
        }
    }
}

```

第二种方式：通过`Set<Map.Entry<K,V>> entrySet()`将Map集合转变成Set集合

```java
package com.ganto;

import java.util.HashMap;
import java.util.Iterator;
import java.util.Map;
import java.util.Set;

public class Main {
    public static void main(String[] args) {
        Map<Integer, String> map = new HashMap<>();
        map.put(1, "a");
        map.put(2, "b");
        map.put(3, "c");
        map.put(4, "d");
        Set<Map.Entry<Integer, String>> set = map.entrySet();

        // 迭代器
        Iterator<Map.Entry<Integer,String>> it = set.iterator();
        while (it.hasNext()) {
            Map.Entry<Integer,String> node = it.next();
            Integer key = node.getKey();
            System.out.println(key + "=" + map.get(key));
        }

        // foreach
        // 这种方式效率比较高，因为获取key和value都是直接从node对象中获取的属性值
        // 这种方式比较适合于大数据量
        for (Map.Entry<Integer, String> node: set) {
            System.out.println(node.getKey() + "=" + node.getValue());
        }
    }
}

```

### 哈希表数据结构

HashMap集合：

*   HashMap结合底层使哈希表/散列表的数据结构
*   哈希表是一个数组和单向链表的结合体：
    *   数组：在查询方面效率很高，随机增删方面效率很低
    *   单向链表：在随机增删方面效率很高，在查询方面效率很低
    *   哈希表将以上的两种数据结构融合在一起，充分发挥它们各自的优点

### HashMap

HashMap集合key部分允许为null，但是HashMap集合的key为null的值只能有一个

```java
package com.ganto;

import java.util.HashMap;
import java.util.Map;

public class Main {
    public static void main(String[] args) {
        Map map = new HashMap<>();
        map.put(null, null);
        map.put(null, 100);
        System.out.println(map.size());
        System.out.println(map.get(null));
    }
}

```

### Hashtable

Hashtable的key和value都不可以为null，否则会报空指针异常

Hashtable方法都带有synchronized：线程安全的

线程安全有其它的方案，这个Hashtable对线程的处理导致效率低，使用较少了

Hashtable和HashMap一样，底层都是哈希表数据结构

Hashtable的初始化容量是：11，默认加载因子：0.75f

Hashtable的扩容：原容量 \* 2 + 1

### Properties

目前只需要掌握Properties属性类对象的相关方法即可

Properties是一个Map集合，继承Hashtable，Properties的key和value都是String类型

Properties被称为属性类对象

Properties是线程安全的

```java
package com.ganto;

import java.util.Properties;

public class Main {
    public static void main(String[] args) {
        // 创建一个Properties对象
        Properties pro = new Properties();
        // 需要掌握Properties的两个方法，一个存，一个取
        pro.setProperty("url", "jdbc:mysql://localhost:3306/ganto");
        pro.setProperty("driver", "com.mysql.jdbc.Driver");
        pro.setProperty("username", "root");
        pro.setProperty("password", "123");
        // 通过key获取value
        System.out.println(pro.getProperty("url"));
    }
}

```

# 泛型机制

JDK5.0之后推出的新特性：泛型

## 不使用泛型

不使用泛型机制，分析程序存在的缺点

```java
package com.ganto;

import java.util.ArrayList;
import java.util.Iterator;
import java.util.List;

/**
 * JDK5.0之后推出的新特性：泛型
 */
public class Main {
    public static void main(String[] args) {
        // 不使用泛型机制，分析程序存在的缺点

        // 创建一个集合
        List myList = new ArrayList();
        // 准备对象
        Cat c = new Cat();
        Bird b = new Bird();
        // 将对象添加到集合当中
        myList.add(c);
        myList.add(b);
        // 遍历集合，取出Cat让它抓老鼠，取出Brid让它飞
        Iterator it = myList.iterator();
        while (it.hasNext()) {
            // 通过迭代器去除的obj就是Object类型，不用泛型的话下面必须强转
            Object obj = it.next();
            if(obj instanceof Animal) {
                ((Animal) obj).move();
            }
        }
    }
}
class Animal {
    public void move() {
        System.out.println("动物在移动！");
    }
}
class Cat extends Animal {
    public void catchMouse() {
        System.out.println("猫抓老鼠！");
    }
}
class Bird extends Animal {
    public void fly() {
        System.out.println("鸟儿在飞翔！");
    }
}

```

## 使用泛型

```java
package com.ganto;

import java.util.ArrayList;
import java.util.Iterator;
import java.util.List;

/**
 * JDK5.0之后推出的新特性：泛型
 */
public class Main {
    public static void main(String[] args) {
        // 使用泛型机制

        // 使用泛型之后,表示List集合中只允许存储Animal类型的数据
        // 用泛型来指定集合中存储的数据类型
        List<Animal> myList = new ArrayList<Animal>();

        // 指定了List集合中只能存储Animal,那么存储String就会编译报错
        // 这样用了泛型之后,集合中元素的数据类型更加统一
        // myList.add("123"); // 报错 需要的类型: Animal 提供的类型: String

        Cat c = new Cat();
        Bird b = new Bird();
        myList.add(c);
        myList.add(b);

        // 获取迭代器
      	// 这个表示迭代器迭代的是Animal类型
        Iterator<Animal> it = myList.iterator();
        while (it.hasNext()) {
          	// 使用泛型之后，每次迭代返回的数据都是Animal类型
            Animal a = it.next();
          	// 这里就不需要进行强制类型转换了
            a.move();
          	// 调用子类特有的方法还是避免不了要向下转型
          	if(a instanceof Cat) {
              	((Cat) a).catchMouse();
            } else if(a instanceof Bird) {
              	((Bird) a).fly();
            }
        }
    }
}
class Animal {
    public void move() {
        System.out.println("动物在移动！");
    }
}
class Cat extends Animal {
    public void catchMouse() {
        System.out.println("猫抓老鼠！");
    }
}
class Bird extends Animal {
    public void fly() {
        System.out.println("鸟儿在飞翔！");
    }
}

```

使用泛型的好处：

*   第一：集合中存储的元素类型统一
*   第二：从集合中取出的元素类型是泛型指定的类型，不需要进行大量的向下转型

泛型的缺点：

*   导致集合中存储的元素缺乏多样性

大多数业务中，集合中元素的类型是统一的，所以泛型使用的很多

JDK8之后引入了自动类型推断机制。（又称为钻石表达式）

`List<Animal> myList = new ArrayList<>();`

## 自定义泛型

```java
package com.ganto;

public class Main {
    public static void main(String[] args) {
        // 实例化对象的时候,定义了String泛型
        Animal<String> a = new Animal<>();
        String a1 = a.doSome("1"); // 这里只能传入String类型的参数
        // String a2 = a.doSome(1); // 报错 类型不匹配
        System.out.println(a1);
    }
}

class Animal<T> {
    public T doSome(T t) {
        return t;
    }
}

```

# for each

```java
package com.ganto;

public class Main {
    public static void main(String[] args) {
        // 定义int类型数组
        int[] arr = {1,2,3,4};
        // 遍历数组 普通for循环
        for (int i = 0; i < arr.length; i++) {
            System.out.println(arr[i]);
        }
        // 遍历数组 foreach
        for(int data: arr) {
            System.out.println(data);
        }
    }
}

```

```java
package com.ganto;

import java.util.ArrayList;
import java.util.Iterator;
import java.util.List;

public class Main {
    public static void main(String[] args) {
        List<String> stringList = new ArrayList<>();
        stringList.add("1");
        stringList.add("2");
        stringList.add("3");
        stringList.add("4");
        // 使用迭代器遍历集合
        Iterator<String> it = stringList.iterator();
        while (it.hasNext()){
            String data = it.next();
            System.out.println(data);
        }
        // 使用普通for循环遍历数组
        for (int i = 0; i < stringList.size(); i++) {
            System.out.println(stringList.get(i));
        }
        // 使用foreach遍历集合
        for (String data: stringList) {
            System.out.println(data);
        }
    }
}


```

# IO流

![](images%5CJava%5CIO%E6%B5%81.png)

IO流

*   I：Input
*   O：Output
*   通过IO可以完成硬盘文件的读和写

## IO流的分类：

有多种分类方式：

*   一种方式是按照流的方向进行分类：
    
    *   以内存作为参照物，往内存中去，叫做输入（Input），或者叫做读（Read）
    *   以内存作为参照物，从内存中出，叫做输出（Output），或者叫做写（Write）
*   另一种方式是按照读取数据方式不同进行分类：
    
    *   有的流是按照字节的方式读取数据，一次读取1个字节byte，等同于一次读取8个二进制位。这种流是万能的，什么类型的文件都可以读取。包括：文本文件、图片、声音文件、视频文件等...
        *   假如文件file.txt（a哈希bc张三），采用字节流的话是这样读的：
            *   第一次读：一个字节，正好读到'a'
            *   第二次读：一个字节，正好读到'中'字符的一半
            *   第三次读：一个字节，正好读到'中'字符的另一半
    *   有的流是按照字符的方式读取数据的，一次读取一个字符，这种流是为了方便读取普通文本文件而存在的。这些流不能读取：图片、声音、视频等文件。只能读取纯文本文件，连word文件都无法读取。
        *   假如文件file.txt（a哈希bc张三），采用字符流的话是这样读的：
            *   第一次读：'a'字符（a字符在windows系统中占用1个字节）
            *   第二次读：'中'字符（'中'字符在windows系统中占用2个字节）

综上所述，流的分类：输入流、输出流、字节流、字符流

Java中的IO流都已经写好了，我们程序员不需要关心，我们最主要还是掌握，在Java中已经提供了哪些流，每个流的特点是什么，每个流对象上的常用方法有哪些？

java中所有的流都是在：java.io.\* 这个包下

java中主要还研究：怎么new流对象，调用流对象的哪个方法是读，哪个方法是写

## Java IO流四大家族

四大家族的首领*都是抽象类*

java.io.InputStream // 字节输入流

java.io.OutputStream // 字节输出流

java.io.Reader // 字符输入流

java.io.Writer // 字符输出流

**注意：在Java中只要类名以Stream结尾的都是字节流；以Reader/Writer结尾的都是字符流**

所有的流都实现了：java.io.Closeable接口，都是可关闭的，都有close()方法。

流毕竟是一个管道，这个是内存和硬盘之间的通道，用完之后一定要关闭，不然会耗费（占用）很多资源。养成好习惯，用完流一定要关闭。

所有的输出流都实现了：java.io.Flushable接口，都是可刷新的，都有flush()方法。

养成一个好习惯，输出流在最终输出之后，一定要记得flush()刷新以下。这个刷新表示将通道/管道当中剩余未输出的数据强行输出完（清空管道），刷新的作用就是清空管道。

注意：如果没有flush()可能会导致丢失数据。

## 重要的流

java.io包下需要掌握的流有16个

### 文件专属

java.io.FileInputStream

java.io.FileOutputStream

java.io.FileReader

java.io.FileWriter

### 转换流

将字节转换成字符

java.io.InputStreamReader

java.io.OutputStreamWriter

### 缓冲流专属

java.io.BufferedReader

java.io.BufferedWriter

java.io.BufferedInputStream

java.io.BufferedOutputStream

### 数据流专属

java.io.DataInputStream

java.io.DataOutputStream

### 标准输出流

java.io.PrintWriter

java.io.PrintStream

### 对象专属流

java.io.ObjectInputStream

java.io.ObjectOutputStream

## java.io.FileInputStream

重点掌握

*   文件字节输入流，万能的，任何类型的文件都可以采用这个流来读。
*   字节的方式，完成输入的操作，完成读的操作（硬盘 --> 内存）

读取本地文件

```java
package com.ganto;

import java.io.FileInputStream;
import java.io.FileNotFoundException;
import java.io.IOException;

public class Main {
    public static void main(String[] args) {
        FileInputStream fileInputStream = null;
        try {
            // 创建文件字节输入流对象
            fileInputStream = new FileInputStream("C:/Users/ganto/Desktop/file");

            // 开始读
            int readData = fileInputStream.read(); // 这个方法的返回值是读取字节本身
            System.out.println(readData);// 97

            readData = fileInputStream.read();
            System.out.println(readData);// 98

            readData = fileInputStream.read();
            System.out.println(readData);// 99

            readData = fileInputStream.read();
            System.out.println(readData);// 100

            readData = fileInputStream.read();
            System.out.println(readData);// 101

            readData = fileInputStream.read();
            System.out.println(readData);// 102

            readData = fileInputStream.read();
            System.out.println(readData);// -1 到不末尾，没有数据了就返回-1

        } catch (FileNotFoundException e) {
            e.printStackTrace();
        } catch (IOException e) {
            throw new RuntimeException(e);
        } finally {
            // 在finally语句块当中确保流一定关。
            if(fileInputStream != null) { // 避免空指针异常。关闭流的前提是：流不是空。流是null的时候没必要关闭
                try {
                    fileInputStream.close();
                } catch (IOException e) {
                    e.printStackTrace();
                }
            }
        }
    }
}

```

改进

```java
package com.ganto;

import java.io.FileInputStream;
import java.io.FileNotFoundException;
import java.io.IOException;

public class Main {
    public static void main(String[] args) {
        FileInputStream fileInputStream = null;
        try {
            // 创建文件字节输入流对象
            fileInputStream = new FileInputStream("C:/Users/ganto/Desktop/file");

            // 开始读
            while(true) {
                int readData = fileInputStream.read(); // 这个方法的返回值是读取字节本身
                if(readData == -1) {
                    break;
                }
                System.out.println(readData);
            }

            // 改造while
            int readData = 0;
            while((readData = fileInputStream.read()) != -1) {
                System.out.println(readData);
            }
        } catch (FileNotFoundException e) {
            e.printStackTrace();
        } catch (IOException e) {
            throw new RuntimeException(e);
        } finally {
            // 在finally语句块当中确保流一定关。
            if(fileInputStream != null) { // 避免空指针异常。关闭流的前提是：流不是空。流是null的时候没必要关闭
                try {
                    fileInputStream.close();
                } catch (IOException e) {
                    e.printStackTrace();
                }
            }
        }
    }
}

```

int read(byte\[\] b)

一次最多读取b.length个字节

减少硬盘和内存的交互，提高程序的执行效率

往byte\[\]数组当中读

```java
package com.ganto;

import java.io.FileInputStream;
import java.io.FileNotFoundException;
import java.io.IOException;

public class Main {
    public static void main(String[] args) {
        FileInputStream fis = null;
        try {
            // IDEA默认的当前路径是工程Project的根，就是IDEA的默认当前路径是Project的根
            fis = new FileInputStream("C:\\Users\\ganto\\Desktop\\file");
            // 开始读，采用byte数组，一次读取多个字节。最多读取”数组.length“个字节
            byte[] bytes = new byte[4];
            // 这个方法的返回值是：读取到的字节数量。（不是字节本身）
            int readCount = fis.read(bytes);
            System.out.println(readCount); // 读取到4个字节
            // System.out.println(new String(bytes)); // abcd 将bytes数组转全部换成字符串
            System.out.println(new String(bytes, 0, readCount));

            readCount = fis.read(bytes);
            System.out.println(readCount); // 读取到2个字节
            // System.out.println(new String(bytes)); // efcd 第二次读取，ef会覆盖ab 所以不应该转换所有数组，应该读取到多少转换多少个
            System.out.println(new String(bytes, 0, readCount));

            readCount = fis.read(bytes);
            System.out.println(readCount); // 没有读到字节，返回-1

        } catch (FileNotFoundException e) {
            throw new RuntimeException(e);
        } catch (IOException e) {
            throw new RuntimeException(e);
        } finally {
            if(fis != null) {
                try {
                    fis.close();
                } catch (IOException e) {
                    throw new RuntimeException(e);
                }
            }
        }
    }
}

```

改造-最终版 需要掌握

```java
package com.ganto;

import java.io.FileInputStream;
import java.io.FileNotFoundException;
import java.io.IOException;

public class Main {
    public static void main(String[] args) {
        FileInputStream fis = null;
        try {
            fis = new FileInputStream("C:\\Users\\ganto\\Desktop\\file");
            byte[] bytes = new byte[4];

            int readCount = 0;
            while((readCount = fis.read(bytes)) != -1) {
                System.out.print(new String(bytes, 0, readCount));
            }
        } catch (FileNotFoundException e) {
            throw new RuntimeException(e);
        } catch (IOException e) {
            throw new RuntimeException(e);
        } finally {
            if(fis != null) {
                try {
                    fis.close();
                } catch (IOException e) {
                    throw new RuntimeException(e);
                }
            }
        }
    }
}

```

### 其他常用方法

*   int available() // 返回流当中剩余的没有读到的字符数量
*   lolng skip(long n) // 跳过几个字节不读

```java
package com.ganto;

import java.io.FileInputStream;
import java.io.FileNotFoundException;
import java.io.IOException;

public class Main {
    public static void main(String[] args) {
        FileInputStream fis = null;
        try {
            fis = new FileInputStream("C:\\Users\\ganto\\Desktop\\file");
            System.out.println("总字节数: " + fis.available());
//            int readByte = fis.read();
//            System.out.println("剩下多少个字节没有读: " + fis.available());
						// 一次性读取全部文件,但是这种方式不适合大文件,因为byte[]数组不能太大
            byte[] bytes = new byte[fis.available()];
            int readCount = fis.read(bytes);
            System.out.print(new String(bytes));
        } catch (FileNotFoundException e) {
            throw new RuntimeException(e);
        } catch (IOException e) {
            throw new RuntimeException(e);
        } finally {
            if(fis != null) {
                try {
                    fis.close();
                } catch (IOException e) {
                    throw new RuntimeException(e);
                }
            }
        }

    }
}

```

skip跳过几个字节不读,这个方法也可能以后会用

```java
package com.ganto;

import java.io.FileInputStream;
import java.io.FileNotFoundException;
import java.io.IOException;

public class Main {
    public static void main(String[] args) {
        FileInputStream fis = null;
        try {
            fis = new FileInputStream("C:\\Users\\ganto\\Desktop\\file");
            fis.skip(3);
            System.out.println(fis.read());
        } catch (FileNotFoundException e) {
            throw new RuntimeException(e);
        } catch (IOException e) {
            throw new RuntimeException(e);
        } finally {
            if(fis != null) {
                try {
                    fis.close();
                } catch (IOException e) {
                    throw new RuntimeException(e);
                }
            }
        }

    }
}

```

## java.io.FileOutputStream

重点掌握

文件字节输出流,负责写

从内存到硬盘

```java
package com.ganto;

import java.io.FileNotFoundException;
import java.io.FileOutputStream;
import java.io.IOException;

public class Main {
    public static void main(String[] args) {
        FileOutputStream fos = null;
        try {
            // 这种方式谨慎使用,这种方式会将原文件清空,然后重新写入
            // fos = new FileOutputStream("C:\\Users\\ganto\\Desktop\\file.text");
            // 以追加的方式在文件末尾写入.不会清空源文件内容.
            fos = new FileOutputStream("C:\\Users\\ganto\\Desktop\\file.text", true);
            // 开始写 myfile文件不存在的时候会自动新建
            byte[] bytes = {97, 98, 99, 121};
            // 将byte数组全部写出
            fos.write(bytes);
            // 将byte数组部分写出
            fos.write(bytes, 0, 2); // 再写出97/a 98/b
            // 写完之后,最好一定要刷新
            fos.flush();
        } catch (FileNotFoundException e) {
            throw new RuntimeException(e);
        } catch (IOException e) {
            throw new RuntimeException(e);
        } finally {
            if(fos != null) {
                try {
                    fos.close();
                } catch (IOException e) {
                    throw new RuntimeException(e);
                }
            }
        }
    }
}

```

```java
package com.ganto;

import java.io.FileNotFoundException;
import java.io.FileOutputStream;
import java.io.IOException;

public class Main {
    public static void main(String[] args) {
        FileOutputStream fos = null;
        try {
            fos = new FileOutputStream("C:\\Users\\ganto\\Desktop\\file.text", true);
            String s = "Java开发";
            byte[] bytes = s.getBytes(); // 将字符串转换成byte[]数组
            fos.write(bytes);
            fos.flush();
        } catch (FileNotFoundException e) {
            throw new RuntimeException(e);
        } catch (IOException e) {
            throw new RuntimeException(e);
        } finally {
            if(fos != null) {
                try {
                    fos.close();
                } catch (IOException e) {
                    throw new RuntimeException(e);
                }
            }
        }
    }
}

```

## 文件复制

使用FileInputStream + FileOutputStream完成文件的拷贝

拷贝的过程应该是一边读，一边写

使用以上的字节流拷贝文件的时候，文件类型随意，什么样的文件都能拷贝

```java
package com.ganto;

import java.io.FileInputStream;
import java.io.FileNotFoundException;
import java.io.FileOutputStream;
import java.io.IOException;

public class Main {
    public static void main(String[] args) {
        FileInputStream fis = null;
        FileOutputStream fos = null;
        try {
            // 创建一个输入流对象
            fis = new FileInputStream("C:\\Users\\ganto\\Desktop\\file.text");
            // 创建一个输出流对象
            fos = new FileOutputStream("D:\\file.text", true);
            // 一次拷贝10*1024个字节，也就是1兆
            byte[] bytes = new byte[10*1024];
            int readCount = 0;
            while ((readCount = fis.read(bytes)) != -1) {
                fos.write(bytes,0,readCount);
            }
            fos.flush();
        } catch (FileNotFoundException e) {
            throw new RuntimeException(e);
        } catch (IOException e) {
            throw new RuntimeException(e);
        } finally {
          	if(fis != null) {
								try {
                    fis.close();
                } catch (IOException e) {
                    throw new RuntimeException(e);
                }
            }
            if(fos != null) {
                
                try {
                    fos.close();
                } catch (IOException e) {
                    throw new RuntimeException(e);
                }
            }
        }
    }
}

```

## java.io.FileReader

文件字符输入流，只能读取普通文本

读取文本内容时，比较方便，快捷

```java
package com.ganto;

import java.io.FileNotFoundException;
import java.io.FileReader;
import java.io.IOException;

public class Main {
    public static void main(String[] args) {
        FileReader reader = null;
        try {
            // 创建文件字符输入流
            reader = new FileReader("C:\\Users\\ganto\\Desktop\\file.text");
            // 开始读
            char[] chars = new char[4]; // 一次读取4个字符
            int readCount = 0;
            while ((readCount = reader.read(chars)) != -1) {
                System.out.print(new String(chars, 0, readCount));
            }
        } catch (FileNotFoundException e) {
            throw new RuntimeException(e);
        } catch (IOException e) {
            throw new RuntimeException(e);
        } finally {
            if(reader != null) {
                try {
                    reader.close();
                } catch (IOException e) {
                    throw new RuntimeException(e);
                }
            }
        }
    }
}

```

## java.io.FileWriter

文件字符输出流。写

只能输出普通文本

```java
package com.ganto;

import java.io.FileNotFoundException;
import java.io.FileWriter;
import java.io.IOException;

public class Main {
    public static void main(String[] args) {
        FileWriter writer = null;
        try {
            writer = new FileWriter("C:\\Users\\ganto\\Desktop\\file.text", true);
            char[] chars = {'我', '是', 'J', 'a', 'v', 'a', '开', '发', '\n'};
            writer.write(chars);
            writer.write(chars, 2, 4);
            writer.write("我是java软件开发工程师！");
            writer.flush();
        } catch (IOException e) {
            throw new RuntimeException(e);
        } finally {
            if(writer != null) {
                try {
                    writer.close();
                } catch (IOException e) {
                    throw new RuntimeException(e);
                }
            }
        }
    }
}

```

## 文件复制

```java
package com.ganto;

import java.io.FileNotFoundException;
import java.io.FileReader;
import java.io.FileWriter;
import java.io.IOException;

public class Main {
    public static void main(String[] args) {
        FileReader reader = null;
        FileWriter writer = null;
        try {
            reader = new FileReader("C:\\Users\\ganto\\Desktop\\file.text");
            writer = new FileWriter("D:\\file.text");
            char[] chars = new char[1024 * 512]; // 1MB
            int readCount = 0;
            while ((readCount = reader.read(chars)) != -1) {
                writer.write(chars, 0, readCount);
            }
            writer.flush();
        } catch (FileNotFoundException e) {
            throw new RuntimeException(e);
        } catch (IOException e) {
            throw new RuntimeException(e);
        } finally {
            if(reader != null) {
                try {
                    reader.close();
                } catch (IOException e) {
                    throw new RuntimeException(e);
                }
            }
            if(writer != null) {
                try {
                    writer.close();
                } catch (IOException e) {
                    throw new RuntimeException(e);
                }
            }
        }
    }
}

```

## java.io.BufferedReader

带有缓冲区的字符输入流

使用这个流的时候不需要自定义char数组，或者说不需要自定义byte数组，自带缓冲

```java
package com.ganto;

import java.io.BufferedReader;
import java.io.FileNotFoundException;
import java.io.FileReader;
import java.io.IOException;

public class Main {
    public static void main(String[] args) throws IOException {
        FileReader reader = new FileReader("C:\\Users\\ganto\\Desktop\\file.text");
        // 当一个流的构造方法中需要一个流的时候，这个被传进来的流叫做：节点流
        // 外部负责包装的这个流，叫做：包装流，还有一个名字叫做：处理流
        // 像当前这个程序来说：FileReader就是一个节点流；BufferedReader就是包装流/处理流
        BufferedReader br = new BufferedReader(reader);

        // 读一行
        // String firstLine = br.readLine(); // 读取一个文本行，但是不带换行符
        // System.out.println(firstLine);

        String s = null;
        while ((s = br.readLine()) != null) {
            System.out.println(s);
        }

        // 关闭流
        // 对于包装流来说，只需要关闭最外层流就行，里面的节点流会自动关闭。（可以看源代码）
        br.close();
    }
}

```

```java
package com.ganto;

import java.io.BufferedReader;
import java.io.FileInputStream;
import java.io.InputStreamReader;

public class Main {
    public static void main(String[] args) throws Exception {
        // 字节流
        FileInputStream in = new FileInputStream("C:\\Users\\ganto\\Desktop\\file.text");

        // 通过转换流转换(InputStreamReander将字节流转换成字符流)
        // in是字节流，reader是包装流
        InputStreamReader reader= new InputStreamReader(in);

        // 这个构造方法只能传一个字符流，不能传字节流
        // reader是节点流，br是包装流
        BufferedReader br = new BufferedReader(reader);

        // 合并的写法
        // BufferedReader br = new BufferedReader(new InputStreamReader(new FileInputStream("C:\\Users\\ganto\\Desktop\\file.text")));

        // 读
        String line = null;
        while ((line = br.readLine()) != null) {
            System.out.println(line);
        }

        // 关闭最外层
        br.close();
    }
}

```

## java.io.BufferedWriter

带有缓冲的字符输出流

```java
package com.ganto;

import java.io.BufferedWriter;
import java.io.FileWriter;

public class Main {
    public static void main(String[] args) throws Exception {
        BufferedWriter bw = new BufferedWriter(new FileWriter("C:\\Users\\ganto\\Desktop\\file.text", true));
        bw.write("哈哈哈哈");
        bw.write("\n");
        bw.write("嘻嘻嘻嘻");
        bw.write("\n");
        bw.flush();
        bw.close();
    }
}

```

```java
package com.ganto;

import java.io.BufferedWriter;
import java.io.FileOutputStream;
import java.io.OutputStreamWriter;

public class Main {
    public static void main(String[] args) throws Exception {
        BufferedWriter bw = new BufferedWriter(new OutputStreamWriter(new FileOutputStream("C:\\Users\\ganto\\Desktop\\file.text", true)));
        bw.write("哈哈哈哈");
        bw.write("\n");
        bw.write("嘻嘻嘻嘻");
        bw.write("\n");
        bw.flush();
        bw.close();
    }
}

```

## java.io.DataInputStream

数据字节输入流

DataOutputStream写的文件只能通过DataInputStream去读

并且读的时候需要提前知道写入的顺序

读的顺序需要和写的顺序一致，才可以正常读取数据

```java
package com.ganto;

import java.io.DataInputStream;
import java.io.FileInputStream;

public class Main {
    public static void main(String[] args) throws Exception {
        DataInputStream dis = new DataInputStream(new FileInputStream("data"));

        // 开始读
        byte b = dis.readByte();
        short s = dis.readShort();
        int i = dis.readInt();
        long l = dis.readLong();
        float f = dis.readFloat();
        double d = dis.readDouble();
        boolean bl = dis.readBoolean();
        char c = dis.readChar();

        System.out.println(b);
        System.out.println(s);
        System.out.println(i);
        System.out.println(l);
        System.out.println(f);
        System.out.println(d);
        System.out.println(bl);
        System.out.println(c);

        dis.close();
    }
}

```

## java.io.DataOutputStream

数据专属的流

这个流可以将数据联通数据的类型一并写入文件

注意：这个文件不是普通文本文档。（用记事本无法正确的浏览）；并且只能通过`java.io.DataInputStream`读出来

```java
package com.ganto;

import java.io.DataOutputStream;
import java.io.FileOutputStream;

public class Main {
    public static void main(String[] args) throws Exception {
        // 创建数据专属的字节输出流
        DataOutputStream dos = new DataOutputStream(new FileOutputStream("data"));
        // 写数据
        byte b = 100;
        short s = 200;
        int i = 300;
        long l = 400L;
        float f = 3.0F;
        double d = 3.14;
        boolean bl = false;
        char c = 'a';
        // 写
        // 把数据以及数据的类型一并写入到文件当中
        dos.writeByte(b);
        dos.writeShort(s);
        dos.writeInt(i);
        dos.writeLong(l);
        dos.writeFloat(f);
        dos.writeDouble(d);
        dos.writeBoolean(bl);
        dos.writeChar(c);
        // 刷新
        dos.flush();
        // 关系
        dos.close();
    }
}

```

## java.io.PrintStream

重点掌握

标准的字节输出流，默认输出到控制台

```java
package com.ganto;

import java.io.FileOutputStream;
import java.io.PrintStream;

public class Main {
    public static void main(String[] args) throws Exception {
        // 联合起来写
        System.out.println("Hello Java");

        // 分来写
        PrintStream ps = System.out;
        ps.println("Hello 张三");
        ps.println("Hello 李四");
        ps.println("Hello 王五");

        // 标准输出流不需要手动close()关闭

        // 可以改变标准输出流的输出方向
        /**
         * 这些是之前System类使用过的方法和属性：
         * System.gc();
         * System.currentTimeMillis();
         * PrintStream ps = System.out;
         * System.exit(0);
         * System.arraycopy(...);
         */
        // 标准输出流不再指向控制台，指向”log“文件
        PrintStream printStream = new PrintStream(new FileOutputStream("log"));
        // 修改输出方向，将输出方向修改到”log“文件
        System.setOut(printStream);
        // 再进行输出
        System.out.println("xixixixixi");
        System.out.println("hahahahaha");
        System.out.println("hehehehehe");
    }
}

```

### 日志工具

Logger.java

```java
package com.ganto;

import java.io.FileNotFoundException;
import java.io.FileOutputStream;
import java.io.PrintStream;
import java.text.SimpleDateFormat;
import java.util.Date;

public class Logger {
    // 记录日志的方法
    public static void log(String msg) {
        try {
            // 指向一个日志文件
            PrintStream out = new PrintStream(new FileOutputStream("log.txt", true));
            // 改变输出方向
            System.setOut(out);
            // 日期当前时间
            Date nowTime = new Date();
            SimpleDateFormat sdf = new SimpleDateFormat("yyyy-MM-dd HH:mm:ss SSS");
            String strTime = sdf.format(nowTime);
            System.out.println(strTime + "：" + msg);
        } catch (FileNotFoundException e) {
            throw new RuntimeException(e);
        }
    }
}

```

LogTest.java

```java
package com.ganto;

public class LogTest {
    public static void main(String[] args) throws Exception {
        // 测试工具类是否好用
        Logger.log("调用了System类的gc()方法，建议启动垃圾回收");
        Logger.log("调用了UserService类的doSome()");
        Logger.log("用户尝试进行登录，验证失败");
    }
}

```

结果：log.txt

```txt
2023-03-01 00:27:01 014：调用了System类的gc()方法，建议启动垃圾回收
2023-03-01 00:27:01 031：调用了UserService类的doSome()
2023-03-01 00:27:01 031：用户尝试进行登录，验证失败

```

## java.io.ObjectInputStream

重点掌握

## java.io.ObjectOutputStream

重点掌握

## java.io.File

*   File类和四大家族没有关系，所以File类不能完成文件的读和写
    
*   File对象代表：文件和目录路径名的抽象变现形式。一个File对象有可能对应的是目录，也可能是文件。File只是一个路径名的抽象表现形式。
    
*   需要掌握File类中常用的方法
    

```java
package com.ganto;

import java.io.File;

public class Main {
    public static void main(String[] args) throws Exception {
        File f1 = new File("D:/log");

        // 判断是否存在
        System.out.println(f1.exists());

        // 如果D盘下的log文件不存在，则以文件的形式创建出来
         /* if(!f1.exists()){
             // 以文件形式新建
             f1.createNewFile();
         } */

        // 如果D盘下的log不存在，则以目录的形式新建出来
        /* if(!f1.exists()) {
            // 以目录的形式新建
            f1.mkdir();
        } */

        File f2 = new File("D:/a/b/c/d/e/f");
        // 以多级目录的形式新建出来
        /* if(!f2.exists()) {
            // 创建多级目录
             f2.mkdirs();
        } */

        File f3 = new File("D:\\Development\\Java\\LearnCoding\\log.txt");
        // 获取文件的父路径
        String parentPath = f3.getParent();
        System.out.println(parentPath); // D:\Development\Java\LearnCoding

        File parentFile = f3.getParentFile();
        System.out.println("获取绝对路径：" + parentFile.getAbsolutePath());

        File f4 = new File("log");
        System.out.println("绝对路径：" + f4.getAbsolutePath()); // D:\Development\Java\LearnCoding\log
    }
}

```

```java
package com.ganto;

import java.io.File;
import java.text.SimpleDateFormat;
import java.util.Date;

public class Main {
    public static void main(String[] args) throws Exception {
        File f1 = new File("D:\\Development\\Java\\LearnCoding\\log.txt");
        // 获取文件名
        System.out.println(f1.getName());

        // 判断是否是一个目录
        System.out.println(f1.isDirectory());

        // 判断是否是一个文件
        System.out.println(f1.isFile());

        // 获取文件最后一次修改时间
        long haoMiao = f1.lastModified(); // 这个毫秒是从1970年到现在的总毫秒数
        // 将总毫秒数转换成日期
        Date time = new Date(haoMiao);
        SimpleDateFormat sdf = new SimpleDateFormat("yyyy-MM-dd HH:mm:ss SSSS");
        String strTime = sdf.format(time);
        System.out.println(strTime);

        // 获取文件大小
        System.out.println(f1.length()); // 单位：字节
    }
}

```

```java
package com.ganto;

import java.io.File;

public class Main {
    public static void main(String[] args) throws Exception {
        /**
         * File[] listFiles()
         * 获取当前目录下所有的子文件
         */
        File f1 = new File("D:\\Development\\Java\\LearnCoding");
        File[] files = f1.listFiles();
        for (File file: files) {
            System.out.println(file.getAbsolutePath());
            System.out.println(file.getName());
        }
    }
}

```

# 进阶👆

# Java中需要注意的点

## 命令行执行Java程序

Hello.java

```java
public class Hello
{
	public static void main(String[] args) {
		System.out.println("Hello Java!");
	}
}

```

命令行编译执行Hello.java程序

```sh
javac Hello.java
java Hello

```

## Java中字符和字符串时两个类型

char是字符，string是字符串

## 常见异常

### java.lang.NullPointerException

空指针异常

### java.lang.ClassCastException

强制类型转换异常

 使用instanceof运算符可以避免出现，该运算符执行结果类型是布尔类型，结果可能是true/false；(a instanceof Animal)：true表示a这个引用指向的对象是一个Animal类型；false表示a这个引用指向的对象不是一个Animal类型。instanceof运算符语法：`引用 instanceof 数据类型`。

### java.lang.ArrayIndexOutOfBoundsException

数组下标越界异常

### java.lang.NumberFormatException

数字格式化异常，如下代码会报异常。

```java
package org.example;

public class Main {
    public static void main(String[] args) {
        Integer integer = new Integer("中文");
        int i = integer.intValue();
        System.out.println(i);
    }
}

```

## 面试题

```java
String s1 = new String("hello");
String s2 = new String("hello");

```

以上代码中，一共三个对象：在方法区常量池中有一个`"hello"`对象，堆内存当中有两个new出来的`String`对象。

判断数组长度和判断字符串长度不一样：判断数组长度使用length属性；判断字符串长度使用length()方法。

```java
package org.example;

public class Main {
    public static void main(String[] args) {
        int[] arr = {1,2,3};
        System.out.println(arr.length);

        System.out.println("abc".length());
    }
}

```

### finally面试题

```java
package org.example;

public class Main {
    public static void main(String[] args) {
        int v = m();
        System.out.println(v);
    }
    public static int m() {
        int i = 100;
        try {
            // 这行代码出现在 int i = 100; 的下面，所以最终结果必须是返回100
            // return 语句还必须保证是最后执行的。一旦执行，整个方法结束
            return i;
        } finally {
            i++;
        }
    }
}

```

以上代码，虽然在finally那里已经讲过了，try...finally的执行顺序是：先try再finally最后return；但是也要遵循java中的一些铁定的规则，所以导致上述代码返回100，而不是101。

java语法规则（有一些规则是不能破坏的，一旦这么说了，就必须这么做！）：

*   java中的有一条这样的规则：方法体中的代码必须遵循自上而下顺序依次逐行执行（亘古不变的语法）
*   java中还有一条语法规则：return语法一旦执行，整个方法必须结束（亘古不变的语法）