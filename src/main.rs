use std::{
  fmt::Debug,
  mem::size_of,
  ops::{Index, IndexMut},
};

#[derive(Debug, Clone)]
#[repr(C)]
struct Node<T> {
  data: T,
  next: Option<isize>,
  prev: Option<isize>,
  position_offset: isize,
}

#[derive(Debug, Clone)]
struct List<T> {
  head: Option<usize>,
  tail: Option<usize>,
  nodes: Vec<Node<T>>,
}

impl<T> List<T> {
  pub fn new() -> Self {
    return List {
      head: None,
      tail: None,
      nodes: Vec::new(),
    };
  }

  pub fn with_capacity(capacity: usize) -> Self {
    return List {
      head: None,
      tail: None,
      nodes: Vec::with_capacity(capacity),
    };
  }

  pub fn push(&mut self, value: T) {
    let index = self.nodes.len();
    self.nodes.push(Node {
      data: value,
      next: None,
      prev: Self::map(self.tail, index),
      position_offset: 0,
    });

    if let Some(tail) = self.tail {
      self.nodes[tail].next = Some((index as isize) - (tail as isize));
    }
    self.tail = Some(index);
    if let None = self.head {
      self.head = Some(index);
    }
  }

  fn map(val: Option<usize>, index: usize) -> Option<isize> {
    return val.map(|tail| (tail as isize) - (index as isize));
  }

  fn get_actual_index(&self, index: usize) -> usize {
    let node = &self.nodes[index];
    return (index as isize + node.position_offset) as usize;
  }

  pub fn insert(&mut self, index: usize, value: T) {
    let len = self.nodes.len();

    if index == 0 {
      // Add like head
      let node = Node {
        data: value,
        next: Self::map(self.head, len),
        prev: None,
        position_offset: 0,
      };

      self.head = Some(len);
      if self.tail == None {
        self.tail = Some(len);
      }

      self.nodes.push(node);
    } else if index == len {
      // Add like tail

      let node = Node {
        data: value,
        next: None,
        prev: Self::map(self.tail, len),
        // 0 because it's the last element
        position_offset: 0,
      };

      if let Some(tail) = self.tail {
        self.nodes[tail].next = Some((index as isize) - (tail as isize));
      }

      self.tail = Some(len);
      if self.head == None {
        self.head = Some(len);
      }

      self.nodes.push(node);

      // We can return early
      // because position_offset will be 0
      // as it's the last element
      return;
    } else if index > len {
      panic!("insertion index (is {index}) should be <= len (is {len})");
    } else {
      // Add in the middle

      let current = self.get_actual_index(index - 1);
      let next = self.get_actual_index(index);

      let node = Node {
        data: value,
        next: Self::map(Some(next), len),
        prev: Self::map(Some(current), index),
        position_offset: 0,
      };

      self.nodes[current].next = Some((len as isize) - (current as isize));
      self.nodes[next].prev = Some((len as isize) - (next as isize));

      self.nodes.push(node);
    }

    // [TODO]: Fix in future version, so it's not O(n), if possible
    if len != 0 {
      let mut current = self.head;
      let mut idx = 0;
      while let Some(current_index) = current {
        self.nodes[idx].position_offset = current_index as isize - idx as isize;
        let node = &self.nodes[current_index];
        current = node
          .next
          .map(|next| ((current_index as isize) + next) as usize);

        idx += 1;
      }
    }
  }

  pub fn len(&self) -> usize {
    return self.nodes.len();
  }

  pub fn capacity(&self) -> usize {
    return self.nodes.capacity();
  }

  pub fn into_iter(self) -> ListIter<T> {
    return ListIter::new(self.nodes);
  }

  pub fn iter(&self) -> ListRefIter<T> {
    return ListRefIter::new(&self.nodes);
  }

  pub fn iter_mut(&mut self) -> ListMutRefIter<T> {
    return ListMutRefIter::new(&mut self.nodes);
  }
}

impl<T: std::fmt::Display> List<T> {
  pub fn print(&self) {
    for node in &self.nodes {
      print!("{} ", node.data);
    }
    println!();
  }
}

