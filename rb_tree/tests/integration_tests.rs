use rb_tree::RBTree;
use rand;

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_inserts() {
        let mut tree = RBTree::<i32>::new();
        for i in 0..100 {
            tree.insert(i);
        }
    }
}