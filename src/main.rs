struct Car {
    brand: String,
    model: String,
    year: u32,
}

impl Car {
    fn new(brand: String, model: String, year: u32) -> Self {
        Car { brand, model, year }
    }
}

fn main() {
    let current_year: u32 = 2024;
    println!("This year is {}", current_year);
}
