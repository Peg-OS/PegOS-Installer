mod install_dialogues;

use std::io;
use crate::install_dialogues::{bootable_usb, this_device, usb_connected, wireless};

fn main() {
    println!(" /$$$$$$$$                        /$$$$$$$   /$$$$$$$ ");
    println!("| $$___  $$                      /$$___  $$ /$$___  $$");
    println!("| $$   \\ $$ /$$$$$$$   /$$$$$$$ | $$   \\ $$| $$   \\__/");
    println!("| $$$$$$$$//$$___  $$ /$$___  $$| $$   | $$|  $$$$$$$ ");
    println!("| $$_____/| $$$$$$$$$| $$   \\ $$| $$   | $$ \\_____  $$");
    println!("| $$      | $$______/| $$   | $$| $$   | $$ /$$   \\ $$");
    println!("| $$      |  $$$$$$$$|  $$$$$$$$|  $$$$$$$/|  $$$$$$$/");
    println!("|___/      \\________/ \\_____  $$ \\_______/  \\_______/ ");
    println!("                      /$$   \\ $$                      ");
    println!("                     |  $$$$$$$/                      ");
    println!("                      \\______/                        ");
    println!("                        Setup                         ");
    println!("\n\n\nWelcome to the installation!");
    
    starting();
}

fn starting() {
    println!("How would you like to continue?");
    println!("1: Installation\n2: Repair\n3: Modification");
    print!("Input: ");

    print!("Input: ");
    let selection:i32 = selection();
    clear_screen();

    if selection == 1 {
        install_dialogue()
    } else if selection == 2 {
        repair_dialogue()
    } else if selection == 3 {
        modification_dialogue()
    } else {
        clear_screen();
        println!("Invalid Selection!");
        starting();
    }
}

fn install_dialogue() {
    println!("Where would you like to install?");
    println!("1: This Device\n2: Over USB connected device\n3: Make bootable USB installer\n4: Wireless (Bluetooth or Wi-Fi)");

    print!("Input: ");
    let selection:i32 = selection();
    match selection {
        1 => { this_device() }
        2 => { usb_connected() }
        3 => { bootable_usb() }
        4 => { wireless() }
        _ => {
            clear_screen();
            println!("Invalid Selection!");
            install_dialogue();
        }
    }
}

fn repair_dialogue() {
    println!("Repair is currently not supported. Please contact a developer for support regarding repair of your system.");
    starting()
}

fn modification_dialogue() {
    println!("Modification is currently not supported. It will come soon.");
    starting()
}

fn selection() -> i32 {
    let mut input:String = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let selection: i32 = input.trim().parse().expect("Invalid input");
    selection
}

fn input() -> String {
    let mut input:String = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input
}

fn clear_screen() {
    print!("{}[2J", 27 as char)
}
