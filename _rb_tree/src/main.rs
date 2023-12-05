use std::fmt::Debug;
use std::rc::Rc;
use std::rc::Weak;
use std::cell::RefCell;

pub struct RBTree<K: std::cmp::PartialOrd + Copy> {
    root: RBNode<K>
}

pub type RBNode<K> = Option<Rc<RefCell<RBNodeInternal<K>>>>;
type WeakRBNode<K> = Option<Weak<RefCell<RBNodeInternal<K>>>>;

pub struct RBNodeInternal<K: std::cmp::PartialOrd + Copy> {
    color: RBColor,
    key: K,
    right: RBNode<K>,
    left: RBNode<K>,
    p: WeakRBNode<K>
}

#[derive(Copy, Clone, PartialEq)]
enum RBColor {
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

fn clone_node<T: std::cmp::PartialOrd + Copy>(node: &RBNode<T>) -> RBNode<T> {
    node.as_ref().map(|rc| Rc::clone(&rc))
}

fn clone_weak_node<T: std::cmp::PartialOrd + Copy>(node: &WeakRBNode<T>) -> WeakRBNode<T> {
    node.as_ref().map(|rc| Weak::clone(&rc))
}

fn get_left<T: std::cmp::PartialOrd + Copy>(node: &RBNode<T>) -> RBNode<T> {
    match node.as_ref() {
        None => None,
        Some(val) => {
            clone_node(&val.borrow().left)
        }
    }
}

fn get_right<T: std::cmp::PartialOrd + Copy>(node: &RBNode<T>) -> RBNode<T> {
    match node.as_ref() {
        None => None,
        Some(val) => {
            clone_node(&val.borrow().right)
        }
    }
}

fn get_parent<T: std::cmp::PartialOrd + Copy>(node: &RBNode<T>) -> WeakRBNode<T> {
    match clone_node(node) {
        None => None,
        Some(val) => {
            clone_weak_node(&val.borrow_mut().p)
        }
    }
}

fn get_color<T: std::cmp::PartialOrd + Copy>(node: &RBNode<T>) -> RBColor {
    match node.as_ref() {
        None => RBColor::Black,
        Some(val) => {
            val.borrow().color
        }
    }
}

fn get_key<T: std::cmp::PartialOrd + Copy>(node: &RBNode<T>) -> Option<T> {
    match node.as_ref() {
        None => None,
        Some(val) => {
            Some(val.borrow().key)
        }
    }
}

fn set_left<T: std::cmp::PartialOrd + Copy>(node: &RBNode<T>, left: RBNode<T>) {
    node.as_ref().map(|val| {
        val.borrow_mut().left = left;
    });
}

fn set_right<T: std::cmp::PartialOrd + Copy>(node: &RBNode<T>, right: RBNode<T>) {
    node.as_ref().map(|val| {
        val.borrow_mut().right = right;
    });
}

fn set_parent<T: std::cmp::PartialOrd + Copy>(node: &RBNode<T>, parent: WeakRBNode<T>) {
    node.as_ref().map(|val| {
        val.borrow_mut().p = parent;
    });
}

fn set_color<T: std::cmp::PartialOrd + Copy>(node: &RBNode<T>, color: RBColor) {
    node.as_ref().map(|val| {
        val.borrow_mut().color = color;
    });
}

fn to_weak<T: std::cmp::PartialOrd + Copy>(node: &RBNode<T>) -> WeakRBNode<T> {
    match node.as_ref() {
        None => None,
        Some(val) => {
            Some(Rc::downgrade(val))
        }
    }
}

fn to_strong<T: std::cmp::PartialOrd + Copy>(node: &WeakRBNode<T>) -> RBNode<T> {
    match node.as_ref() {
        None => None,
        Some(val) => {
            Some(Weak::upgrade(val).expect("INVALID STATE!"))
        }
    }
}    

impl<K: std::cmp::PartialOrd + Copy> PartialEq<RBNodeInternal<K>> for RBNodeInternal<K> {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key && self.left == other.left && self.right == other.right
    }
} 

impl<K: std::cmp::PartialOrd + Copy + Debug> RBTree<K> {
    pub fn new() -> RBTree<K> {
        RBTree {root: None}
    }

