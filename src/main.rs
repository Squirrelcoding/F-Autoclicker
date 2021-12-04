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
    println!("Left Control + Fast clicking = really fast clicking");
    println!("Close the program or Ctrl + Q to quit");


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

    //Quit
    QKey.bind(|| {
        if LControlKey.is_pressed() {
            process::exit(0);
        }
    });
    
    handle_input_events();
}