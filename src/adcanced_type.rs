/*

高级类型

1. 使用类型别名创建类型同义词
    给某个类型标注起个别名
*/

// 1.1 简单用法
type Kilometers = i32;

fn type_alias() {
    let x: i32 = 5;
    let y: Kilometers = 5;
    println!("x + y = {}", x + y);
}

// 1.2 组合
type Thunk = Box<dyn Fn() + Send + 'static>;
fn takes_long_type(f: Thunk){
    f();
}

fn returns_long_type() -> Thunk {
    Box::new(|| println!("hi"))
}

// 1.3 部分替换

use std::io::Error;
use std::fmt;

// 代替Result<T,E>中默认的E
// type Result<T> = Result<T, std::io::Error>;
// 但是标准库已经实现了，所以使用下面的更合适
type Result<T> = std::io::Result<T>;

pub trait Write{
    fn write(&mut self, buf:&[u8])-> Result<usize>;
    fn flush(&mut self) -> Result<()>;
}

/*
2. Never类型
有一个名为!的特殊类型：
    它没有任何值，称为 空类型（empty type）
    我们倾向于叫他never类型
不返回值的函数也被称为发散函数（diverging function）

几个例子：
* 如果一个match中的条件返回了continue ，那么就是never类型，但是它会强制转换为其他条件中返回的类型，比如说i32
* 如果一个match中的条件返回了panic!，那么就是never类型 ，因为会中断程序，不会返回值
* 在loop循环中，永远不会结束，那么就是返回never类型。
*/


/*
3. 动态大小
* Rust 需要在编译时确定为一个特定类型的值分配多少空间。
* 动态大小的类型（Dynamic Sized Types, DST) 的概念：
    编写代码时使用只有在运行时才确定大小的值

str就是动态大小的类型（注意不是&str）：只有运行时才能确定字符串的长度
下面的代码无法工作：
let s1:str = "hello";
let s2:str = "h";
使用&str来解决, 也就是字符串切片，可以知道str的地址，str的长度

Rust使用动态大小类型的通用方式
* 附带一些额外的元数据来存储动态信息的大小

另外一种动态大小的类型：trait
* 每个trait都是一个动态大小的类型，可以通过名称对其进行引用。
* 为了将trait用作trait对象，必须将它放置在某种指针之后

*/

/*
4. Sized Trait
为了处理动态大小的类型，Rust提供了一个Sized Trait来确定一个类型的大小在编译时是否已知。
* 编译时可计算出大小的类型会自动实现这一 trait
* Rust还会为每一个泛型函数隐式的添加Sized 约束

默认情况下，泛型函数只能被用于编译时已经知道大小的类型，可以通过特殊语法解除这一限制
<T: ?Sized> 表达一种不确定性，表示T可能是Sized，也可能不是
注意Trait前面加? 只能用于Sized Trait，不能用于其他Trait


比如下面的函数
fn generic<T>(t: T){}
其实会被编译器自动转换为
fn generic<T: Sized>(t: T){}

我们可以手动表示为不确定
fn generic<T: ?Sized>(t: &T){}  // 注意这时候&T，因为T是不确定的
*/