impl<T: std::fmt::Debug> List<T> {
  pub fn print_debug(&self) {
    for node in &self.nodes {
      print!("{:#?} ", node.data);
    }
    println!();
  }
}

impl<T> Index<usize> for List<T> {
  type Output = T;

  fn index(&self, idx: usize) -> &Self::Output {
    return &self.nodes[self.get_actual_index(idx)].data;
  }
}

impl<T> IndexMut<usize> for List<T> {
  fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
    let idx = self.get_actual_index(idx);
    return &mut self.nodes[idx].data;
  }
}

macro_rules! impl_ref_iter {
  ($struct_name: ident, $const_or_mut:tt, $ptr_fn: ident, $next_data_return:expr $(, $lifetime:lifetime, $maybe_ref:tt)? $(; $only_mut:tt)?) => {
    struct $struct_name<$($lifetime,)? T> {
      ptr: *$const_or_mut Node<T>,
      len: usize,
      #[allow(dead_code)]
      nodes: $($maybe_ref)? $($lifetime)? Vec<Node<T>>,
      current: usize,
    }

    impl<$($lifetime,)? T> $struct_name<$($lifetime,)? T> {
      pub fn new(nodes: $($maybe_ref)? $($lifetime)? $($only_mut)? Vec<Node<T>>) -> $struct_name<T> {
        return {
          $struct_name {
            ptr: nodes.$ptr_fn(),
            len: nodes.len(),
            nodes,
            current: 0,
          }
        };
      }
    }

    impl<$($lifetime,)? T> Iterator for $struct_name<$($lifetime,)? T> {
      type Item = $($maybe_ref)? $($lifetime)? $($only_mut)? T;

      fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.len {
          None
        } else if size_of::<T>() == 0 {
          panic!("ZSTs are not supported");
        } else {
          let node = unsafe { self.ptr.clone().add(self.current) };
          let data = unsafe { node.offset((*node).position_offset) }.cast::<T>();

          self.current += 1;

          return Some($next_data_return(data));
        }
      }
    }

    impl<$($lifetime,)? T> IntoIterator for $($maybe_ref)? $($lifetime)? $($only_mut)? List<T> {
      type Item = $($maybe_ref)? $($lifetime)? $($only_mut)?  T;
      type IntoIter = $struct_name<$($lifetime,)? T>;

      fn into_iter(self) -> Self::IntoIter {
        return $struct_name::new($($maybe_ref)? $($only_mut)? self.nodes);
      }
    }
  };
}

impl_ref_iter! {
  ListIter,
  const,
  as_ptr,
  |data: *const T| {
    unsafe { std::ptr::read_unaligned(data) }
  }
}

impl_ref_iter! {
  ListRefIter,
  const,
  as_ptr,
  |data: *const T| {
    unsafe { &*data }
  },
  'a,
  &
}

