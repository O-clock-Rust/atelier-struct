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

    fn is_classic(&self, current_year: u32) -> Result<bool, String> {
        let age = self.car_age(current_year)?;
        Ok(age > 10)
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

    match my_car.is_classic(current_year) {
        Ok(true) => println!("The {} is a classic!", my_car.model),
        Ok(false) => println!("This car is not a classic."),
        Err(e) => println!("Error: {}", e)
    }
}

#[cfg(test)]
mod tests {
    use super::*; // importe la struct et les implémentations dans le module de tests

    fn setup() -> Car {
        Car::new(
            String::from("Fiat"),
            String::from("Multipla"),
            1998
        )
    }

    #[test]
    fn test_car_age() {
        let car = setup();

        // test pour une année valide → plus de 10 ans
        assert_eq!(car.car_age(2024), Ok(26));
        // test pour une année valide → moins de 10 ans
        assert_eq!(car.car_age(2000), Ok(2));
        // test pour une année valide → moins de 10 ans > année construction
        assert_eq!(car.car_age(1998), Ok(0));
        // test pour une année invalide
        assert_eq!(
            car.car_age(1900),
            Err(String::from("Current year cannot be less than the car's year of manufacture."))
        );
    }

    #[test]
    fn test_is_classic() {
        let car = setup();
        
        // test pour une année valide → plus de 10 ans
        assert_eq!(car.is_classic(2024), Ok(true));
        // test pour une année valide → moins de 10 ans
        assert_eq!(car.is_classic(2000), Ok(false));
        // test pour une année valide → moins de 10 ans > année construction
        assert_eq!(car.is_classic(1998), Ok(false));
        // test pour une année invalide
        assert_eq!(
            car.is_classic(1900),
            Err(String::from("Current year cannot be less than the car's year of manufacture."))
        );
    }
}
