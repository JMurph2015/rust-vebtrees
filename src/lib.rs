#[cfg(test)]
mod tests;

#[cfg(test)]
mod test_asymptotes;

// Realized that this was the only useful data type to use as keys
// for the tree.  usize is the default pointer size for the system.


/// An implementation of Van Emde Boas Trees in Rust
///
/// # Fields
/// * children: Vec<VEBTree> - the child VEBTrees of this tree
/// * aux: Vec<VEBTree> - a single element Vec that holds to aux tree
///     uses a Vec wrapper to allocate it on the heap without hardcore
///     mutable reference nonsense.
/// * max: Option<T> - The maximum of the currently stored elements,
///     none if there are no stored elements, equal to min if there is
///     only one element
/// * min: Option<T> - The minimum of the currently stored elements,
///     none if there are no stored elements, equal to max if there is
///     only one element
#[derive(Clone,Debug, PartialEq, Eq)]
pub struct VEBTree<T> {
    children: Vec<VEBTree<T>>,
    aux: Vec<VEBTree<T>>,
    max: Option<T>,
    min: Option<T>

}

impl<T: Int> VEBTree<T> {
    /// Creates a new VEBTree with can contain every Element of Type T (0..MAX_VALUE).
    ///
    /// # Arguments
    /// * max_size: the maximum capacity with which to
    ///     initialize the tree
    ///
    /// # Returns
    /// * A tree initialized to the maximum capacity
    ///     specified
    pub fn new() -> Self {
        let max_size = T::MAX_VALUE;
        // Takes the square root of the max_size, then casts
        // it back to an usize integer 
        let self_size: usize;
        let pass_size: usize;
        if max_size <= T::TWO {
            self_size = 0;
            pass_size = 0;
        } else {
            let tmp = max_size.into_f64().sqrt().ceil() as usize;
            self_size = tmp;
            pass_size = tmp;
        }
        let mut children_seed: Vec<VEBTree<T>> = Vec::with_capacity(self_size);
        let mut aux_seed: Vec<VEBTree<T>> = Vec::with_capacity(1);
        if pass_size > 0 {
            for _ in 0..self_size {
                children_seed.push(Self::with_capacity(pass_size));
            }
            let aux = Self::with_capacity(pass_size);
            aux_seed.push(aux);
        }
        Self {
            children: children_seed,
            aux: aux_seed,
            max: None,
            min: None
        }
    }

    /// Creates a new VEBTree with given max capacity.
    ///
    /// # Arguments
    /// * max_size: the maximum capacity with which to
    ///     initialize the tree
    ///
    /// # Returns
    /// * A tree initialized to the maximum capacity
    ///     specified
    pub fn with_capacity(max_size: usize) -> Self {
        // Takes the square root of the max_size, then casts
        // it back to an usize integer 
        let self_size: usize;
        let pass_size: usize;
        if max_size <= 2 {
            self_size = 0;
            pass_size = 0;
        } else {
            let tmp = (max_size as f64).sqrt().ceil() as usize;
            self_size = tmp;
            pass_size = tmp;
        }
        let mut children_seed: Vec<VEBTree<T>> = Vec::with_capacity(self_size);
        let mut aux_seed: Vec<VEBTree<T>> = Vec::with_capacity(1);
        if pass_size > 0 {
            for _ in 0..self_size {
                children_seed.push(Self::with_capacity(pass_size));
            }
            let aux = Self::with_capacity(pass_size);
            aux_seed.push(aux);
        }
        Self {
            children: children_seed,
            aux: aux_seed,
            max: None,
            min: None
        }
    }
}

impl<T: Int> VEBTree<T> {
    /// Returns the quotient of the given number with respect to the
    ///     instance's number of children.
    /// # Arguments
    /// * self: the instance of the VEBTree
    /// * value: the value to divide by the number of
    ///     children
    ///
    /// # Returns
    /// * The quotient of the number w.r.t. self.children.len()
    fn high(&self, value: T) -> T {
        return value / T::from_usize(self.children.len());
    }

    /// Returns the modulus of the given number with respect to the
    ///     instance's number of children.
    /// # Arguments
    /// * self: &Self - the instance of the VEBTree
    /// * value: T==usize - the value to modulo by the number of
    ///     children
    ///
    /// # Returns
    /// * The modulus of the number w.r.t. self.children.len()
    fn low(&self, value: T) -> T {
        return value % T::from_usize(self.children.len());
    }

