/*定义方法
在impl块里定义方法
方法的第一个参数可以是&self， 也可以获得其所有权 或 可变借用（& mut self)
这样能更好的组织代码*/
#[derive(Debug)]
struct Rectangle {
    width: u32,
    length: u32,
}

impl Rectangle {
    //定义一个方法
    fn area(&self) -> u32 {
        self.width * self.length
    }

    //是否能容纳另一个长方形
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.length >= other.length
    }

    // 关联函数，没有self，返回自己的一个实例
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            length: size
        }
    }
}

pub fn main() {
    let rect1 = Rectangle {
        width: 30,
        length: 50,
    };
    println!("area is {}", rect1.area());  // &传递一个引用
    // 以便还继续拥有权，下面可以继续使用
    println!("{:#?}", rect1);

    let rect2 = Rectangle {
        width: 10,
        length: 20,
    };

    let rect3 = Rectangle {
        width: 100,
        length: 200,
    };

    println!("{}", rect1.can_hold(&rect2));
    println!("{}", rect1.can_hold(&rect3));

    println!("square is {:?}", Rectangle::square(22))
}
