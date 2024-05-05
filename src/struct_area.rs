
#[derive(Debug)]
struct Info {
    width: u32,
    length: u32,
}

pub fn main() {
    let info = Info {
        width: 30,
        length: 50,
    };
    println!("area is {}", get_area(&info));  // &传递一个引用
    // 以便还继续拥有权，下面可以继续使用
    println!("{:#?}", info);
}

fn get_area(info: &Info) -> u32 {
    // 不会获得Info的所有权
    info.width * info.length
}