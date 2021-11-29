fn adder(a: u32, b: u32) -> u32 {
    let mut res  : u32 = 0b0;
    let mut mask : u32 = 0b1;

    for _i in 0..32 {
        let a_masked : u32 = a & mask;
        let b_masked : u32 = b & mask;

        res ^= ((a_masked & b_masked) << 0b1)                       // Add 0b10 if both == 1
            | (((a_masked ^ b_masked) & (res & mask)) << 0b1)       // || one == 1 and carry
            | (a_masked ^ b_masked);                                 // Add 0b1 If one == 1 

        mask <<= 0b1;
    }
    res
}

fn multiplier(a: u32, b: u32) -> u32 {
    let mut res   : u32 = 0b0;
    let mut tmp_a : u32 = a;
    let mut tmp_b : u32 = b;
    for _i in 0..32 {
        if tmp_b & 0b1 == 0b1 {
            res = adder(res, tmp_a);
        }
        tmp_a <<= 0b1;
        tmp_b >>= 0b1;
    }
    res
}

fn main() {
    println!("          3 *  1 = {}",   multiplier( 3,  1));
    println!("         10 *  3 = {}",   multiplier(10,  3));
    println!("         37 * 21 = {}",   multiplier(37, 21));
    println!(" {} *  2 = {}", u32::MAX, multiplier(u32::MAX, 2));

    for _i in 0..10 {
        println!(" -> {}", _i);
        for _y in 0..10 {
            println!("      * {} = {}", _y, multiplier( _i, _y));
        }
    }
}