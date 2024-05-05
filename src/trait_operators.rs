#[test]
fn main() {
    use std::cmp::{Ord, Ordering, PartialEq, PartialOrd};

    //假如我们有一个“员工”对象，我们想要按员工的薪水排序，如果我们想要使用Vec::sort()方法，我们就需要实现这个对象的各种“比较”方法。
    // 这些方法在 std::cmp 内—— 其中有四个Trait : Ord、PartialOrd 、Eq 和 PartialEq  。

    #[derive(Debug)]
    struct Employee {
        name: String,
        salary: i32,
    }
    impl Ord for Employee {
        fn cmp(&self, rhs: &Self) -> Ordering {
            self.salary.cmp(&rhs.salary) // 小、大、相等
        }
    }
    impl PartialOrd for Employee {
        fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
            Some(self.cmp(rhs))
        }
    }
    impl Eq for Employee {}
    impl PartialEq for Employee {
        fn eq(&self, rhs: &Self) -> bool {
            self.salary == rhs.salary
        }
    }

    let mut v = vec![
        Employee {
            name: String::from("Bob"),
            salary: 2048,
        },
        Employee {
            name: String::from("Alice"),
            salary: 3208,
        },
        Employee {
            name: String::from("Tom"),
            salary: 2359,
        },
        Employee {
            name: String::from("Jack"),
            salary: 4865,
        },
        Employee {
            name: String::from("Marray"),
            salary: 3743,
        },
        Employee {
            name: String::from("Hao"),
            salary: 2964,
        },
        Employee {
            name: String::from("Chen"),
            salary: 4197,
        },
    ];

    // 1. 使用for循环找出薪水最高的
    let mut max_salary = &v[0];
    for e in v.iter() {
        if e > max_salary {
            max_salary = e;
        }
    }
    println!("the max salary is {:?}", max_salary);

    // 2. 使用vec标准方法
    println!("the max salary is {:?}", v.iter().max().unwrap());

    // 3.使用排序方法
    v.sort();  // 从小到大排序
    println!("the max salary is {:?}", v[v.len() - 1]);
}
