pub trait Draw {
    fn draw(&self);
}

pub struct Screen{
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self){
        for component in self.components.iter(){
            component.draw();
        }
    }

}



// 具体实现
pub struct Button{
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button{
    fn draw(&self){
        println!("绘制Button的逻辑");
    }
}

// 具体实现
pub struct SelectBox{
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox{
    fn draw(&self){
        println!("绘制SelectBox的逻辑");
    }
}

fn main() {
    let screen: Screen = Screen{
        components: vec![
            Box::new(SelectBox{
                width: 75,
                height: 10,
                options: vec![
                    String::from("yes"),
                    String::from("maybe"),
                    String::from("no"),
                ],
            }),
            Box::new(Button{
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),

        ],
    };
    screen.run();
}