use std::rc::Rc;
use std::rc::Weak;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;

#[test]
fn main() {
    //声明两个未初始化的指针变量
    let weak;
    let mut strong;
    {
        let five = Rc::new(5); //局部变量
        strong = five.clone(); //进行强引用
        weak = Rc::downgrade(&five); //对局部变量进行弱引用
    }

    //此时，five已析构，所以 Rc::strong_count(&strong)=1， Rc::weak_count(&strong)=1
    println!(
        "strong_count = {}, weak_count = {}",
        Rc::strong_count(&strong),
        Rc::weak_count(&strong)
    );

    //如果调用 drop(strong)，那个整个内存就释放了
    //drop(strong);

    //如果要访问弱引用的值，需要把弱引用 upgrade 成强引用，才能安全的使用
    //在弱引用需要使用内存的时候需要“升级”成强引用 ，但是这个升级可能会不成功，因为内存可能已经被别人清空了
    //所以，这个操作会返回一个 Option 的枚举值
    match weak.upgrade() {
        Some(r) => println!("{}", r),
        None => println!("None"),
    }

    // 如果你要修改 Rc 里的值，Rust 会给你两个方法，一个是 get_mut()，一个是 make_mut() ，这两个方法都有副作用或是限制。

    // get_mut() 需要做一个“唯一引用”的检查，也就是没有任何的共享才能修改
    //修改引用的变量 - get_mut 会返回一个Option对象
    //但是需要注意，仅当（只有一个强引用 && 没有弱引用）为真才能修改
    drop(weak); // 析构掉弱引用
    if let Some(val) = Rc::get_mut(&mut strong) {
        *val = 555;
        println!("rc value changed.")
    }
    println!("strong = {}", &strong);

    // make_mut() 则是会把当前的引用给clone出来，再也不共享了， 是一份全新的。
    //此处可以修改，但是是以 clone 的方式，也就是让strong这个指针独立出来了。
    *Rc::make_mut(&mut strong) = 666;
    println!("strong = {}", &strong);
    // 如果不这样做，就会出现很多内存不安全的情况。这些小细节一定要注意，不然你的代码怎么运作的你会一脸蒙逼的。
}

#[test]
fn test_cell_refcell() {
    use std::cell::Cell;
    use std::cell::RefCell;
    //智能指针，这里还有个选择 – Cell 和 RefCell，
    // 它们弥补了 Rust 所有权机制在灵活性上和某些场景下的不足。他们提供了 set()/get() 以及 borrow()/borrow_mut() 的方法
    //需要注意的是 Cell 和 RefCell 不是线程安全的。在多线程下，需要使用Mutex进行互斥。

    /*

    Cell<T> 提供了内部可变性,允许在具有不可变引用的情况下修改其内部的值。
    Cell<T> 适用于 T 实现了 Copy trait 的类型,如基本类型 i32、bool 等。
    Cell<T> 通过 get 方法获取值,通过 set 方法设置新值。
    Cell<T> 通常用于需要在不可变引用的上下文中修改数据,且数据类型较小且可以复制的情况。

    */
    let x = Cell::new(1);
    let y = &x; //引用（借用）
    let z = &x; //引用（借用）
    x.set(2); // 可以进行修改，x，y，z全都改了
    y.set(3); // 可以进行修改，x，y，z全都改了
    z.set(4); // 可以进行修改，x，y，z全都改了
    println!("x={} y={} z={}", x.get(), y.get(), z.get());

    /*

    RefCell<T> 也提供了内部可变性,允许在运行时借用规则检查的情况下修改其内部的值。
    与 Cell<T> 不同,RefCell<T> 适用于任何类型 T,而不仅限于实现了 Copy trait 的类型。
    RefCell<T> 通过 borrow 和 borrow_mut 方法分别获取不可变和可变引用,在运行时检查借用规则。
    如果违反了借用规则(如同时存在多个可变引用或者同时存在可变引用和不可变引用),RefCell<T> 会在运行时 panic。
    RefCell<T> 通常用于需要在不可变引用的上下文中修改复杂数据结构,或者需要在运行时进行借用规则检查的情况。

    */
    let x = RefCell::new(vec![1, 2, 3, 4]);
    {
        println!("{:?}", *x.borrow())
    }

    {
        let mut my_ref = x.borrow_mut();
        my_ref.push(1);
    }
    println!("{:?}", *x.borrow());

    /*
        使用场景:

        如果你需要在不可变引用的上下文中修改基本类型或实现了 Copy trait 的小型数据,可以使用 Cell<T>。
        如果你需要在不可变引用的上下文中修改复杂数据结构,或者需要在运行时进行借用规则检查,可以使用 RefCell<T>。
        如果你需要在多个所有者之间共享只读数据,可以使用 Rc<T>。

    需要注意的是,Cell<T> 和 RefCell<T> 提供了内部可变性,但它们并不是线程安全的。如果需要在多线程环境中进行内部可变性,应该使用 Mutex<T> 或 RwLock<T>。
        */
}

///线程与智能指针
fn test_thread() {
    const TOTAL_SIZE: usize = 100 * 1000; //数组长度
    const NTHREAD: usize = 6; //线程数

    let data: Vec<i32> = (1..((TOTAL_SIZE + 1) as i32)).collect(); //初始化一个数据从1到n数组
    //需要向每个线程传入一个只读的数组，我们用Arc 智能指针把这个数组包了一层。
    //      注意：Arc 所包的对象是不可变的，所以，如果要可变的，那要么用原子对象，或是用Mutex/Cell对象再包一层。
    let arc_data = Arc::new(data); //data 的所有权转给了 ar_data
    let result = Arc::new(AtomicU64::new(0)); //收集结果的数组(原子操作)

    let mut thread_handlers = vec![]; // 用于收集线程句柄

    for i in 0..NTHREAD {

        // clone Arc 准备move到线程中，只增加引用计数，不会深拷贝内部数据
        let test_data = arc_data.clone();
        let res = result.clone();

        thread_handlers.push(thread::spawn(move || {
            let id = i;
            //找到自己的分区
            let chunk_size = TOTAL_SIZE / NTHREAD + 1;
            let start = id * chunk_size;
            let end = std::cmp::min(start + chunk_size, TOTAL_SIZE);
            //进行求和运算
            let mut sum = 0;
            for i in start..end {
                sum += test_data[i];
            }
            //原子操作
            res.fetch_add(sum as u64, Ordering::SeqCst);
            //fetch_add 是 AtomicU64 提供的原子操作方法,用于原子地将一个值加到原子变量上,并返回原子变量的前一个值。
            //Ordering::SeqCst 指定了内存顺序,表示顺序一致性(Sequentially Consistent)。这是最强的内存顺序保证,确保所有线程观察到内存操作的顺序与程序中指定的顺序一致。
            println!("id={}, sum={}", id, sum);
        }));
    }
    //等所有的线程执行完
    for th in thread_handlers {
        th.join().expect("The sender thread panic!!!");
    }
    //输出结果
    println!("result = {}", result.load(Ordering::SeqCst));
}
