use crate::{
  params::*,
  ntt::*,
  reduce::*,
  cbd::*,
  symmetric::*
};
use std::time::{Duration, Instant};

#[derive(Clone, Debug)]
pub struct Poly {
  pub coeffs: [i16; KYBER_N]  
}

impl Copy for Poly {}

impl Default for Poly {
  fn default() -> Self {
    Poly {
      coeffs: [0i16; KYBER_N]
    }
  }
}

// new() is nicer
impl Poly {
  pub fn new() -> Self {
    Self::default()
  }
}

// Name:        poly_compress
//
// Description: Compression and subsequent serialization of a polynomial
//
// Arguments:   - [u8] r: output byte array (needs space for KYBER_POLYCOMPRESSEDBYTES bytes)
//              - const poly *a:    input polynomial
pub fn poly_compress(r: &mut[u8], a: Poly)
{
  let mut t = [0u8; 8];
  let mut k = 0usize;
  let mut u: i16;

  match KYBER_POLYCOMPRESSEDBYTES {
    128 => {
      for i in 0..KYBER_N/8 {
        for j in 0..8 {
          // map to positive standard representatives
          u = a.coeffs[8*i+j];
          u += (u >> 15) & KYBER_Q as i16;
          t[j] = (((((u as u16) << 4) + KYBER_Q as u16 /2) / KYBER_Q as u16) & 15) as u8;
        }
        r[k]   = t[0] | (t[1] << 4);
        r[k+1] = t[2] | (t[3] << 4);
        r[k+2] = t[4] | (t[5] << 4);
        r[k+3] = t[6] | (t[7] << 4);
        k += 4;
      }
    },
    160 => {
      for i in 0..(KYBER_N/8) {
        for j in 0..8 {
          // map to positive standard representatives
          u = a.coeffs[8*i+j];
          u += (u >> 15) & KYBER_Q as i16;
          t[j] = (((((u as u32) << 5) + KYBER_Q as u32/2) / KYBER_Q as u32) & 31) as u8;
        }
        r[k]   =  t[0]       | (t[1] << 5);
        r[k+1] = (t[1] >> 3) | (t[2] << 2) | (t[3] << 7);
        r[k+2] = (t[3] >> 1) | (t[4] << 4);
        r[k+3] = (t[4] >> 4) | (t[5] << 1) | (t[6] << 6);
        r[k+4] = (t[6] >> 2) | (t[7] << 3);
        k += 5;
      }
    },
    _ => panic!("KYBER_POLYCOMPRESSEDBYTES needs to be one of (128, 160)")
  }
}


// Name:        poly_decompress
//
// Description: De-serialization and subsequent decompression of a polynomial;
//              approximate inverse of poly_compress
//
// Arguments:   - poly *r:                output polynomial
//              - const [u8] a: input byte array (of length KYBER_POLYCOMPRESSEDBYTES bytes)
pub fn poly_decompress(r: &mut Poly, a: &[u8])
{
  match KYBER_POLYCOMPRESSEDBYTES {
    128 => {
      let mut idx = 0usize;
      for i in 0..KYBER_N/2 {
        r.coeffs[2*i+0] = ((((a[idx] & 15) as usize * KYBER_Q) + 8) >> 4) as i16;
        r.coeffs[2*i+1] = ((((a[idx] >> 4) as usize * KYBER_Q) + 8) >> 4) as i16;
        idx += 1;
      }
    },
    160 => {
      let mut idx = 0usize;
      let mut t = [0u8;8];
      for i in 0..KYBER_N/8 {
        t[0] = a[idx+0];
        t[1] = (a[idx+0] >> 5) | (a[idx+1] << 3);
        t[2] = a[idx+1] >> 2;
        t[3] = (a[idx+1] >> 7) | (a[idx+2] << 1);
        t[4] = (a[idx+2] >> 4) | (a[idx+3] << 4);
        t[5] = a[idx+3] >> 1;
        t[6] = (a[idx+3] >> 6) | (a[idx+4] << 2);
        t[7] = a[idx+4] >> 3;
        idx += 5;
        for j in 0..8 {
          r.coeffs[8*i+j] = ((((t[j] as u32) & 31)*KYBER_Q as u32 + 16) >> 5) as i16;
        }
      }
    },
    _ => panic!("KYBER_POLYCOMPRESSEDBYTES needs to be either (128, 160)")
  }
}

