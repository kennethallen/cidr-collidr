use rand::prelude::*;
use rand_distr::Normal;
use std::cmp::{max, min};
use std::net::Ipv4Addr;

use crate::mark::Mark;

#[derive(Clone, Debug)]
pub struct Cidrv4 {
  pub ip: Ipv4Addr,
  pub subnet_bits: u8,
}

impl Cidrv4 {
  pub fn new(ip: Ipv4Addr, subnet_bits: u8) -> Self {
    debug_assert!(subnet_bits <= 32);
    debug_assert!({
      let ip: u32 = ip.into();
      let inv_subnet_mask = 1u32.checked_shl((32 - subnet_bits).into()).unwrap_or(0).wrapping_sub(1);
      ip & inv_subnet_mask == 0
    });
    Self { ip, subnet_bits }
  }

  pub fn after(&self) -> Option<Ipv4Addr> {
    1u32.checked_shl((32 - self.subnet_bits).into())
      .and_then(|bits| bits.checked_add(self.ip.into()))
      .map(Into::into)
  }

  pub fn random(rng: &mut impl Rng) -> Self {
    let bit_distr = Normal::new(16f32, 5f32).unwrap();
    let subnet_bits = bit_distr.sample(rng) as u8;
    let subnet_bits = max(0, min(32, subnet_bits));
    let ip = rng.next_u32();
    let ip = ip & !1u32.checked_shl((32 - subnet_bits).into()).unwrap_or(0).wrapping_sub(1);
    Self::new(ip.into(), subnet_bits)
  }
}

pub fn random_routers(n: Mark, rng: &mut impl Rng) -> Vec<(Mark, Cidrv4)> {
  (0..n).into_iter()
    .map(|i| (i, Cidrv4::random(rng)))
    .collect()
}
