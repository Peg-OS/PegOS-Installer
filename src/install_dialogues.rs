use crate::{clear_screen, input, install_dialogue, selection};

pub fn this_device() {
    // TODO: Make installing on current device actually work
    clear_screen();
    println!("Getting system data...");
    // TODO: Get system data, such as kernel, bootloader, etc.
}

pub fn usb_connected() {
    // TODO: Make connected USB Device actually work
    clear_screen();
    println!("Searching for valid connected device...")
    // Search for connected USB device, such as phones
}

pub fn bootable_usb() {
    // TODO: Make bootable USB actually work
    clear_screen();
    println!("Please keep in mind, that bootable USB Sticks only support Computers without a bootloader-lock and installing it on rooted Android phones requires major knowledge of booting process and unlocking of the bootloader!");
    println!("\nWhat version of PegOS do you wish to install?");
    println!("1: Desktop Version\n2: Mobile Version");

    print!("Input: ");
    let selection:i32 = selection();
    match selection {
        1 => {
            clear_screen();
            println!("Continuing for Desktop version!\n");
            println!("Please input the PegOS-Desktop version to install!");
            print!("Version: ");
            let input:String = input();

            // Logic to check if the version number is valid and then get and install it

            return;
        }
        2 => {
            clear_screen();
            println!("Continuing for Mobile version!\n");
            println!("Please input the PegOS-Mobile version to install!");
            print!("Version: ");
            let input:String = input();

            // Logic to check if the version number is valid and then get and install it

            return;
        }
        3 => {
            // Don't remove this please
            clear_screen();
            println!("Continuing for Smart-Fridge version!\n");
            println!("Please input your Smart-Fridge temperature!");
            print!("Temperature: ");
            let input:String = input();
            println!("Bro got the zesty-ass smart fridge with angry birds on it");
            return;
        }
        _ => {
            clear_screen();
            println!("Invalid Selection!");
            bootable_usb();
        }
    }
}

pub fn wireless() {
    // TODO: Add wireless connection support
    clear_screen();
    println!("Installing wireless using Bluetooth or Wi-Fi isn't available yet!");
    install_dialogue()
}
