use array_linked_list::List;

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

#[test]
fn remove_multiple() {
  let mut list = List::<usize>::new();

  list.push(25);
  list.push(75);

  assert_eq!(list[0], 25);
  assert_eq!(list[1], 75);

  list.remove(1);

  assert_eq!(list.len(), 1);
  assert_eq!(list[0], 25);

  list.remove(0);

  assert_eq!(list.len(), 0);
}

#[test]
fn remove_all_from_start() {
  let mut list = List::<usize>::new();

  list.push(25);
  list.push(50);
  list.push(75);
  list.push(100);

  assert_eq!(list.len(), 4);
  assert_eq!(list[0], 25);
  assert_eq!(list[1], 50);
  assert_eq!(list[2], 75);
  assert_eq!(list[3], 100);

  list.remove(0);

  assert_eq!(list.len(), 3);
  assert_eq!(list[0], 50);
  assert_eq!(list[1], 75);
  assert_eq!(list[2], 100);

  list.remove(0);

  assert_eq!(list.len(), 2);
  assert_eq!(list[0], 75);
  assert_eq!(list[1], 100);

  list.remove(0);

  assert_eq!(list.len(), 1);
  assert_eq!(list[0], 100);

  list.remove(0);

  assert_eq!(list.len(), 0);
}

#[test]
fn remove_all_from_end() {
  let mut list = List::<usize>::new();

  list.push(25);
  list.push(50);
  list.push(75);
  list.push(100);

  assert_eq!(list.len(), 4);
  assert_eq!(list[0], 25);
  assert_eq!(list[1], 50);
  assert_eq!(list[2], 75);
  assert_eq!(list[3], 100);

  list.remove(3);

  assert_eq!(list.len(), 3);
  assert_eq!(list[0], 25);
  assert_eq!(list[1], 50);
  assert_eq!(list[2], 75);

  list.remove(2);

  assert_eq!(list.len(), 2);
  assert_eq!(list[0], 25);
  assert_eq!(list[1], 50);

  list.remove(1);

  assert_eq!(list.len(), 1);
  assert_eq!(list[0], 25);

  list.remove(0);

  assert_eq!(list.len(), 0);
}

#[test]
fn remove_all_middle_till_end() {
  let mut list = List::<usize>::new();

  list.push(0);
  list.push(25);
  list.push(50);
  list.push(75);
  list.push(100);

  assert_eq!(list.len(), 5);
  assert_eq!(list[0], 0);
  assert_eq!(list[1], 25);
  assert_eq!(list[2], 50);
  assert_eq!(list[3], 75);
  assert_eq!(list[4], 100);

  list.remove(1);

  assert_eq!(list.len(), 4);
  assert_eq!(list[0], 0);
  assert_eq!(list[1], 50);
  assert_eq!(list[2], 75);
  assert_eq!(list[3], 100);

  list.remove(1);

  assert_eq!(list.len(), 3);
  assert_eq!(list[0], 0);
  assert_eq!(list[1], 75);
  assert_eq!(list[2], 100);

  list.remove(1);

  assert_eq!(list.len(), 2);
  assert_eq!(list[0], 0);
  assert_eq!(list[1], 100);

  list.remove(1);

  assert_eq!(list.len(), 1);
  assert_eq!(list[0], 0);
}

#[test]
fn remove_all_middle_till_start() {
  let mut list = List::<usize>::new();

  list.push(0);
  list.push(25);
  list.push(50);
  list.push(75);
  list.push(100);

  assert_eq!(list.len(), 5);
  assert_eq!(list[0], 0);
  assert_eq!(list[1], 25);
  assert_eq!(list[2], 50);
  assert_eq!(list[3], 75);
  assert_eq!(list[4], 100);

  list.remove(3);

  assert_eq!(list.len(), 4);
  assert_eq!(list[0], 0);
  assert_eq!(list[1], 25);
  assert_eq!(list[2], 50);
  assert_eq!(list[3], 100);

  list.remove(2);

  assert_eq!(list.len(), 3);
  assert_eq!(list[0], 0);
  assert_eq!(list[1], 25);
  assert_eq!(list[2], 100);

  list.remove(1);

  assert_eq!(list.len(), 2);
  assert_eq!(list[0], 0);
  assert_eq!(list[1], 100);

  list.remove(0);

  assert_eq!(list.len(), 1);
  assert_eq!(list[0], 100);
}

