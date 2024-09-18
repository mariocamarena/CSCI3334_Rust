const FREEZE_F: f64 = 32.0;

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - FREEZE_F) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 9.0 / 5.0) + FREEZE_F
}

fn main() {
    let mut fah_temp: f64 = 100.0;
    println!("Current temp: {:.2}°C", fahrenheit_to_celsius(fah_temp));

    for i in 1..=5 {
        let next_temp = fah_temp + i as f64;
        println!("Current temp #{}°F: New temp #{}: {:.2}°C", fah_temp + i as f64, i, fahrenheit_to_celsius(next_temp));
    }
}