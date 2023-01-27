use rand::Rng;
use rand_core::{Error, impls};
use pqc_kyber::*;
use std::time::{Duration, Instant};

const QINV: i32 = 62209; // q^(-1) mod 2^16
const KYBER_Q: i32 = 3329;
const KYBER_N: i32 = 256;

pub fn montgomery_reduce_a(a: i32) -> i16
{
  let ua = a.wrapping_mul(QINV) as i16;
  let u = ua as i32;
  let mut t = u * KYBER_Q as i32;
  t = a - t;
  t >>= 16;
  t as i16
}

// pub fn barrett_reduce(a: i16) -> i16
// {
//   let v = ((1u32 << 26)/KYBER_Q as u32 + 1) as i32;
//   let mut t = v * a as i32 + (1 << 25);
//   t >>= 26;
//   t *= KYBER_Q as i32;
//   a - t as i16
// }

pub fn fqmul(a: i16, b: i16) -> i16
{
  montgomery_reduce_a(a as i32 * b as i32)
}

pub fn frommont(r: i32) -> i16
{
  let f = ((1u64 << 32) % KYBER_Q as u64) as i16;
  let a = r as i32 * f as i32;
  montgomery_reduce_a(a)
}

// fn main() {
//   let a = -581;
//   let b = 588;
//   let m = montgomery_reduce(a as i32 + b as i32);
//   println!("{:?}", m);
//   println!("{:?}", frommont(m as i32));
// }

#[derive(Clone, Debug)]
pub struct CustomRng(u64);

impl RngCore for CustomRng {
  fn next_u32(&mut self) -> u32 {
    self.next_u64() as u32
  }

  fn next_u64(&mut self) -> u64 {
    self.0 += 1;
    self.0
  }

  fn fill_bytes(&mut self, dest: &mut [u8]) {
    impls::fill_bytes_via_next(self, dest)
  }

  fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
    Ok(self.fill_bytes(dest))
  }
}

impl CryptoRng for CustomRng {
}

fn main () -> Result<(), KyberError> {
  // let mut rng = rand::thread_rng();
  let now = Instant::now();
  let mut rng = CustomRng(2);

  // let mut t: [i32; 65536] = [0i32; 65536];
  // let mut shift4: [i32; 65536] = [0i32; 65536];
  // let mut shift8: [i32; 65536] = [0i32; 65536];
  // let mut diff: [i32; 65536] = [0i32; 65536];
  // let mut count = 0;
  // let mut counta = 0;
  // let mut vetcount: [i32; 65536] = [0i32; 65536];
  // for a in 0..65536 {
  //   t[a] = (((a as i32)) & 15)*169 - (((a as i32) & 48) >> 4 )*625 + (((a as i32) & 192) >> 6)*829 - ((((a as i32) & 32512) >> 8)*13);
  //   shift4[a] = (a as i32) >> 4;
  //   if a > 0{
  //     diff[a] = t[a] - t[a - 1];
  //   }
  //   println!("{} {} {}", t[a], counta, diff[a]);
  //   if a == 20457 {
  //     println!("CIAON");
  //   }
  //   if diff[a] == -3160 {
  //     // println!("{}", count);
  //     count = 0;
  //     counta += 1;
  //   }
  //   count += 1;
  //   vetcount[a] = counta;
  // }
  // println!("{:?}", diff);
  // println!("{}", counta);
  // counta = 0;
  // for i in 0..5000 {
  //   if i <= 0 {
  //     continue;
  //   }
  //   if diff[i] == -3160 && diff[i-1] == 169 && shift4[i] - shift4[i-1] == 1 {
  //     println!("\tCHANGE a: {} - a >> 4 << 4: {} a<<4: {}: test: {} t: {} counta: {}", i, shift4[i] << 4, shift4[i], shift4[i] << 2, t[i], vetcount[i]);
  //     counta += 1;
  //   }
  //   println!("a: {} - a >> 4 << 4: {} a<<4: {}: test: {} t: {} counta: {}", i, shift4[i] << 4, shift4[i], shift4[i] << 2, t[i], vetcount[i]);
  // }
  // println!("{}", counta);

  // Alice generates a keypair
  let alice_keys = keypair(&mut rng);

  // println!("{:?}", alice_keys);

  // Bob encapsulates a shared secret
  let (ciphertext, shared_secret_bob) = encapsulate(&alice_keys.public, &mut rng)?;

  // try_occ();

  // Alice decapsulates the shared secret
  let shared_secret_alice = decapsulate(&ciphertext, &alice_keys.secret)?;
  println!("Basemul: {}", now.elapsed().as_nanos());

  println!("{:?}", shared_secret_bob);
  println!("{:?}", shared_secret_alice);

  // Both can now communicate symetrically
  assert_eq!(shared_secret_alice, shared_secret_bob);
  Ok(())
}