    /// Returns whether or not the given element is in the tree
    ///
    /// # Arguments
    /// * self: the instance of the VEBTree
    /// * value: the value for which to check membership
    ///
    /// # Returns
    /// * Whether or not the value is contained in the tree
    pub fn contains(&self, value: T) -> bool {
        match self.min {
            Some(min_val) => {
                if value == min_val {
                    return true;
                } else {
                    match self.max {
                        Some(max_val) => {
                            if value == max_val {
                                return true;
                            } else {
                                if self.children.len() > 0 {
                                    let tmp = self.high(value).into_usize();
                                    return self
                                        .children[tmp]
                                        .contains(self.low(value));
                                } else {
                                    return false;
                                }
                            }
                        },
                        None => {return false;}
                    }
                }
            },
            None => {return false;}
        }
    }

    /// Searches the tree for the given value and returns the value if
    ///     it is in the tree, None if not.
    ///
    /// # Arguments
    /// * self: the instance of the VEBTree to operate on
    /// * value: the value to search for in the tree
    ///
    /// # Returns
    /// * The value being searched for or None if the value
    ///     is not in the tree.
    pub fn search(&self, value: T) -> Option<T> {
        let min_val = self.min?;
        if value == min_val {
            return self.min;
        }
        let max_val = self.max?;
        if value == max_val {
            return self.max;
        }
        if self.children.len() == 0 {
            return None;
        } else {
            let local_idx = self.high(value).into_usize();
            let pass_value = self.low(value);
            self.children[local_idx].minimum()?;
            let result = self.children[local_idx].search(pass_value)?;
            return Some(result+self.high(value)*T::from_usize(self.children.len()));
        }
    }

    /// Convenience function to handle making the recursive insert calls
    ///     into the child trees.
    ///
    /// # Arguments
    /// * self: the instance of the VEBTree to operate on
    /// * value: the value to insert into the tree
    fn insert_into_tree(&mut self, value: T) {
        if self.children.len() > 0 {
            let local_idx = self.high(value);
            let index = local_idx.into_usize();
            let pass_value = self.low(value);
            match self.children[index].minimum() {
                Some(_) => (),
                None => {
                    self.aux[0].insert(local_idx);
                }
            };
            self.children[index].insert(pass_value);
        }
    }

