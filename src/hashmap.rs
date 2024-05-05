use std::collections::HashMap;

// hashmap 数据存储在heap中
// 同构的：所有的key必须是同一个类型，所有的value都必须是同一个类型
pub fn main() {
    // 创建
    let mut hashmap: HashMap<String, i32> = HashMap::new();
    hashmap.insert(String::from("blue"), 10);

    // 使用collect创建
    let teams = vec![String::from("blue"), String::from("yellow")];
    let scores = vec![10, 40];

    let kv: HashMap<_, _> = teams.iter().zip(scores.iter()).collect();
    println!("kv: {:?}", kv);

    // 所有权
    // 对于拥有所有权的值，比如string，值会被移动，所有权会转移给HashMap
    let hello = String::from("hello");
    let world = String::from("world");
    let mut hashmap2 = HashMap::new();
    hashmap2.insert(hello, world); //所有权转移
    // hashmap2.insert(&hello, &world); // 值的所有权不会转移
    // 当时在HashMap有效的期间，被引用的值必须保持有效

    //访问HashMap的值
    let key = String::from("hello");
    let value = hashmap2.get(&key);
    match value {
        None => {println!("not in hashmap")}
        Some(v) => {println!("the value is {v}")}
    }

    //遍历HashMap
    for (k,v) in &hashmap2{
        println!("key is {k}, value is {v}")
    }

    //更新HashMap
    // 1. 直接更新现有的
    hashmap2.insert(String::from("hello"), String::from("fugai"));
    // 2. 检查如果不存在指定的key，再更新
    // entry方法：检查指定的K是否对应一个V
    // Entry的or_insert()方法: 返回k对应v的可变引用。如果k不存在就将新的k插入进入
    hashmap2.entry(String::from("hello")).or_insert(String::from("new_v"));
    println!("updated: {:?}", hashmap2);
    // 3. 基于现有的v更新
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for char in text.split_whitespace(){
        let count = map.entry(char).or_insert(0);
        *count += 1;
    }
    println!("统计单词次数: {:?}", map)

}