#[test]
fn push_after_remove() {
  let mut list = List::<usize>::new();

  list.push(25);
  list.push(50);
  list.push(75);

  assert_eq!(list[0], 25);
  assert_eq!(list[1], 50);
  assert_eq!(list[2], 75);

  list.remove(1);

  assert_eq!(list.len(), 2);
  assert_eq!(list[0], 25);
  assert_eq!(list[1], 75);

  list.push(50);

  assert_eq!(list.len(), 3);
  assert_eq!(list[0], 25);
  assert_eq!(list[1], 75);
  assert_eq!(list[2], 50);

  list.push(75);

  assert_eq!(list.len(), 4);
  assert_eq!(list[0], 25);
  assert_eq!(list[1], 75);
  assert_eq!(list[2], 50);
  assert_eq!(list[3], 75);

  list.remove(0);

  assert_eq!(list.len(), 3);
  assert_eq!(list[0], 75);
  assert_eq!(list[1], 50);
  assert_eq!(list[2], 75);
}

#[test]
fn insert_after_remove() {
  let mut list = List::<usize>::new();

  list.push(25);
  list.push(50);
  list.push(75);

  assert_eq!(list[0], 25);
  assert_eq!(list[1], 50);
  assert_eq!(list[2], 75);

  list.remove(1);

  assert_eq!(list.len(), 2);
  assert_eq!(list[0], 25);
  assert_eq!(list[1], 75);

  list.insert(1, 50);

  assert_eq!(list.len(), 3);
  assert_eq!(list[0], 25);
  assert_eq!(list[1], 50);
  assert_eq!(list[2], 75);
}

#[test]
fn insert_after_remove_more() {
  let mut list = List::<isize>::new();

  list.push(0);
  list.push(25);
  list.push(50);
  list.push(75);
  list.push(100);

  assert_eq!(list.len(), 5);
  assert_eq!(list[0], 0);
  assert_eq!(list[1], 25);
  assert_eq!(list[2], 50);
  assert_eq!(list[3], 75);
  assert_eq!(list[4], 100);

  // Let's delete the middle 3

  list.remove(3);

  assert_eq!(list.len(), 4);
  assert_eq!(list[0], 0);
  assert_eq!(list[1], 25);
  assert_eq!(list[2], 50);
  assert_eq!(list[3], 100);

  list.remove(2);

  assert_eq!(list.len(), 3);
  assert_eq!(list[0], 0);
  assert_eq!(list[1], 25);
  assert_eq!(list[2], 100);

  list.remove(1);

  assert_eq!(list.len(), 2);
  assert_eq!(list[0], 0);
  assert_eq!(list[1], 100);

  // Onto inserting

  list.insert(0, -20);

  assert_eq!(list.len(), 3);
  assert_eq!(list[0], -20);
  assert_eq!(list[1], 0);
  assert_eq!(list[2], 100);

  list.insert(3, 120);

  assert_eq!(list.len(), 4);
  assert_eq!(list[0], -20);
  assert_eq!(list[1], 0);
  assert_eq!(list[2], 100);
  assert_eq!(list[3], 120);

  list.insert(2, 50);

  assert_eq!(list.len(), 5);
  assert_eq!(list[0], -20);
  assert_eq!(list[1], 0);
  assert_eq!(list[2], 50);
  assert_eq!(list[3], 100);
  assert_eq!(list[4], 120);

  list.insert(2, 25);

  assert_eq!(list.len(), 6);
  assert_eq!(list[0], -20);
  assert_eq!(list[1], 0);
  assert_eq!(list[2], 25);
  assert_eq!(list[3], 50);
  assert_eq!(list[4], 100);
  assert_eq!(list[5], 120);
}
