use rppal::gpio::Gpio;
use std::error::Error;
use std::thread::sleep;
use std::time::Duration;

/// Blink them lights
pub fn blink_led(gpio: &Gpio) -> Result<(), Box<dyn Error>> {
    println!("Executing blink function...");
    // Set the pin number and into_output to configure for writing
    let mut pin = gpio.get(17)?.into_output();

    // Loop to flash
    for _ in 0..5 {
        pin.set_high(); // Turns the LED on
        println!("LED on!");
        sleep(Duration::from_millis(500));

        pin.set_low();
        println!("LED off!");
        sleep(Duration::from_millis(500));
    }

    println!("Blink function complete...");
    // TODO - release the pin here?
    Ok(())
}
