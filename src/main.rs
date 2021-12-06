use std::env;

fn main() {
    let fahrenheits: String = "-f".to_string();
    let args: Vec<String> = env::args().collect();

    let input_temperature: f64 = args[1].parse().unwrap();
    
    let input_degrees: String = {
        if args.contains(&fahrenheits) {
            String::from("F")
        } else {
            String::from("C")
        }
    };
    let output_degrees: String = {
        if args.contains(&fahrenheits) {
            String::from("C")
        } else {
            String::from("F")
        }
    };
    let result: f64 = {
        if args.contains(&fahrenheits) {
            convert_temperature(&Temperature::F(input_temperature))
        } else {
            convert_temperature(&Temperature::C(input_temperature))
        }
    };

    println!("{}", result);
    println!("{}{} = {}{}", input_temperature, input_degrees, result, output_degrees);
}

enum Temperature {
    C(f64),
    F(f64),
}


fn convert_temperature(input_temp: &Temperature) -> f64 {
    match input_temp {
        &Temperature::C(degrees) => (degrees * 1.8) + 32.0,
        &Temperature::F(degrees) => (degrees - 32.0) / 1.8,
    }
}
