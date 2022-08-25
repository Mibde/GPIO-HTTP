

fn main() {
    let body = reqwest::blocking::get("https://mibde.fr/update_door").unwrap()
    .text().unwrap();

    println!("body = {:?}", body);

}
