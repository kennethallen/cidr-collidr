use crate::cidr::Cidrv4;
use crate::mark::Mark;
use crate::overlap::Overlap;

pub trait Collider {
  fn mark(&mut self, cidr: &Cidrv4, mark: Mark);
  fn overlaps(&self) -> Vec<Overlap>;
}
