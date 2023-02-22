use std::{env, process, iter::Map, collections::HashMap, thread};

use minigrep::Config;


fn main() {
    let list = vec![1, 2, 3];
    let mut iter = list.iter();

    assert_eq!(iter.next(), Some(&1));
}


#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Color {
    Red,
    Blue,
}

struct Profile {
    mail: String,
    favorite_color: Option<Color>,
}

struct Tshirt {
    count: i32,
    color: Color,
}

struct Inventory {
    t_shirt_list: HashMap<Color, i32>,
}

impl Inventory {
    fn get_most(&self) -> Color {
        let red_count = self.t_shirt_list.get(&Color::Red).unwrap();
        let blue_count = self.t_shirt_list.get(&Color::Blue).unwrap();
        let color;
        if red_count > blue_count {
            color = Color::Red;
        } else {
            color = Color::Blue;
        }

        color
    }

    pub fn promotion(& mut self) {

        let customer_list = vec![
            Profile{ mail: String::from("a@gmail.com"), favorite_color: Some(Color::Red)  },
            Profile{ mail: String::from("b@gmail.com"), favorite_color: Some(Color::Blue) },
            Profile{ mail: String::from("c@gmail.com"), favorite_color: Some(Color::Blue) },
            Profile{ mail: String::from("d@gmail.com"), favorite_color: None }
        ];

        for customer in customer_list {
            let color = customer.favorite_color.unwrap_or_else(|| self.get_most());

            println!("customer {} get {:?} t_shirt", customer.mail, color);
            let count = self.t_shirt_list.get(&color).unwrap();
            self.t_shirt_list.insert(color, count - 1);
        }
    }

}

