use std::fmt::Debug;
use std::rc::Rc;
use std::rc::Weak;
use std::cell::RefCell;

pub struct RBTree<K: std::cmp::PartialOrd> {
    root: RBNode<K>
}

pub type RBNode<K> = Option<Rc<RefCell<RBNodeInternal<K>>>>;
type WeakRBNode<K> = Option<Weak<RefCell<RBNodeInternal<K>>>>;

#[derive(Debug)]
pub struct RBNodeInternal<K: std::cmp::PartialOrd> {
    color: RBColor,
    key: K,
    right: RBNode<K>,
    left: RBNode<K>,
    p: WeakRBNode<K>
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum RBColor {
    Red,
    Black
}


/**
 RB-Insert(T,z)
    y = nil[T]
    x = root[T]
    while x != nil[T]
         y = x
         if key[z] < key[x] then
             x = left[x]
         else
             x = right[x]
    p[z] = y
    if y = nil[T]
         root[T] = z
    else
       if key[z] < key[y] then
          left[y] = z
       else
          right[y] = z
    left[z] = nil[T]
    right[z] = nil[T]
    color[z] = RED
    RB-Insert-fixup(T,z)
*/

fn clone_node<T: std::cmp::PartialOrd>(node: &RBNode<T>) -> RBNode<T> {
    node.as_ref().map(|rc| Rc::clone(&rc))
}

fn clone_weak_node<T: std::cmp::PartialOrd>(node: &WeakRBNode<T>) -> WeakRBNode<T> {
    node.as_ref().map(|rc| Weak::clone(&rc))
}

fn get_left<T: std::cmp::PartialOrd>(node: &RBNode<T>) -> RBNode<T> {
    match node.as_ref() {
        None => None,
        Some(val) => {
            clone_node(&val.borrow().left)
        }
    }
}

fn get_right<T: std::cmp::PartialOrd>(node: &RBNode<T>) -> RBNode<T> {
    match node.as_ref() {
        None => None,
        Some(val) => {
            clone_node(&val.borrow().right)
        }
    }
}

fn get_parent<T: std::cmp::PartialOrd>(node: &RBNode<T>) -> WeakRBNode<T> {
    match clone_node(node) {
        None => None,
        Some(val) => {
            clone_weak_node(&val.borrow_mut().p)
        }
    }
}

fn get_color<T: std::cmp::PartialOrd>(node: &RBNode<T>) -> RBColor {
    match node.as_ref() {
        None => RBColor::Black,
        Some(val) => {
            val.borrow().color
        }
    }
}

fn set_left<T: std::cmp::PartialOrd>(node: &RBNode<T>, left: RBNode<T>) {
    node.as_ref().map(|val| {
        val.borrow_mut().left = left;
    });
}

fn set_right<T: std::cmp::PartialOrd>(node: &RBNode<T>, right: RBNode<T>) {
    node.as_ref().map(|val| {
        val.borrow_mut().right = right;
    });
}

fn set_parent<T: std::cmp::PartialOrd>(node: &RBNode<T>, parent: WeakRBNode<T>) {
    node.as_ref().map(|val| {
        val.borrow_mut().p = parent;
    });
}

fn set_color<T: std::cmp::PartialOrd>(node: &RBNode<T>, color: RBColor) {
    node.as_ref().map(|val| {
        val.borrow_mut().color = color;
    });
}

fn to_weak<T: std::cmp::PartialOrd>(node: &RBNode<T>) -> WeakRBNode<T> {
    match node.as_ref() {
        None => None,
        Some(val) => {
            Some(Rc::downgrade(val))
        }
    }
}

fn to_strong<T: std::cmp::PartialOrd>(node: &WeakRBNode<T>) -> RBNode<T> {
    match node.as_ref() {
        None => None,
        Some(val) => {
            Some(Weak::upgrade(val).expect("INVALID STATE!"))
        }
    }
}    

impl<K: std::cmp::PartialOrd> PartialEq<RBNodeInternal<K>> for RBNodeInternal<K> {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key && self.left == other.left && self.right == other.right
    }
} 

#[derive(Copy, Clone, PartialEq)]
enum NodeChildType {
    LEFT,
    RIGHT
}

impl<K: std::cmp::PartialOrd + Debug> RBTree<K> {
    pub fn new() -> RBTree<K> {
        RBTree {root: None}
    }

