pub fn main() {
    // 字面值在编译的时候就硬编码了，所以不可变
    // 可以使用from函数从字符串 字面值 创建出String类型
    // String类型为了支持可变性，是在heap上分配内存
    let mut str = String::from("hello");
    // 字符串可以被修改
    str.push_str(", world");

    println!("{str}");
}

pub fn show_move() {
    let s1 = String::from("hello");
    let s2 = s1;

    // print!("{s1}");
    // 这里会报错，因为    let s2 = s1;
    // 这样会让s1变量失效，也就是：移动move ， 类似与浅拷贝，但是会让s1失效
}

pub fn show_clone() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    print!("{s1}, {s2}");
    // 如果真想对heap上面的string数据进行深度拷贝
    // 而不是仅仅拷贝stack上的数据，应该使用clone方法
}

pub fn show_copy() {
    let x = 5;
    let y = x;
    println!("{x}, {y}");
    // copy trait 可以用于像整数一样完全放在stack上面的类型。
    // 如果一个类型实现了copy，那么旧的变量在赋值后仍然可用

    // 一些拥有copy trait的类型：
    // 所有的整数类型 u32等
    // bool
    // char
    // 所有的浮点数类型 如f64
    // 元祖Tuple ，如果其中所有的元素都是copy的，那么这个元祖就是copy的
    //          (i32, i32) 是  (i32, String) 不是

}