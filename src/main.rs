struct Car {
    brand: String,
    model: String,
    year: u32,
}

impl Car {
    fn new(brand: String, model: String, year: u32) -> Self {
        Car { brand, model, year }
    }

    fn describe(&self) -> String {
        format!("In {}, {} created the {}", self.year, self.brand, self.model)
    }

    fn car_age(&self, current_year: u32) -> u32 {
        current_year - self.year
    }

    fn is_classic(&self, current_year: u32) -> Option<&Self> {
        if self.car_age(current_year) > 10 {
            Some(self)
        } else {
            None
        }
    }
}

fn main() {
    let current_year: u32 = 2024;
    println!("This year is {}", current_year);

    let my_car = Car::new(
        String::from("Fiat"),
        String::from("Multipla"),
        1998
    );
    println!("{}", my_car.describe());
}