    pub fn insert(&mut self, key: K) -> &mut Self {
        // println!("Inserting {:?}!", key);
        // self.print();
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
        let z_node = Some(Rc::new(RefCell::new(z)));
        match y {
            None => {
                self.root = clone_node(&z_node);
            }
            Some(rc) => {
                if key < rc.borrow().key {
                    rc.borrow_mut().left = clone_node(&z_node);
                } else {
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

    pub fn find(&self, key: K) -> RBNode<K> {
        let mut x = clone_node(&self.root);
        while x.is_some() {
            let node_key = &get_key(&x).expect("Key must exist!");
            if &key < node_key {
                x = get_left(&x);
            } else if &key == node_key {
                return x;
            } else {
                x = get_right(&x);
            }
        }
        return None; // x must be None
    }

    pub fn delete(&mut self, key: K) -> &mut Self {
        self.delete_node(&self.find(key))
    }

    pub fn delete_node(&mut self, z: &RBNode<K>) -> &mut Self {
        if z.is_none() {return self;}
        let mut y = clone_node(z);
        let mut y_original_color = get_color(&y);
        let mut x: RBNode<K>;
        let delete_node: RBNode<K>;
        if get_left(z).is_none() {
            x = get_right(z);
            delete_node = Self::create_phantom_leaf(&mut x, &y, false);
            self.transplant(z, &get_right(z));
        } else if get_right(z).is_none() {
            x = get_left(z);
            delete_node = Self::create_phantom_leaf(&mut x, &y, true);
            self.transplant(z, &get_left(z));
        } else {
            y = Self::get_minimum(&get_right(z));
            y_original_color = get_color(&y);
            x = get_right(&y);
            delete_node = Self::create_phantom_leaf(&mut x, &y, false);
            if y != get_right(&z) {
                self.transplant(&y, &get_right(&y));
                set_right(&y, get_right(&z));
                set_parent(&get_right(&y), to_weak(&y));
            } else {
                set_parent(&x, to_weak(&y));
            }
            self.transplant(&z, &y);
            set_left(&y, get_left(&z));
            set_parent(&get_left(&y), to_weak(&y));
            set_color(&y, get_color(&z));
        }
        if y_original_color == RBColor::Black {
            self.delete_fixup(clone_node(&x));
        }
        if delete_node.is_some() { self.transplant(&delete_node, &None); }
        return self;
    }

    fn get_minimum(z: &RBNode<K>) -> RBNode<K> {
        let mut z_node = clone_node(&z);
        while get_left(&z_node).is_some() {
            z_node = get_left(&z_node);
        }
        return z_node;
    }

    fn create_phantom_leaf(mut x: &mut RBNode<K>, parent: &RBNode<K>, left: bool) -> RBNode<K> {
        if x.is_none() {
            let z = Some(Rc::new(RefCell::new(RBNodeInternal { color: RBColor::Black, key: get_key(&parent).unwrap(), right: None, left: None, p: to_weak(&parent)})));
            if left {set_left(parent, clone_node(&z))} else {set_right(parent, clone_node(&z))};
            *x = clone_node(&z);
            return z;
        }
        return None;
    }

    fn delete_fixup(&mut self, mut x: RBNode<K>) {
        while x != self.root && x.is_some() && get_color(&x) == RBColor::Black {
            if x == get_left(&to_strong(&get_parent(&x))) {
                let mut w = get_right(&to_strong(&get_parent(&x)));
                if get_color(&w) == RBColor::Red {
                    set_color(&w, RBColor::Black);
                    set_color(&to_strong(&get_parent(&x)), RBColor::Red);
                    self.left_rotate(&mut to_strong(&get_parent(&x)));
                    w = get_right(&to_strong(&get_parent(&x)));
                }
                if get_color(&get_left(&w)) == RBColor::Black && get_color(&get_right(&w)) == RBColor::Black {
                    set_color(&w, RBColor::Red);
                    x = to_strong(&get_parent(&x));
                } else {
                    if get_color(&get_right(&w)) == RBColor::Black {
                        set_color(&get_left(&w), RBColor::Black);
                        set_color(&w, RBColor::Red);
                        self.right_rotate(&mut w);
                        w = get_right(&to_strong(&get_parent(&x)));
                    }
                    set_color(&w, get_color(&to_strong(&get_parent(&x))));
                    set_color(&to_strong(&get_parent(&x)), RBColor::Black);
                    set_color(&get_right(&x), RBColor::Black);
                    self.left_rotate(&mut to_strong(&get_parent(&x)));
                    x = clone_node(&self.root);
                }
            } else {
                let mut w = get_left(&to_strong(&get_parent(&x)));
                if get_color(&w) == RBColor::Red {
                    set_color(&w, RBColor::Black);
                    set_color(&to_strong(&get_parent(&x)), RBColor::Red);
                    self.right_rotate(&mut to_strong(&get_parent(&x)));
                    w = get_left(&to_strong(&get_parent(&x)));
                }
                if get_color(&get_left(&w)) == RBColor::Black && get_color(&get_right(&w)) == RBColor::Black {
                    set_color(&w, RBColor::Red);
                    x = to_strong(&get_parent(&x));
                } else {
                    if get_color(&get_left(&w)) == RBColor::Black {
                        set_color(&get_right(&w), RBColor::Black);
                        set_color(&w, RBColor::Red);
                        self.left_rotate(&mut w);
                        w = get_left(&to_strong(&get_parent(&x)));
                    }
                    set_color(&w, get_color(&to_strong(&get_parent(&x))));
                    set_color(&to_strong(&get_parent(&x)), RBColor::Black);
                    set_color(&get_left(&x), RBColor::Black);
                    self.right_rotate(&mut to_strong(&get_parent(&x)));
                    x = clone_node(&self.root);
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
impl<K: std::cmp::PartialOrd + Copy + Debug> RBTree<K> {
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

fn main() {
    let mut tree = RBTree::<i32>::new();
    let mut v: Vec<i32> = vec![];
    for i in 1..31 {
        tree.insert(i);
        assert!(tree.is_rb_tree());
    }
    tree.print();
    
    tree.delete_node(&tree.find(5));
    tree.print();

    tree.delete_node(&tree.find(16));
    tree.print();

    tree.delete_node(&tree.find(17));
    tree.print();

    tree.delete_node(&tree.find(2));
    tree.print();

    tree.delete_node(&tree.find(12));
    tree.print();

    println!();



}
