fn adder(a: u32, b: u32) -> u32 {
    let mut res: u32 = 0b0;
    let mut mask: u32 = 0b1;

    for _i in 0..32 {
        let a_masked: u32 = a & mask;
        let b_masked: u32 = b & mask;

        res ^= ((a_masked & b_masked) << 0b1)                       // Add 0b10 if both == 1
            | (((a_masked ^ b_masked) & (res & mask)) << 0b1)       // || one == 1 and carry
            | (a_masked ^ b_masked); // Add 0b1 If one == 1

        mask <<= 0b1;
    }
    res
}

fn main() {
    println!("          3 +  1 = {}", adder(3, 1));
    println!("         10 +  3 = {}", adder(10, 3));
    println!("         37 + 21 = {}", adder(37, 21));
    println!(" {} +  1 = {}", u32::MAX, adder(u32::MAX, 1));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic() {
        for i in 0..100 {
            for y in 0..100 {
                assert_eq!(adder(i, y), i + y);
            }
        }
    }

    #[test]
    fn overflow() {
        let max: u32 = u32::MAX;
        assert_eq!(adder(u32::MAX, 0), max.wrapping_add(0));
        assert_eq!(adder(u32::MAX, 1), max.wrapping_add(1));
        assert_eq!(adder(u32::MAX, u32::MAX), max.wrapping_add(max));
        assert_eq!(
            adder(adder(u32::MAX, u32::MAX), u32::MAX),
            max.wrapping_add(max).wrapping_add(max)
        );
    }
}
