use std::cmp::Ordering;
use std::collections::{BTreeMap, HashSet};
use std::ops::Bound;

use crate::cidr::Cidrv4;
use crate::collider::Collider;
use crate::mark::Mark;
use crate::overlap::Overlap;

pub struct RangeTree {
  map: BTreeMap<u32, HashSet<Mark>>,
}

impl Default for RangeTree {
  fn default() -> Self {
    let mut map = BTreeMap::new();
    map.insert(0, HashSet::new());
    Self { map }
  }
}

impl Collider for RangeTree {
  fn mark(&mut self, cidr: &Cidrv4, mark: Mark) {
    let lo: u32 = cidr.ip.into();
    let hi: Option<u32> = cidr.after().map(Into::into);

    let mut split_lo: Option<HashSet<Mark>> = None;
    let mut split_hi: Option<HashSet<Mark>> = None;
    let mut first = true;
    let hi_bound = if let Some(hi) = hi { Bound::Included(hi) } else { Bound::Unbounded };
    for (entry_lo, entry_marks) in self.map.range_mut((Bound::Unbounded, hi_bound)).rev() {
      // For first, if our exclusive high boundary exists, skip it. If it doesn't, plan to split at it later.
      if first {
        first = false;
        if hi == Some(*entry_lo) {
          continue;
        } else {
          split_hi = Some(entry_marks.clone());
        }
      }

      // If this key is beyond our inclusive lower bound, end and split at it later.
      // If it equals our inclusive lower bound, process it normally and end (planning not to split).
      // Otherwise, process normally and continue.
      match lo.cmp(entry_lo) {
        Ordering::Greater => {
          split_lo = Some(entry_marks.clone());
          break;
        }, 
        Ordering::Equal   => {
          entry_marks.insert(mark);
          break;
        },
        Ordering::Less    => {
          entry_marks.insert(mark);
        },
      }
    }

    if let Some(mut lo_marks) = split_lo {
      lo_marks.insert(mark);
      self.map.insert(lo, lo_marks);
    }
    if let (Some(hi), Some(hi_marks)) = (hi, split_hi) {
      self.map.insert(hi, hi_marks);
    }
  }

  fn overlaps(&self) -> Vec<Overlap> {
    let mut overlaps = vec![];
    
    let mut prev: Option<(&u32, &HashSet<Mark>)> = None;
    for kv in &self.map {
      if let Some((&ip, marks)) = prev {
        overlaps.push(Overlap::new(ip.into()..=(kv.0 - 1).into(), marks.clone()));
      }
      prev = if kv.1.len() > 1 { Some(kv) } else { None };
    }
    if let Some((&ip, marks)) = prev {
      overlaps.push(Overlap::new(ip.into()..=0xFFFFFFFF.into(), marks.clone()));
    }

    overlaps
  }
}
