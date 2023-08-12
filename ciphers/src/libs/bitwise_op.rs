use num::Integer;
use std::ops::{Add, BitAnd, BitOr, BitXor, Not, Rem, Sub};

pub fn xor<T>(operand: T, key: T) -> T
where
    T: Integer + BitXor<Output = T>,
{
    operand ^ key
}

pub fn not<T>(operand: T, _: T) -> T
where
    T: Integer + Not<Output = T> + BitAnd<u16, Output = T>,
{
    !operand & 0xff
}

pub fn and<T>(operand: T, key: T) -> T
where
    T: Integer + BitAnd<Output = T>,
{
    operand & key
}

pub fn or<T>(operand: T, key: T) -> T
where
    T: Integer + BitOr<Output = T>,
{
    operand | key
}

pub fn add<T>(operand: T, key: T) -> T
where
    T: Integer + Add<Output = T> + Rem<u16, Output = T>,
{
    (operand + key) % 256
}

pub fn sub<T>(operand: T, key: T) -> T
where
    T: Sub + Add<u16, Output = T> + Integer,
{
    let result = operand - key;
    if result < T::zero() {
        result + 256
    } else {
        result
    }
}