// Name:        poly_tobytes
//
// Description: Serialization of a polynomial
//
// Arguments:   - [u8] r: output byte array (needs space for KYBER_POLYBYTES bytes)
//              - const poly *a:    input polynomial
pub fn poly_tobytes(r: &mut[u8], a: Poly)
{
  let (mut t0, mut t1);

  for i in 0..(KYBER_N/2) {
    // map to positive standard representatives
    t0 = a.coeffs[2*i];
    t0 += (t0 >> 15) & KYBER_Q as i16;
    t1 = a.coeffs[2*i+1];
    t1 += (t1 >> 15) & KYBER_Q as i16;
    r[3*i+0] = (t0 >> 0) as u8;
    r[3*i+1] = ((t0 >> 8) | (t1 << 4)) as u8;
    r[3*i+2] = (t1 >> 4) as u8;
  }
}

// Name:        poly_frombytes
//
// Description: De-serialization of a polynomial;
//              inverse of poly_tobytes
//
// Arguments:   - poly *r:                output polynomial
//              - const [u8] a: input byte array (of KYBER_POLYBYTES bytes)
pub fn poly_frombytes(r: &mut Poly, a: &[u8])
{
  for i in 0..(KYBER_N/2) {
    r.coeffs[2*i+0] = ((a[3*i+0] >> 0) as u16 | ((a[3*i+1] as u16) << 8) & 0xFFF) as i16;
    r.coeffs[2*i+1] = ((a[3*i+1] >> 4) as u16 | ((a[3*i+2] as u16) << 4) & 0xFFF) as i16;
  }
}

// Name:        poly_getnoise_eta1
//
// Description: Sample a polynomial deterministically from a seed and a nonce,
//              with output polynomial close to centered binomial distribution
//              with parameter KYBER_ETA1
//
// Arguments:   - poly *r:                   output polynomial
//              - const [u8] seed: input seed (pointing to array of length KYBER_SYMBYTES bytes)
//              - [u8]  nonce:       one-byte input nonce
pub fn poly_getnoise_eta1(r: &mut Poly, seed: &[u8], nonce: u8)
{
  const LENGTH: usize = KYBER_ETA1*KYBER_N/4;
  let mut buf = [0u8; LENGTH];
  prf(&mut buf, LENGTH, seed, nonce);
  poly_cbd_eta1(r, &buf);
}

// Name:        poly_getnoise_eta2
//
// Description: Sample a polynomial deterministically from a seed and a nonce,
//              with output polynomial close to centered binomial distribution
//              with parameter KYBER_ETA2
//
// Arguments:   - poly *r:                   output polynomial
//              - const [u8] seed: input seed (pointing to array of length KYBER_SYMBYTES bytes)
//              - [u8]  nonce:       one-byte input nonce
pub fn poly_getnoise_eta2(r: &mut Poly, seed: &[u8], nonce: u8)
{
  const LENGTH: usize = KYBER_ETA2*KYBER_N/4;
  let mut buf = [0u8; LENGTH];
  prf(&mut buf, LENGTH, seed, nonce);
  poly_cbd_eta2(r, &buf);
}



// Name:        poly_ntt
//
// Description: Computes negacyclic number-theoretic transform (NTT) of
//              a polynomial in place;
//              inputs assumed to be in normal order, output in bitreversed order
//
// Arguments:   - Poly r: in/output polynomial
pub fn poly_ntt(r: &mut Poly) 
{
  ntt(&mut r.coeffs);
  poly_reduce(r);
}

// Name:        poly_invntt
//
// Description: Computes inverse of negacyclic number-theoretic transform (NTT) of
//              a polynomial in place;
//              inputs assumed to be in bitreversed order, output in normal order
//
// Arguments:   - Poly a: in/output polynomial
pub fn poly_invntt_tomont(r: &mut Poly)
{
  invntt(&mut r.coeffs);
}

fn check(original: &[i16], modified: &[i16]){
  let mut count = 0;
  for i in 0..KYBER_N {
    if original[i] != modified[i] {
      count += 1;
      println!("index: {} original: {} modified: {}", i, original[i], modified[i]);
    }
  }
  println!("\n{}", count);
}

#[derive(Debug)]
pub struct Addends {
  a: i32,
  b: i32,
  has_primes: bool
}

