use std::error::Error;
use std::thread;
use std::time::Duration;

use rppal::gpio::Gpio;
use rppal::system::DeviceInfo;

const GPIO_LED: u8 = 27;

pub fn blink() -> Result<(), Box<dyn Error>> {
    println!("Blinking an LED on a {}.", DeviceInfo::new()?.model());

    let mut pin = Gpio::new()?.get(GPIO_LED)?.into_output();

    thread::spawn(move || {
        pin.set_high();
        thread::sleep(Duration::from_millis(1000));
        pin.set_low();
    });

    Ok(())
}
