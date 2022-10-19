use std::collections::HashSet;
use std::mem;

use crate::cidr::Cidrv4;
use crate::collider::Collider;
use crate::mark::Mark;
use crate::overlap::Overlap;

enum Node {
  Inner(Box<(Node, Node)>),
  Leaf(HashSet<Mark>),
}

impl Node {
  fn mark_descend(mut self, ip: u32, bits: u8, mark: Mark) -> Self {
    if bits == 0 {
      self.mark_apply(mark);
      self
    } else {
      let mut children = match self {
        Node::Inner(c) => c,
        Node::Leaf(marks) => {
          let l_marks = marks.clone();
          Box::new((
            Node::Leaf(l_marks),
            Node::Leaf(marks),
          ))
        },
      };
      if ip & (1 << 31) == 0 {
        children.0 = children.0.mark_descend(ip << 1, bits - 1, mark);
      } else {
        children.1 = children.1.mark_descend(ip << 1, bits - 1, mark);
      }
      Node::Inner(children)
    }
  }

  fn mark_apply(&mut self, mark: Mark) {
    match self {
      Node::Inner(c) => {
        c.0.mark_apply(mark);
        c.1.mark_apply(mark);
      },
      Node::Leaf(marks) => {
        marks.insert(mark);
      },
    };
  }

  fn accum_overlaps(&self, overlaps: &mut Vec<Overlap>, ip: u32, depth: u32) {
    match self {
      Node::Inner(c) => {
        c.0.accum_overlaps(overlaps, ip, depth + 1);
        c.1.accum_overlaps(overlaps, ip | (1 << (31 - depth)), depth + 1);
      },
      Node::Leaf(marks) => {
        if marks.len() > 1 {
          let end_ip = ip + 1u32.checked_shl(32 - depth).unwrap_or_default().wrapping_sub(1);
          overlaps.push(Overlap::new(ip.into()..=end_ip.into(), marks.clone()));
        }
      }
    }
  }
}

impl Default for Node {
  fn default() -> Self {
    Self::Leaf(HashSet::new())
  }
}

#[derive(Default)]
pub struct BitTrie {
  node: Node,
}

impl Collider for BitTrie {
  fn mark(&mut self, cidr: &Cidrv4, mark: Mark) {
    self.node = mem::take(&mut self.node).mark_descend(cidr.ip.into(), cidr.subnet_bits, mark);
  }

  fn overlaps(&self) -> Vec<Overlap> {
    let mut overlaps = vec![];
    self.node.accum_overlaps(&mut overlaps, 0, 0);
    overlaps
  }
}