    pub fn insert(&mut self, key: K) -> &mut Self {
        // println!("Inserting {:?}!", key);
        // self.print();
        if self.exists(&key) {return self;} /*  */
        let mut y: RBNode<K> = None;
        let mut x: RBNode<K> = clone_node(&self.root);
        while let Some(rc_node) = clone_node(&x) {
            y = x;
            if key < rc_node.borrow().key {
                x = clone_node(&rc_node.borrow().left);
            } else {
                x = clone_node(&rc_node.borrow().right);
            }
        }
        let z: RBNodeInternal<K> = RBNodeInternal {
            color: RBColor::Red,
            key: key,
            right: None,
            left: None,
            p: y.as_ref().map(|rc| Rc::downgrade(&rc))
        };
        let z_node;
        match y {
            None => {
                z_node = Some(Rc::new(RefCell::new(z)));
                self.root = clone_node(&z_node);
            }
            Some(rc) => {
                if z.key < rc.borrow().key {
                    z_node = Some(Rc::new(RefCell::new(z)));
                    rc.borrow_mut().left = clone_node(&z_node);
                } else {
                    z_node = Some(Rc::new(RefCell::new(z)));
                    rc.borrow_mut().right = clone_node(&z_node);
                }
            }
        }
        self.insert_fixup(z_node);
        return self;
    }

/*
RB-Insert-fixup(T,z)
  while color[p[z]] = RED {
    if p[z] == left[p[p[z]]] {
         y = right[p[p[z]]]
         if color[y] = RED {
             color[p[z]] = BLACK
             color[y] = BLACK
             color[p[p[z]]] = RED
             z = p[p[z]]
         }
         else {
             if z = right[p[z]] {
                  z = p[z]
                  LEFT-Rotate(T,z)
             }
             color[p[z]] = BLACK
             color[p[p[z]]] = RED
             RIGHT-Rotate(T,p[p[z]])
         }
    }
    else {
         y = left[p[p[z]]]
         if color[y] = RED {
             color[p[z]] = BLACK
             color[y] = BLACK
             color[p[p[z]]] = RED
             z = p[p[z]]
         }
         else
             {
             if z = left[p[z]] {
                  z = p[z]
                  RIGHT-Rotate(T,z)
             }
             color[p[z]] = BLACK
             color[p[p[z]]] = RED
             LEFT-Rotate(T,p[p[z]])
         }
    }
    color[root[T]] = BLACK
  }
*/

    fn insert_fixup(&mut self, mut z: RBNode<K>) {
        while get_color(&to_strong(&get_parent(&z))) == RBColor::Red {
            // println!("\nAttempted fixup for where k={:?}.", get_key(&z).unwrap());
            // self.print();
            if to_strong(&get_parent(&z)) == get_left(&to_strong(&get_parent(&to_strong(&get_parent(&z))))) {
                /* If the parent is to the left of the grandparent */
                /* y is z's uncle */
                let y = get_right(&to_strong(&get_parent(&to_strong(&get_parent(&z)))));
                if get_color(&y) == RBColor::Red {
                    set_color(&to_strong(&get_parent(&z)), RBColor::Black);
                    set_color(&y, RBColor::Black);
                    set_color(&to_strong(&get_parent(&to_strong(&get_parent(&z)))), RBColor::Red);
                    z = to_strong(&get_parent(&to_strong(&get_parent(&z))));
                } else {
                    if z == get_right(&to_strong(&get_parent(&z))) {
                        z = to_strong(&get_parent(&z));
                        self.left_rotate(&mut z);
                    }
                    set_color(&to_strong(&get_parent(&z)), RBColor::Black);
                    set_color(&to_strong(&get_parent(&to_strong(&get_parent(&z)))), RBColor::Red);
                    self.right_rotate(&mut to_strong(&get_parent(&to_strong(&get_parent(&z)))));
                }
            } else {
                /* If the parent is to the right of the grandparent */
                let y = get_left(&to_strong(&get_parent(&to_strong(&get_parent(&z)))));
                if get_color(&y) == RBColor::Red {
                    set_color(&to_strong(&get_parent(&z)), RBColor::Black);
                    set_color(&y, RBColor::Black);
                    set_color(&to_strong(&get_parent(&to_strong(&get_parent(&z)))), RBColor::Red);
                    z = to_strong(&get_parent(&to_strong(&get_parent(&z))));
                } else {
                    if z == get_left(&to_strong(&get_parent(&z))) {
                        z = to_strong(&get_parent(&z));
                        self.right_rotate(&mut z);
                    }
                    set_color(&to_strong(&get_parent(&z)), RBColor::Black);
                    set_color(&to_strong(&get_parent(&to_strong(&get_parent(&z)))), RBColor::Red);
                    self.left_rotate(&mut to_strong(&get_parent(&to_strong(&get_parent(&z)))));
                }
            }
        }
        set_color(&self.root, RBColor::Black);
    }

