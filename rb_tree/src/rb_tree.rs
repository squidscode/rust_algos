use std::rc::Rc;
use std::rc::Weak;
use std::cell::RefCell;


pub struct RBTree<K: std::cmp::PartialOrd + Copy> {
    root: StrongNodeRef<K>
}

type StrongNodeRef<K> = Option<Rc<RBNode<K>>>;
type WeakNodeRef<K> = Option<Weak<RBNode<K>>>;

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
            
        });
    }

}

impl<T: std::cmp::PartialOrd + Copy> Drop for RBTree<T> {
    fn drop(&mut self) {

    }
}

fn main() {
    let mut tree = RBTree::<i32>::new();
    tree.insert(12);
}
