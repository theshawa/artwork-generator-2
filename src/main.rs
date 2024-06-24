use std::{env, process};

use artwork_generator::generator::Generator;

fn main() {
    let mut config_path = String::from("config.json");
    if env::args().len() > 1 {
        config_path = env::args().nth(1).unwrap();
    }

    let mut generator = match Generator::new(&config_path) {
        Err(e) => {
            eprintln!("{e}");
            process::exit(1);
        }
        Ok(generator) => generator,
    };

    let maximum_generations = generator.config.get_maximum_generations_count();
    println!(
        "total {maximum_generations} artwork{} can be generated.",
        if maximum_generations == 1 { "" } else { "s" }
    );

    let mut count = maximum_generations;
    if env::args().len() > 2 {
        count = match env::args().nth(2).unwrap().parse() {
            Err(val) => {
                eprintln!("invalid count argument: {}", val);
                process::exit(1);
            }
            Ok(val) => val,
        };
    }

    match generator.generate(count) {
        Ok(_) => println!("generated {} artworks", count),
        Err(e) => {
            eprintln!("{e}");
            process::exit(1);
        }
    }
}
