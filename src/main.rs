use rppal::gpio::Gpio;
use rppal::gpio;
use std::thread;
use std::time::Duration;
fn main() {

    let state = gpio::Level::Low;
    let mut last = gpio::Level::Low;
    
    let gpio = Gpio::new().unwrap();
    let pin_analog = gpio.get(23).unwrap().into_input();

    loop {
        state = pin_analog.read();
        if state != last {
            last = state;
            if last == gpio::Level::Low {
                let _body = reqwest::blocking::get("https://mibde.fr/door&door=open");
            }
            else {
                let _body = reqwest::blocking::get("https://mibde.fr/door&door=close");
            }
        }
        thread::sleep(Duration::from_secs(1));
    }
   
}
