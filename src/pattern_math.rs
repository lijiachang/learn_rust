/*

模式匹配

模式是Rust中一种特殊的语法，用于匹配复杂和简单类型的结构
将模式与匹配表达式和其他构造结合使用，可以更好的控制程序的控制流。


*/

// 1. match的arm
/*
match value {
    pattern => expr,
    pattern => expr,
    _ => expr,  // _可以匹配任何东西
}
*/
fn match_() {
    let x = 5;
    match x {
        1..=5 => println!("x is 1~5"),
        6 | 7 => println!("x is 6 or 7"),
        _ => println!("other"),
    }

    // 字符也可以用范围
    let x = 'c';
    match x {
        'a'..='j' => println!("early ascii letter"),
        'k'..='z' => println!("late ascii letter"),
        _ => println!("other"),
    }
}

// 2. if let 作为一种简短的方式等价的代替只有一个匹配性的match，不会检查穷尽性

fn if_let() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        // if let 语句在 Rust 中用于处理只关心某个枚举变体或模式匹配的情况
        println!("use you favorite color: {color}");
    } else if is_tuesday {
        println!("tuesday is green day");
    } else if let Ok(age_) = age {
        if age_ > 30 {
            println!("age > 30");
        } else {
            println!("age <= 30");
        }
    } else {
        println!("use blue")
    }
}

// 3. while let 只要模式继续满足匹配条件，那它允许while循环一直运行
fn while_let() {
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{top}")
    }
}

// 4. for循环中，模式就是紧随for关键字之后的值
fn for_() {
    let nums = vec![1, 2, 3];
    for (index, num) in nums.iter().enumerate() {
        println!("index={index}, num={num}")
    }
}

// 5. let语句也是 模式 let pattern = expr

// 6. 函数的参数也可以是模式
fn print(&(x, y): &(i32, i32)) {
    println!("location: ({x}, {y})")
}

// 7. 解构操作（ 类似与拆包）
// 可以使用模式来结构struct、enum、tuple 从而引用这些类型值的不同部分
fn resolve() {
    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 22, y: 33 };
    let Point { x: a, y: b } = p;
    assert_eq!(a, 22);
    assert_eq!(b, 33);

    match p {
        Point { x, y: 22 } => println!("on the x axis at {x}"), // 要求x值随意，y的值是22
        Point { x: 22, y } => println!("on the y axis at {y}"), // 要求x值是22，y的值随意
        Point { x, y } => println!("on neither axis:({x}, {y})"), // x和y的值都随意
    }

    // 解构嵌套的struct和enum
    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }
    enum Message {
        Quit,
        Move { x: i32, y: i32 }, // 匿名结构体struct
        Write(String),
        ChangeColor(Color), // 枚举嵌套
    }
    let msg = Message::ChangeColor(Color::Hsv(0, 123, 255));

    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("change the color to: red={r}, green={g}, blue={b}");
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("change the color to: hue={h}, saturation={s}, value{v}");
        }
        _ => (),
    }

    // 忽略一个值的某个部分
    let numbers = (1, 2, 3, 4, 5);
    match numbers {
        (first, _, third, _, fifth) => {
            println!("some numbers:{first}, {third}, {fifth}");
        }
    }

    // 使用_不会发生绑定的操作
    let s = Some("hello".to_string());
    if let Some(_) = s {
        // 使用_不会发生绑定操作，不会移动所有权 。 但是如果Some(a) 这时所有权会转移给a，后面s不可在使用了
        println!("found a string");
    }
    println!("{:?}", s);

    // 使用..忽略剩余部分
    struct NewPoint {
        x: i32,
        y: i32,
        z: i32,
    }
    let origin = NewPoint { x: 0, y: 0, z: 0 };
    match origin {
        NewPoint { x, .. } => println!("x is {}", x),
    }
    let numbers = (1, 2, 3, 4, 5);
    match numbers {
        (first, .., last) => println!("some numbers:{first}, {last}"),
    }

    // 给match增加额外条件
    let x = 4;
    let y = false;
    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }

    // @绑定
    // @符号让我们可以创建一个变量，该变量可以在测试某个值是否与模式匹配的同时保存该值
    enum Msg {
        Hello { id: i32 }, // 匿名结构体
    }
    let msg = Msg::Hello { id: 5 };
    match msg {
        Msg::Hello { id: id_var @ 3..=7 } => println!("found id in range: {:?}", id_var), // 匹配同时赋值
        Msg::Hello { id: 10..=12 } => println!("found id in another range"),
        Msg::Hello { id } => println!("found other id: {id}"), //可以直接使用 id 作为变量名，因为这是一种特殊的简写语法，Rust 会自动为你创建一个同名的变量。
    }
}

pub fn main() {
    while_let();
    for_();

    let point = (2, 5);
    print(&point);

    match_();
    resolve();
}
