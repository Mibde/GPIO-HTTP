use rppal::gpio::Gpio;
use rppal::gpio;

fn send(pin : gpio::Level::Low) {
    match pin {
        gpio::Level::Low => { let _body = reqwest::blocking::get("https://mibde.fr/websocketDoor/?door=open");},
        _ => {let _body = reqwest::blocking::get("https://mibde.fr/websocketDoor/?door=close");},
    }
}
fn main() {
    let gpio = Gpio::new().unwrap();
    let mut pin_analog = gpio.get(23).unwrap().into_input();
    send(pin_analog);
    let last = pin_analog;
    loop {
        pin_analog = gpio.get(23).unwrap().into_input();
        if pin_analog != last {
            send(pin_analog);
            last = pin_analog;
        }
    }
}
