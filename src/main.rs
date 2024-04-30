use std::io;
use brew_calc::*;

fn main() {
    let mut calc_type = String::new();
    println!("Welcome to the brew calculator!");
    println!("What calculation would you like to compute?");
    println!("1.  Sugar + Vol => Brix + ABV");
    println!("2.  Brix => ABV");
    println!("3.  ABV => Brix");
    loop {
        io::stdin().read_line(&mut calc_type)
            .expect("Failed to read line!");
        let calc_type: u32 = match calc_type.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        if calc_type == 1 {
            let sugar = get_value("What's the mass of the sugar: ");
            let vol = get_value("What's the volume: ");
            let abv = sug_to_abv(sugar, vol);
            
            println!("Alcohol by volume: {abv}%");
        }
        else if calc_type == 2 {
            let brix = get_value("What's the brix: ");
            let abv = brix_to_abv(brix);
            println!("ABV: {abv} %");
        }
        else if calc_type == 3 {
            let abv = get_value("What's the ABV: ");
            let brix = abv_to_brix(abv);
            println!("Brix: {brix}");
        }
    }
}

fn get_value(prompt: &str) -> f64 {
    loop {
        println!("{}", prompt);
        let mut val = String::new();
        io::stdin()
            .read_line(&mut val)
            .expect("Failed to read line!");
        let val: f64 = match val.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        return val
    }
}
