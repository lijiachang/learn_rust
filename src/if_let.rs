// if let 可以看做是match的语法糖，更简洁

fn main() {
    // match写法
    let v = Some(0u8);
    match v {
        Some(3) => println!("three"),
        _ => println!("other"),
    }

    // if let写法
    if let Some(3) = v{
        println!("three");
    } else {
        println!("other");
    }
}