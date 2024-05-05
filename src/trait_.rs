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