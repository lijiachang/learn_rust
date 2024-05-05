use crate::trait_::Summary;

/*trait 作为参数

1.
impl trait 语法：使用于简单情况
fn test(a: impl Summary)

2.
trait bound语法：可用于复杂情况
fn test<T:Summary+Display, U:Clone+Debug>(a:T, b:U)

3.
使用where, 在方法签名后面指定where子句
fn test<T, U>(a:T, b:U)
where T: Summary+Display,
      U: Cone+Debug,

trait 作为返回类型
fn test() -> impl Summary
注意：impl Trait只能返回确定的同一个类型，不能返回不同类型
*/