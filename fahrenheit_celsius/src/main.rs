use std::io;

fn main() {
    println!("Print temperature in Fahrenheit");

    let mut input_text = String::new();

    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read temperature from stdin");

    let temperature_in_fah: f32 = match input_text.trim().parse() {
        Ok(num) => num,
        Err(err) => {
            println!("Got error trying to parser temperature: {}", err);
            return
        },
    };

    let cel = celsius_from_fahrenheit(temperature_in_fah);

    println!("Temperature in Celsiust is {}", cel);
}

fn celsius_from_fahrenheit(fah: f32) -> f32 {
    (fah - 32.0) * 5.0 / 9.0
}
