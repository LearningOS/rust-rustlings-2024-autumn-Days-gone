/*
    heap
    This question requires you to implement a binary heap function
*/

use std::cmp::Ord;
use std::default::Default;
use std::fmt::Display;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn debug(&self, info: &str)
    where
        T: Display,
    {
        println!("---{}---", info);
        print!("[");
        for i in 1..=self.count {
            if i == self.count {
                print!("{}", self.items[i]);
            } else {
                print!("{},", self.items[i]);
            }
        }
        println!("]");
        println!("---END---\n");
    }

    pub fn add(&mut self, value: T)
    where
        T: Display,
    {
        //TODO // 4 2 9 11
        self.items.push(value);
        self.count += 1;

        self.up(self.count);
        self.debug("ADD");
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        //TODO
        idx
    }

    fn up(&mut self, mut idx: usize) {
        while idx > 0 && idx / 2 > 0 {
            let p_idx = idx / 2;
            let change = (self.comparator)(&self.items[idx], &self.items[p_idx]);
            if change {
                self.items.swap(idx, p_idx);
                idx = p_idx;
            } else {
                break;
            }
        }
    }

    fn down(&mut self, mut idx: usize) {
        while idx * 2 <= self.count {
            // check whether the right child exist
            let mut f_chd = 0;
            let  l_chd = idx * 2;
            let  r_chd = if idx * 2 < self.count { idx * 2 + 1 } else { 0 };
            if r_chd == 0 {
                f_chd = l_chd;
            } else {
                f_chd = if (self.comparator)(&self.items[l_chd], &self.items[r_chd]) {
                    l_chd
                } else {
                    r_chd
                };
                self.items.swap(idx, f_chd);
            }
            idx = f_chd;
        }
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default + Clone
{
    type Item = T;

    fn next(&mut self) -> Option<T>
    where
        T: Clone,
    {
        //TODO
        if self.count == 0 {
            return None;
        } else {
            let result = self.items[1].clone();
            self.items.swap(1, self.count);

            // delete the last
            self.count -= 1;
            self.items.pop();
            self.down(1);
            Some(result)
        }
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.debug("OUT");
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}
