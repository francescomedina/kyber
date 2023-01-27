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
// println!("{}\n\n", f);
    /*let a = -581;
    let b = 588;
    //let m = montgomery_reduce(a as i32 + b as i32);
    let a = 12;
    let b = 21;
    let r = b;
    let m = a*b;
    let z = 287;
    let f = 1353;
    let F = 1441;
    let m1 = montgomery_reduce((m + 63) as i32 * 31 + 321);
    let m2 = montgomery_reduce(montgomery_reduce(a as i32 * 220 as i32 * f as i32 + frommont(63) as i32) as i32 * 31 + 321);
    println!("Montgomery_0: {:?}", m1);
    println!("Montgomery_1: {:?}", m2);
    println!("FromMont: {:?}", frommont(3 as i32));*/
    let r = -1 as i32;
    let z = 31 as i32;
    let F = 1441 as i32;
    let at = 2 as u32;
    let a = -5423531 as i32;
// println!("{}", ((((at & 1 << 8) - 1 >> 24) )  + 768) as usize);
// println!("{}", ((((at & 1 << 8) - 1 >> 16) )  + 512) as usize);
// println!("{}", ((((at & 1 << 8) - 1 >> 8) )  + 256) as usize);
    let m1 = montgomery_reduce(a); //-2184364
// println!("{:#034b}", a);
// let mask = a & 1 << 8;
// println!("{:#034b}", mask - 1 >> 24);
// println!("{:#034b}", mask - 1 >> 16);
// println!("{}", a & ((1 << 8) - 1 << 24));
// println!("{}", ((a & 1 << 8) - 1 >> 24) + 256*3);
// println!("{}", montgomery_reduce(a & ((1 << 8) - 1 << 24)));
    let t1 = a & ((1 << 8) - 1 << 24);
    let t2 = a & ((1 << 8) - 1 << 16);
    let t3 = a & ((1 << 8) - 1 << 8);
    let t4 = a & ((1 << 8) - 1 << 0);
    println!("Indici: {} + {} + {} + {}", t1 >> 24, t2 >> 16, t3 >> 8, t4);
    println!("Valori: {} + {} + {} + {}", t1, t2, t3, t4);
    println!("Mont: {} + {} + {} + {}", montgomery_reduce(t1), montgomery_reduce(t2), montgomery_reduce(t3), montgomery_reduce(t4));
    let m2 = montgomery_reduce(t1)
        + montgomery_reduce(t2)
        + montgomery_reduce(t3)
        + montgomery_reduce(t4);
    let m3 = montgomery_reduce(montgomery_reduce(z*r) as i32);
    let m4 = -1 * montgomery_reduce(montgomery_reduce(z) as i32);
// let m4 = z as i32 * montgomery_reduce(montgomery_reduce(r) as i32) as i32;
    println!("\nasd: {:?}", m1);
    println!("qwe: {:?}", m2);
    println!("asd: {:?}", m3);
    println!("qwe: {:?}", m4);
    println!("\n\nFromMont: {:?}", frommont(m1 as i32));
    println!("FromMont: {:?}", frommont(m2 as i32));
    println!("FromMont m3: {:?}", frommont(-729 as i32));
    println!("FromMont m4: {:?}", frommont(1617));

    /*println!("\n\nNuovo Esperimento!");
    let r = 7;
    let m3 = montgomery_reduce(montgomery_reduce(b*r) as i32);
    let m4 = montgomery_reduce(montgomery_reduce(b) as i32 *r);
    println!("Montgomery_3: {:?}", m3);
    println!("Montgomery_4: {:?}", m4);
    println!("Montgomery_3: {:?}", montgomery_reduce(-1641 as i32));
    println!("MONT^-1 R: {:?}", frommont(1665 as i32));
    println!("MONT^-1 FIN: {:?}\n\n", frommont(-1664 as i32));*/
// let mut prev = 1;
// for i in 0..10 {
//     prev = montgomery_reduce(prev as i32);
//     println!("Mont: {}", prev);
//     println!("FromMont: {}\n", frommont(prev as i32));
// }
}