impl_ref_iter! {
  ListMutRefIter,
  mut,
  as_mut_ptr,
  |data: *mut T| {
    unsafe { &mut *data }
  },
  'a,
  &;
  mut
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn new() {
    let list = List::<usize>::new();
    assert_eq!(list.len(), 0);
  }

  #[test]
  fn with_capacity() {
    const CAP: usize = 10;

    let list = List::<usize>::with_capacity(CAP);
    assert_eq!(list.capacity(), CAP);
  }

  #[test]
  fn push() {
    let mut list = List::<usize>::new();

    list.push(25);

    assert_eq!(list[0], 25);
  }

  #[test]
  fn push_multiple() {
    let mut list = List::<usize>::new();

    list.push(25);
    list.push(50);
    list.push(75);
    list.push(100);

    assert_eq!(list[0], 25);
    assert_eq!(list[1], 50);
    assert_eq!(list[2], 75);
    assert_eq!(list[3], 100);
  }

  #[test]
  fn insert_start() {
    let mut list = List::<usize>::new();

    list.push(25);
    list.insert(0, 20);

    assert_eq!(list[0], 20);
    assert_eq!(list[1], 25);
  }

  #[test]
  fn insert_multiple_start() {
    let mut list = List::<usize>::new();

    list.push(25);
    list.insert(0, 20);
    list.insert(0, 15);
    list.insert(0, 10);

    assert_eq!(list[0], 10);
    assert_eq!(list[1], 15);
    assert_eq!(list[2], 20);
    assert_eq!(list[3], 25);
  }

  #[test]
  fn insert_end() {
    let mut list = List::<usize>::new();

    list.push(25);
    list.insert(1, 50);

    assert_eq!(list[0], 25);
    assert_eq!(list[1], 50);
  }

  #[test]
  fn insert_multiple_end() {
    let mut list = List::<usize>::new();

    list.push(25);
    list.insert(1, 50);
    list.insert(2, 75);
    list.insert(3, 100);

    assert_eq!(list[0], 25);
    assert_eq!(list[1], 50);
    assert_eq!(list[2], 75);
    assert_eq!(list[3], 100);
  }

  #[test]
  fn insert() {
    let mut list = List::<usize>::new();

    list.push(25);
    list.push(75);
    list.insert(1, 50);

    assert_eq!(list[0], 25);
    assert_eq!(list[1], 50);
    assert_eq!(list[2], 75);
  }

  #[test]
  fn insert_multiple() {
    let mut list = List::<usize>::new();

    list.push(25);
    list.push(75);
    list.insert(1, 50);
    list.insert(1, 45);
    list.insert(1, 40);

    assert_eq!(list[0], 25);
    assert_eq!(list[1], 40);
    assert_eq!(list[2], 45);
    assert_eq!(list[3], 50);
    assert_eq!(list[4], 75);
  }

  #[test]
  #[should_panic]
  fn insert_out_of_bounds() {
    let mut list = List::<usize>::new();

    list.push(25);
    list.push(75);
    list.insert(25, 50);
  }

  #[test]
  fn into_iter_use_after() {
    let mut list = List::<usize>::new();

    list.push(25);
    list.push(50);
    list.push(75);

    for (idx, value) in list.clone().into_iter().enumerate() {
      assert_eq!(list[idx], value);
    }
  }

  #[test]
  fn iter() {
    let mut list = List::<usize>::new();

    list.push(25);
    list.push(50);
    list.push(75);

    for (idx, value) in list.iter().enumerate() {
      assert_eq!(&list[idx], value);
    }
  }

  #[test]
  fn iter_mut() {
    let mut list = List::<usize>::new();

    list.push(25);
    list.push(50);
    list.push(75);

    let old = list.clone();

    for value in &mut list {
      *value += 100;
    }

    for (idx, value) in list.iter().enumerate() {
      assert_eq!(&list[idx], value);
      assert_eq!(&(old[idx] + 100), value);
    }
  }
}

fn main() {
  let mut list = List::<f32>::new();

  list.push(1.0);
  list.push(2.0);
  list.push(3.0);

  // for value in &list {
  //   print!("{} ", value);
  // }
  list.print();

  list.insert(0, 0.0);

  // for value in &list {
  //   print!("{} ", value);
  // }
  list.print();

  list.insert(4, 4.0);

  // // for value in &list {
  // //   print!("{} ", value);
  // // }
  list.print();
  // println!();

  list.insert(1, 0.5);

  println!("list: {:#?}", list);

  list.print();
  // println!("list: {:#?}", list);
  for value in &list {
    print!("{} ", value);
  }
  println!();
  for value in &list {
    print!("{} ", value);
  }
  println!();

  for value in &mut list {
    *value += 100.0;
  }

  for value in &list {
    print!("{} ", value);
  }
  println!();

  for value in &mut list {
    *value -= 100.0;
  }

  for value in &list {
    print!("{} ", value);
  }
  println!();

  // list.insert(4, 2.5);

  // // println!("list: {:#?}", list);

  // // // for value in &list {
  // // //   print!("{} ", value);
  // // // }
  // list.print();
  // println!();

  // println!("list: {:#?}", list);

  ///////////////////////////////////////////////////

  // let mut vec = Vec::new();

  // vec.push(1);
  // vec.push(2);
  // vec.push(3);

  // for value in &vec {
  //   println!("vec: {}", value);
  // }

  // println!("vec: {:#?}", vec);
}
