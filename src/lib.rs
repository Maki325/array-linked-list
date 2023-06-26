use std::{
  fmt::{Debug, Display, Write},
  mem::size_of,
  ops::{Index, IndexMut},
};

#[derive(Debug, Clone)]
#[repr(C)]
pub struct Node<T> {
  data: T,
  next: Option<isize>,
  prev: Option<isize>,
  position_offset: isize,
}

#[derive(Clone)]
pub struct List<T> {
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

impl<T: Debug> Debug for List<T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    return f.debug_list().entries(self.iter()).finish();
  }
}

impl<T: Display> Display for List<T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut is_first = true;
    f.write_char('[')?;
    for node in &self.nodes {
      if !is_first {
        f.write_str(", ")?;
      }
      write!(f, "{}", node.data)?;

      is_first = false;
    }
    f.write_char(']')?;

    return Ok(());
  }
}

impl<T: std::fmt::Debug> List<T> {
  pub fn print_node_debug(&self) {
    println!("{:#?}", self.nodes);
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
    pub struct $struct_name<$($lifetime,)? T> {
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
