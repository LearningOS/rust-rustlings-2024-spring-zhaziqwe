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

    pub fn add(&mut self, value: T) {
        if self.count == 0 {
            self.items[0] = value; // 替换默认值为新值
        } else {
            self.items.push(value);
            self.bubble_up(self.count);
        }
        self.count += 1;
    }
    

    fn bubble_up(&mut self, idx: usize) {
        if idx > 0 {
            let parent = (idx - 1) / 2;
            if (self.comparator)(&self.items[idx], &self.items[parent]) {
                self.items.swap(parent, idx);
                self.bubble_up(parent);
            }
        }
    }

    fn bubble_down(&mut self, idx: usize) {
        let left = 2 * idx + 1;
        let right = 2 * idx + 2;
        let mut min_idx = idx;

        // 找到当前节点，左子节点和右子节点中最小的那个
        if left < self.items.len() && (self.comparator)(&self.items[left], &self.items[min_idx]) {
            min_idx = left;
        }
        if right < self.items.len() && (self.comparator)(&self.items[right], &self.items[min_idx]) {
            min_idx = right;
        }

        // 如果当前节点不是最小的，那么它就需要下沉
        if min_idx != idx {
            self.items.swap(min_idx, idx);
            self.bubble_down(min_idx);
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
    T: Default + Copy + Display
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        let min = self.items[0];
        let last = self.items[self.count - 1];
        if !self.items.is_empty() {
            self.items[0] = last;
            self.bubble_down(0);
        }
        Some(min)
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
        println!("{:?}", heap.items);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        println!("{:?}", heap.items);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}