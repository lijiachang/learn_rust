/*
实现Drop Trait 可以让我们自定义：当值将要离开作用域时发生的工作。
例如 文件、网络资源的释放

要求实现drop方法，该方法的参数是对self的可变引用
Drop Trait在预导入模块里面 （prelude）


没有办法手动调用Drop Trait中的drop方法
但是可以使用标准库中的sdt::mem::drop函数，来提前drop值
drop函数是预导入的 （prelude）
*/

struct CustomSmartPointer{
    data:String,
}

impl Drop for CustomSmartPointer{
    fn drop(&mut self) {
        println!("dropping with data:{}", self.data)
    }
}

pub fn main() {
    let _a = CustomSmartPointer { data: String::from("a") };
    drop(_a); // 提前drop值
    let _b = CustomSmartPointer { data: String::from("b") };
    println!("CustomSmartPointer created");
}