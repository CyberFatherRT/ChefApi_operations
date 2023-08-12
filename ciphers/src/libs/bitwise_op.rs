use num::Integer;
use std::ops::{Add, BitAnd, BitOr, BitXor, Not, Rem, Sub};

pub fn xor<T, V, R>(operand: T, key: V) -> R
where
    T: BitXor<V, Output = R>,
{
    operand ^ key
}

pub fn not<T, V, R>(operand: T, _: V) -> R
where
    T: Not<Output = T> + BitAnd<isize, Output = R>,
{
    !operand & 0xff
}

pub fn and<T, V, R>(operand: T, key: V) -> R
where
    T: BitAnd<V, Output = R>,
{
    operand & key
}

pub fn or<T>(operand: T, key: T) -> T
where
    T: BitOr<Output = T>,
{
    operand | key
}

pub fn add<T, V, R>(operand: T, key: V) -> R
where
    T: Add<V, Output = R>,
    R: Rem<isize, Output = R>,
{
    (operand + key) % 256
}

pub fn sub<T, V, R>(operand: T, key: V) -> R
where
    T: Sub<V, Output = R>,
    R: Integer + Add<u16, Output = R>,
{
    let result = operand - key;
    if result < R::zero() {
        result + 256
    } else {
        result
    }
}
