/*

闭包：可以捕获其所在环境的匿名函数。
* 是匿名函数
* 可以保存为变量、作为变量
* 可以在一个地方创建闭包，然后在另一个上下文中调用闭包来完成运算

闭包可以不要求标注参数和返回值的类型。
因为闭包通常在狭小的上下文中工作，编译器能推断出类型。
当然也可以手动类型标准。

普通函数：
fn add_one_v1(x: u32) -> u32 {x+1}

闭包函数：
let add_one_v2 = |x: u32| -> u32 {x+1};
let add_one_v3 = |x|  {x+1};
let add_one_v4 = |x|   x+1;

Fn trait
由标准库提供
所有闭包都至少实现了以下trait之一：
* Fn
* FnMut
* FnOnce


闭包可以捕获他们所在的环境
* 闭包可以访问定义它的作用域内的变量，而普通函数则不能。
* 会产生内存开销
闭包从所在环境捕获值的方式
* 取得所有权：FnOnce (所有的闭包都实现了FnOnce)
* 可变借用：FnMut  (没有移动捕获变量的实现了FnMut)
* 不可变借用：Fn  (无需可变访问捕获变量的闭包实现了Fn)

move 关键字
在参数列表前使用move关键字，可以强制闭包取得它所使用的环境值的所有权
* 当将闭包传递给新线程以移动数据使其归新线程所有时，此技术最有用。
let x = vec![1,2,3];
let equal_to_x = move |z| z ==x;
println!("can't use x here: {:?}", x);


*/

use std::collections::HashMap;
use std::thread;
use std::time::Duration;

struct Cache<T>
where
    T: Fn(u32) -> u32
{
    func: T,
    // value: Option<u32>, // 运行闭包之前为None，运行之后是u32
    value_map: HashMap<u32, u32>,
}

impl<T> Cache<T>
    where
    T: Fn(u32) -> u32
{
    pub fn new(func: T) -> Cache<T> {
        Cache {
            func,
            value_map: HashMap::new(),
        }
    }

    pub fn value(&mut self, v: u32) -> u32 {
        match self.value_map.get(&v) {
            None => {
                let new_value = (self.func)(v);
                // self.value = Some(new_value);
                self.value_map.insert(v, new_value);
                new_value
            }
            Some(value) => *value,
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_closure = Cache::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2)); // 模拟耗时操作
        num
    });

    if intensity < 25{
        println!("today, do {} pushups", expensive_closure.value(intensity));
        println!("today, do {} situps", expensive_closure.value(intensity));
    } else {
        if random_number == 3{
            println!("take a break today!");
        } else {
            println!("today, run for {} minutes", expensive_closure.value(intensity));
        }
    }
}

pub fn main() {
    let value = 10;
    let random_number = 7;

    generate_workout(value, random_number);
}