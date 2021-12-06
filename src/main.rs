const CELCIUM_KELVIN_DIFFERENCE: f32 = 273.15;

fn main() {
    println!("temperature-converter");
}

fn celciums_to_kelvins(amount: f32) -> f32 { amount - CELCIUM_KELVIN_DIFFERENCE }

fn kelvins_to_celciums(amount: f32) -> f32 { amount + CELCIUM_KELVIN_DIFFERENCE }
