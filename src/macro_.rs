macro_rules! say_hello {
    () => {
        println!("hello macro");
    };
}

macro_rules! my_vec {
    (
        $($x:expr),* $(,)?
    ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

macro_rules! my_log {
    ($message:expr) => {
        println!("[INFO] {}", $message);
    };
    ($level:expr, $message:expr) => {
        println!("[{}] {}", $level, $message);
    };
}

macro_rules! define_struct_with_getter {
    (
        struct $name:ident{
            $( $field_name:ident: $field_type:ty, )*
        }
    ) => {
        #[derive(Debug)]
        struct $name{
            $(
                $field_name: $field_type,
            )*
        }

        impl $name{
            $(
                pub fn $field_name(&self) -> &$field_type{
                    &self.$field_name
                }
            )*
        }
    };
}

pub fn main() {
    say_hello!();

    let list = my_vec![1, 2, 3];
    println!("list={:?}", list);

    my_log!("a info log msg");
    my_log!("ERROR", "a error log msg");

    define_struct_with_getter!(
        struct User {
            name: String,
            age: u32,
            email: String,
        }
    );

    let li = User {
        name: "lijiachang".into(),
        age: 30,
        email: "example@gmail.com".into(),
    };

    println!("name={}", li.name());
    println!("age={}", li.age());
    println!("email={}", li.email());
}
