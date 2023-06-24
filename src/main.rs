use std::{
  fmt::Debug,
  mem::{self, size_of},
  ops::{self, DerefMut, Index, IndexMut},
  slice::{self, SliceIndex},
};

#[derive(Debug)]
#[repr(C)]
struct Node<T> {
  data: T,
  next: Option<isize>,
  prev: Option<isize>,
  position_offset: isize,
}

#[derive(Debug)]
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
}

impl<T: std::fmt::Display + std::fmt::Debug> List<T> {
  pub fn print(&self) {
    let mut current = self.head;
    let mut idx = 0;
    while let Some(index) = current {
      let node = &self.nodes[index];

      let data = unsafe {
        ((node) as *const Node<T>)
          .clone()
          .offset(node.position_offset)
      }
      .cast::<T>();
      print!(
        "self.ptr: {:#?}, node: {:#?}, data: {:#?}, current: {:#?}, position_offset: {:#?}, data: {:#?}",
        self.nodes.as_ptr(),
        node as *const Node<T>,
        data,
        idx,
        node.position_offset,
        node.data
      );

      println!(" {}", node.data);
      // print!("{} ", node.data);
      current = node.next.map(|next| ((index as isize) + next) as usize);
      idx += 1;
    }
    println!();
  }
}

impl<T> Index<usize> for List<T> {
  type Output = T;

  fn index(&self, idx: usize) -> &Self::Output {
    return &self.nodes[idx as usize].data;
  }
}

impl<T> IndexMut<usize> for List<T> {
  fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
    return &mut self.nodes[idx as usize].data;
  }
}

struct ListIter<T> {
  ptr: *const Node<T>,
  len: usize,
  current: usize,
}

impl<T: Debug> Iterator for ListIter<T> {
  type Item = T;

  fn next(&mut self) -> Option<Self::Item> {
    if self.current >= self.len {
      None
    } else if size_of::<T>() == 0 {
      // purposefully don't use 'ptr.offset' because for
      // vectors with 0-size elements this would return the
      // same pointer.
      self.ptr = self.ptr.cast::<u8>().wrapping_add(1).cast::<Node<T>>();
      // .with_metadata_of(self.ptr);

      // self.ptr = self.ptr.wrapping_byte_add(1);

      // Make up a value of this ZST.
      Some(unsafe { mem::zeroed() })
    } else {
      let node = unsafe { self.ptr.clone().add(self.current) };
      let data = unsafe { node.offset((*node).position_offset) }.cast::<T>();
      print!(
        "self.ptr: {:#?}, node: {:#?}, data: {:#?}, current: {:#?}, position_offset: {:#?}, data: {:#?}",
        self.ptr,
        node,
        data,
        self.current,
        unsafe { (*node).position_offset },
        unsafe { &(*node).data }
      );

      self.current += 1;

      let read_memory = unsafe { std::ptr::read_unaligned(data) };
      println!(" read_memory: {:#?}", &read_memory as *const T,);
      Some(read_memory)
    }
    // return self.inner.nodes.get(self.current).map(|node| {
    //   self.current += 1;
    //   return node.data;
    // });
  }
}

impl<T: Debug> IntoIterator for List<T> {
  type Item = T;
  type IntoIter = ListIter<T>;

  fn into_iter(self) -> Self::IntoIter {
    let ptr = self.nodes.as_ptr();
    return ListIter {
      // inner: self.nodes.into_iter(),
      ptr,
      len: self.nodes.len(),
      // end: unsafe { ptr.add(self.nodes.len()) },
      current: 0,
    };
  }
}

struct ListRefIter<'a, T> {
  inner: std::slice::Iter<'a, Node<T>>,
}

impl<'a, T> Iterator for ListRefIter<'a, T> {
  type Item = &'a T;

  fn next(&mut self) -> Option<Self::Item> {
    return self.inner.next().map(|node| &node.data);
  }
}

impl<'a, T> IntoIterator for &'a List<T> {
  type Item = &'a T;
  type IntoIter = ListRefIter<'a, T>;

  fn into_iter(self) -> Self::IntoIter {
    return ListRefIter {
      inner: self.nodes.iter(),
    };
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
  println!();
  for value in list {
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
