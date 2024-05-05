#[derive(Debug)]
struct User {
    name: String,
    email: String,
    sign_in_count: u32,
    active: bool,
}

fn build_user(email: String, name: String) -> User {
    User {
        email: email,
        name: name,
        active: true,
        sign_in_count: 1,
    }
}

pub fn main() {
    let mut user_1 = build_user(String::from("abc@qq.com"), String::from("li"));
    user_1.active = false;
    println!("user 1: {:?}", user_1);

    // struct 更新语法
    let mut user2 = User {
        email: String::from("bbb@qq.com"),
        name:String::from("sun"),
        ..user_1  // 表示剩下的值于user 1相同
    };
    println!("user 2: {:?}", user2)
}