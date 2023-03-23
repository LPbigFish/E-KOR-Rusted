// src/modules/ecdsa.rs
use num::bigint::BigInt;

#[derive(Debug)]
pub struct Point {
    pub x: BigInt,
    pub y: BigInt
}

#[macro_export]
macro_rules! bigint {
    ($x:expr) => {
        BigInt::parse_bytes($x.as_bytes(), 16).unwrap()
    };
}

 #[macro_export]
 macro_rules! bigint_array {
        ($x:expr) => {
            BigInt::from_bytes_be(num::bigint::Sign::Plus, $x)
        };
 }

impl Point {
    pub fn clone(&self) -> Point {
        Point {
            x: self.x.clone(),
            y: self.y.clone()
        }
    }
}

pub fn get_g() -> Point {
    Point {
        x: BigInt::parse_bytes(b"79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798", 16).unwrap(),
        y: BigInt::parse_bytes(b"483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8", 16).unwrap(),
    }
}

pub fn get_p() -> BigInt {
    let x = BigInt::parse_bytes(b"FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F", 16).unwrap();
    x
}

pub fn get_n() -> BigInt {
    let x = BigInt::parse_bytes(b"FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141", 16).unwrap();
    x
}

pub fn get_a() -> BigInt {
    let x = BigInt::from(0);
    x
}

pub fn get_b() -> BigInt {
    let x = BigInt::from(7);
    x
}

pub fn to_slice(x: &BigInt) -> [u8; 32] {
    let mut array = [0u8; 32];
    let bytes = x.to_bytes_be();
    let mut i = 0;
    for byte in bytes.1 {
        array[i] = byte;
        i += 1;
    }
    array
}

pub fn inverse(a: BigInt) -> Option<BigInt> {
    let mut a = a % get_p();
    let mut b = get_p();
    let mut x = bigint!("0");
    let mut y = bigint!("1");
    let mut last_x = bigint!("1");
    let mut last_y = bigint!("0");
    while b != bigint!("0") {
        let q = &a / &b;
        let temp = b.clone();
        b = &a % &b;
        a = temp.to_owned();
        let temp = x.clone();
        x = last_x - &q * x;
        last_x = temp;
        let temp = y.clone();
        y = last_y - q * &y;
        last_y = temp;
    }
    Some(last_x % get_p())
}

pub fn double(point : &Point) -> Point {
    let slope = (BigInt::from(3) * point.x.pow(2) + get_a()) * inverse(BigInt::from(2) * &point.y).unwrap() % get_p();

    let x = (slope.pow(2) - BigInt::from(2) * &point.x) % get_p();

    let y = (slope * (&point.x - &x) - &point.y) % get_p();

    Point { x , y }
}

pub fn add(point1 : &Point, point2 : &Point) -> Point {
    let slope = (&point2.y - &point1.y) * inverse(&point2.x - &point1.x).unwrap() % get_p();

    let x = (slope.pow(2) - &point1.x - &point2.x) % get_p();

    let y = (slope * (&point1.x - &x) - &point1.y) % get_p();

    Point { x , y }
}

pub fn multiply(k : BigInt) -> Point {

    let k = k;
    let point = get_g();

    let mut current = point.clone();

    //convert string to binary
    let binary = k.to_str_radix(2);

    for bit in binary.chars().skip(1) {
        current = double(&current);
        if bit == '1' {
            current = add(&current, &point);
        }
    }   

    Point {x: current.x.clone(), y: current.y.clone()}
}