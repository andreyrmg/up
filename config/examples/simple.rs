extern crate config;

use config::Config;

fn main() {
    let conf = config::load();
    println!("The answer is: {}", conf.getString("simple-app.answer"));
}
