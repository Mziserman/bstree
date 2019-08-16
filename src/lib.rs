#[derive(Debug)]
pub struct Node {
  pub value: u32,
  pub left: Option<Box<Node>>,
  pub right: Option<Box<Node>>,
}

impl Node {
  pub fn new(value: u32, left: Option<Box<Node>>, right: Option<Box<Node>>) -> Node {
    Node { value, left, right }
  }

  pub fn leaf(value: u32) -> Node {
    Node {
      value,
      right: None,
      left: None,
    }
  }

  pub fn add(&mut self, value: u32) {
    if value >= self.value {
      match self.right {
        None => self.right = Some(Box::new(Self::leaf(value))),
        Some(ref mut node) => node.add(value),
      }
    } else {
      match self.left {
        None => self.left = Some(Box::new(Self::leaf(value))),
        Some(ref mut node) => node.add(value),
      }
    }
  }

  pub fn get_left(&self) -> Option<&Self> {
    if self.left.is_some() {
      Some(&(*(self.left.as_ref().unwrap())))
    } else {
      None
    }
  }

  pub fn get_right(&self) -> Option<&Self> {
    if self.right.is_some() {
      Some(&(*(self.right.as_ref().unwrap())))
    } else {
      None
    }
  }

  pub fn find(&mut self, value: u32) -> bool {
    if value > self.value {
      match self.right {
        None => false,
        Some(ref mut node) => node.find(value),
      }
    } else if value < self.value {
      match self.left {
        None => false,
        Some(ref mut node) => node.find(value),
      }
    } else {
      return true
    }
  }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find() {
      let mut root = Node::leaf(10);
      root.add(5);
      root.add(3);
      root.add(50);
      root.add(30);

      assert_eq!(true, root.find(50));
      assert_eq!(true, root.find(30));
      assert_eq!(true, root.find(3));
      assert_eq!(true, root.find(5));
      assert_eq!(true, root.find(10));
      assert_eq!(false, root.find(510));
      assert_eq!(false, root.find(310));
      assert_eq!(false, root.find(0));
      assert_eq!(false, root.find(1));
      assert_eq!(false, root.find(110));
    }
}
