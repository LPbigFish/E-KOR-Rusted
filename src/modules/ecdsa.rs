// src/modules/ecdsa.rs
use num::bigint::BigInt;

#[derive(Debug)]
pub struct Point {
    pub x: BigInt,
    pub y: BigInt
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

pub fn inverse(a: BigInt) -> BigInt {
    let mut m = get_p();
    let m_origin = get_p();
    let mut a = a;
    if a < BigInt::from(0) {
        a = a % &m;
    }

    let mut prevy = BigInt::from(0);
    let mut y = BigInt::from(1);
    let mut temp : BigInt;

    while a > BigInt::from(1) {
        let q = &get_p() / &a;
        temp = y.clone();
        y = &prevy - &q * y;
        prevy = temp;

        temp = a.clone();
        a = m % &a;
        m = temp;
    }

    y % m_origin
}

pub fn double(point : Point) -> Point {
    let slope = (BigInt::from(3) * point.x.pow(2) + get_a()) * inverse(BigInt::from(2) * &point.y) % get_p();

    let x = (slope.pow(2) - BigInt::from(2) * &point.x) % get_p();

    let y = (slope * (point.x - &x) - point.y) % get_p();

    Point { x , y }
}

pub fn add(point1 : Point, point2 : Point) -> Point {
    let slope = (point2.y - &point1.y) * inverse(&point2.x - &point1.x) % get_p();

    let x = (slope.pow(2) - &point1.x - &point2.x) % get_p();

    let y = (slope * (&point1.x - &x) - &point1.y) % get_p();

    Point { x , y }
}

pub fn multiply(k : BigInt) -> Point {

    let mut k = k;
    let point = get_g();

    let mut current = point.clone();

    //convert string to binary
    let binary = format!("{:b}", k);

    for bit in binary.chars().skip(1) {
        current = double(current);
        if bit == '1' {
            current = add(current, point.clone());
        }
    }

current
}




#[macro_export]
macro_rules! bigint {
    ($x:expr) => {
        BigInt::parse_bytes($x.as_bytes(), 16).unwrap()
    };
}