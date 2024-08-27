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
}

fn main() {
    let current_year: u32 = 2024;
    println!("This year is {}", current_year);
}
