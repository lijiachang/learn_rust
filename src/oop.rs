/*

面向对象程序设计（英语：Object-oriented programming，缩写：OOP）

trait对象有些类似于其它语言中的对象
他们某种程度上组合数据与行为

trait对象与传统对象不同的地方：
无法对trait对象添加数据

Trait 对象执行的动态派发（dynamic dispatch）
* 产生运行时开销
* 无法在编译过程中确定调用的是哪一个方法
* 编译器会产生额外的代码以便在运行时找出希望调用的方法
*/

pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>, // 只要实现Draw这个trait就可以放到这个Vec里面
    // Box<T> 是一种在堆上分配空间的方式，它允许你将一个值放在堆上而不是栈上，并提供了一个指向它的指针。
    // 这对于处理 trait 对象特别有用，因为它允许你使用动态分发（运行时决定调用哪个方法），这是实现多态的关键。
}

impl Screen {
    pub fn run(&self) {
        // 不关心类型，只关心实现了draw这个trait就可以
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// 下面开始实现具体的类

struct Button {
    width: u32,
    height: u32,
    label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("draw for button")
    }
}

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("draw for select box")
    }
}

pub fn main() {
    let screen = Screen {
        components: vec![
            Box::new(Button {
                width: 32,
                height: 32,
                label: String::from("my button"),
            }),
            Box::new(SelectBox {
                width: 22,
                height: 22,
                options: vec![String::from("big")],
            }),
        ],
    };

    screen.run()
}
