use std::fmt;
use std::ops::{Add, Div, Mul, Rem, Sub};

// A struct representing a 256-bit unsigned integer
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq)]
pub struct U256(pub [u64; 4]);

impl U256 {
    // Constructor to create a new U256 from a u64 array
    pub fn new(value: [u64; 4]) -> Self {
        U256(value)
    }

    // Create a U256 from a single u64 value
    pub fn from_u64(value: u64) -> Self {
        U256([value, 0, 0, 0])
    }

    // Function to return the zero U256 value
    pub fn zero() -> Self {
        U256([0, 0, 0, 0])
    }

    // Function to return the one U256 value
    pub fn one() -> Self {
        U256([1, 0, 0, 0])
    }

    // Function to check if the U256 value is zero
    pub fn is_zero(&self) -> bool {
        self.0 == [0, 0, 0, 0]
    }
}

// Implementing the Add trait for U256
impl Add for U256 {
    type Output = U256;

    fn add(self, rhs: U256) -> U256 {
        let mut carry = 0u64;
        let mut result = [0u64; 4];
        for i in 0..4 {
            let (sum, overflow1) = self.0[i].overflowing_add(rhs.0[i]);
            let (sum, overflow2) = sum.overflowing_add(carry);
            result[i] = sum;
            carry = (overflow1 as u64) + (overflow2 as u64);
        }
        U256(result)
    }
}

// Implementing the Sub trait for U256
impl Sub for U256 {
    type Output = U256;

    fn sub(self, rhs: U256) -> U256 {
        let mut borrow = 0u64;
        let mut result = [0u64; 4];
        for i in 0..4 {
            let (diff, overflow1) = self.0[i].overflowing_sub(rhs.0[i]);
            let (diff, overflow2) = diff.overflowing_sub(borrow);
            result[i] = diff;
            borrow = (overflow1 as u64) + (overflow2 as u64);
        }
        U256(result)
    }
}

// Implementing the Mul trait for U256
impl Mul for U256 {
    type Output = U256;

    fn mul(self, rhs: U256) -> U256 {
        let mut result = U256::zero();
        for i in 0..4 {
            let mut carry = 0u64;
            for j in 0..(4 - i) {
                let (low, high) = mul_u64(self.0[i], rhs.0[j]);
                let (sum_low, carry1) = result.0[i + j].overflowing_add(low);
                let (sum_low, carry2) = sum_low.overflowing_add(carry);
                let (sum_high, carry3) =
                    result.0[i + j + 1].overflowing_add(high + (carry1 as u64) + (carry2 as u64));

                result.0[i + j] = sum_low;
                result.0[i + j + 1] = sum_high;
                carry = carry3 as u64;
            }
        }
        result
    }
}

// A helper function to multiply two u64 values, returning the low and high parts of the result
fn mul_u64(lhs: u64, rhs: u64) -> (u64, u64) {
    let (low, high) = ((lhs as u128) * (rhs as u128)).overflowing_shr(64);
    (low as u64, high as u64)
}

// Implementing the Div trait for U256 (naive long division)
impl Div for U256 {
    type Output = U256;

    fn div(self, rhs: U256) -> U256 {
        if rhs.is_zero() {
            panic!("Division by zero");
        }

        let mut quotient = U256::zero();
        let mut remainder = U256::zero();

        for i in (0..256).rev() {
            remainder = remainder << 1;
            remainder.0[0] |= (self >> i).0[0] & 1;

            if remainder >= rhs {
                remainder = remainder - rhs;
                quotient.0[0] |= 1 << i;
            }
        }

        quotient
    }
}

// Implementing the Rem trait for U256
impl Rem for U256 {
    type Output = U256;

    fn rem(self, rhs: U256) -> U256 {
        let mut remainder = U256::zero();

        for i in (0..256).rev() {
            remainder = remainder << 1;
            remainder.0[0] |= (self >> i).0[0] & 1;

            if remainder >= rhs {
                remainder = remainder - rhs;
            }
        }

        remainder
    }
}

// Implementing the Display trait for U256 for pretty printing
impl fmt::Display for U256 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x")?;
        for &val in self.0.iter().rev() {
            write!(f, "{:016x}", val)?;
        }
        Ok(())
    }
}

// Implementing shift left and right
impl std::ops::Shl<u32> for U256 {
    type Output = U256;

    fn shl(self, shift: u32) -> U256 {
        let mut result = [0u64; 4];
        let word_shift = (shift / 64) as usize;
        let bit_shift = shift % 64;

        for i in (0..4).rev() {
            if i + word_shift < 4 {
                result[i + word_shift] = self.0[i] << bit_shift;
                if i + word_shift + 1 < 4 && bit_shift != 0 {
                    result[i + word_shift + 1] |= self.0[i] >> (64 - bit_shift);
                }
            }
        }
        U256(result)
    }
}

impl std::ops::Shr<u32> for U256 {
    type Output = U256;

    fn shr(self, shift: u32) -> U256 {
        let mut result = [0u64; 4];
        let word_shift = (shift / 64) as usize;
        let bit_shift = shift % 64;

        for i in 0..4 {
            if i >= word_shift {
                result[i - word_shift] = self.0[i] >> bit_shift;
                if i >= word_shift + 1 && bit_shift != 0 {
                    result[i - word_shift - 1] |= self.0[i] << (64 - bit_shift);
                }
            }
        }
        U256(result)
    }
}

// implement BitOrAssign for U256

impl std::ops::BitOrAssign for U256 {
    fn bitor_assign(&mut self, rhs: U256) {
        for i in 0..4 {
            self.0[i] |= rhs.0[i];
        }
    }
}

// core::ops::bit::BitAnd
impl std::ops::BitAnd for U256 {
    type Output = U256;

    fn bitand(self, rhs: U256) -> U256 {
        let mut result = [0u64; 4];
        for i in 0..4 {
            result[i] = self.0[i] & rhs.0[i];
        }
        U256(result)
    }
}

// Example usage of U256
fn main() {
    let a = U256::from_u64(123456789);
    let b = U256::from_u64(987654321);
    let c = a + b;
    println!("a + b = {}", c);

    let d = a * b;
    println!("a * b = {}", d);
}
