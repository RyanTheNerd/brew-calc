use std::io;
use brew_calc::*;

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

fn main() {
    println!("Welcome to the brew calculator!");
    loop {
        println!("What calculation would you like to compute? (0 to exit)");
        println!("1.  Sugar + Vol => Brix + ABV");
        println!("2.  Brix => ABV");
        println!("3.  Brix Booster");
        println!("4.  ABV => Brix");
        
        let mut calc_type = String::new();
        io::stdin().read_line(&mut calc_type)
            .expect("Failed to read line!");
        let calc_type: u32 = match calc_type.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        if calc_type == 0 {
            return;
        }
        else if calc_type == 1 {
            let sugar = get_value("What's the mass of the sugar? ");
            let vol = get_value("What's the volume? ");
            let abv = sug_to_abv(sugar, vol);
            
            println!("Alcohol by volume: {:.2}%", abv);
        }
        else if calc_type == 2 {
            let brix = get_value("What's the brix?");
            let abv = brix_to_abv(brix);
            println!("ABV: {:.2}%", abv);
        }
        else if calc_type == 3 {
            let abv = get_value("What's the ABV?");
            let brix = abv_to_brix(abv);
            println!("Brix: {:.2}", brix);
        }
        else if calc_type == 4 {
            let brix = get_value("Please give the current brix");
            let target_brix = get_value("Now give the target brix!");
            let vol = get_value("Finally, give the volume (ml)!");
            let sugar_addition = boost_brix(vol, brix, target_brix);
            println!("To reach a brix of {:.2}, add {:.2}g of sugar!", brix, sugar_addition);
        }
    }
}


