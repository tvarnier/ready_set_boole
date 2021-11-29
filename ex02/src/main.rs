fn gray_code(n: u32) -> u32
{
    let mut res : u32 = n >> 31 << 31;
    let mut msb_mask : u32 = 0b1 << 31; // Most Signifiant Bit

    for _i in 0..32 {
        res |= (n & msb_mask) ^ (n >> 1 & msb_mask);

        msb_mask >>= 1;
    }
    res
}

fn main() {
    for _i in 0..15  {
        println!(" -> {} = {} : {:b}", _i, gray_code( _i), gray_code( _i));
    }
    println!(" -> {} = {} : {:b}", 2147483648 as u32, gray_code( 2147483648), gray_code( 2147483648));
    println!(" -> {} = {} : {:b}", u32::MAX, gray_code( u32::MAX), gray_code( u32::MAX));
}

