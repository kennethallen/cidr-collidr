#![feature(test)]
#![feature(option_get_or_insert_default)]

mod bit_trie;
mod cidr;
mod collider;
mod layered_bit_trie;
mod mark;
mod overlap;
mod range_tree;

use crate::bit_trie::BitTrie;
use crate::cidr::random_routers;
use crate::collider::Collider;
use crate::layered_bit_trie::LayeredBitTrie;
use crate::range_tree::RangeTree;

fn main() {
  let mut a = BitTrie::default();
  let mut b = RangeTree::default();
  let mut c = LayeredBitTrie::default();

  for (mark, cidr) in random_routers(50, &mut rand::thread_rng()) {
    println!("{}: {:?}", mark, &cidr);
    a.mark(&cidr, mark);
    b.mark(&cidr, mark);
    c.mark(&cidr, mark);
  }

  println!();
  for o in a.overlaps() { println!("{:?}", o); }
  println!();
  for o in b.overlaps() { println!("{:?}", o); }
  println!();
  for o in c.overlaps() { println!("{:?}", o); }
}

#[cfg(test)]
mod tests {
  use crate::mark::Mark;
  use super::*;
  
  extern crate test;
  use test::Bencher;

  const BENCH_ROUTERS: Mark = 10_000;

  #[bench]
  fn bench_bit_trie(b: &mut Bencher) {
    let cidrs = random_routers(BENCH_ROUTERS, &mut rand::thread_rng());
    b.iter(|| {
      let mut col = BitTrie::default();
      for (mark, cidr) in &cidrs {
        col.mark(cidr, *mark);
      }
      col.overlaps()
    });
  }

  #[bench]
  fn bench_layered_bit_trie(b: &mut Bencher) {
    let cidrs = random_routers(BENCH_ROUTERS, &mut rand::thread_rng());
    b.iter(|| {
      let mut col = LayeredBitTrie::default();
      for (mark, cidr) in &cidrs {
        col.mark(cidr, *mark);
      }
      col.overlaps()
    });
  }

  #[bench]
  fn bench_range_tree(b: &mut Bencher) {
    let cidrs = random_routers(BENCH_ROUTERS, &mut rand::thread_rng());
    b.iter(|| {
      let mut col = RangeTree::default();
      for (mark, cidr) in &cidrs {
        col.mark(cidr, *mark);
      }
      col.overlaps()
    });
  }
}
