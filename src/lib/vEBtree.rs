use num::Integer;
use std::ops::Index;

type T = usize;

#[derive(Debug, PartialEq, Eq)]
pub struct vEBTree {
    children: Vec<vEBTree>,
    aux: Vec<vEBTree>,
    max: Option<T>,
    min: Option<T>

}

impl vEBTree {
    pub fn new(max_size: usize) -> Self {
        // Takes the square root of the max_size, then casts
        // it back to an usize integer 
        let self_size: usize;
        let pass_size: usize;
        if max_size <= 2 {
            self_size = 0;
            pass_size = 0;
        } else {
            let tmp = (max_size as f64).sqrt() as usize;
            if max_size % tmp == 0 {
                // max_size is a perfect square
                self_size = tmp;
            } else {
                self_size = tmp + 1;
            }
            pass_size = tmp;
        }
        let mut children_seed = Vec::with_capacity(self_size);
        let mut aux_seed = Vec::with_capacity(1);
        if pass_size > 0 {
            for index in 0..self_size {
                children_seed.push(Self::new(pass_size));
            }
            let mut aux = Self::new(pass_size);
            aux_seed.push(aux);
        }
        let mut tree = vEBTree {
            children: children_seed,
            aux: aux_seed,
            max: None,
            min: None
        };
        return tree
    }
}

/*
impl<'a, Q: ?Sized, T: Integer> Index<&'a Q> for vEBTree<T> {
    type Output = T;
    /*
     * index(&self, index: &Q)
     * if i in bounds:
     *     if i==0
     *         return self.min
     *     
     *
     *
     *
     *
    */
    fn index(&self, index: &Q) -> Option<&T> {
        let local_idx: Q = index / self.children.len();
        let pass_idx: Q = index % self.children.len();
        if self.children.len() == 0 {
            match index {
                1 => {
                    return self.max;
                }
                0 => {
                    return self.min;
                }
                _ => {
                    panic!("The bottom recursion didn't get a clean value!");
                }
            }
                Some(val) => Some(local_idx * val),
                None => None
            };
        } else if local_idx > self.children.len() {
            panic!("Index out of bounds error");
        }
    }
}
*/
impl vEBTree {
    pub fn search(&self, value: T) -> Option<T> {
        match self.min {
            Some(min_val) => {
                if min_val == value {
                    return self.min;
                } else {
                    match self.max {
                        Some(max_val) => {
                            if max_val == value {
                                return self.max;
                            } else {
                                if self.children.len() == 0 {
                                    return None;
                                } else {
                                    let local_idx = value / self.children.len();
                                    let pass_value = value % self.children.len();
                                    return self.children[local_idx].search(value);
                                }
                            }
                        },
                        None => {
                            return None;
                        }
                    };
                }
            },
            None => {
                return None;
            }
        };
    }
    fn insert_into_tree(&mut self, value: T) {
        if self.children.len() > 0 {
            let local_idx = value / self.children.len();
            let pass_value = value % self.children.len();
            match self.children[local_idx].minimum() {
                Some(min_value) => (),
                None => {
                    self.aux[0].insert(local_idx);
                }
            };
            self.children[local_idx].insert(pass_value);
        }
    }
    pub fn insert(&mut self, value: T){
        match self.min {
            Some(min_value) => {
                if value == min_value {
                    return;
                } else {
                    match self.max {
                        Some(max_value) => {
                            if max_value == value {
                                return;
                            } else {
                                if value < min_value {
                                    self.min = Some(value);
                                    self.insert_into_tree(value);
                                    return;
                                } else if value > max_value {
                                    self.max = Some(value);
                                    self.insert_into_tree(value);
                                    return;
                                } else {
                                    self.insert_into_tree(value);
                                    return;
                                }
                            }
                        },
                        None => {
                            self.max = Some(value);
                            self.insert_into_tree(value);
                            return;
                        }
                    };
                }
            },
            None => {
                self.min = Some(value);
                self.max = Some(value);
                self.insert_into_tree(value);
                return;
            }
        };

    }

