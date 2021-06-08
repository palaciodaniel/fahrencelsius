use std::io;

fn main() {
    println!("\n\nWELCOME TO THE FAHRENHEIT/CELSIUS CONVERTER!\n
    Please select an option:\n
    1) Fahrenheit to Celsius
    2) Celsius to Fahrenheit");
    
    let mut measure = String::new();
    
    io::stdin()
        .read_line(&mut measure)
        .expect("Wrong input!");
    
    let measure: u8 = match measure.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    println!("\nPlease enter the temperature you want to convert.\n
            NOTE: Default is '0'.");

    let mut temperature = String::new();

                io::stdin()
                    .read_line(&mut temperature)
                    .expect("Wrong input!");

                let temperature: i32 = match temperature.trim().parse() {
                    Ok(num) => num,
                    Err(_) => 0,
                };    
    
    match measure {
        1 => {println!("\n- Option 1 ('F' to 'C') selected.\n
                - - RESULT: {} in Fahrenheit is {} in Celsius. - -\n", temperature, (5 as f32/9 as f32 * (temperature as f32 - 32 as f32)));
             },
        2 => {println!("\n- Option 2 ('C' to 'F') selected.\n
                - - RESULT: {} in Celsius is {} in Fahrenheit. - -\n", temperature, ((9 as f32/5 as f32 * temperature as f32) + 32 as f32));
             },
        _ => {println!("\n- - ERROR: Please choose either option 1 or 2! - -\n")},
    };
}