#[derive(Debug)]
pub struct Info {
  mont: i16,
  addends: Vec<Addends>
}

fn is_prime(n: u32) -> bool {
  let limit = (n as f64).sqrt() as u32;
  for i in 2..=limit {
    if n % i == 0 {
      return false;
    }
  }
  true
}

pub fn try_all(){
  let mut inv_mont = [0i16; 3328];
  let offset = 1664;
  let f = ((1u64 << 32) % KYBER_Q as u64) as i16;
  for i in -1664..1664 {
    inv_mont[(i + offset) as usize] = montgomery_reduce(i as i32 * f as i32);
  }
  println!("{:?}", inv_mont);
  return;
  // const OFFSET: i32 =  20_000;
  // let mut occ = [0i32; 20_000];
  // let occ: [Vec<i16>; 36_000_000] = std::iter::repeat_with(|| Vec::new())
  //     .take(36_000_000)
  //     .collect::<Vec<_>>()
  //     .try_into()
  //     .unwrap();
  // let mut occ = std::iter::repeat_with(|| Vec::new())
  //     .take(20_000)
  //     .collect::<Vec<_>>();
  let mut info = std::iter::repeat_with(|| Vec::<Info>::new())
      .take(2_000)
      .collect::<Vec<_>>();
  for i in -1_000..1_000 {
    for j in (i)..1_000 {
      let index = (i + j) as usize;
      if i + j < 0 {
        continue;
      }
      let m = montgomery_reduce(i) + montgomery_reduce(j);
      let mut new = true;
      for k in info[index].iter_mut() {
        if k.mont == m {
          new = false;
          let mut prime = false;
          if i >= 0 {
            prime = is_prime(i as u32);
          }else if j >= 0 {
            prime = is_prime(j as u32);
          }
          k.addends.push(Addends { a: i, b: j, has_primes: prime});
          break;
        }
      }
      // for a in occ[index].iter_mut() {
      //   if *a == m {
      //     new = false;
      //     break;
      //   }
      // }
      if new {
        // occ[index].push(m);
        info[index].push(Info{ mont: m, addends: Vec::new() });
      }
    }
  }
  // for i in -10_000..0 {
  //   for j in (i)..10_000 {
  //     let index = (i + j) as usize;
  //     if i + j > 0 {
  //       let m = montgomery_reduce(i) + montgomery_reduce(j);
  //       let mut new = true;
  //       // if occ[index].last().is_none() {
  //       //   new = true;
  //       // }
  //       for a in occ[index].iter_mut() {
  //         if *a == m {
  //           new = false;
  //           break;
  //         }
  //       }
  //       if new {
  //         occ[index].push(m);
  //       }
  //     }
  //   }
  // }
  for i in 0..2_000 {
    // println!("{}: {:?}", i, occ[i]);
    println!("{}: {:?}", i, info[i]);
  }
  // println!("{:?}", occ);
}

const MMZ: [i16; 64] = [-456, 549, -1343, 1595, 346, -986, 508, -524, -1567, 463, -1454, -571, 1262, 1503, 198, -545, 1513, -617, -1377, -598, -783, 1096, 563, 1359, -1101, 559, -943, 763, 704, -1198, -1314, -1528, 1376, -1131, 1366, 1553, 124, 1340, 336, -1631, -119, 647, -752, 1431, -1472, 1597, 629, -1042, 1158, 1453, 1527, 286, 85, 489, -414, -71, 1395, -1570, 451, 793, 387, -6, -240, 1165];

