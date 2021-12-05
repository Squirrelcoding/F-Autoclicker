//Import dependencies
use inputbot::{MouseButton::*, handle_input_events};
use std::{thread,time::Duration};
use inputbot::KeybdKey::{LControlKey, QKey};
use std::process;
use rand::Rng;


//Get random number to match it to a random interval (makes it harder for anti-cheats to detect)
fn get_rands(n: &i32) -> u64 {
    match n {
        1 => 612,
        2 => 600,
        3 => 636,
        4 => 648,
        5 => 440,
        6 => 660,
        7 => 720,
        8 => 840,
        9 => 564,
        _ => 672
    }
}

fn main() { 
    //Menu
    println!("F AUTO-CLICKER");
    println!("The very cool (and totally not sus) auto clicker");
    println!("Left Control + Fast clicking (left and right) = really fast clicking");
    println!("Close the program or Ctrl + Q to quit");
    println!("================================================================================");
    println!("https://github.com/Squirrelcoding/F-Autoclicker");

    //When the left mouse button is clicked execute the code
    LeftButton.bind( || {
        while LeftButton.is_pressed() && LControlKey.is_pressed() {
            let random = rand::thread_rng().gen_range(1..10); //Gets random number
            let time_between_clicks = get_rands(&random);
            thread::sleep(Duration::from_millis(time_between_clicks)); //Interval
            LeftButton.press();
            LeftButton.release();
        }
    });

    //When the right mouse button is clicked execute the code
    //i forgor to add right button in version 1.0 💀
    RightButton.bind( || {
        while RightButton.is_pressed() && LControlKey.is_pressed() {
            let random = rand::thread_rng().gen_range(1..10); //Gets random number
            let time_between_clicks = get_rands(&random);
            thread::sleep(Duration::from_millis(time_between_clicks)); //Interval
            RightButton.press();
            RightButton.release();
        }
    });

    //Quit
    QKey.bind(|| {
        if LControlKey.is_pressed() {
            process::exit(0);
        }
    });
    
    handle_input_events();
}