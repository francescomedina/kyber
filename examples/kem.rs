use pqc_kyber::*;

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

pub const ZETAS: [i16; 128] = [
  -1044,  -758,  -359, -1517,  1493,  1422,   287,   202,
  -171,   622,  1577,   182,   962, -1202, -1474,  1468,
  573, -1325,   264,   383,  -829,  1458, -1602,  -130,
  -681,  1017,   732,   608, -1542,   411,  -205, -1571,
  1223,   652,  -552,  1015, -1293,  1491,  -282, -1544,
  516,    -8,  -320,  -666, -1618, -1162,   126,  1469,
  -853,   -90,  -271,   830,   107, -1421,  -247,  -951,
  -398,   961, -1508,  -725,   448, -1065,   677, -1275,
  -1103,   430,   555,   843, -1251,   871,  1550,   105,
  422,   587,   177,  -235,  -291,  -460,  1574,  1653,
  -246,   778,  1159,  -147,  -777,  1483,  -602,  1119,
  -1590,   644,  -872,   349,   418,   329,  -156,   -75,
  817,  1097,   603,   610,  1322, -1285, -1465,   384,
  -1215,  -136,  1218, -1335,  -874,   220, -1187, -1659,
  -1185, -1530, -1278,   794, -1510,  -854,  -870,   478,
  -108,  -308,   996,   991,   958, -1460,  1522,  1628
];

static MZETA: [i32; 128] = [1, -1600, -749, -40, -687, 630, -1432, 848, 1062, -1410, 193, 797, -543, -69, 569, -1583, 296, -882, 1339, 1476, -283, 56, -1089, 1333, 1426, -1235, 535, -447, -936, -450, -1355, 821, 289, 331, -76, -1573, 1197, -1025, -1052, -1274, 650, -1352, -816, 632, -464, 33, 1320, -1414, -1010, 1435, 807, 452, 1438, -461, 1534, -927, -682, -712, 1481, 648, -855, -219, 1227, 910, 17, -568, 583, -680, 1637, 723, -1041, 1100, 1409, -667, -48, 233, 756, -1173, -314, -279, -1626, 1651, -540, -1540, -1482, 952, 1461, -642, 939, -1021, -892, -941, 733, -992, 268, 641, 1584, -1031, -1292, -109, 375, -780, -1239, 1645, 1063, 319, -556, 757, -1230, 561, -863, -735, -525, 1092, 403, 1026, 1143, -1179, -554, 886, -1607, 1212, -1455, 1029, -1219, -394, 885, -1175];

fn ffqmul(k: usize, r: i16) -> i16 {
  // let mr = MZETA[k]*r as i32;
  ((MZETA[k]*r as i32) - 3329*((MZETA[k]*r as i32)/3329)) as i16
}

static MRZETA: [i64; 128] = [1, -1600, -749, -40, -687, 13004459256, -1432, 848, 1062, -1410, 193, 797, -543, -69, 569, -1583, 296, -882, 1339, 1476, -283, 56, -1089, 1333, 1426, -1235, 535, -447, -936, -450, -1355, 821, 289, 331, -76, -1573, 1197, -1025, -1052, -1274, 650, -1352, -816, 632, -464, 33, 1320, -1414, -1010, 1435, 807, 452, 1438, -461, 1534, -927, -682, -712, 1481, 648, -855, -219, 1227, 910, 17, -568, 583, -680, 1637, 723, -1041, 1100, 1409, -667, -48, 233, 756, -1173, -314, -279, -1626, 1651, -540, -1540, -1482, 952, 1461, -642, 939, -1021, -892, -941, 733, -992, 268, 641, 1584, -1031, -1292, -109, 375, -780, -1239, 1645, 1063, 319, -556, 757, -1230, 561, -863, -735, -525, 1092, 403, 1026, 1143, -1179, -554, 886, -1607, 1212, -1455, 1029, -1219, -394, 885, -1175];

// // 13004459256
// fn ffqmul2(mz: i32, mrz: i64, r: i16) -> i16 {
//   ((mz * r as i32) - 3329*((mrz * r as i64)>>36) as i32) as i16
//   // let a = 18924602 * r as i64;
//   // ((a - ((((a*10995)>>40)*1600000000)>>4))*3329) as i16
// }

