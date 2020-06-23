#![allow(non_snake_case)]

use std::cmp::Ordering;
use std::fs;
use std::io::Read;
use std::ops::{Add, BitAnd};

#[derive(Copy, Clone, Debug, PartialEq)]
struct Address(u8, u8, u8, u8);

#[derive(Debug, PartialEq)]
struct Subnet {
    base: Address,
    mask: Address,
    subnet_str: String
}

impl std::cmp::PartialOrd for Subnet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            return Some(Ordering::Equal);
        }
        

        if self.base.0 > other.base.0 || self.base.1 > other.base.1 ||
            self.base.2 > other.base.2 || self.base.3 > other.base.3 {
            Some(Ordering::Greater)
        } else {
            Some(Ordering::Less)
        }
    }
}

impl Eq for Subnet {}

impl std::cmp::Ord for Subnet {
    fn cmp(&self, other: &Self) -> Ordering {
        if self == other {
            return Ordering::Equal;
        }

        if self.base.0 > other.base.0 || self.base.1 > other.base.1 ||
            self.base.2 > other.base.2 || self.base.3 > other.base.3 {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    }
}

impl BitAnd for Address {
    type Output = Address;

    fn bitand(self, rhs: Self) -> Self::Output {
        let mut result = Address(0, 0, 0, 0);
        result.0 = self.0 & rhs.0;
        result.1 = self.1 & rhs.1;
        result.2 = self.2 & rhs.2;
        result.3 = self.3 & rhs.3;

        return result;
    }
}

impl Add<u32> for Address {
    type Output = Address;

    fn add(self, rhs: u32) -> Self::Output {
        let mut result = Address(0, 0, 0, 0);

        let first = self.3 as u32 + (rhs & 0xFF);

        if  first > 255 {
            result.2 += 1;
            result.3 = (255 - (rhs & 0xFF)) as u8;
        } else {
            result.3 += first as u8;
        }

        let second = self.2 as u32 + ((rhs >> 8) & 0xFF);

        if second > 255 {
            result.1 += 1;
            result.2 = (255 - ((rhs >> 8) & 0xFF)) as u8;
        } else {
            result.2 += second as u8;
        }

        let third = self.1 as u32 + ((rhs >> 16) & 0xFF);

        if third > 255 {
            result.0 += 1;
            result.1 = (255 - ((rhs >> 8) & 0xFF)) as u8;
        } else {
            result.1 += third as u8;
        }

        let forth = self.0 as u32 + ((rhs >> 24) & 0xFF);

        if  forth > 255 {
            //Error
        } else {
            result.0 += forth as u8;
        }

        return result;
    }
}

impl From<Address> for u32 {
    fn from(address: Address) -> Self {
        let temp = ((address.0 as u32) << 24)  | ((address.1 as u32) << 16) | ((address.2 as u32) << 8) | address.3 as u32;
        return temp;
    }
}

impl Subnet {
    #[cfg(test)]
    fn new(base: Address, mask: Address) -> Subnet {
        Subnet {
            base,
            mask,
            subnet_str: String::new(),
        }
    }

    fn new_str(base: Address, mask: Address, subnet_str: String) -> Self {
        Subnet {
            base,
            mask,
            subnet_str
        }
    }

    pub fn number_hosts(&self) -> u32 {
        let mut number = 0;

        number += number_ones(self.mask.0);
        number += number_ones(self.mask.1);
        number += number_ones(self.mask.2);
        number += number_ones(self.mask.3);

        if number == 32 {
            0
        } else {
            2_u32.pow(32 - (number as u32)) - 2
        }
    }

    pub fn number_addresses(&self) -> u32 {
        return self.number_hosts() + 2;
    }

    pub fn network_address(&self) -> Address {
        self.base & self.mask
    }

    pub fn generate_addresses(&self) -> Vec<Address> {
        let mut addresses = Vec::with_capacity(self.number_addresses() as usize);

        for i in 0..self.number_addresses() as usize {
            addresses.push(self.network_address() + i as u32);
        }

        addresses
    }

