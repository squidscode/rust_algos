use std::fmt::Debug;
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

impl<K: std::cmp::PartialOrd + Copy> RBTree<K> {
    fn new() -> RBTree<K> {
        RBTree {root: None}
    }

    fn insert(&mut self, key: K) -> &mut Self {
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
        self.insert_fixup(&mut z_node);
        return self;
    }

    fn insert_fixup(&mut self, x: &mut StrongNodeRef<K>) {

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
        x.as_ref().map(|rc| {
            rc.borrow_mut().right = clone_node(&get_left(&y));
        });
        get_left(&y).as_mut().map(|rc| {
            rc.borrow_mut().p = to_weak(&x);
        });
        y.as_mut().map(|rc| {
            rc.borrow_mut().p = get_parent(&x);
        });

        if get_parent(x).is_none() {
            self.root = clone_node(&y);
        } else {
            if x == &mut get_left(&to_strong(&get_parent(x))) {
                to_strong(&get_parent(&x)).map(|rc| {
                    rc.borrow_mut().left = clone_node(&y);
                });
            } else {
                to_strong(&get_parent(&x)).map(|rc| {
                    rc.borrow_mut().right = clone_node(&y);
                });
            }
        }
        y.as_mut().map(|rc| {
            rc.borrow_mut().left = clone_node(&x);
        });
        x.as_mut().map(|rc| {
            rc.borrow_mut().p = to_weak(&y);
        });
    }

    fn right_rotate(&mut self, x: &mut StrongNodeRef<K>) {
        if x.is_none() {
            return;
        }
        let mut y = get_left(x);
        x.as_ref().map(|rc| {
            rc.borrow_mut().left = clone_node(&get_right(&y));
        });
        get_right(&y).as_mut().map(|rc| {
            rc.borrow_mut().p = to_weak(&x);
        });
        y.as_mut().map(|rc| {
            rc.borrow_mut().p = get_parent(&x);
        });

        if get_parent(x).is_none() {
            self.root = clone_node(&y);
        } else {
            if x == &mut get_left(&to_strong(&get_parent(x))) {
                to_strong(&get_parent(&x)).map(|rc| {
                    rc.borrow_mut().left = clone_node(&y);
                });
            } else {
                to_strong(&get_parent(&x)).map(|rc| {
                    rc.borrow_mut().right = clone_node(&y);
                });
            }
        }
        y.as_mut().map(|rc| {
            rc.borrow_mut().right = clone_node(&x);
        });
        x.as_mut().map(|rc| {
            rc.borrow_mut().p = to_weak(&y);
        });
    }
}

const INDENT: &str = "  ";
impl<K: std::cmp::PartialOrd + Copy + Debug> RBTree<K> {
    fn print(&self) {
        Self::print_internal(&self.root, 0, false);
    }

    fn print_internal(node: &StrongNodeRef<K>, indent: u32, left_branch: bool) {
        if let Some(rf) = node {
            let left_node = &rf.borrow().left;
            let right_node = &rf.borrow().right;

            let (le, re) = (left_node.is_some(), right_node.is_some());

            Self::print_internal(right_node, indent + 1, false);

            for _ in 0..(std::cmp::max(indent, 1)-1) {
                print!("{}", INDENT);
            }
            if 0 < indent {
                if left_branch {
                    print!("└ ");
                } else {
                    print!("┌ ");
                }
            }
            println!("{} {:?} ct:{}", 
                match rf.borrow().color {
                    RBColor::Black => "(Black)", 
                    RBColor::Red => "(Red)"
                }, 
                rf.borrow().key,
                Rc::strong_count(rf)
            );
            Self::print_internal(left_node, indent + 1, true);
        }
    }
}

impl<T: std::cmp::PartialOrd + Copy> Drop for RBTree<T> {
    fn drop(&mut self) {

    }
}

fn main() {
    let mut tree = RBTree::<i32>::new();
    tree.insert(12);
    tree.insert(10);
    tree.insert(11);
    tree.insert(20);
    tree.print();
    println!();
    let mut root = &mut clone_node(&tree.root);
    tree.left_rotate(&mut root);
    tree.print();
    println!();
    tree.right_rotate(&mut root);
    tree.print();
}