// Name:        poly_basemul
//
// Description: Multiplication of two polynomials in NTT domain
//
// Arguments:   - poly *r:       output polynomial
//              - const poly *a: first input polynomial
//              - const poly *b: second input polynomial
pub fn poly_basemul(r: &mut Poly, a: &Poly, b: &Poly)
{
  let mut r1 = Poly{
    coeffs: [0i16; KYBER_N]
  };
  let mut ac = &Poly {
    coeffs: [802, 317, 219, 1074, 3017, 3079, 1237, 803, 2033, 2402, 2229, 2786, 699, 2416, 1995, 230, 951, 2234, 2980, 3062, 1492, 1702, 553, 644, 2936, 3117, 240, 2407, 78, 2837, 3247, 2738, 211, 2639, 653, 2281, 1781, 2450, 3236, 711, 934, 1170, 2541, 1226, 1527, 1985, 3045, 1384, 822, 498, 1825, 1664, 793, 2912, 1891, 3061, 1730, 751, 745, 1362, 578, 1885, 1626, 989, 2082, 1980, 467, 571, 1918, 1908, 1520, 28, 2245, 1853, 1033, 1459, 1861, 2943, 2648, 1517, 1796, 387, 2891, 701, 285, 1279, 957, 2447, 1961, 1065, 2818, 1689, 1047, 1727, 1519, 197, 3278, 2532, 105, 1214, 1179, 2219, 920, 347, 2506, 1827, 1805, 614, 572, 3149, 421, 1312, 918, 785, 112, 2325, 2267, 1467, 1050, 1355, 1008, 217, 3242, 3040, 1538, 621, 1254, 1023, 1363, 3049, 1906, 2824, 59, 1213, 3231, 3074, 31, 3276, 1422, 3216, 383, 1149, 606, 2737, 894, 1657, 2697, 1805, 2548, 462, 221, 868, 1661, 1450, 1879, 2923, 778, 1473, 2318, 480, 1827, 396, 1105, 3186, 1634, 274, 2609, 2578, 2900, 1202, 766, 793, 2534, 413, 1452, 949, 2779, 82, 604, 2931, 263, 1022, 13, 1715, 1264, 584, 776, 3011, 217, 2021, 1646, 794, 2047, 1414, 791, 2608, 704, 88, 68, 522, 165, 3258, 1221, 1683, 868, 31, 1317, 2104, 1414, 1055, 2734, 1830, 2766, 3117, 1745, 1267, 83, 2465, 1444, 131, 1893, 90, 1450, 2346, 2487, 2428, 2828, 3291, 769, 1681, 1810, 3216, 1507, 1157, 791, 3001, 3093, 1480, 2405, 13, 1736, 2308, 3, 1821, 2504, 3289, 2938, 1120, 1870, 1968, 2577, 2952, 300, 2231, 428, 2723]
  };
  let mut bc = &Poly {
    coeffs: [-1175, -966, 611, -604, -984, 385, 1584, 317, 1124, 188, -32, 1029, 112, 1211, 351, -606, 1229, 1064, 1566, -1275, 0, -1182, -327, 555, -945, -1477, -671, 1191, 382, -759, -180, 21, 677, 1465, -680, 1432, -1531, -1173, 684, 223, 1030, 1603, 1014, -294, -404, 1235, 681, 117, 104, -1591, -1136, -1324, 1472, 516, -1234, -1525, -1590, -908, 297, -719, -1115, 767, -963, -1364, -220, -293, 180, 704, 168, 1494, 556, 724, -724, 923, 1382, -597, -863, -641, -959, 591, 1475, 966, -998, -943, 970, -1578, -853, -1604, -614, 1477, 1237, -1194, -1142, 471, 672, -358, -725, -1509, 109, 1550, -921, -369, -1133, 469, 1566, -1236, 59, -733, 1518, -1042, -125, -1261, 1445, 1465, -114, 59, 1173, -677, -813, -310, -721, 106, 464, 377, -1590, 1052, 261, -631, 60, -729, -1097, 164, -1329, -325, 1594, 488, -987, -928, -1053, 254, 1153, -762, 891, -768, 1404, 597, -1226, 44, -1272, 1296, 1014, 449, -1024, 802, 1541, -154, -1337, 951, -1119, -139, 712, -1457, 75, -191, -96, -1496, 10, -1590, -1608, -1440, 940, 10, -1228, -11, 785, -890, 1462, 1445, 1169, -1512, -1302, 1109, 911, 1097, -976, -125, 1394, 755, -9, -1086, -915, -1388, -1142, 233, -279, -718, -1435, -1309, 397, -1458, 315, -584, -1175, -1598, -335, 976, -1564, 756, -1530, -615, -1560, 581, -1091, -1619, -1034, -1329, 1518, 1278, 518, -664, -533, -1369, -1041, -597, 1095, -411, -1559, -184, 1130, -1330, 1568, 19, 620, -841, 615, 393, -731, -1135, 1011, 1556, 187, -1108, -509, -1019, -367, 1565, 383, -1651, 1374, -657, 1016, 1224, -873, -876, -1111, -158]
  };
  // let now = Instant::now();
  // println!("\n\nR: {:?}", r);
  // println!("R1: {:?}", r1);
  for i in 0..(KYBER_N/4) {
    
    basemul_m(
      &mut r.coeffs[4*i..], 
      &ac.coeffs[4*i..],
      &bc.coeffs[4*i..],
      ZETAS[64 + i]
    );
    basemul_m(
      &mut r.coeffs[4*i+2..], 
      &ac.coeffs[4*i+2..],
      &bc.coeffs[4*i+2..],
      -(ZETAS[64 + i]));
  }
  // for i in 0..(KYBER_N/4) {
  //
  //   basemul_m(
  //     &mut r1.coeffs[4*i..],
  //     &ac.coeffs[4*i..],
  //     &bc.coeffs[4*i..],
  //     ZETAS[64 + i]
  //   );
  //   basemul_m(
  //     &mut r1.coeffs[4*i+2..],
  //     &ac.coeffs[4*i+2..],
  //     &bc.coeffs[4*i+2..],
  //     -(ZETAS[64 + i]));
  // }
  // check(&r.coeffs,&r1.coeffs);
  // println!("{:?}", r);
  // println!("{:?}", r1);
  // println!("Basemul: {}", now.elapsed().as_nanos());
}