    fn delete_from_tree(&mut self, value: T) {
        if self.children.len() > 0 {
            let local_idx = value / self.children.len();
            let pass_value = value % self.children.len();
            self.children[local_idx].delete(pass_value);
            if self.children[local_idx].minimum() == None {
                self.aux[0].delete(local_idx);
            }
        }
    }

    pub fn delete(&mut self, value: T){
        match self.min {
            Some(min_value) => {
                match self.max {
                    Some(max_value) => {
                        if max_value == min_value {
                            self.delete_from_tree(value);
                            self.min = None;
                            self.max = None;
                            return;
                        } else if self.children.len() == 0 {
                            if value == min_value {
                                self.min = Some(max_value);
                            } else {
                                self.max = Some(min_value);
                            }
                        } else if value == min_value {
                            self.delete_from_tree(value);
                            let first_populated = self.aux[0].minimum();
                            match first_populated {
                                Some(first_cluster) => {
                                    let new_min = self
                                        .children[first_cluster as usize]
                                        .minimum();
                                    match new_min {
                                        Some(min) => {
                                            self.min = Some((first_cluster
                                                            * self.children.len() as T
                                                             + min));
                                            return;
                                        },
                                        // Not sure how one gets here, probably
                                        // going to panic because structure corruption
                                        None => {
                                            panic!("Data structure appears corrupt");
                                        }
                                    };
                                },
                                None => {
                                    self.max = None;
                                    self.min = None;
                                }
                            };
                        } else if value == max_value {
                            self.delete_from_tree(value);
                            let last_populated = self.aux[0].maximum();
                            match last_populated {
                                Some(last_cluster) => {
                                    let new_max = self
                                        .children[last_cluster as usize]
                                        .maximum();
                                    match new_max {
                                        Some(max) => {
                                            self.max = Some((last_cluster
                                                             * self.children.len() as T
                                                             + max));
                                            return;
                                        },
                                        None => {
                                            panic!("Data structure appears corrupt");
                                        }
                                    };
                                },
                                None => {
                                    self.max = None;
                                    self.min = None;
                                }
                            };
                        } else if value < max_value && value > min_value {
                            self.delete_from_tree(value);
                            return;
                        } else {
                            return;
                        }

                    },
                    None => {
                        // This case shouldn't happen as there should always
                        // be a max when there is a min.
                        self.min = None;
                        self.max = None;
                        return;
                    }
                };
            },
            None => {
                self.min = None;
                self.max = None;
                return;
            }
        };
    }
    pub fn minimum(&self) -> Option<T> {
        return self.min;
    }
    pub fn maximum(&self) -> Option<T> {
        return self.max;
    }
    pub fn findnext(&self){
        
    }
    pub fn findprev(&self){
        
    }
}

#[cfg(test)]
mod tests{
    use lib::vEBtree::*;
    #[test]
    fn test_index_correctness(){
        
    }

    #[test]
    fn test_insert_and_delete_correctness(){
        let mut test_tree = vEBTree::new(16);
        let mut reference_tree = vEBTree::new(16);
        for i in 0..16 {
            test_tree.insert(i);
        }
        for i in 0..16 {
            test_tree.delete(i);
        }
        assert_eq!(test_tree, reference_tree);
    }

    #[test]
    fn test_minimum_correctness(){
        let mut test_tree = vEBTree::new(16);
        for i in (0..16).rev() {
            test_tree.insert(i);
            assert_eq!(test_tree.minimum(), Some(i));
        }
        for i in (0..15) {
            test_tree.delete(i);
            assert_eq!(test_tree.minimum(), Some(i+1));
        }
        for i in (0..16) {
            test_tree.insert(i);
        }
        for i in (0..16) {
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
        let mut test_tree = vEBTree::new(16);
        for i in (0..16) {
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
    }

    #[test]
    fn test_findprev_correctness(){
    }
}
