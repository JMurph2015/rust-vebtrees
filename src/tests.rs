use VEBTree;

use std::time::{Instant, Duration};



#[test]
fn test_contains_correctness(){
    for j in 0..16 {
        let mut test_tree = VEBTree::new(16);
        for i in 0..j {
            test_tree.insert(i);
        }
        for i in 0..j {
            assert!(test_tree.contains(i));
        }
        for i in j..16 {
            assert!(!test_tree.contains(i));
        }
    }
}

#[test]
fn test_search_correctness(){
    for i in 0..16 {
        let mut test_tree = VEBTree::new(16);
        for j in 0..i {
            test_tree.insert(j);
        }
        for j in 0..i {
            println!("{:?}", test_tree.contains(j));
            assert_eq!(test_tree.search(j),Some(j));
        }
    }
}

#[test]
fn test_insert_and_delete_correctness(){
    let mut test_tree = VEBTree::new(16);
    let mut reference_tree = VEBTree::new(16);
    for i in 0..16 {
        test_tree.insert(i);
    }
    for i in 0..16 {
        test_tree.delete(i);
    }
    assert_eq!(test_tree, reference_tree);
}

#[test]
fn test_insert_and_delete_correctness_odd(){
    for i in 17..32 {
        println!("{}", i);
        let mut test_tree = VEBTree::new(i);
        let mut reference_tree = VEBTree::new(i);
        for j in 0..i {
            test_tree.insert(j);
        }
        for j in 0..i {
            test_tree.delete(j);
        }
        assert_eq!(test_tree, reference_tree);
    }
}
#[test]
fn test_minimum_correctness(){
    let mut test_tree = VEBTree::new(16);
    for i in (0..16).rev() {
        test_tree.insert(i);
        assert_eq!(test_tree.minimum(), Some(i));
    }
    for i in 0..15 {
        test_tree.delete(i);
        assert_eq!(test_tree.minimum(), Some(i+1));
    }
    for i in 0..16 {
        test_tree.insert(i);
    }
    for i in 0..16 {
        if i % 2 == 0 {
            test_tree.delete(i);
        } else {
            assert_eq!(test_tree.minimum(), Some(i));
            test_tree.delete(i);
        }
    }
}

#[test]
fn test_maximum_correctness(){
    let mut test_tree = VEBTree::new(16);
    for i in 0..16 {
        test_tree.insert(i);
        assert_eq!(test_tree.maximum(), Some(i));
    }
    for i in (0..16).rev() {
        assert_eq!(test_tree.maximum(), Some(i));
        test_tree.delete(i);
    }
}

#[test]
fn test_findnext_correctness(){
    let mut test_tree = VEBTree::new(16);
    for i in 0..16 {
        if i % 3 == 0 {
            test_tree.insert(i);
        }
    }
    for i in 0..13 {
        if i % 3 == 0 {
            assert_eq!(test_tree.findnext(i), Some(i+3));
        }
    }
}

#[test]
fn test_findprev_correctness(){
    let mut test_tree = VEBTree::new(16);
    for i in 0..16 {
        if i % 3 == 0 {
            test_tree.insert(i);
        }
    }
    for i in 3..16 {
        if i % 3 == 0 {
            assert_eq!(test_tree.findprev(i), Some(i-3));
        }
    }
}

