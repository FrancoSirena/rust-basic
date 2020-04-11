#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub struct Node {
  right: Option<Box<Node>>,
  left: Option<Box<Node>>,
  pub value: u32,
}

impl Node {
  /// Returns a new Node
  ///
  /// ### Arguments
  /// * `value`: u32 - node value
  fn new(value: u32) -> Node {
    return Node {
      right: None,
      left: None,
      value: value,
    };
  }

  /// DFS to look for a value in a balanced tree.
  ///
  /// Returns true/false
  /// ### Arguments
  /// * `value`: u32 - to look for
  fn find(&self, value: u32) -> bool {
    if self.value == value {
      return true;
    }
    if self.right.is_none() && self.left.is_none() {
      return false;
    }
    let to_find = if value > self.value {
      &self.right
    } else {
      &self.left
    };
    match to_find {
      &Some(ref sub) => return sub.find(value),
      &None => {
        return false;
      }
    }
  }

  /// Add a new value from the given Node
  ///
  /// ### Arguments
  /// * `value`: u32 - to be added
  pub fn add(&mut self, value: u32) {
    if self.value == value {
      return;
    }
    // If value is lesser than current, we return the MUTABLE reference of LEFT
    // Else we return the MUTABLE reference of RIGHT
    let target = if value < self.value {
      &mut self.left
    } else {
      &mut self.right
    };

    // Checks if target is empty or not
    // We need to use the `&mut` because we don't want to steal the values reference
    match target {
      // If we do have a value already, we need to try to add from that node
      &mut Some(ref mut sub) => sub.add(value),
      // If we don't have a value we add the value right here
      &mut None => {
        let new_n = Box::new(Node::new(value));
        // We take the raw pointer of target to assign the new value
        // We know who target is because of our closure, so it is safe to do so
        *target = Some(new_n);
      }
    }
  }
}

#[derive(Debug)]
pub struct Tree {
  root: Node,
}

impl Tree {
  /// Returns a new Tree instance with the root Node
  ///
  /// ### Arguments
  /// * `value`: u32 - root value
  pub fn new(value: u32) -> Tree {
    return Tree {
      root: Node::new(value),
    };
  }
  /// From root adds a value in the tree
  ///
  /// ### Arguments
  /// * `value`: u32 - to be added
  pub fn add(&mut self, value: u32) {
    self.root.add(value)
  }

  /// Looks for a value in the tree, returns true or false
  /// ### Arguments
  /// * `value`: u32 - to find
  pub fn find(&self, value: u32) -> bool {
    return self.root.find(value);
  }
}
