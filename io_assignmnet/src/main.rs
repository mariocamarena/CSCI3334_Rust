use std::io::{self, Read, Write};
use std::fs::File;

struct Car {
    color: String, 
    maker: String,
    year: String,
}

impl Car {
    fn to_string_2(&self) -> String {
        format!("{}\n{}\n{}", self.maker, self.year, self.color)
    }

    fn from_file(path: &str) -> Car {
        let mut file = File::open(path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let mut lines = contents.lines();
        let maker = lines.next().unwrap_or("").to_string();
        let year = lines.next().unwrap_or("").to_string();
        let color = lines.next().unwrap_or("").parse().unwrap();

        Car { maker, year, color }
    }
}

fn reading_from_file() {
    let car = Car::from_file("user_info.txt");
    println!("make: {}", car.maker);
    println!("color: {}", car.color);
    println!("year: {}", car.year);
}

fn main() {
    let mut buffer = String::new();

    print!("What color is your car?: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let color = buffer.trim().to_string();
    buffer.clear();

    print!("What year is your car?: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let year = buffer.trim().to_string();
    buffer.clear();
    
    print!("What make is your car?: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let maker= buffer.trim().to_string();
    buffer.clear();

    //println!("Hello, {} {} {}", maker,year,color);

    let car = Car {
        color: color, 
        maker: maker,
        year: year,
    };

    //println!("Hello, {} {} {}", car.maker,car.year,car.color);

    let mut file = File::create("user_info.txt").unwrap();
    file.write_all(car.to_string_2().as_bytes());

    println!("====From File====");

    reading_from_file();
}