    fn exists(&self, key: &K) -> bool {
        return self.find(key).is_some();
    }

    fn find(&self, key: &K) -> RBNode<K> {
        let mut x = clone_node(&self.root);
        while x.is_some() {
            let rc = clone_node(&x).unwrap();
            let node_key = &rc.borrow().key;
            if key < node_key {
                x = clone_node(&rc.borrow().left);
            } else if key == node_key {
                return x;
            } else {
                x = clone_node(&rc.borrow().right);
            }
        }
        return None; // x must be None
    }

    pub fn remove(&mut self, key: &K) -> &mut Self {
        self.remove_node(&self.find(key))
    }

    
    pub fn remove_node(&mut self, z: &RBNode<K>) -> &mut Self {
        if z.is_none() {return self;}
        let mut y = clone_node(z);
        let mut y_original_color = get_color(&y);
        let mut x_parent: RBNode<K>;
        let mut x_parent_relationship: NodeChildType;
        let mut x: RBNode<K>;
        if get_left(z).is_none() {
            x = get_right(z);
            x_parent = clone_node(&to_strong(&get_parent(&z)));
            x_parent_relationship = if z == &get_left(&to_strong(&get_parent(&z))) {
                NodeChildType::LEFT
            } else {
                NodeChildType::RIGHT
            };
            self.transplant(z, &get_right(z));
        } else if get_right(z).is_none() {
            x = get_left(z);
            x_parent = clone_node(&to_strong(&get_parent(&z)));
            x_parent_relationship = if z == &get_left(&to_strong(&get_parent(&z))) {
                NodeChildType::LEFT
            } else {
                NodeChildType::RIGHT
            };
            self.transplant(z, &get_left(z));
        } else {
            y = Self::get_minimum(&get_right(z));
            y_original_color = get_color(&y);
            x = get_right(&y);
            x_parent = clone_node(&to_strong(&get_parent(&y)));
            if y != get_right(&z) { /* the minimum is farther down the tree. */
                self.transplant(&y, &get_right(&y));
                set_right(&y, get_right(&z));
                set_parent(&get_right(&y), to_weak(&y));
                x_parent_relationship = NodeChildType::LEFT;
            } else {
                set_parent(&x, to_weak(&y));
                x_parent = clone_node(&y);
                x_parent_relationship = NodeChildType::RIGHT;
            }
            self.transplant(&z, &y);
            set_left(&y, get_left(&z));
            set_parent(&get_left(&y), to_weak(&y));
            set_color(&y, get_color(&z));
        }
        if y_original_color == RBColor::Black {
            self.remove_fixup(x, x_parent, x_parent_relationship);
        }
        return self;
    }

    fn get_minimum(z: &RBNode<K>) -> RBNode<K> {
        let mut z_node = clone_node(&z);
        while get_left(&z_node).is_some() {
            z_node = get_left(&z_node);
        }
        return z_node;
    }

