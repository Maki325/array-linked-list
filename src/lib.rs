use std::{
  fmt::{Debug, Display, Write},
  mem::{size_of, MaybeUninit},
  ops::{Index, IndexMut},
};

#[derive(Debug, Clone)]
#[repr(C)]
pub struct Node<T> {
  /**
   * Can we somehow replace Optional<T> with just T
   * But still be able to extract the data from the node?
   * Idk really
   *
   * I meaaaaannn
   * We can just not delete it, but just skip the node
   * But I don't think that's smart
   * So some way to like, replace it with an empty value
   * BUT without Option<T> lol
   */
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
  len: usize,
  free_to_use: Vec<usize>,
}

impl<T> List<T> {
  pub fn new() -> Self {
    return List {
      head: None,
      tail: None,
      nodes: Vec::new(),
      len: 0,
      free_to_use: Vec::new(),
    };
  }

  pub fn with_capacity(capacity: usize) -> Self {
    return List {
      head: None,
      tail: None,
      nodes: Vec::with_capacity(capacity),
      len: 0,
      free_to_use: Vec::with_capacity(capacity),
    };
  }

  pub fn push(&mut self, value: T) {
    // Get the index of the free nodes it possible, otherwise append to the end!
    let index = self.free_to_use.pop().unwrap_or(self.nodes.len());

    let node = Node {
      data: value,
      next: None,
      prev: Self::map(self.tail, index),
      position_offset: 0,
    };

    if index == self.nodes.len() {
      self.nodes.push(node);
    } else {
      // Leave the position offset
      // And get the correct offset to here
      self.nodes[index] = node;
    }

    if let Some(tail) = self.tail {
      self.nodes[tail].next = Some((index as isize) - (tail as isize));
    }
    self.tail = Some(index);
    if let None = self.head {
      self.head = Some(index);
    }

    // You can CERTANLY not call the function, and calculate it on the spot, but OK lol
    if index != self.nodes.len() {
      self.fix_position_offsets();
    }

    self.len += 1;
  }

  fn map(val: Option<usize>, index: usize) -> Option<isize> {
    return val.map(|tail| (tail as isize) - (index as isize));
  }

  fn get_actual_index(&self, index: usize) -> usize {
    let node = &self.nodes[index];
    return (index as isize + node.position_offset) as usize;
  }

