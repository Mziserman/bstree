use binary_search_tree::Node;

fn main() {
  let rgt = Node::leaf(2);
  let lft = Node::leaf(3);
  let mut root = Node::new(1, Some(Box::new(rgt)), Some(Box::new(lft)));
  root.add(4);
  root.add(6);
  // root.get_left().unwrap().add(10);
  println!("{:?}", root)
}
