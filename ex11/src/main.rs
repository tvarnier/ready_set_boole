fn rot(n: u64, x: &mut u64, y: &mut u64, rx: u64, ry: u64) {
    if ry == 0 {
        if rx == 1 {
            let tmp : u64 = *x;
            *x = n - 1 - tmp;
            let tmp : u64 = *y;
            *y = n - 1 - tmp;
        }

        //Swap x and y
        let t : u64 = *x;
        *x = *y;
        *y = t;
    }
}

fn map(x: u16, y: u16) -> f64 {
    let mut tmp_x : u64 = x as u64;
    let mut tmp_y : u64 = y as u64;

    let n : u64 = 65536;
    let mut rx : u64;
    let mut ry : u64;
    let mut s : u64 = n / 2;
    let mut d : u64 = 0;
    while s > 0 {
        rx = if tmp_x & s > 0 { 1 } else { 0 };
        ry = if tmp_y & s > 0 { 1 } else { 0 };
        d += s * s * ((3 * rx) ^ ry);
        rot(n, &mut tmp_x, &mut tmp_y, rx, ry);
        s /= 2;
    }
    return d as f64 / 4294967295.0;
}

fn reverse_map(d : f64) -> (u16, u16) {
    let n : u64 = 65536;

    let mut rx : u64;
    let mut ry : u64;
    let mut s : u64 = 1;
    let mut t : u64 = (d * 4294967295.0) as u64;

    let mut x: u64 = 0;
    let mut y: u64 = 0;

    while s < n {

        rx = 1 & (t / 2);
        ry = 1 & (t ^ rx);
        rot(s, &mut x, &mut y, rx, ry);
        x += s * rx;
        y += s * ry;
        t /= 4;

        s *= 2;
    }
    return (x as u16, y as u16);
}

fn main() {
    let res : f64 = map(0, 0);
    println!("{}", res);
    println!(" -> {:?}", reverse_map(res));

    let res : f64 = map(0, u16::MAX);
    println!("{}", res);
    println!(" -> {:?}", reverse_map(res));

    let res : f64 = map(u16::MAX, u16::MAX);
    println!("{}", res);
    println!(" -> {:?}", reverse_map(res));

    let res : f64 = map(u16::MAX,0);
    println!("{}", res);
    println!(" -> {:?}", reverse_map(res));
}