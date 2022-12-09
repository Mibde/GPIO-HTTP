use std::time::Duration;
use std::thread;
use reqwest::blocking::Response;
use reqwest::Error;
use rppal::gpio::Gpio;
use rppal::gpio;

fn send(pin : gpio::Level) -> Result<Response, Error> {
    match pin {
        gpio::Level::Low => reqwest::blocking::get("https://mibde.fr/websocketDoor/?door=open"),
        _ => reqwest::blocking::get("https://mibde.fr/websocketDoor/?door=close"),
    }
}
fn init(pin : gpio::Level) {
    let un_second = Duration::from_millis(500);
    loop {
        match send(pin) {
            Ok(_x) => {
                break;
            },
            Err(_err) => {
                thread::sleep(un_second);
            }
        }
    }
}
fn main() {
    let gpio = Gpio::new().unwrap();
    let mut pin_analog = gpio.get(23).unwrap().into_input().read();
    init(pin_analog);
    let mut last = pin_analog;
    let un_second = Duration::from_millis(500);
    loop {
        pin_analog = gpio.get(23).unwrap().into_input().read();
        if pin_analog != last {
            send(pin_analog);
            last = pin_analog;
        }
        thread::sleep(un_second);
    }
}
