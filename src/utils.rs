use num::Integer;

/// 割り算の結果を切り上げる関数
/// 
/// # 引数
/// * `a` - 割られる数
/// * `b` - 割る数
/// 
/// # 戻り値
/// `T` - 割り算の結果
/// 
pub fn div_cell<T>(a: T, b: T) -> T 
 where T: Integer + Copy {
    if b == T::zero() {
        return T::zero();
    }
    (a + b - T::one()) / b
}