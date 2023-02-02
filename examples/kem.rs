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

pub fn barrett_reduce(a: i16) -> i16
{
  let v = ((1u32 << 26)/KYBER_Q as u32 + 1) as i32;
  let mut t = v * a as i32 + (1 << 25);
  t >>= 26;
  t *= KYBER_Q as i32;
  a - t as i16
}

// pub fn fbr(a: i16) -> i16 {
//   let c = a - 3329*(((a>>8) + 6)/13);
//   if c < 0 && c < -1664 {
//     return c + 3329;
//   }
//   if c > 1664 {
//     return c - 3329;
//   }
//   return c;
//   // a - 3329*(((a)>>8)/11)
// }

pub fn fbrm6(b: i16) -> i16 {
  let a = b.abs();
  if a < 18310 {
    if a < 8323 {
      return match a {
        x if x < 1665 => if b < 0 { b } else { a },
        x if x < 4994 => if b < 0 { b + 3329 } else { a - 3329},
        _ => if b < 0 { b + 6658 } else { a - 6658}
      };
    }
    return match a {
      x if x < 11652 => if b < 0 { b + 9987 } else { a - 9987},
      x if x < 14981  => if b < 0 { b + 13316 } else { a - 13316},
      _ => if b < 0 { b + 16645 } else { a - 16645 }
    };
  }
  if a < 24968 {
    return match a {
      x if x < 18310 => if b < 0 { b + 16645 } else { a - 16645 }
      x if x < 21639 => if b < 0 { b + 19974 } else { a - 19974 },
      _ => if b < 0 { b + 23303 } else { a - 23303 }
    };
  }
  return match a {
    x if x < 28297 => if b < 0 { b + 26632 } else { a - 26632 },
    _ => if b < 0 { b + 29961 } else { a - 29961 }
  };
}

pub fn fbrm4(b: i16) -> i16 {
  let a = b.abs();
  if (a & 16384) == 0 {
    return match a {
      x if x < 1665 => if b < 0 { b } else { a },
      x if x < 4994 => if b < 0 { b + 3329 } else { a - 3329},
      x if x < 8323 => if b < 0 { b + 6658 } else { a - 6658},
      x if x < 11652 => if b < 0 { b + 9987 } else { a - 9987},
      x if x < 14981  => if b < 0 { b + 13316 } else { a - 13316},
      _ => if b < 0 { b + 16645 } else { a - 16645 }
    };
  }
  return match a {
    x if x < 18310 => if b < 0 { b + 16645 } else { a - 16645 }
    x if x < 21639 => if b < 0 { b + 19974 } else { a - 19974 },
    x if x < 24968 => if b < 0 { b + 23303 } else { a - 23303 },
    x if x < 28297 => if b < 0 { b + 26632 } else { a - 26632 },
    _ => if b < 0 { b + 29961 } else { a - 29961 }
  };
}
pub fn fbr(b: i16) -> i16 {
  let a = b.abs();
  return if (a & 16384) == 0 {
    if a < 1665 {
      if b < 0 {
        return b;
      }
      return a;
    }
    if a < 4994 {
      if b < 0 {
        return b + 3329;
      }
      return a - 3329;
    }
    if a < 8323 {
      if b < 0 {
        return b + 6658;
      }
      return a - 6658;
    }
    if a < 11652 {
      if b < 0 {
        return b + 9987;
      }
      return a - 9987;
    }
    if a < 14981 {
      if b < 0 {
        return b + 13316;
      }
      return a - 13316;
    }
    if b < 0 {
      return b + 16645;
    }
    a - 16645
  } else {
    if a < 18310 {
      if b < 0 {
        return b + 16645;
      }
      return a - 16645;
    }
    if a < 21639 {
      if b < 0 {
        return b + 19974;
      }
      return a - 19974;
    }
    if a < 24968 {
      if b < 0 {
        return b + 23303;
      }
      return a - 23303;
    }
    if a < 28297 {
      if b < 0 {
        return b + 26632;
      }
      return a - 26632;
    }
    if b < 0 {
      return b + 29961;
    }
    a - 29961
  }
  // a - 3329*((a>>11)-2)
  // a -  (3329*(((a>>8) + 16)>>4)) + ((((a <= 1664) as i16) << 15) >> 15 & 3329)
  // a - (3329*(((a>>8)+16)>>4) + (((a+2431)>>8)>>4)) + (((a+3198)>>8)>>4)
  // - 3329*((a>>8) == 0) as i16
    // + ((a >> 14) & 1) * 64
    // + ((a >> 13) & 1) * 65
  // + (a >> 1  & 0b10000000000000000000000000000000) ^ 169;
  // a - 3329*(((a - 1665)/3329) + 1)
  // if c < 0 && c < -1664 {
  //   return c + 3329;
  // }
  // if c > 1664 {
  //   return c - 3329;
  // }
  // return c;
  // a - 3329*(((a)>>8)/11)
}

// pub fn fbr(a: i16) -> i16 {
//   a - 3329*(((a>>8) + 8 - ((a>>15) & 14))/13)
// }

// pub fn fbr(a: i16) -> i16 {
//   ((a>>15) & ()) | a - 3329*((((a>>8) + 1) + 7 - ((a>>15) & 14))/13)
// }

