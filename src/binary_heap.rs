// Reference code can be found at: https://takeuforward.org/heap/binary-heap-implementation/
use std::cmp::Ordering;

pub struct BinaryHeap {
    pub node_list: Vec<i8>,
    size: usize,
}

impl BinaryHeap {
    pub fn new() -> Self {
        BinaryHeap {
            node_list: Vec::new(),
            size: 0,
        }
    }

    pub fn display(&mut self) {
        let length = self.node_list.len() - 1;
        for (index, element) in self.node_list.iter().enumerate() {
            match index.cmp(&length) {
                Ordering::Less => {
                    print!("{element}, ")
                }
                Ordering::Equal => {
                    println!("{element}");
                }
                Ordering::Greater => {}
            }
        }
    }

    // Return parent node array index.
    fn parent(index: usize) -> usize {
        (index - 1) / 2
    }

    // Return left child node array index.
    fn left(index: usize) -> usize {
        2 * index + 1
    }

    // Return right child node array index.
    fn right(index: usize) -> usize {
        2 * index + 2
    }

    fn check_max_heap_property(&mut self) {
        let mut index = self.size - 1;
        while index != 0 && self.node_list[Self::parent(index)] < self.node_list[index] {
            self.node_list.swap(Self::parent(index), index);
            index = Self::parent(index);
        }
    }

    pub fn insert(&mut self, data: i8) {
        self.node_list.push(data); // Always insert in the end.
        self.size += 1;
        Self::check_max_heap_property(self);
    }

    pub fn max(&self) -> i8 {
        self.node_list[0]
    }

    // Assusme whole tree is disturbed.
    fn re_max_heapify(&mut self) {
        let mut max_index = 0;
        let left_index = Self::left(max_index);
        let right_index = Self::right(max_index);

        if left_index < self.size && self.node_list[left_index] > self.node_list[max_index] {
            max_index = left_index;
        }

        if right_index < self.size && self.node_list[right_index] > self.node_list[max_index] {
            max_index = right_index;
        }

        if max_index != 0 {
            self.node_list.swap(0, max_index);
            Self::re_max_heapify(self);
        }
        Self::check_max_heap_property(self);
    }

    pub fn del_max(&mut self) {
        self.node_list[0] = self.node_list[self.size - 1];
        self.node_list.remove(self.size - 1);
        self.size -= 1;
        Self::re_max_heapify(self);
    }
}
