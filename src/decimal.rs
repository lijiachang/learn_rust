use rust_decimal::prelude::*;
use rust_decimal::Decimal;

#[test]
pub(crate) fn main() {
    let mut amount = Decimal::new(9571, 2); // 代表 95.76
    let scale = 1; // 指定小数点后的位数
    amount = amount.round_dp_with_strategy(scale, RoundingStrategy::ToZero);
    println!("向下取整后的值: {}", amount); // 输出: 向下取整后的值: 95


    let mut amount = Decimal::new(1002000, 4); //
    amount = amount.normalize();
    println!("amount: {}", amount);
}

/*
从字符串创建 Decimal：
rust

use rust_decimal::Decimal;

let d = Decimal::from_str("3.14159").unwrap();

这种方式允许你从一个字符串表示的十进制数字创建 Decimal。如果字符串不是有效的十进制数字格式，from_str 方法会返回一个 Result，需要适当地处理可能的错误。

从整数和小数部分创建 Decimal：
rust

use rust_decimal::Decimal;

let d = Decimal::new(3141, 3);

这种方式允许你从一个整数部分和一个表示小数位数的 scale 参数创建 Decimal。上面的例子创建了数字 "3.141"。

从 i32、i64、u32 或 u64 创建 Decimal：
rust

use rust_decimal::Decimal;

let d1 = Decimal::from(42);
let d2 = Decimal::from(42_u64);

这种方式允许你直接从 i32、i64、u32 或 u64 类型的整数创建 Decimal。

使用 const fn 从整数字面量创建 Decimal：
rust

use rust_decimal::Decimal;

let pi = Decimal::PI;
let e = Decimal::E;

rust_decimal 库提供了一些常量，如 Decimal::PI 和 Decimal::E，可以直接使用。
*/