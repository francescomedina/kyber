use rand; // 0.8.5

const QINV: i32 = 62209; // q^(-1) mod 2^16
const KYBER_Q: i32 = 3329;
const KYBER_N: i32 = 256;

pub fn montgomery_reduce(a: i32) -> i16
{
    let ua = a.wrapping_mul(QINV) as i16;
    let u = ua as i32;
    let mut t = u * KYBER_Q as i32;
    t = a - t;
    t >>= 16;
    t as i16
}

pub fn barrett_reduce(a: i16) -> i16
{
    let v = ((1u32 << 26)/KYBER_Q as u32 + 1) as i32;
    let mut t = v * a as i32 + (1 << 25);
    t >>= 26;
    t *= KYBER_Q as i32;
    a - t as i16
}

pub fn fqmul(a: i16, b: i16) -> i16
{
    montgomery_reduce(a as i32 * b as i32)
}

pub fn frommont(r: i32) -> i16
{
    let f = ((1u64 << 32) % KYBER_Q as u64) as i16;
    let a = r as i32 * f as i32;
    montgomery_reduce(a)
}

fn main() {
    let f = ((1u64 << 32) % KYBER_Q as u64) as i16;
    println!("{}\n\n", f);
    let a = -581;
    let b = 588;
//let m = montgomery_reduce(a as i32 + b as i32);
    let a = 12;
    let b = 21;
    let m = a*b;
    let z = 287;
    let f = 1353;
    let m1 = montgomery_reduce((m + 63) as i32 * 31 + 321);
    let m2 = montgomery_reduce(montgomery_reduce(a as i32 * 220 as i32 * f as i32 + frommont(63) as i32) as i32 * 31 + 321);
    println!("Montgomery_0: {:?}", m1);
    println!("Montgomery_1: {:?}", m2);
    println!("FromMont: {:?}", frommont(3 as i32));
    println!("Mont: {:?}", montgomery_reduce(-1353 as i32));
    println!("\n\nTest: {:?}", montgomery_reduce(1 as i32));
    println!("Test: {:?}", montgomery_reduce(montgomery_reduce(252) as i32));
//println!("FromMont Expanded {:?}", montgomery_reduce(montgomery_reduce(m as i32*f as i32) as i32));
}