/*
高级 Trait

关联类型：
在Trait定义中使用关联类型来指定占位类型
关联类型（associated type) 是Trait中的类型占位符，可以用于Trait的方法签名中
    可以定义出包含某些类型的Trait，而在实现前无需知道这些类型是什么

*/

pub trait Iterator {
    type Item; // Item是关联类型。 是类型占位符

    fn next(&mut self) -> Option<Self::Item>;
}

fn main() {
    println!("hello world!")
}

struct Counter {}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
/*

【泛型】和【关联类型】的区别：

泛型：每次实现Trait时标注类型。可以为一个类型多次实现某个Trait（不同的泛型参数）
关联类型：无需标注类型。无法为单个类型多次实现某个Trait
*/

// 下面是用泛型 ,根据不同的泛型参数，多次实现Trait
pub trait Iterator2<T> {
    fn next(&mut self) -> Option<T>;
}

impl Iterator2<String> for Counter {
    fn next(&mut self) -> Option<String> {
        todo!()
    }
}

impl Iterator2<u32> for Counter {
    fn next(&mut self) -> Option<u32> {
        todo!()
    }
}

/*
默认泛型参数和运算符重载

可以在使用泛型参数时为泛型指定一个默认的具体类型。
这种技术常用于运算符重载（operator overloading）
可以通过实现std::ops中列出来的那些trait来重载一部分运算符
*/

// 为自己定义的结构体，实现运算符
use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

// 为两个不同类型的结构体，实现运算符
struct Millimeters(u32);
struct Meters(u32);

// 元组结构体（Tuple Struct）的声明方式。
// 元组结构体是Rust中一种特殊的结构体，它允许你定义一个有名字的结构体，但是它的字段没有名字，只有类型。
// 这种结构体的用途通常是为了给某个特定类型创建一个新的类型标识，而不需要关心里面具体的字段名。

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, rhs: Meters) -> Self::Output {
        Millimeters(self.0 + (rhs.0 * 1000))
    }
}

/*
调用方法的选择
*/

struct Human;

trait Pilot {
    // pilot: 驾驶员
    fn fly(&self);
}

trait Wizard {
    // wizard: 向导
    fn fly(&self);
}

impl Pilot for Human {
    fn fly(&self) {
        todo!()
    }
}
impl Wizard for Human {
    fn fly(&self) {
        todo!()
    }
}

impl Human {
    fn fly(&self) {}
}

fn test_fly() {
    let human = Human;
    println!("fly:{:?}", human.fly()); // 这里是Human自己实现的fly
    println!("fly:{:?}", Pilot::fly(&human)); // 指明是某个Trait的fly
    println!("fly:{:?}", Wizard::fly(&human));
}

/*
完全限定语法（Fully Qualified Syntax）
如何调用同名方法

<Type as Trait>::function(method, arg, ...);
* 可以在任何调用函数或方法的地方使用
* 允许忽略那些从其他上下文能推导出来的部分
* 当Rust无法区分调用哪个具体实现的时候，才用这种语法。
*/

struct Dog;

trait Animal {
    fn baby_name() -> String;
}

impl Dog {
    fn baby_name() -> String {
        String::from("spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

fn test_baby_name() {
    println!("baby dog is called:{}", Dog::baby_name());
    println!("baby dog is called:{}", <Dog as Animal>::baby_name());
}

/*
使用super trait来要求trait附带其他trait的功能
需要在一个trait中使用其他trait的功能：
* 需要被依赖的trait也被实现
* 那个被间接依赖的trait就是当前trait的super trait
*/

use std::fmt;
use std::fmt::Debug;

trait OutlinePrint: fmt::Display { // :后面表示这个trait依赖的trait
    fn outline_print(&self) {
        let output = self.to_string(); // 利用了fmt::Display trait的to_string方法来获取当前对象的字符串表示
        let len = output.len();

        println!("{}", "*".repeat(len + 4));
        println!("*{}*", "".repeat(len + 2))
    }
}

struct Point2 {
    x: i32,
    y: i32,
}

impl OutlinePrint for Point2 {

}

// 还要为Point2实现依赖的Display trait
impl fmt::Display for Point2{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
        // write!是Rust标准库中的一个宏，用于将格式化文本写入到给定的输出流中。
        // 它是println!宏的底层实现之一
        // write!允许你指定输出的目的地，比如字符串、文件或网络流等。

        // write!(destination, format_string, args...)
        // destination：表示写入的目标。这可以是标准输出、文件、字符串...

        // 在实际使用中，write!后面通常会跟一个.unwrap()或者?来处理可能发生的错误。
        // .unwrap()会在出现错误时导致程序panic，而?操作符则会将错误返回给调用者处理，这要求你的函数返回类型兼容Result。
    }
}

/*

使用 newtype 模式在外部类型上实现外部trait
* 孤儿规则： 只有当trait或类型定义在本地包时，才能为该类型实现这个trait
* 可以通过newtype模式来绕过这一规则
    利用tuple struct（元祖结构体）创建一个新的类型
*/

// 比如想为Vec实现Display 这个trait
struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Vec[{}]", self.0.join(", "))
    }
}

pub fn vec_display() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w={}", w)
}