use std::vec;

fn main() {
    let scr = Screen {
        components: vec![
            Box::new(Button {
                width: 75,
                height: 25,
                lable: String::from("submit"),
            }),
            Box::new(DropDownList {
                width: 12,
                height: 22,
                options: vec![String::from("option1"), String::from("option2")],
                option_values: vec![1, 2],
            }),
        ],
    };

    scr.run();
}

struct Screen {
    components: Vec<Box<dyn Draw>>,
}

impl Screen {
    fn run(&self) {
        for comp in self.components.iter() {
            comp.draw();
        }
    }
}

trait Draw {
    fn draw(&self);
}

#[allow(dead_code)]
struct Button {
    width: u32,
    height: u32,
    lable: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Button has been drawn.")
    }
}

#[allow(dead_code)]
struct DropDownList {
    width: u32,
    height: u32,
    options: Vec<String>,
    option_values: Vec<u32>,
}

impl Draw for DropDownList {
    fn draw(&self) {
        println!("DropDownList has been drawn.")
    }
}
