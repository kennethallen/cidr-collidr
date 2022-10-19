use std::collections::HashSet;

use crate::cidr::Cidrv4;
use crate::collider::Collider;
use crate::mark::Mark;
use crate::overlap::Overlap;

#[derive(Default)]
struct Node {
  left: Option<Box<Node>>,
  right: Option<Box<Node>>,
  marks: HashSet<Mark>,
}

impl Node {
  fn mark_descend(&mut self, ip: u32, bits: u8, mark: Mark) {
    if bits == 0 {
      self.marks.insert(mark);
    } else if ip & (1 << 31) == 0 {
      self.left.get_or_insert_default().mark_descend(ip << 1, bits - 1, mark);
    } else {
      self.right.get_or_insert_default().mark_descend(ip << 1, bits - 1, mark);
    }
  }

  fn accum_overlaps(&self, overlaps: &mut Vec<Overlap>, ip: u32, depth: u32, mut marks: HashSet<Mark>) {
    for &mark in &self.marks {
      marks.insert(mark);
    }
    let child_size = 1u32 << (31 - depth);
    if let Some(left) = &self.left {
      left.accum_overlaps(overlaps, ip, depth + 1, marks.clone());
      if let Some(right) = &self.right {
        right.accum_overlaps(overlaps, ip + child_size, depth + 1, marks);
      } else if marks.len() > 1 {
        let start_ip = ip + child_size;
        let end_ip = start_ip + (child_size - 1);
        overlaps.push(Overlap::new(start_ip.into()..=end_ip.into(), marks));
      }
    } else if let Some(right) = &self.right {
      if marks.len() > 1 {
        let end_ip = ip + (child_size - 1);
        overlaps.push(Overlap::new(ip.into()..=end_ip.into(), marks.clone()));
      }

      right.accum_overlaps(overlaps, ip + child_size, depth + 1, marks);
    } else if marks.len() > 1 {
      let end_ip = ip + (child_size << 1).wrapping_sub(1);
      overlaps.push(Overlap::new(ip.into()..=end_ip.into(), marks.clone()));
    }
  }
}

#[derive(Default)]
pub struct LayeredBitTrie {
  node: Node,
}

impl Collider for LayeredBitTrie {
  fn mark(&mut self, cidr: &Cidrv4, mark: Mark) {
    self.node.mark_descend(cidr.ip.into(), cidr.subnet_bits, mark);
  }

  fn overlaps(&self) -> Vec<Overlap> {
    let mut overlaps = vec![];
    self.node.accum_overlaps(&mut overlaps, 0, 0, HashSet::new());
    overlaps
  }
}
