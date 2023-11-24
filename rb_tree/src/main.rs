use std::rc::Rc;

struct RBTree<K> {
    root: Option<Rc<RBNode<K>>>
}

impl <K> RBTree<K> {
    fn new() -> RBTree<K> {
        RBTree {root: None}
    }

    fn insert(&mut self, key: K) -> &mut Self {
        match &mut self.root {
            Some(ptr) => {
                
            }
            None => {
                let node = RBNode {
                    color: RBColor::Black,
                    key: key,
                    right: None,
                    left: None,
                    p: None
                };
                self.root = Some(Rc::new(node));
            }
        }
        return self;
    }
}

struct RBNode<K> {
    color: RBColor,
    key: K,
    right: Option<Rc<RBNode<K>>>,
    left: Option<Rc<RBNode<K>>>,
    p: Option<Rc<RBNode<K>>>
}

enum RBColor {
    Red,
    Black
}

fn main() {
    let tree = RBTree::<i32>::new();
}