    pub fn collision(&self, other_subnet: &Self) -> bool {
        let subnet_address = self.generate_addresses();
        let other_subnet_addresses = other_subnet.generate_addresses();
        let mut collision = false;

        for address in &subnet_address {
            for other_subnet in &other_subnet_addresses {
                if other_subnet == address {
                    println!("{:?}", other_subnet);
                    collision = true;
                }
            }
        }

        collision
    }
}

fn number_ones(byte: u8) -> u8 {
    let mut number = 0;

    for i in 0..8 {
        let bit = (byte >> i) & 0x01;
        number += bit;
    }

    number
}

fn generate_mask(mask: u32) -> Address {
    let mut new_mask = Address(0, 0, 0, 0);
    let mask_bytes = 0xFFFFFFFFu32 << (32 - mask);

    new_mask.0 = ((mask_bytes >> 24) & 0xFF) as u8;
    new_mask.1 = ((mask_bytes >> 16) & 0xFF) as u8;
    new_mask.2 = ((mask_bytes >> 8) & 0xFF) as u8;
    new_mask.3 = ((mask_bytes >> 0) & 0xFF) as u8;

    return new_mask;
}

fn main() {
    let mut file = fs::File::open("./networks.txt").unwrap();
    let mut file_string = String::new();
    let _a = file.read_to_string(&mut file_string);
    let subnet_string = file_string.split("\r\n");
    let mut subnets: Vec<Subnet> = Vec::new();

    for subnet_str in subnet_string {
        let subnet_split: Vec<&str> = subnet_str.split('/').collect();
        let base_split_str: Vec<&str> = subnet_split[0].split('.').collect();

        let first_net = base_split_str[0].parse::<u8>().unwrap();
        let second_net = base_split_str[1].parse::<u8>().unwrap();
        let third_net = base_split_str[2].parse::<u8>().unwrap();
        let forth_net = base_split_str[3].parse::<u8>().unwrap();

        let base = Address(first_net, second_net, third_net, forth_net);

        let mask_str = subnet_split[1];
        let mask_u32 = mask_str.parse::<u32>().unwrap();

        let mask = generate_mask(mask_u32);

        let subnet = Subnet::new_str(base, mask, subnet_str.to_string());

        subnets.push(subnet);
    }

    subnets.sort();

    for subnet_first in &subnets {
        let index = subnets.binary_search(subnet_first).unwrap() + 1;

        for subnet_second in  subnets.iter().skip(index) {
            if subnet_first != subnet_second {
                let collision = subnet_first.collision(subnet_second);
                println!("Collision between {} and {}: {}", subnet_first.subnet_str, subnet_second.subnet_str, collision);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Add;

    use crate::{Address, generate_mask, Subnet};

    #[test]
    fn host_count() {
        let subnet = Subnet::new(Address(140, 10, 10, 0), Address(255, 255, 255, 128));
        let hosts = subnet.number_hosts() ;
        assert_eq!(hosts, 126);
    }

    #[test]
    fn add_address() {
        let address = Address(140, 10, 10, 0);
        assert_eq!(address.add(0), Address(140, 10, 10, 0));
        assert_eq!(address.add(1), Address(140, 10, 10, 1));
        assert_eq!(address.add(256), Address(140, 10, 11, 0));
        assert_eq!(address.add(100), Address(140, 10, 10, 100));
        assert_eq!(address.add(65536), Address(140, 11, 10, 0));
    }

    #[test]
    fn check_addresses() {
        let address_1 = Address(140, 10, 10, 0);
        let address_2 = Address(140, 10, 10, 0);
        let subnet_mask = Address(255, 255, 255, 240);

        let subnet_1 = Subnet::new(address_1, subnet_mask);
        let subnet_2 = Subnet::new(address_2, subnet_mask);

        assert_eq!(true, subnet_1.collision(&subnet_2));

        let address_2 = Address(140, 20, 10, 0);
        let subnet_2 = Subnet::new(address_2, subnet_mask);

        assert_eq!(false, subnet_1.collision(&subnet_2));
    }

    #[test]
    fn mask_generation() {
        let mask_u32 = 8;
        let subnet_mask = generate_mask(mask_u32);
        assert_eq!(subnet_mask, Address(255, 0, 0, 0));

        let mask_u32 = 16;
        let subnet_mask = generate_mask(mask_u32);
        assert_eq!(subnet_mask, Address(255, 255, 0, 0));

        let mask_u32 = 24;
        let subnet_mask = generate_mask(mask_u32);
        assert_eq!(subnet_mask, Address(255, 255, 255, 0));

        let mask_u32 = 32;
        let subnet_mask = generate_mask(mask_u32);
        assert_eq!(subnet_mask, Address(255, 255, 255, 255));
    }
}