// Name:        poly_frommont
//
// Description: Inplace conversion of all coefficients of a polynomial 
//              from Montgomery domain to normal domain
//
// Arguments:   - poly *r:       input/output polynomial
pub fn poly_frommont(r: &mut Poly)
{
  let f = ((1u64 << 32) % KYBER_Q as u64) as i16;
  for i in 0..KYBER_N {
    let a = r.coeffs[i] as i32 * f as i32;
    r.coeffs[i] = montgomery_reduce(a);
  }
}

// Name:        poly_reduce
//
// Description: Applies Barrett reduction to all coefficients of a polynomial
//              for details of the Barrett reduction see comments in reduce.c
//
// Arguments:   - poly *r:       input/output polynomial
pub fn poly_reduce(r: &mut Poly)
{
  for i in 0..KYBER_N {
    r.coeffs[i] = barrett_reduce(r.coeffs[i]);
  }
}

// Name:        poly_add
//
// Description: Add two polynomials; no modular reduction is performed
//
// Arguments: - poly *r:       output polynomial
//            - const poly *a: first input polynomial
//            - const poly *b: second input polynomial
pub fn poly_add(r: &mut Poly, b: &Poly)
{
  for i in 0..KYBER_N {
    r.coeffs[i] += b.coeffs[i];
  }
}

// Name:        poly_sub
//
// Description: Subtract two polynomials; no modular reduction is performed
//
// Arguments: - poly *r:       output polynomial
//            - const poly *a: first input polynomial
//            - const poly *b: second input polynomial
pub fn poly_sub(r: &mut Poly, a: &Poly)
{
  for i in 0..KYBER_N {
    r.coeffs[i] = a.coeffs[i] -  r.coeffs[i];
  }
}

// Name:        poly_frommsg
//
// Description: Convert `KYBER_SYMBYTES`-byte message to polynomial
//
// Arguments:   - poly *r:                  output polynomial
//              - const [u8] msg: input message (of length KYBER_SYMBYTES)
pub fn poly_frommsg(r: &mut Poly, msg: &[u8])
{
  let mut mask;
  for i in 0..KYBER_SYMBYTES {
    for j in 0..8 {
      mask = ((msg[i] as u16 >> j) & 1 ).wrapping_neg();
      r.coeffs[8*i+j] = (mask & ((KYBER_Q+1)/2) as u16) as i16;
    }
  }
}

// Name:        poly_tomsg
//
// Description: Convert polynomial to 32-byte message
//
// Arguments:   - [u8] msg: output message
//              - const poly *a:      input polynomial
pub fn poly_tomsg(msg: &mut[u8], a: Poly)
{
  let mut t;

  for i in 0..KYBER_SYMBYTES {
    msg[i] = 0;
    for j in 0..8 {
      t  = a.coeffs[8*i+j];
      t += (t >> 15) & KYBER_Q as i16;
      t  = (((t << 1) + KYBER_Q as i16 /2) / KYBER_Q as i16) & 1;
      msg[i] |= (t << j) as u8;
    }
  }
}