    fn remove_fixup(&mut self, mut x: RBNode<K>, mut x_parent: RBNode<K>, mut x_parent_relationship: NodeChildType) {
        while x != self.root && get_color(&x) == RBColor::Black {
            if x_parent_relationship == NodeChildType::LEFT {
            // if x == get_left(&to_strong(&get_parent(&x))) { 
                // let mut w = get_right(&to_strong(&get_parent(&x)));
                let mut w = get_right(&x_parent);
                if get_color(&w) == RBColor::Red {
                    set_color(&w, RBColor::Black);
                    set_color(&x_parent, RBColor::Red);
                    self.left_rotate(&mut x_parent); /* x_parent is still the parent of x. */
                    w = get_right(&x_parent);
                }
                if get_color(&get_left(&w)) == RBColor::Black && get_color(&get_right(&w)) == RBColor::Black {
                    set_color(&w, RBColor::Red);
                    x = x_parent;
                    x_parent = to_strong(&get_parent(&x));
                    x_parent_relationship = if x == get_left(&x_parent) {
                        NodeChildType::LEFT
                    } else {
                        NodeChildType::RIGHT
                    };
                } else {
                    if get_color(&get_right(&w)) == RBColor::Black {
                        set_color(&get_left(&w), RBColor::Black);
                        set_color(&w, RBColor::Red);
                        self.right_rotate(&mut w);
                        w = get_right(&x_parent);
                    }
                    set_color(&w, get_color(&x_parent));
                    set_color(&x_parent, RBColor::Black);
                    set_color(&get_right(&w), RBColor::Black);
                    self.left_rotate(&mut x_parent);
                    break;
                }
            } else {
                let mut w = get_left(&x_parent);
                if get_color(&w) == RBColor::Red {
                    set_color(&w, RBColor::Black);
                    set_color(&x_parent, RBColor::Red);
                    self.right_rotate(&mut x_parent);
                    w = get_left(&x_parent);
                }
                if get_color(&get_left(&w)) == RBColor::Black && get_color(&get_right(&w)) == RBColor::Black {
                    set_color(&w, RBColor::Red);
                    x = x_parent;
                    x_parent = to_strong(&get_parent(&x));
                    x_parent_relationship = if x == get_left(&x_parent) {
                        NodeChildType::LEFT
                    } else {
                        NodeChildType::RIGHT
                    };
                } else {
                    if get_color(&get_left(&w)) == RBColor::Black {
                        set_color(&get_right(&w), RBColor::Black);
                        set_color(&w, RBColor::Red);
                        self.left_rotate(&mut w);
                        w = get_left(&x_parent);
                    }
                    set_color(&w, get_color(&x_parent));
                    set_color(&x_parent, RBColor::Black);
                    set_color(&get_left(&w), RBColor::Black);
                    self.right_rotate(&mut x_parent);
                    break;
                }
            }
        }
        set_color(&x, RBColor::Black);
    }

    fn transplant(&mut self, u: &RBNode<K>, v: &RBNode<K>) {
        if get_parent(u).is_none() {
            self.root = clone_node(v);
        } else if u == &get_left(&to_strong(&get_parent(u))) {
            set_left(&to_strong(&get_parent(u)), clone_node(v));
        } else {
            set_right(&to_strong(&get_parent(u)), clone_node(v));
        }
        set_parent(&v, get_parent(u));
    }

/*
Left-Rotate(T,x)
    y = right[x]
    right[x] = left[y]
    p[left[y]] = x
    p[y] = p[x]


    if p[x] == nil[T] then root[T] = y
    else
       if x == left[p[x]] then left[p[x]] = y
       else
          right[p[x]] = y
    left[y] = x
    p[x] = y
*/
    fn left_rotate(&mut self, x: &mut RBNode<K>) {
        if x.is_none() {
            return;
        }
        let y = get_right(x);
        assert!(y.is_some());
        set_right(x, clone_node(&get_left(&y)));
        set_parent(&get_left(&y), to_weak(&x));
        set_parent(&get_left(&y), to_weak(&x));
        set_parent(&y, get_parent(&x));

        if get_parent(x).is_none() {
            self.root = clone_node(&y);
        } else {
            if x == &mut get_left(&to_strong(&get_parent(x))) {
                set_left(&to_strong(&get_parent(&x)), clone_node(&y));
            } else {
                set_right(&to_strong(&get_parent(&x)), clone_node(&y));
            }
        }
        set_left(&y, clone_node(&x));
        set_parent(&x, to_weak(&y));
    }

    fn right_rotate(&mut self, x: &mut RBNode<K>) {
        if x.is_none() {
            return;
        }
        let y = get_left(x);
        assert!(y.is_some());
        set_left(x, clone_node(&get_right(&y)));
        set_parent(&get_right(&y), to_weak(&x));
        set_parent(&get_right(&y), to_weak(&x));
        set_parent(&y, get_parent(&x));

        if get_parent(x).is_none() {
            self.root = clone_node(&y);
        } else {
            if x == &mut get_left(&to_strong(&get_parent(x))) {
                set_left(&to_strong(&get_parent(&x)), clone_node(&y));
            } else {
                set_right(&to_strong(&get_parent(&x)), clone_node(&y));
            }
        }
        set_right(&y, clone_node(&x));
        set_parent(&x, to_weak(&y));
    }

    fn is_rb_tree(&self) -> bool {
        if get_color(&self.root) == RBColor::Red {
            return false;
        }
        if !self.black_height_invariant_satisfied() {
            return false;
        }
        if !self.adjacent_red_invariant_satisfied(&self.root) {
            return false;
        }
        return true;
    }
    
