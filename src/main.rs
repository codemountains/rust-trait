fn main() {
    let dog = Dog {
        name: "Miniature dachshund".to_string()
    };
    let cat = Cat {
        name: "Scottish fold".to_string()
    };

    show_animal_data(dog);
    show_animal_data(cat);
}

trait Animal {
    fn lifespan(&self) -> u32;
    fn name(&self) -> String;
}

struct Dog {
    name: String
}

impl Animal for Dog {
    fn lifespan(&self) -> u32 {
        13
    }

    fn name(&self) -> String {
        self.name.to_string()
    }
}

struct Cat {
    name: String
}

impl Animal for Cat {
    fn lifespan(&self) -> u32 {
        16
    }

    fn name(&self) -> String {
        self.name.to_string()
    }
}

fn show_animal_data<T: Animal>(animal: T) {
    println!("{}!", animal.name());
    println!("Lifespan: {}", animal.lifespan());
}
