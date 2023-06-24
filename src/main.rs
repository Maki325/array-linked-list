#[derive(Debug)]
struct Node<T: std::fmt::Display + std::fmt::Debug> {
  data: T,
  next: Option<isize>,
  prev: Option<isize>,
  position_offset: isize,
}

#[derive(Debug)]
struct List<T: std::fmt::Display + std::fmt::Debug> {
  head: Option<usize>,
  tail: Option<usize>,
  nodes: Vec<Node<T>>,
}

impl<T: std::fmt::Display + std::fmt::Debug> List<T> {
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

  pub fn insert(&mut self, index: usize, value: T) {
    // self.nodes.insert(index, element);

    let len = self.nodes.len();

    if index == 0 {
      // Add like head
      let node = Node {
        data: value,
        next: Self::map(self.head, len),
        prev: None,

        // [TODO]
        position_offset: 0,
      };

      // println!("Node: {:?}", &node);

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

        // [TODO]
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
    } else if index > len {
      panic!("insertion index (is {index}) should be <= len (is {len})");
    } else {
      // Add in the middle

      let mut current = self.head;
      let mut next = None;
      let mut idx = 0;
      while let Some(current_index) = current {
        let node = &self.nodes[current_index];
        next = node
          .next
          .map(|node_next| ((current_index as isize) + node_next) as usize);

        if idx >= index - 1 {
          break;
        }
        idx += 1;
        current = next;
      }

      let node = Node {
        data: value,
        next: Self::map(next, len),
        prev: Self::map(current, index),

        // [TODO]
        position_offset: 0,
      };

      self.nodes[current.unwrap()].next = Some((len as isize) - (current.unwrap() as isize));
      self.nodes[next.unwrap()].prev = Some((len as isize) - (next.unwrap() as isize));

      self.nodes.push(node);

      // unimplemented!();
    }
    // let node = Node {
    //   data: value,
    //   next: None,
    //   prev: self.tail.map(|tail| (tail as isize) - (index as isize)),
    //   position_offset: 0,
    // };
    // self.nodes.push(node);
  }

  pub fn print(&self) {
    let mut current = self.head;
    while let Some(index) = current {
      let node = &self.nodes[index];
      print!("{} ", node.data);
      current = node.next.map(|next| ((index as isize) + next) as usize);
    }
    println!();
  }
}

struct ListIter<T: std::fmt::Display + std::fmt::Debug> {
  inner: std::vec::IntoIter<Node<T>>,
}

impl<T: std::fmt::Display + std::fmt::Debug> Iterator for ListIter<T> {
  type Item = T;

  fn next(&mut self) -> Option<Self::Item> {
    return self.inner.next().map(|node| node.data);
  }
}

impl<T: std::fmt::Display + std::fmt::Debug> IntoIterator for List<T> {
  type Item = T;
  type IntoIter = ListIter<T>;

  fn into_iter(self) -> Self::IntoIter {
    return ListIter {
      inner: self.nodes.into_iter(),
    };
  }
}

struct ListRefIter<'a, T: std::fmt::Display + std::fmt::Debug> {
  inner: std::slice::Iter<'a, Node<T>>,
}

impl<'a, T: std::fmt::Display + std::fmt::Debug> Iterator for ListRefIter<'a, T> {
  type Item = &'a T;

  fn next(&mut self) -> Option<Self::Item> {
    return self.inner.next().map(|node| &node.data);
  }
}

impl<'a, T: std::fmt::Display + std::fmt::Debug> IntoIterator for &'a List<T> {
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

  // println!("list: {:#?}", list);

  // // for value in &list {
  // //   print!("{} ", value);
  // // }
  list.print();
  // println!();

  list.insert(4, 2.5);

  // println!("list: {:#?}", list);

  // // for value in &list {
  // //   print!("{} ", value);
  // // }
  list.print();
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
