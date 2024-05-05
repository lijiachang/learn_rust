pub fn main() {
    // 1. 生成string
    let data = "init contents"; //字符串字面值
    let s = data.to_string(); // 转换为字符串
                              //生成string
    let s = String::from("init contents");

    //2. 更新string
    let mut s = String::from("foo");
    // 把一个字符串切片附加到string
    s.push_str("bar"); // 不会获得所有权
                       //把单个字符附加到string
    s.push('a'); // 要用单引号

    // 连接字符串: 加号
    let hello = String::from("hello");
    let world = String::from("world");
    let uni = hello + &world;  // 使用加号，类似是fn add(self, s: &str) -> String
    println!("uni: {uni}");

    // 连接字符串：format宏
    // format不会取得参数的所有权，所有参数在后续都可以使用
    let hello = String::from("hello");
    let world = String::from("world");
    let uni2 = format!("{hello}-{world}");
    println!("uni2: {uni2}");

    // 字符串切片
    // 需要谨慎使用，如果切割时跨域边界，程序会panic。有些字符会占用2个字节或更多
    let hello = String::from("hello");
    let s = &hello[0..2];
    print!("slice hello: {s}");

    // 字符串遍历
    // 1. 如果想得到 标量值：chars()方法
    // 2. 如果想得到 字节：bytes()方法
    // 3. 如果想得到 字形簇：标准库未提供

}
