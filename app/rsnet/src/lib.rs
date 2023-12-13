use std::{usize, collections::hash_map::DefaultHasher, hash::Hasher, hash::Hash, io::Stdin};

use pyo3::prelude::*;

#[pymodule]
fn rsnet(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Address>()?;
    m.add_class::<Net>()?;
    Ok(())
}
#[pyclass]
#[derive(Debug, Clone, Copy)]
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
        vec![self.first_byte, self.second_byte, self.third_byte, self.fourth_byte]
    }
}
#[pyclass]
pub struct Net {
    network_address: Address,
    broadcast: Address,

    mask: Address,
    wildcard: Address,
    prefix: usize,
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
    pub fn get_prefix(&self) -> usize {
        self.prefix
    }
    pub fn __hash__(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        (self.prefix as u64
        +self.network_address.first_byte as u64
         +self.network_address.second_byte as u64
         +self.network_address.third_byte as u64
         +self.network_address.fourth_byte as u64
         ).hash(&mut hasher);
        hasher.finish()
    }
}

fn prefix(mask: &Address) -> usize {
    let first_binary = format!("{:b}", mask.first_byte);
    let second_binary = format!("{:b}", mask.second_byte);
    let third_binary = format!("{:b}", mask.third_byte);
    let fourth_binary = format!("{:b}", mask.fourth_byte);
    let prefix = first_binary.matches("1").count()
                +second_binary.matches("1").count()
                +third_binary.matches("1").count()
                +fourth_binary.matches("1").count();
    prefix
}

fn broadcast(network_address: Address, mask: &Address) ->Address {
    let first_byte = (network_address.first_byte)|(!mask.first_byte);
    let second_byte = (network_address.second_byte)|(!mask.second_byte);
    let third_byte = (network_address.third_byte)|(!mask.third_byte);
    let fourth_byte = (network_address.fourth_byte)|(!mask.fourth_byte);
    Address {
        first_byte,
        second_byte,
        third_byte,
        fourth_byte,
    }
}
fn wildcard(mask: &Address) -> Address {
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
pub fn create_addresses() -> Vec<Net> {
    let mut netlist: Vec<Net> = vec![];
    let mut iterator: u16 = 1;
    println!("Welcome to CLI net creation!");
    println!("What do you want your base network to have as an address?");

    netlist
}
fn create_base_net() -> Net {
    let mut network_address = String::new();
    let mut mask_or_prefix = String::new();
    let mut is_mask: bool;
    println!("Zadej adresu (formát XXX.XXX.XXX.XXX)");
    std::io::stdin().read_line(&mut network_address).expect("error");
    
    println!("Do you want to use prefix or mask for the base network?");
    std::io::stdin().read_line(&mut mask_or_prefix).expect("error");
    let mask_or_prefix: u8 = match mask_or_prefix.trim().parse::<u8>() {
        Ok(num) => num,
        Err(_) => pass
    };
    if &mask_or_prefix == 1 {
        
    }

    Net::new(Address{first_byte: 192, second_byte: 168, third_byte: 0, fourth_byte: 0,},
             Address{first_byte: 255, second_byte: 255, third_byte: 255, fourth_byte: 0})
}