    /// Insert a value into the array, does nothing if the value
    ///     is already present.
    ///
    /// # Arguments
    /// * self: the instance of the VEBTree to operate on
    /// * value: the value to insert into the tree
    pub fn insert(&mut self, value: T) {
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

    /// Convenience function to manage making recursive delete calls
    ///     into the VEBTree
    ///
    /// # Arguments
    /// * self: the instance of the VEBTree to operate on
    /// * value: the value to delete from the child trees
    fn delete_from_tree(&mut self, value: T) {
        if self.children.len() > 0 {
            let local_idx = self.high(value);
            let index = local_idx.into_usize();
            let pass_value = self.low(value);
            self.children[index].delete(pass_value);
            if self.children[index].minimum() == None {
                self.aux[0].delete(local_idx);
            }
        }
    }

    /// Deletes an element from the VEBTree
    ///
    /// # Arguments
    /// * self: the instance of the VEBTree to operate on
    /// * value: the value to delete from the tree
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
                                    let index = first_cluster.into_usize();
                                    let new_min = self
                                        .children[index]
                                        .minimum();
                                    match new_min {
                                        Some(min) => {
                                            self.min = Some(first_cluster
                                                            * T::from_usize(self.children.len())
                                                             + min);
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
                                    let index = last_cluster.into_usize();
                                    let new_max = self
                                        .children[index]
                                        .maximum();
                                    match new_max {
                                        Some(max) => {
                                            self.max = Some(last_cluster
                                                             * T::from_usize(self.children.len())
                                                             + max);
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

    /// Gets the minimum of the currently stored elements
    ///
    /// # Arguments
    /// * self: the instance of VEBTree to operate on
    ///
    /// # Returns
    /// * The minimum element currently stored in the tree
    pub fn minimum(&self) -> Option<T> {
        return self.min;
    }

    /// Gets the maximum of the currently stored elements
    ///
    /// # Arguments
    /// * self: &Self - the instance of VEBTree to operate on.
    ///
    /// # Returns
    /// * The maximum element currently stored in the tree
    pub fn maximum(&self) -> Option<T> {
        return self.max;
    }

    /// Finds the next consecutive element currently in the tree
    ///
    /// # Arguments
    /// * self: the instance of VEBTree to operate on.
    /// * value: the value to find the successor of.
    ///
    /// # Returns
    /// * The successor of 'value' or None if not found
    pub fn findnext(&self, value: T) -> Option<T> {
        if self.children.len() == 0 {
            let max_val = self.max?;
            if value == T::ZERO && max_val == T::ONE {
                return self.max;
            } else {
                return None;
            }
        } else {
            match self.min {
                Some(min_value) => {
                    if value < min_value {
                        return self.min
                    }
                },
                None => ()
            };
            let index = self.high(value).into_usize();
            let cur_cluster_max = self.children[index].maximum();
            match cur_cluster_max {
                Some(max_value) => {
                    if self.low(value) < max_value {
                        let offset = self.children[index]
                            .findnext(self.low(value))?;
                        return match self.children[index].search(offset) {
                            Some(n) => {
                                Some(n + self.high(value)*T::from_usize(self.children.len()))
                            },
                            None => {
                                None
                            }
                        };
                    }
                },
                None => ()
            };
            let next_cluster = self.aux[0].findnext(self.high(value))?;
            let index = next_cluster.into_usize();
            let offset = self.children[index].minimum()?;
            return match self.children[index].search(offset) {
                Some(n) => {
                    Some(n + next_cluster*T::from_usize(self.children.len()))
                },
                None => {
                    None
                }
            };
        }
    }

    /// Finds the immediate previous element currently in the array
    ///
    /// # Arguments
    /// * self: the instance of VEBTree to operate on
    /// * value: the value to find the predecessor of
    ///
    /// # Returns
    /// * The predecessor of 'value' or None if not found
    pub fn findprev(&self, value: T) -> Option<T> {
        if self.children.len() == 0 {
            let max_value = self.maximum()?;
            let min_value = self.minimum()?;
            if max_value == value && max_value != min_value {
                return self.min;
            } else {
                return None;
            }
        } else {
            match self.maximum() {
                Some(max_value) => {
                    if value > max_value {
                        return self.max;
                    }
                },
                None => ()
            };
            let index = self.high(value).into_usize();
            let cur_cluster_min = self.children[index].minimum();
            match cur_cluster_min {
                Some(min_value) => {
                    if self.low(value) > min_value {
                        let offset = self.children[index]
                            .findprev(self.low(value))?;
                        return match self.children[index].search(offset) {
                            Some(n) => {
                                Some(n + self.high(value)*T::from_usize(self.children.len()))
                            },
                            None => {
                                None
                            }
                        };
                    }
                },
                None => ()
            };
            let next_cluster = self.aux[0].findprev(self.high(value))?;
            let index = next_cluster.into_usize();
            let offset = self.children[index].maximum()?;
            return match self.children[index].search(offset) {
                Some(n) => {
                    Some(n + next_cluster*T::from_usize(self.children.len()))
                },
                None => {
                    None
                }
            };
        }
    }
}

pub trait Int: Copy + PartialEq + Eq + Ord + std::ops::Add<Self, Output=Self> + std::ops::Mul<Self, Output=Self> + 
    std::ops::Div<Self, Output=Self> + std::ops::Rem<Self, Output=Self>  {
    const MAX_VALUE: Self;
    const MIN_VALUE: Self;
    const ZERO: Self;
    const ONE: Self;
    const TWO: Self;

    fn into_usize(&self) -> usize;
    fn into_f64(&self) -> f64;
    fn from_usize(usize) -> Self;
}

impl Int for usize {
    const MAX_VALUE: Self = Self::max_value();
    const MIN_VALUE: Self = Self::min_value();

    const ZERO: Self = 0_usize;
    const ONE: Self = 1_usize;
    const TWO: Self = 2_usize;
    

    fn into_f64(&self) -> f64 {
        *self as f64
    }

    fn into_usize(&self) -> usize {
        *self
    }

    fn from_usize(val: usize) -> Self {
        val
    }
}

impl Int for u64 {
    const MAX_VALUE: Self = Self::max_value();
    const MIN_VALUE: Self = Self::min_value();

    const ZERO: Self = 0_u64;
    const ONE: Self = 1_u64;
    const TWO: Self = 2_u64;
    

    fn into_f64(&self) -> f64 {
        *self as f64
    }

    fn into_usize(&self) -> usize {
        *self as usize
    }

    // runs on 64bit systems
    fn from_usize(val: usize) -> Self {
        val as u64
    }
}




