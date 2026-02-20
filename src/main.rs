use rppal::gpio::Gpio;
use rppal::pwm::{Channel, Polarity, Pwm};
use std::thread::sleep;
use std::time::Duration;
use std::error::Error;
use std::thread;
use std::sync::Arc;

fn main() {
    println!("Starting program...");
    let gpio = Gpio::new().expect("Error initializing GPIO service");
    blink_led(&gpio).expect("Error blinking LED");
    // Because stupid thread thing we can't pass reference
    // let gpio = Arc::new(Gpio::new().expect("Failed to init gpio service"));
    //
    // // For now, we're just gonna throw this in a separate thread and move on with our lives
    // let gpio_clone = Arc::clone(&gpio);
    // thread::spawn(move || {
    //     blink_led(gpio_clone).expect("Error blinking LED"); // Remember to change this
    // });
    //
    // let gpio_clone2 = Arc::clone(&gpio);
    // thread::spawn(move || {
    //     spin_servo(gpio_clone2).expect("Error spinning");
    // });

    spin_servo().expect("Problem spinning the bitch");

    println!("Program complete...");
}

fn blink_led(gpio: &Gpio) -> Result<(), Box<dyn Error>> {
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
    Ok(())
}

fn spin_servo() -> Result<(), Box<dyn Error>> {
    println!("Sit and spin");
    let pwm = Pwm::with_period(
        Channel::Pwm1,
        Duration::from_millis(20),
        Duration::from_micros(1500),
        Polarity::Normal,
        true
    ).expect("Failed configuring PWM...");

    set_servo_angle(&pwm, 0.0).expect("Failed to zero servo");
    std::thread::sleep(Duration::from_secs(1));
    set_servo_angle(&pwm, 45.0).expect("Error spinning the bitch...");

    Ok(())
}

fn set_servo_angle(pwm: &Pwm, angle: f64) -> rppal::pwm::Result<()> {
    // Map 0-180 degrees to .5ms - 2.5ms pulse width
    let pulse_ms = 0.5 + (angle / 180.0) * 2.0;
    pwm.set_pulse_width(Duration::from_micros((pulse_ms * 1000.0) as u64))?;
    Ok(())
}