fn main () -> Result<(), KyberError> {
  // let mut rng = rand::thread_rng();
  let now = Instant::now();
  let mut rng = CustomRng(2);

  let mut t: [i16; 65536] = [0i16; 65536];
  let mut shift4: [i16; 65536] = [0i16; 65536];
  let mut diff: [i16; 65536] = [0i16; 65536];
  let mut counta: i16 = 0;
  let mut countb: i16 = 0;
  let mut countc: i16 = 0;
  let mut vetcount: [i16; 65536] = [0i16; 65536];
  let mut vetcountb: [i16; 65536] = [0i16; 65536];
  let mut vetcountc: [i16; 65536] = [0i16; 65536];
  let mut c = 0;
  let mut p_original = 0;
  let mut p_mod = 0;
  let mut p_modMatch = 0;
  let mut pareggio = 0;
  for a in 0..62000 {
    let d = (a as i32 - 31000) as i16;
    let b=  rng.gen_range(d..(d + 314));
    // t[a] = ((b & 15)*169 - ((b & 48) >> 4 )*625 + ((b & 192) >> 6)*829 - (((b & 32512) >> 8)*13) + (a as i32 >> 16)) as i16;
    t[a] = barrett_reduce(b);
    // t[a] = fbrm4(b);
    let f = fbrm6(b);
    // let now = Instant::now();
    // fbr(b);
    // let elapsed = now.elapsed().as_nanos();
    let now3 = Instant::now();
    // fbrm(b);
    fbrm6(b);
    let elapsed3 = now3.elapsed().as_nanos();
    // t[a] = barrett_reduce(b);
    let now2 = Instant::now();
    barrett_reduce(b);
    // fbrm4(b);
    let elapsed2 = now2.elapsed().as_nanos();
    if elapsed2 < elapsed3{
      p_original += 1;
      // println!("{}", b);
    }else if elapsed3 == elapsed2{
      pareggio += 1;
    }
    // else if elapsed < elapsed3 && elapsed < elapsed2{
    //   p_mod += 1;
    // }
    else{
      p_modMatch += 1;
    }
    // println!("Value: {}", b);
    // println!("MATCH: {}", elapsed3);
    // // println!("MOD: {}", elapsed);
    // println!("ORIGINAL: {}\n", elapsed2);
    if t[a] != f {
      println!("\tDIVERSO {}-> barrett: {} fbr: {}", b, t[a], f);
      c += 1;
    }
    // else{
    // let mut mull = 0;
    // for i in 0..10 {
    //   if t[a] == b - 3329*i {
    //     mull = i;
    //     break;
    //   }
    // }
      // println!("{}-> barrett: {} -> {}\t\t\t\t\t\t{}\t{}\t{}\t{}\t{}", b, t[a], mull, ((a>>8)+16)>>4, (a>>8)>>4,((a>>9)+16)>>4, (a>>9)>>4, ((a+1664)>>8)>>4);
      // println!("{}-> barrett: {} -> {}\t\t\t\t\t\t{}\t{}\t{}\t{}\t{}", b, t[a], mull, ((a+2431)>>8)>>4, ((a+3198)>>8)>>4, ((a+3965)>>8)>>4, ((a+4732)>>8)>>4,((a+5499)>>8)>>4);
    // }
    // shift4[a] = (b >> 8);
    // if a > 0{
    //   diff[a] = t[a] - t[a - 1];
    // }
    // println!("{} {} {}", t[a], counta, diff[a]);
    // if b == 4993 {
    //   println!("CIAON");
    // }
    // println!("Number {}, shift {}", b, shift4[a]);
    // if a > 0 && diff[a] != 1 {
    //   counta += 1;
    //   println!("Diff {} between outputs {} and {} of inputs {} and {}, shift {}", diff[a], t[a-1], t[a], b-1, b, shift4[a]);
    // }
/*    if diff[a] == -3160 || diff[a] == -1496 || diff[a] == 169 {
      // println!("CIAON");
      let g = 3;
    }else{
      println!("{}", diff[a]);
    }*/
    vetcount[a] = counta;
    vetcountb[a] = countb;
    vetcountc[a] = countc;
  }
  // println!("{:?}", t);
  // println!("{:?}", diff);
  // println!("{}", counta);
  println!("DIVERSI: {}", c);
  println!("Original {}", p_original);
  println!("Mod {}", p_mod);
  println!("Match {}", p_modMatch);
  println!("Pareggio {}", pareggio);
  // counta = 0;
  // for i in 126546..127546 {
  //   if i <= 0 {
  //     continue;
  //   }
  //   if diff[i] == -3160 && diff[i-1] == 169 && shift4[i] - shift4[i-1] == 1 {
  //     println!("\tCHANGE a: {} - a >> 4 << 4: {} a<<4: {}: test: {} t: {} counta: {}, count_b:  {}, count_c:  {}", i, shift4[i] << 4, shift4[i], shift4[i] << 2, t[i], vetcount[i], vetcountb[i], vetcountc[i]);
  //     counta += 1;
  //   }
  //   println!("a: {} - a >> 4 << 4: {} a<<4: {}: test: {} t: {} counta: {}, count_b:  {}, count_c:  {}", i, shift4[i] << 4, shift4[i], shift4[i] << 2, t[i], vetcount[i], vetcountb[i], vetcountc[i]);
  // }
  // println!("{}", counta);

/*  // Alice generates a keypair
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
  assert_eq!(shared_secret_alice, shared_secret_bob);*/
  Ok(())
}