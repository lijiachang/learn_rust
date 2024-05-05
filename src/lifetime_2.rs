#[test]
fn main() {
    fn order_string<'a>(s1 : &'a str, s2 : &'a str) -> (&'a str, &'a str) {
        if s1.len() < s2.len() {
            return (s1, s2);
        }
        return (s2, s1);
    }

    let str1 = String::from("long long long long string");
    let str2 = "short string";

    let (short_str,long_str) = order_string(str1.as_str(), str2);

    println!("long={} \nshort={} ", long_str, short_str);
}

#[test]
fn test_struct() {
    // 引用 ref1 和 ref2 的生命周期与结构体一致
    // 生命周期标识 'life 定义在结构体上，被使用于其成员引用上。意思是声明规则——“结构体的生命周期 <= 成员引用的生命周期”
    struct Test <'life> {
        ref_int : &'life i32,
        ref_str : &'life str,
    }

    // 生命周期变量 'life 声明在 impl 上，用于结构体和其方法的入参上。 意思是声明规则——“结构体方法的“引用参数”的生命周期 >= 结构体的生命周期”
    impl<'life> Test<'life>{
        fn set_string(&mut self, value: &'life str){
            self.ref_str = value
        }

        fn set_int(&mut self, value: &'life i32) {
            self.ref_int = value
        }
    }
}