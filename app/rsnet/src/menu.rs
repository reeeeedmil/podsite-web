use std::num::IntErrorKind;

use crate::calculations::*;
use pyo3::prelude::*;

#[pyfunction]
pub fn init_menu() {
    println!(
        r#"
     ____   _     _    _      _    _   _____  _________
    / ___| | |   | |  | |    |  \ | | | ____| |___ ___|
   / /     | |   | |  | |__  |   \| | | |_       | |
   \ \     | |   | |  | /\ \ | |\ | | | __|      | |
 __/ /     | \___/ |  | \/ | | | \  | | |___     | |
|___/      \_______/  |___/  |_|  \_| |_____|    |_|
             "#
    );
    main_menu();
}
fn main_menu() {
    println!(r#"MAIN MENU"#);
    let mut menu_choice;
    let mut base_net: Option<Net> = None;
    let mut net_list: Option<Vec<Net>> = None;
    'main_loop: loop {
        println!(
            r#"Leave the choice empty to quit
1. Create base address
2. Create subnets
3. Print base address"#
        );
        let menu_choice: u8 = loop {
            menu_choice = String::new();
            std::io::stdin().read_line(&mut menu_choice).expect("error");
            break match menu_choice.trim().parse::<u8>() {
                Ok(num) => num,
                Err(error) => {
                    if error.kind() == &IntErrorKind::Empty {
                        break 'main_loop;
                    };
                    println!("{error}");
                    continue;
                }
            };
        };
        match menu_choice {
            // create base net
            1 => base_net = Some(create_base_net()),

            // create subnets (needs base net)
            2 => net_list = Some(create_subnets(&base_net.as_ref().unwrap())),

            // print base net
            3 => match base_net {
                Some(_) => (&base_net.as_ref().unwrap()).display(),
                None => println!("base net not initialized"),
            },

            _ => {
                println!("prazdne");
                continue;
            }
        };
        continue;
    }
}

pub fn create_subnets(base_network: &Net) -> Vec<Net> {
    let mut net_list: Vec<Net> = vec![];
    println!("What do you want your base network to have as an address?");

    println!("Do you want to use number of hosts or prefixes for your networks?");
    println!("1: prefixes\n2: hosts");

    let mut hosts_or_prefixes = String::new();
    let hosts_or_prefixes: u8 = loop {
        std::io::stdin()
            .read_line(&mut hosts_or_prefixes)
            .expect("error");
        break match hosts_or_prefixes.trim().parse::<u8>() {
            Ok(num) => num,
            Err(error) => {
                println!("{error}");
                continue;
            }
        };
    };
    if net_list.is_empty() {}
    net_list
}

#[pyfunction]
fn create_base_net() -> Net {
    let mut mask_or_prefix = String::new();
    let is_mask: bool;

    let base_address: Address = create_address();
    println!("Do you want to use prefix or mask for the base network?");
    println!("1: prefix\n2: mask");
    let mask_or_prefix: u8 = loop {
        std::io::stdin()
            .read_line(&mut mask_or_prefix)
            .expect("error");
        break match mask_or_prefix.trim().parse::<u8>() {
            Ok(num) => num,
            Err(error) => {
                println!("{error}");
                continue;
            }
        };
    };
    match mask_or_prefix {
        1 => is_mask = false,
        2 => is_mask = true,
        _ => {
            println!("Invalid number, defaulting to prefix.");
            is_mask = false
        }
    };
    let mask: Address = loop {
        break match is_mask {
            true => create_mask(),
            false => loop {
                let mut input: String = String::new();
                std::io::stdin().read_line(&mut input).expect("error");
                break match input.trim().parse::<u8>() {
                    Ok(num) => mask_from_prefix(num),
                    Err(error) => {
                        println!("{error}");
                        continue;
                    }
                };
            },
        };
    };
    println!("ahoj");
    let net = Net::new(base_address, mask);
    net.display();
    net
}
fn create_address() -> Address {
    let mut net_bytes: Vec<u8> = Vec::new();
    let mut byte: u8;
    for number in 0..4 {
        println!("Add {}. byte", number + 1);
        byte = loop {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).expect("error");
            break match input.trim().parse::<u8>() {
                Ok(num) => num,
                Err(error) => {
                    println!("{error}");
                    continue;
                }
            };
        };
        net_bytes.push(byte.clone());
    }
    println!(
        "{}.{}.{}.{}",
        net_bytes[0], net_bytes[1], net_bytes[2], net_bytes[3]
    );
    Address::new(net_bytes[0], net_bytes[1], net_bytes[2], net_bytes[3])
}
fn create_mask() -> Address {
    let mut mask_bytes: Vec<u8> = Vec::new();
    let mut byte: u8;
    for number in 0..4 {
        println!("Add {}. byte", number + 1);
        byte = loop {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).expect("error");
            break match input.trim().parse::<u8>() {
                Ok(num) => num,
                Err(error) => {
                    println!("{error}");
                    continue;
                }
            };
        };
        mask_bytes.push(byte.clone());
    }
    println!(
        "{}.{}.{}.{}",
        mask_bytes[0], mask_bytes[1], mask_bytes[2], mask_bytes[3]
    );
    Address::new(mask_bytes[0], mask_bytes[1], mask_bytes[2], mask_bytes[3])
}
