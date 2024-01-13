use pyo3::prelude::*;
use std::{collections::hash_map::DefaultHasher, hash::Hash, hash::Hasher, str::FromStr};

#[pyclass]
#[derive(Clone, Copy)]
pub struct Address {
    first_byte: u8,
    second_byte: u8,
    third_byte: u8,
    fourth_byte: u8,
}
#[pymethods]
impl Address {
    #[new]
    pub fn new(first_byte: u8, second_byte: u8, third_byte: u8, fourth_byte: u8) -> Self {
        Address {
            first_byte,
            second_byte,
            third_byte,
            fourth_byte,
        }
    }
    #[getter]
    pub fn get(&self) -> Vec<u8> {
        vec![
            self.first_byte,
            self.second_byte,
            self.third_byte,
            self.fourth_byte,
        ]
    }
}
#[pyclass]
pub struct Net {
    network_address: Address,
    broadcast: Address,

    mask: Address,
    wildcard: Address,
    prefix: u8,
}
#[pymethods]
impl Net {
    #[new]
    pub fn new(network_address: Address, mask: Address) -> Self {
        Net {
            network_address,
            broadcast: broadcast(network_address, &mask),

            mask,
            prefix: prefix(&mask),
            wildcard: wildcard(&mask),
        }
    }
    pub fn get_network_address(&self) -> Vec<u8> {
        self.network_address.get()
    }
    pub fn get_broadcast(&self) -> Vec<u8> {
        self.broadcast.get()
    }
    pub fn get_mask(&self) -> Vec<u8> {
        self.mask.get()
    }
    pub fn get_wildcard(&self) -> Vec<u8> {
        self.wildcard.get()
    }
    pub fn get_prefix(&self) -> u8 {
        self.prefix
    }
    pub fn __hash__(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        (self.prefix as u64
            + self.network_address.first_byte as u64
            + self.network_address.second_byte as u64
            + self.network_address.third_byte as u64
            + self.network_address.fourth_byte as u64)
            .hash(&mut hasher);
        hasher.finish()
    }
    pub fn display(&self) {
        println!(
            "network address: {}.{}.{}.{}",
            &self.network_address.first_byte,
            &self.network_address.second_byte,
            &self.network_address.third_byte,
            &self.network_address.fourth_byte
        );
        println!(
            "broadcast: {}.{}.{}.{}",
            &self.broadcast.first_byte,
            &self.broadcast.second_byte,
            &self.broadcast.third_byte,
            &self.broadcast.fourth_byte
        );

        println!(
            "mask: {}.{}.{}.{}",
            &self.mask.first_byte,
            &self.mask.second_byte,
            &self.mask.third_byte,
            &self.mask.fourth_byte
        );
        println!(
            "wildcard: {}.{}.{}.{}",
            &self.wildcard.first_byte,
            &self.wildcard.second_byte,
            &self.wildcard.third_byte,
            &self.wildcard.fourth_byte
        );
        println!("prefix: {}", &self.prefix);
    }
}
pub fn prefix(mask: &Address) -> u8 {
    let first_binary = format!("{:b}", mask.first_byte);
    let second_binary = format!("{:b}", mask.second_byte);
    let third_binary = format!("{:b}", mask.third_byte);
    let fourth_binary = format!("{:b}", mask.fourth_byte);
    let prefix = u8::try_from(
        first_binary.matches("1").count()
            + second_binary.matches("1").count()
            + third_binary.matches("1").count()
            + fourth_binary.matches("1").count(),
    );
    match prefix {
        Ok(num) => num,
        Err(error) => {
            println!("{error}");
            return 0;
        }
    }
}
pub fn mask_from_prefix(prefix: u8) -> Address {
    // needs prefix.clone() in parameters
    let mut prefix_for_calculation = prefix;
    let mut address_vec: Vec<u8> = vec![];
    for _ in 0..4 {
        if prefix_for_calculation >= 8 {
            address_vec.push(255);
            prefix_for_calculation -= 8;
        } else {
            let last_byte: String = format!(
                "{:b}{}",
                prefix_for_calculation.clone(),
                "0".repeat(match usize::try_from(8 - prefix_for_calculation) {
                    Ok(num) => num,
                    Err(err) => {
                        println!("{err}");
                        0
                    }
                })
            );
            let decimal_byte = match u8::from_str_radix(last_byte.as_str(), 2) {
                Ok(num) => num,
                Err(err) => {
                    println!("{err}");
                    0
                }
            };
            address_vec.push(decimal_byte);
            break;
        }
    }
    while address_vec.len() < 4 {
        address_vec.push(0)
    }
    return Address {
        first_byte: address_vec[0],
        second_byte: address_vec[1],
        third_byte: address_vec[2],
        fourth_byte: address_vec[3],
    };
}

pub fn broadcast(network_address: Address, mask: &Address) -> Address {
    let first_byte = (network_address.first_byte) | (!mask.first_byte);
    let second_byte = (network_address.second_byte) | (!mask.second_byte);
    let third_byte = (network_address.third_byte) | (!mask.third_byte);
    let fourth_byte = (network_address.fourth_byte) | (!mask.fourth_byte);
    Address {
        first_byte,
        second_byte,
        third_byte,
        fourth_byte,
    }
}
pub fn wildcard(mask: &Address) -> Address {
    let first_byte = !mask.first_byte;
    let second_byte = !mask.second_byte;
    let third_byte = !mask.third_byte;
    let fourth_byte = !mask.fourth_byte;
    Address {
        first_byte,
        second_byte,
        third_byte,
        fourth_byte,
    }
}
pub fn normalize_number(input: &mut u32) {
    *input += 1; //1, not 2, because that will change only the last bit -> if i added 2, network
                 // with 2 host would turn into 8
                 // if i add 1, the network will stay in prefix 30 as wanted
    *input = u32::pow(
        2,
        match u32::try_from(format!("{:b}", *input).len()) {
            Ok(num) => num,
            Err(error) => {
                println!("{error}");
                0
            }
        },
    )
}
