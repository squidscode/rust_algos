use std::fmt::Debug;
use std::process::exit;
use std::rc::Rc;
use std::rc::Weak;
use std::cell::RefCell;

pub struct RBTree<K: std::cmp::PartialOrd + Copy> {
    root: StrongNodeRef<K>
}

type StrongNodeRef<K> = Option<Rc<RefCell<RBNode<K>>>>;
type WeakNodeRef<K> = Option<Weak<RefCell<RBNode<K>>>>;

struct RBNode<K: std::cmp::PartialOrd + Copy> {
    color: RBColor,
    key: K,
    right: StrongNodeRef<K>,
    left: StrongNodeRef<K>,
    p: WeakNodeRef<K>
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

fn clone_node<T: std::cmp::PartialOrd + Copy>(node: &StrongNodeRef<T>) -> StrongNodeRef<T> {
    node.as_ref().map(|rc| Rc::clone(&rc))
}

fn clone_weak_node<T: std::cmp::PartialOrd + Copy>(node: &WeakNodeRef<T>) -> WeakNodeRef<T> {
    node.as_ref().map(|rc| Weak::clone(&rc))
}

fn get_left<T: std::cmp::PartialOrd + Copy>(node: &StrongNodeRef<T>) -> StrongNodeRef<T> {
    match node.as_ref() {
        None => None,
        Some(val) => {
            clone_node(&val.borrow().left)
        }
    }
}

fn get_right<T: std::cmp::PartialOrd + Copy>(node: &StrongNodeRef<T>) -> StrongNodeRef<T> {
    match node.as_ref() {
        None => None,
        Some(val) => {
            clone_node(&val.borrow().right)
        }
    }
}

fn get_parent<T: std::cmp::PartialOrd + Copy>(node: &StrongNodeRef<T>) -> WeakNodeRef<T> {
    match clone_node(node) {
        None => None,
        Some(val) => {
            clone_weak_node(&val.borrow_mut().p)
        }
    }
}

fn get_color<T: std::cmp::PartialOrd + Copy>(node: &StrongNodeRef<T>) -> RBColor {
    match node.as_ref() {
        None => RBColor::Black,
        Some(val) => {
            val.borrow().color
        }
    }
}

fn get_key<T: std::cmp::PartialOrd + Copy>(node: &StrongNodeRef<T>) -> Option<T> {
    match node.as_ref() {
        None => None,
        Some(val) => {
            Some(val.borrow().key)
        }
    }
}

fn set_left<T: std::cmp::PartialOrd + Copy>(node: &StrongNodeRef<T>, left: StrongNodeRef<T>) {
    node.as_ref().map(|val| {
        val.borrow_mut().left = left;
    });
}

fn set_right<T: std::cmp::PartialOrd + Copy>(node: &StrongNodeRef<T>, right: StrongNodeRef<T>) {
    node.as_ref().map(|val| {
        val.borrow_mut().right = right;
    });
}

fn set_parent<T: std::cmp::PartialOrd + Copy>(node: &StrongNodeRef<T>, parent: WeakNodeRef<T>) {
    node.as_ref().map(|val| {
        val.borrow_mut().p = parent;
    });
}

fn set_color<T: std::cmp::PartialOrd + Copy>(node: &StrongNodeRef<T>, color: RBColor) {
    node.as_ref().map(|val| {
        val.borrow_mut().color = color;
    });
}

fn to_weak<T: std::cmp::PartialOrd + Copy>(node: &StrongNodeRef<T>) -> WeakNodeRef<T> {
    match node.as_ref() {
        None => None,
        Some(val) => {
            Some(Rc::downgrade(val))
        }
    }
}

fn to_strong<T: std::cmp::PartialOrd + Copy>(node: &WeakNodeRef<T>) -> StrongNodeRef<T> {
    match node.as_ref() {
        None => None,
        Some(val) => {
            Some(Weak::upgrade(val).expect("INVALID STATE!"))
        }
    }
}    

impl<K: std::cmp::PartialOrd + Copy> PartialEq<RBNode<K>> for RBNode<K> {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key && self.left == other.left && self.right == other.right
    }
} 

impl<K: std::cmp::PartialOrd + Copy + Debug> RBTree<K> {
    fn new() -> RBTree<K> {
        RBTree {root: None}
    }

    fn insert(&mut self, key: K) -> &mut Self {
        // println!("Inserting {:?}!", key);
        // self.print();
        let mut y: StrongNodeRef<K> = None;
        let mut x: StrongNodeRef<K> = clone_node(&self.root);
        while let Some(rc_node) = clone_node(&x) {
            y = x;
            if key < rc_node.borrow().key {
                x = clone_node(&rc_node.borrow().left);
            } else {
                x = clone_node(&rc_node.borrow().right);
            }
        }
        let mut z: RBNode<K> = RBNode {
            color: RBColor::Red,
            key: key,
            right: None,
            left: None,
            p: y.as_ref().map(|rc| Rc::downgrade(&rc))
        };
        let mut z_node = Some(Rc::new(RefCell::new(z)));
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

    fn insert_fixup(&mut self, mut z: StrongNodeRef<K>) {
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
    fn left_rotate(&mut self, x: &mut StrongNodeRef<K>) {
        if x.is_none() {
            return;
        }
        let mut y = get_right(x);
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

    fn right_rotate(&mut self, x: &mut StrongNodeRef<K>) {
        if x.is_none() {
            return;
        }
        let mut y = get_left(x);
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
}

const INDENT: &str = "  ";
#[derive(Copy, Clone, PartialEq)]
enum node_type {ROOT, LEFT, RIGHT}
impl<K: std::cmp::PartialOrd + Copy + Debug> RBTree<K> {
    fn print(&self) {
        let s = String::from("");
        Self::print_internal(&self.root, 0, node_type::ROOT, s);
    }
    
    fn print_internal(node: &StrongNodeRef<K>, indent: u32, nt: node_type, s: String) {
        if let Some(rf) = node {
            let left_node = &rf.borrow().left;
            let right_node = &rf.borrow().right;
            let mut lft_str = s.clone();
            let mut rgt_str = s.clone();
            if nt == node_type::ROOT {
                lft_str.push_str("  ");
                rgt_str.push_str("  ");
            } else if nt == node_type::LEFT {
                lft_str.push_str("  ");
                rgt_str.push_str("| ");
            } else {
                lft_str.push_str("| ");
                rgt_str.push_str("  ");
            }

            Self::print_internal(right_node, indent + 1, node_type::RIGHT, rgt_str);

            print!("{}", s);
            if 0 < indent {
                if nt == node_type::LEFT {
                    print!("└ ");
                } else {
                    print!("┌ ");
                }
            }
            println!("{} {:?} ct:{}", 
                match rf.borrow().color {
                    RBColor::Black => "B", 
                    RBColor::Red => "R"
                }, 
                rf.borrow().key,
                Rc::strong_count(rf)
            );
            Self::print_internal(left_node, indent + 1, node_type::LEFT, lft_str);
        }
    }
}

impl<T: std::cmp::PartialOrd + Copy> Drop for RBTree<T> {
    fn drop(&mut self) {

    }
}

fn main() {
    let mut tree = RBTree::<i32>::new();
    for i in 1..100 {
        tree.insert(i);
    }
    tree.print();
    println!();



}
