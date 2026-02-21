pub mod controllers;

use rppal::gpio::Gpio;

fn main() {
    println!("Starting program...");
    let gpio = Gpio::new().expect("Error initializing GPIO service");
    controllers::lights::blink_led(&gpio).expect("Error blinking LED");
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

    controllers::servos::spin_servo().expect("Problem spinning the bitch");

    println!("Program complete...");
}