  fn fix_position_offsets(&mut self) {
    // [TODO]: Fix in future version, so it's not O(n), if possible
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

  pub fn insert(&mut self, index: usize, value: T) {
    // Get the index of the free nodes it possible, otherwise append to the end!
    let insert_idex = self.free_to_use.pop().unwrap_or(self.nodes.len());
    let actual_len = self.len;

    if index == 0 {
      // Add like head

      let node = Node {
        data: value,
        next: Self::map(self.head, insert_idex),
        prev: None,
        position_offset: 0,
      };

      self.head = Some(insert_idex);
      if self.tail == None {
        self.tail = Some(insert_idex);
      }

      if insert_idex == self.nodes.len() {
        self.nodes.push(node);
      } else {
        self.nodes[insert_idex] = node;
      }
    } else if index == actual_len {
      // Add like tail

      let node = Node {
        data: value,
        next: None,
        prev: Self::map(self.tail, insert_idex),
        // 0 because it's the last element
        position_offset: 0,
      };

      if let Some(tail) = self.tail {
        // Figure out why we needed to change from `index` to `insert_idex`
        self.nodes[tail].next = Some((insert_idex as isize) - (tail as isize));
      }

      self.tail = Some(insert_idex);
      if self.head == None {
        self.head = Some(insert_idex);
      }

      if insert_idex == self.nodes.len() {
        self.nodes.push(node);
      } else {
        self.nodes[insert_idex] = node;

        self.fix_position_offsets();
      }

      // We can return early
      // because position_offset will be 0
      // as it's the last element

      // And this is why we don't return early, lol
      self.len += 1;

      return;
    } else if index > actual_len {
      panic!("insertion index (is {index}) should be <= len (is {actual_len})");
    } else {
      // Add in the middle

      let current = self.get_actual_index(index - 1);
      let next = self.get_actual_index(index);

      let node = Node {
        data: value,
        next: Self::map(Some(next), insert_idex),
        prev: Self::map(Some(current), index),
        position_offset: 0,
      };

      self.nodes[current].next = Some((insert_idex as isize) - (current as isize));
      self.nodes[next].prev = Some((insert_idex as isize) - (next as isize));

      if insert_idex == self.nodes.len() {
        self.nodes.push(node);
      } else {
        self.nodes[insert_idex] = node;
      }
    }

    // [TODO]: Fix in future version, so it's not O(n), if possible
    if actual_len != 0 {
      self.fix_position_offsets();
    }

    self.len += 1;
  }

  #[allow(unused_doc_comments)]
  pub fn remove(&mut self, index: usize) -> T {
    /**
     * Sooooo
     * We need to keep the item in the list
     * Because of the position_offset
     *
     * But we (can) need remove the data, hmmmmmmmmmmmmmm
     *
     *
     * So it should probably be best to change T to Option<T> in Node
     * Cause we need to be able to TAKE the value out
     */
    let actual_index = self.get_actual_index(index);
    let (data, prev, next) = {
      let node = &mut self.nodes[actual_index];

      /**
       * So lets see how to take the value out lol
       */
      let data = core::mem::replace(&mut node.data, unsafe {
        MaybeUninit::<T>::uninit().assume_init()
      });

      (data, node.prev, node.next)
    };

    if let Some(prev) = prev {
      let prev_index = (actual_index as isize + prev) as usize;

      if index == self.len - 1 {
        self.tail = Some(prev_index);
      }

      self.nodes[prev_index].next =
        next.map(|next| (actual_index as isize + next) - prev_index as isize);
    } else if index == self.len - 1 {
      self.tail = None;
    }

    if let Some(next) = next {
      let next_index = (actual_index as isize + next) as usize;

      if index == 0 {
        self.head = Some(next_index);
      }

      self.nodes[next_index].prev =
        prev.map(|prev| (actual_index as isize + prev) - next_index as isize);
    } else if index == 0 {
      self.head = None;
    }

    self.fix_position_offsets();

    self.len -= 1;

    if self.len == 0 {
      self.free_to_use.clear();
      self.nodes.clear();
    } else {
      self.free_to_use.push(actual_index);
    }

    return data;
  }

  pub fn len(&self) -> usize {
    return self.len;
  }

  pub fn capacity(&self) -> usize {
    return self.nodes.capacity();
  }

  pub fn into_iter(self) -> ListIter<T> {
    return ListIter::new(self.nodes, self.len);
  }

  pub fn iter(&self) -> ListRefIter<T> {
    return ListRefIter::new(&self.nodes, self.len);
  }

  pub fn iter_mut(&mut self) -> ListMutRefIter<T> {
    return ListMutRefIter::new(&mut self.nodes, self.len);
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
    // WHY????
    // for node in &self.nodes {
    for val in self {
      if !is_first {
        f.write_str(", ")?;
      }
      write!(f, "{}", val)?;
      // write!(f, "{}", node.data.as_ref().expect("As"))?;

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
      pub fn new(nodes: $($maybe_ref)? $($lifetime)? $($only_mut)? Vec<Node<T>>, len: usize) -> $struct_name<T> {
        return {
          $struct_name {
            ptr: nodes.$ptr_fn(),
            len: len,
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
          // I FOUND OUT!!!
          // It stopped working because I changed the type from T to Option<T>
          // And the first part of an option, at least in the bytes, is if it's full, or empty

          let data = unsafe { node.offset((*node).position_offset) }.cast::<T>();

          self.current += 1;

          /*
           * Soooooooo
           * I need to find a way to point to the data within the Option
           * IDK how, lol
           */

          return Some($next_data_return(data));
        }
      }
    }

    impl<$($lifetime,)? T> IntoIterator for $($maybe_ref)? $($lifetime)? $($only_mut)? List<T> {
      type Item = $($maybe_ref)? $($lifetime)? $($only_mut)?  T;
      type IntoIter = $struct_name<$($lifetime,)? T>;

      fn into_iter(self) -> Self::IntoIter {
        let len = self.len;
        return $struct_name::new($($maybe_ref)? $($only_mut)? self.nodes, len);
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
