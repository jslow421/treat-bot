use rppal::pwm::{Channel, Polarity, Pwm};
use std::error::Error;
use std::time::Duration;

pub fn spin_servo() -> Result<(), Box<dyn Error>> {
    println!("Sit and spin");
    let pwm = Pwm::with_period(
        Channel::Pwm2,
        Duration::from_millis(20),
        Duration::from_micros(1500),
        Polarity::Normal,
        true,
    )
    .expect("Failed configuring PWM..."); // Todo - handle this error better

    // TODO this is worth studying
    if let Err(e) = set_servo_angle(&pwm, 0.0) {
        println!("Error setting servo angle: {:?}", e);
        return Err(e.into());
    }

    std::thread::sleep(Duration::from_secs(1));
    set_servo_angle(&pwm, 45.0).expect("Error spinning the bitch...");

    Ok(())
}

fn set_servo_angle(pwm: &Pwm, angle: f64) -> rppal::pwm::Result<()> {
    // Map 0-180 degrees to .5ms - 2.5ms pulse width
    let pulse_ms = 0.5 + (angle / 180.0) * 2.0;
    // Return the result - handle error in caller
    pwm.set_pulse_width(Duration::from_micros((pulse_ms * 1000.0) as u64))
}
