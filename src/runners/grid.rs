use std::borrow::Borrow;
use std::cmp;
use std::collections::hash_map;
use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct Grid<V: Clone> {
  items: HashMap<(isize, isize), V>,
  extents: [isize; 4],
}

impl<V: Clone> Grid<V> {
  pub fn new() -> Self {
    Self {
      items: HashMap::new(),
      extents: [0, 0, 0, 0],
    }
  }

  pub fn get<T>(&self, loc: T) -> Option<&V>
  where
    T: Borrow<(isize, isize)>,
  {
    self.items.get(loc.borrow())
  }

  pub fn insert(&mut self, loc: (isize, isize), value: V) {
    let (x, y) = loc;
    self.mark_extents(x, y);

    self.items.insert(loc, value);
  }

  #[allow(dead_code)]
  pub fn get_extent(&self, extent: Extent) -> isize {
    match extent {
      Extent::TOP => self.extents[0],
      Extent::RIGHT => self.extents[1],
      Extent::BOTTOM => self.extents[2],
      Extent::LEFT => self.extents[3],
    }
  }

  pub fn iter(&self) -> hash_map::Iter<'_, (isize, isize), V> {
    self.items.iter()
  }

  fn mark_extents(&mut self, x: isize, y: isize) {
    let [top, bottom, left, right] = self.extents;

    self.extents[0] = cmp::max(top, y);
    self.extents[1] = cmp::max(right, x);
    self.extents[2] = cmp::min(bottom, y);
    self.extents[3] = cmp::min(left, x);
  }
}

#[allow(dead_code)]
pub enum Extent {
  TOP,
  BOTTOM,
  LEFT,
  RIGHT,
}

#[test]
fn test_grid() {
  let mut g: Grid<i32> = Grid::new();
  assert_eq!(g.get((0, 0)), None);
  g.insert((0, 0), 42);
  assert_eq!(g.get((0, 0)), Some(&42));
  assert_eq!(g.get(&(0, 0)), Some(&42));
  g.insert((3, 4), 34);
  assert_eq!(g.get((3, 4)), Some(&34));

  assert_eq!(g.get_extent(Extent::TOP), 4);
  assert_eq!(g.get_extent(Extent::BOTTOM), 0);
  assert_eq!(g.get_extent(Extent::LEFT), 0);
  assert_eq!(g.get_extent(Extent::RIGHT), 3);
}
