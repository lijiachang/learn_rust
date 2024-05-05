#[derive(Debug)]
enum IpAddrKind{
    V4(u8, u8, u8, u8),  //将数据附加到枚举的变体中
    V6(String),
    Move {x:i32, y:i32}, // 匿名结构体
    Flag,
}

impl IpAddrKind{
    fn call(&self) {

    }
}

pub fn ip() {
    let ipv4 = IpAddrKind::V4(127, 0, 0, 1);
    let ipv6 = IpAddrKind::V6(String::from(":1"));

    router(ipv4);
    router(ipv6);
    router(IpAddrKind::Flag);
}

fn router(ip: IpAddrKind) {
    ip.call();
    println!("{:?} has called", ip)
}