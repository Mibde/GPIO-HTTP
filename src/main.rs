use rppal::gpio::Gpio;
use rppal::gpio;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

fn main() {

    let answer = Arc::new(Mutex::new(gpio::Level::Low));
    let answer_ref = answer.clone();
    let answer_ref2 = answer.clone();

    let t = thread::spawn(move || {
        let gpio = Gpio::new().unwrap();
        let pin_analog = gpio.get(23).unwrap().into_input();

        loop {
            let mut answer = answer_ref.lock().unwrap();
            *answer = pin_analog.read();
            drop(answer);
            thread::sleep(Duration::from_secs(1));
        }
    });
    let t2 = thread::spawn(move || {

        let mut last = gpio::Level::Low;
        loop {
            let answer = answer_ref2.lock().unwrap();
            if *answer != last {
                last = *answer;
                if last == gpio::Level::Low {
                    let _body = reqwest::blocking::get("https://mibde.fr/door&door=open").unwrap_or("OUPS")
                    .text();
                }
                else {
                    let _body = reqwest::blocking::get("https://mibde.fr/door&door=close").unwrap_or("OUPS")
                    .text();
                }
                //println!("Second tread : {}", answer);
            }
                drop(answer);
                thread::sleep(Duration::from_secs(1));
        }
    });

    t.join().unwrap();
    t2.join().unwrap();
}