    fn black_height_invariant_satisfied(&self) -> bool {
        return self.get_black_height(&self.root) != -1;
    }
    
    /* Returns: the black height of the node (inclusive) or -1 if the black height
     * invariant is broken */
    fn get_black_height(&self, node: &RBNode<K>) -> i32 {
        if node.is_none() {
            return 0;
        }
        let lft_height = self.get_black_height(&get_left(node));
        let rgt_height = self.get_black_height(&get_right(node));
        if lft_height == rgt_height {
            return lft_height + match get_color(node) {RBColor::Red => 0, RBColor::Black => 1};
        }
        println!("bad node:{:?}", node);
        return -1;
    }
    
    fn adjacent_red_invariant_satisfied(&self, node: &RBNode<K>) -> bool {
        return node.is_none() || match get_color(node) {
            RBColor::Black => true,
            RBColor::Red => get_color(&get_left(node)) == RBColor::Black && get_color(&get_right(node)) == RBColor::Black
        } && self.adjacent_red_invariant_satisfied(&get_left(node)) && self.adjacent_red_invariant_satisfied(&get_right(node));
    }
}

#[derive(Copy, Clone, PartialEq)]
enum NodeType {ROOT, LEFT, RIGHT}
impl<K: std::cmp::PartialOrd + Debug> RBTree<K> {
    fn print(&self) {
        let s = String::from("");
        Self::print_internal(&self.root, 0, NodeType::ROOT, s);
    }
    
    fn print_internal(node: &RBNode<K>, indent: u32, nt: NodeType, s: String) {
        if let Some(rf) = node {
            let left_node = &rf.borrow().left;
            let right_node = &rf.borrow().right;
            let mut lft_str = s.clone();
            let mut rgt_str = s.clone();
            if nt == NodeType::ROOT {
                lft_str.push_str("  ");
                rgt_str.push_str("  ");
            } else if nt == NodeType::LEFT {
                lft_str.push_str("  ");
                rgt_str.push_str("| ");
            } else {
                lft_str.push_str("| ");
                rgt_str.push_str("  ");
            }

            Self::print_internal(right_node, indent + 1, NodeType::RIGHT, rgt_str);

            print!("{}", s);
            if 0 < indent {
                if nt == NodeType::LEFT {
                    print!("└");
                } else {
                    print!("┌");
                }
            }
            println!("{}- {:?} {}", 
                match rf.borrow().color {
                    RBColor::Black => "", 
                    RBColor::Red => "\x1b[41m"
                }, 
                rf.borrow().key,
                // Rc::strong_count(rf),
                "\x1b[0m"
            );
            Self::print_internal(left_node, indent + 1, NodeType::LEFT, lft_str);
        }
    }
}

#[cfg(test)]
mod fuzzer {
    use super::*;
    use rand;
    use std::collections::HashSet;

    #[test]
    fn test_inserts() {
        for _ in 0..100 {
            let mut tree = RBTree::<i32>::new();
            for _ in 0..100 {
                tree.insert(rand::random::<i32>());
                assert!(tree.is_rb_tree());
            }
        }
    }

    #[test]
    fn test_inserts_and_delete() {
        for test_num in 1..101 {
            println!("Test #{} beginning.", test_num);
            let mut tree = RBTree::<usize>::new();
            let mut set = HashSet::<usize>::new();
            for _ in 0..5000 {
                let val = rand::random::<usize>();
                let v2 = rand::random::<usize>();
                match rand::random::<u32>() % 3 {
                    0 => { /* insert */
                        tree.insert(val);
                        set.insert(val);
                        assert!(tree.is_rb_tree(), "tree is not rb tree{}", {tree.print(); ""});
                    },
                    1 => { /* delete */
                        if v2 % 3 <= 0 {continue;}
                        let sz = set.len();
                        let mut i = 0;
                        let mut del: usize = 0;
                        for v in set.iter() {
                            if val % sz == i {
                                del = *v;
                                break;
                            }
                            i += 1;
                        }
                        tree.remove(&del);
                        set.remove(&del);
                        assert!(tree.is_rb_tree(), "tree is not rb tree{}", {tree.print(); ""});
                    },
                    2 => { /* exists */
                        for v in set.iter() {
                            assert_eq!(tree.exists(v), set.contains(v),
                            "assert failed for tree of size {}", {tree.print(); set.len()});
                        }
                    },
                    _ => unreachable!()
                }
            }
            // println!("Test ended with set.len() = {}", set.len());
        }
    }
}