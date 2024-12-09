use std::ops::{Div, Sub};

use num::Integer;

/// 割り算の結果を切り上げる関数
pub fn div_cell<T>(a: T, b: T) -> T 
 where T: Integer + Copy {
    if b == T::zero() {
        return T::zero();
    }
    (a + b - T::one()) / b
}