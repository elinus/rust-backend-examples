use std::fmt;

struct Animal {
    name: String,
}

trait CatTrait {
    fn meow(&self) {
        println!("Cat meow!");
    }

    fn jump(&self) {
        println!("Cat jumps!");
    }
}

impl CatTrait for Animal {
    fn meow(&self) {
        println!("{} meow!", self.name);
    }
}

struct Cat {
    name: String,
    age: u8,
}

impl fmt::Display for Cat {}

fn main() {
    let coco = Animal {
        name: "Coco".to_string(),
    };
    coco.meow();
    coco.jump();
}
