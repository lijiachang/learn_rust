/*
迭代器负责：
* 遍历每个项
* 确定序列遍历合适完成


iterator trait
仅要求实现一个方法：next
next：
* 每次返回迭代器中的一项
* 返回结果包裹在Some里
* 迭代结束，返回None
可以直接在迭代器上调用next方法

几个迭代方法：
iter方法：在不可变引用上创建迭代器 （是指被迭代的元素不可变）
into_iter方法：创建的迭代器会获得所有权
iter_mut方法：迭代可变的引用

产生其他迭代器的方法
定义在Iterator trait上的另一些方法叫做：迭代器适配器（把迭代器转换为不同种类的迭代器）
例如map（和Python的map方法很像）：接收一个闭包，闭包作用于每个元素，产生一个新的迭代器

使用闭包捕获环境
filter方法（和Python的filter方法很像）：
*/

#[cfg(test)]
mod test {
    #[test]
    fn iterator_test() {
        let v1 = vec![1, 2, 3];
        let mut v1_iter = v1.iter();
        // 使用mut的原因是每次调用next的时候，都改变了迭代器中有个属性记录迭代器的位置
        // 但是使用for循环不需要加mut，是因为for循环会取得iter的所有权，然后把他变为可变对象

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
    }

    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];
        let mut v1_iter = v1.iter();

        let sum: u32 = v1_iter.sum();
        assert_eq!(sum, 6);
    }

    #[test]
    fn iterator_map() {
        let v1 = vec![1, 2, 3];
        let v1_map_iter = v1.iter().map(|x| x + 1);
        // 此时的v1_map_iter还是一个迭代器，没有对元素+1，除非后面有代码消耗他才开始计算

        // 比如使用collect
        let result: Vec<_> = v1_map_iter.collect(); //Vec<_>表示然编译器自己推断
        assert_eq!(result, vec![2, 3, 4]);
    }

    //#[derive]属性用于自动为结构体或枚举派生一些特定的trait实现，从而避免了手动实现它们所需的样板代码
    // PartialEq是一个trait，它允许你比较两个类型的实例是否相等。如果一个类型实现了PartialEq，你可以使用==和!=运算符来比较该类型的两个实例。在测试中，当你需要断言两个实例是否相等时，这会非常有用。
    // Debug是一个trait，它允许你使用{:?}或{:#?}格式化符号来打印类型的实例，这在调试时特别有用，因为你可以很容易地看到一个实例的内部值。
    #[derive(PartialEq, Debug)]
    struct Shoe {
        size: u32,
        style: String,
    }

    fn shoes_in_my_size(shoes: Vec<Shoe>, size: u32) -> Vec<Shoe> {
        // 使用into_iter获得所有权
        shoes.into_iter().filter(|x| x.size == size).collect()
    }

    #[test]
    fn iterator_filter() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("a"),
            },
            Shoe {
                size: 12,
                style: String::from("b"),
            },
            Shoe {
                size: 10,
                style: String::from("c"),
            },
        ];

        let my_shoes = shoes_in_my_size(shoes, 10);

        assert_eq!(
            my_shoes,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("a"),
                },
                Shoe {
                    size: 10,
                    style: String::from("c"),
                },
            ]
        )
    }

    #[test]
    fn my_iterator() {
        struct Counter {
            count: u32,
        }

        impl Counter {
            fn new() -> Counter {
                Counter { count: 0 }
            }
        }

        impl Iterator for Counter {
            type Item = u32;
            /*
            type关键字在这个上下文中用于定义一个类型别名或者在trait定义中指定关联类型。
            Item是一个关联类型，它在Iterator trait中被用来指定迭代器将产生的元素的类型。
            这意味着当你为某个类型实现Iterator trait时，你需要指定Item的具体类型，
            以表明你的迭代器会产生什么类型的值。
            */

            fn next(&mut self) -> Option<Self::Item> {
                // 大写的Self指的是实现Iterator trait的类型本身，在这个例子中就是Counter
                // 因此，Self::Item相当于Counter::Item，即u32。
                if self.count < 5 {
                    self.count += 1;
                    Some(self.count)
                } else {
                    None
                }
            }
        }

        // 测试自定义的迭代器
        let mut count = Counter::new();
        assert_eq!(count.next(), Some(1));
        assert_eq!(count.next(), Some(2));
        assert_eq!(count.next(), Some(3));
        assert_eq!(count.next(), Some(4));
        assert_eq!(count.next(), Some(5));
        assert_eq!(count.next(), None);

        // 测试两个迭代器的操作
        let sum: u32 = Counter::new()
            .zip(Counter::new().skip(1)) // 两个迭代器zip到一起， 第二个迭代器skip前n个元素，也就是跳过1
            .map(|(x, y)| x * y) // 两个迭代器的元素相乘
            .filter(|x| x % 3 == 0) // 过滤出得到的乘数能把3整除的结果
            .sum(); // 把结果求和

        assert_eq!(sum, 18);
    }
}
