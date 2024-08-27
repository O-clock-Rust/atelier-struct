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

    fn car_age(&self, current_year: u32) -> Result<u32, String> {
        if current_year < self.year {
            Err(String::from("Current year cannot be less than the car's year of manufacture."))
        } else {
            Ok(current_year - self.year)
        }
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
    let current_year: u32 = 1900;
    println!("This year is {}", current_year);

    let my_car = Car::new(
        String::from("Fiat"),
        String::from("Multipla"),
        1998
    );
    println!("{}", my_car.describe());

    match my_car.is_classic(current_year) {
        Some(car) => println!("The {} is a classic!", car.model),
        None => println!("This car is not a classic.")
    }
}