pub fn fqmul(c: i16, b: i16) -> i16
{
  // let a = c as i32 * b as i32;
  // let ua = a.wrapping_mul(QINV) as i16;
  // let u = ua as i32;
  // let mut t = u * KYBER_Q as i32;
  // t = a - t;
  // t >>= 16;
  // t as i16
  (((c as i32 * b as i32) - 3329*(((c as i32 * b as i32).wrapping_mul(62209) as i16) as i32))>>16) as i16
}

fn ffqmul2(k:usize, r: i16) -> i16 {

  // a = z * r;
  //   m(a) = a*QINV mod 3329
  // m( z * r) = m(z)*r - 3329(floor(m(z)*r / 3329))

  (MZETA[k] as i32 * r as i32 - 3329*((MRZETA[k] as i64 * r as i64)>>36) as i32) as i16
}

fn fqmulr(r: i16) -> i16 {
  (r as i32 - 3329*(r/3329) as i32) as i16
}

fn main () -> Result<(), KyberError> {
  let mut rng = rand::thread_rng();

  let mut t: [i16; 65536] = [0i16; 65536];
  let mut shift4: [i32; 65536] = [0i32; 65536];
  let mut shift8: [i32; 65536] = [0i32; 65536];
  let mut diff: [i16; 65536] = [0i16; 65536];
  let mut count = 0;
  let mut counta = 0;
  let mut vetcount: [i32; 65536] = [0i32; 65536];
  let mut y_1 = 0;
  let mut co = 0;
  let mut f1 = 0;
  let mut f2 = 0;
  let mut f3 = 0;
  let mut ffqmul2_time = 0;
  // println!("mont zerta5 {}\n", montgomery_reduce_a(ZETAS[5] as i32));
  for a in 0..65536 {
    // t[a] = (((a as i32)) & 15)*169 - (((a as i32) & 48) >> 4 )*625 + (((a as i32) & 192) >> 6)*829 - ((((a as i32) & 32512) >> 8)*13);
    // print!("{}: ", a);
    // for i in 0..128 {
    //   let g = (ZETAS[i] as i32 * (a as i32)) as i16;
      // print!("{} ", g);
      // let ii = ZETAS[5] as i16;
    if a == 21893 {
      println!("CIAO");
    }
      let now = Instant::now();
      fqmul(ZETAS[5], a as i16);
      let elapsed = now.elapsed().as_nanos();
      let now2 = Instant::now();
      ffqmul(5, a as i16);
      let elapsed2 = now2.elapsed().as_nanos();
      let now3 = Instant::now();
      // ffqmul2(MZETA[5], MRZETA[5],a as i16);
      ffqmul2(5,a as i16);
      let elapsed3 = now3.elapsed().as_nanos();
      let v  = fqmul(ZETAS[5], a as i16);
      // let u = ffqmul2(MZETA[5], MRZETA[5],a as i16);
      let u = ffqmul2(5,a as i16);
      println!("fqmul {} time {}",v,  elapsed);
      println!("ffqmul time {}",  elapsed2);
      println!("ffqmul2 {} time {}\n",u, elapsed3);
    // }
    // println!("\n");
    // shift4[a] = (a as i32) >> 4;
    if a <= 0{
      continue;
    }
    // diff[a] = t[a] - t[a-1];
    // if diff[a] != -3160 && diff[a] != 169 {
    //   // println!("{}", d);
    //   count = 0;
    //   counta += 1;
    // }
    // println!("{} {} {}", t[a], counta, diff[a]);
    // count += 1;
    // vetcount[a] = counta;
    if elapsed < elapsed3 && elapsed < elapsed2{
      f1 +=1;
    }else if elapsed == elapsed3 && elapsed3 == elapsed2{
      f2 +=1;
    }else if elapsed2 < elapsed3{
      ffqmul2_time += 1;
    }else{
      f3 +=1;
    }
  }
  println!("FQMUL {}", f1);
  println!("Pareggio {}", f2);
  println!("FQMUL2: {}", ffqmul2_time);
  println!("FQMUL3: {}\n", f3);
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