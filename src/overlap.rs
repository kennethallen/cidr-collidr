use std::collections::HashSet;
use std::net::Ipv4Addr;
use std::ops::RangeInclusive;

use crate::mark::Mark;

#[derive(Clone, Debug)]
pub struct Overlap {
  range: RangeInclusive<Ipv4Addr>,
  marks: HashSet<Mark>,
}

impl Overlap {
  pub fn new(range: RangeInclusive<Ipv4Addr>, marks: HashSet<Mark>) -> Self {
    Self { range, marks }
  }
}
