pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}
impl AveragedCollection {
    pub fn new() -> AveragedCollection {
        return AveragedCollection {
            list: vec![],
            average: 0.0,
        };
    }
    pub fn add(&mut self, value: i32) {
        self.list.push(value);

        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();

        match result {
            Some(x) => {
                self.update_average();
                Some(x)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        self.average = self.list.iter().fold(0, |a, b| a + b) as f64 / self.list.len() as f64;
    }
}

pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// pub struct Screen<T: Draw> {
//     pub components: Vec<T>,
// }

// impl<T> Screen<T>
// where
//     T: Draw,
// {
//     pub fn run(&self) {
//         for component in self.components.iter() {
//             component.draw();
//         }
//     }
// }

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("{}", self.width)
    }
}

pub struct TextBox {
    pub width: u32,
    pub height: u32,
    pub text: String,
}

impl Draw for TextBox {
    fn draw(&self) {
        println!("{}", self.width)
    }
}
