use rppal::gpio::Gpio;
use std::thread::sleep;
use std::time::Duration;
use std::error::Error;

fn main() {
    println!("Starting program...");
    blink_led().expect(""); // Remember to change this
}

fn blink_led() -> Result<(), Box<dyn Error>> {
    // Init the GPIO interface
    let gpio = Gpio::new()?;

    // Set the pin number and into_output to configure for writing
    let mut pin = gpio.get(17)?.into_output();

    // Loop to flash
    for _ in 0..50 {
        pin.set_high(); // Turns the LED on
        println!("LED on!");
        sleep(Duration::from_millis(500));

        pin.set_low();
        println!("LED off!");
        sleep(Duration::from_millis(500));
    }

    Ok(())
}