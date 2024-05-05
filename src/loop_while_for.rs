pub fn loop_() {
    let mut counter = 0;

    let x: i32 = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        };
    };

    println!("x is {x}");
}

pub fn while_() {
    let mut number =3;

    while number != 0 {
        println!("number is {number}");
        number -= 1
    }

    println!("end")
}

pub fn for_() {
    let array = [1,2,3,4,5];

    for element in array.iter(){
        println!("element is {element}");
    };

    println!("end");

}

pub fn range_() {
    //实现倒计时 3 2 1
    for n in (1..4).rev(){
        println!("number is {n}");
    };

    println!("end");
}