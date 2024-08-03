// trait 类似于接口，类似于Python abc
pub trait Summary{
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author : String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet{
    pub username:String,
    pub content:String,
    pub reply:bool, // 回复
    pub retweet:bool,
}

impl Summary for Tweet{
    fn summarize(&self) -> String {
        format!("{}, by {}", self.content, self.username)
    }
}

pub fn main() {
    let tweet = Tweet {
        username: String::from("books"),
        content:String::from("this is content."),
        reply:false,
        retweet:false,
    };

    println!("1 new tweet:{}", tweet.summarize())
}

#[test]
fn test_trait_method() {

    trait MyTrait {
        fn method1(&self);
        fn method2(&self) {
            println!("Default implementation of method2");
        }
    }

    struct MyStruct;

    impl MyTrait for MyStruct {
        fn method1(&self) {
            println!("MyStruct's implementation of method1");
        }
        // 未提供 method2 的实现,将继承默认实现
    }

    impl MyStruct {
        //method3 将成为 MyStruct 的关联函数,而不是 MyTrait 的一部分。你只能在 MyStruct 的实例上调用 method3,而不能通过 MyTrait 的引用或指针调用它。
        fn method3(&self) {
            println!("MyStruct's implementation of method3");
        }
    }

    let s = MyStruct;
    s.method1(); // 输出: MyStruct's implementation of method1
    s.method2(); // 输出: Default implementation of method2
    s.method3(); // 输出: MyStruct's implementation of method3
}
