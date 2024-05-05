use std::any::Any;
use std::f32::consts::PI;

#[test]
fn main() {
    struct Rectangle {
        width: u32,
        height: u32,
    }

    struct Circle {
        x: u32,
        y: u32,
        radius: u32,
    }

    trait IShape {
        fn area(&self) -> f32;
        fn to_string(&self) -> String;
    }

    impl IShape for Rectangle {
        fn area(&self) -> f32 {
            (self.width * self.height) as f32
        }

        fn to_string(&self) -> String {
            format!(
                "Rectangle: width={} height={} area={}",
                self.width,
                self.height,
                self.area()
            )
        }
    }

    impl IShape for Circle {
        fn area(&self) -> f32 {
            (self.radius * self.radius) as f32 * PI
        }

        fn to_string(&self) -> String {
            format!("Circle: radius={} area={}", self.radius, self.area())
        }
    }

    // 多态的使用方式（我们使用独占的智能指针类 Box）：
    let rectangle = Rectangle {
        width: 2,
        height: 3,
    };
    let circle = Circle {
        x: 2,
        y: 3,
        radius: 4,
    };
    let mut list: Vec<Box<dyn IShape>> = Vec::new();
    list.push(Box::new(rectangle));
    list.push(Box::new(circle));

    for shape in list.iter() {
        println!("{}", shape.to_string());
    }
    use std::vec::Vec;
}

// 向下转型
// 这是类似于Python的isinstance判断类型
// 在面向对象编程中,向下转型(Downcasting)是一种将父类引用转换为子类引用的操作。这允许我们访问子类特有的属性和方法,但需要谨慎使用,因为如果转型失败,可能会导致运行时错误。
// 在 Rust 中,没有直接的向下转型运算符,因为 Rust 的类型系统在编译时就要求明确类型。然而,我们可以使用模式匹配和 Any trait 来实现类似的功能。Any trait 提供了 is 和 downcast_ref 方法,可以在运行时检查和转换类型。
#[test]
fn test_down_casting() {
    struct Rectangle {
        width: u32,
        height: u32,
    }

    struct Circle {
        x: u32,
        y: u32,
        radius: u32,
    }

    trait IShape: Any + 'static {
        fn as_any(&self) -> &dyn Any;
        fn area(&self) -> f32;
        fn to_string(&self) -> String;
    }

    impl IShape for Rectangle {
        fn as_any(&self) -> &dyn Any {
            self
        }

        fn area(&self) -> f32 {
            (self.width * self.height) as f32
        }

        fn to_string(&self) -> String {
            format!(
                "Rectangle: width={} height={} area={}",
                self.width,
                self.height,
                self.area()
            )
        }
    }

    impl IShape for Circle {
        fn as_any(&self) -> &dyn Any {
            self
        }

        fn area(&self) -> f32 {
            (self.radius * self.radius) as f32 * PI
        }

        fn to_string(&self) -> String {
            format!("Circle: radius={} area={}", self.radius, self.area())
        }
    }

    // 多态的使用方式（我们使用独占的智能指针类 Box）：
    let rectangle = Rectangle {
        width: 2,
        height: 3,
    };
    let circle = Circle {
        x: 2,
        y: 3,
        radius: 4,
    };
    let mut list: Vec<Box<dyn IShape>> = Vec::new();
    list.push(Box::new(rectangle));
    list.push(Box::new(circle));

    for shape in list.iter() {
        if let Some(r) = shape.as_any().downcast_ref::<Rectangle>(){
            println!("type is rectangle: {}", r.to_string());
        } else if let Some(c) = shape.as_any().downcast_ref::<Circle>() {
            println!("type is circle: {}", c.to_string());
        } else {
            println!("other type error")
        }
    }
}
