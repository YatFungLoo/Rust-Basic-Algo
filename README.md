# Rust-Basic-Algo
Simple algorithms implemented with Rust, as self-learning.

*Goal: a sorting a day until i finish chapter 2 of the book on sorting.* <- Done!

All implemented are referencing from Algorithms 4th Edition by Robert Sedgewick, Kevin Wayne.

[![Build](https://github.com/YatFungLoo/Rust-Basic-Algo/actions/workflows/rust.yml/badge.svg)](https://github.com/YatFungLoo/Rust-Basic-Algo/actions/workflows/rust.yml)

<!-- markdown-toc start - Don't edit this section. Run M-x markdown-toc-refresh-toc -->
**Table of Contents**

- [Rust-Basic-Algo](#rust-basic-algo)
- [Elementary Sorts](#elementary-sorts)
  - [Bubble Sort ($O(n^2)$)](#bubble-sort-on2)
  - [Selection Sort](#selection-sort)
  - [Insertion Sort](#insertion-sort)
  - [Shell Sort](#shell-sort)
- [Merge Sorts](#merge-sorts)
  - [Abstract In-Place Merge Sort](#abstract-in-place-merge-sort)
  - [Top-Down Merge Sort](#top-down-merge-sort)
  - [Bottom-Up Merge Sort](#bottom-up-merge-sort)
- [Quicksort](#quicksort)
  - [Basic](#basic)
  - [3-Way Partitioning Partitioning Quick Sort](#3-way-partitioning-partitioning-quick-sort)
- [Priority Queues](#priority-queues)
  - [Binary Heap Based Priority Queues](#binary-heap-based-priority-queues)
  - [Array Implementation](#array-implementation)
  - [More on Heap](#more-on-heap)
  - [Heap-sort and Sort-down](#heap-sort-and-sort-down)
- [How to run the code](#how-to-run-the-code)

<!-- markdown-toc end -->

# Elementary Sorts
Doesn't require extra space, only ever swapping in-place. Good for partially sorted array, the more partially sorted an array is the better.

## Bubble Sort ($O(n^2)$)
Simplest sorting algorithm, compare neighbour element one by one form start to end index, repeat process for array length of times or until there is no swap while compare neighbour elements as a slight optimisation.

## Selection Sort
Find smallest item in the array and exchange it to the first entry. Repeat to the next smallest item and exchange with the second entry (since it will be the second smallest). Repeat until the whole array is sorted.

Pros: Simple to create, doesn't use extra space. Cons: For completely sorted array, slow.

## Insertion Sort
Compare each entry starting from index 1, one to the left of itself, if the entry is greater than itself, swap entry. Otherwise move to the next entry. Repeat until the whole array is sorted.

## Shell Sort
A simple extent ion to insertion sort, instead of comparing one to the left of itself, compare the array entries of defined gap size (` h_gap `), repeat for the next entry until reaching the end of the array. Then by reducing ` h_gap ` on each iteration until the array is sorted.

` h_gap ` is chosen by the programmer, this implementation uses ` while h_gap < array.len()/3) h_gap = 3*h_gap + 1; ` which was provided by the book at section 2.3.

# Merge Sorts
Idea is to dividing the array into slices, sort each slices recursively and merge the results.

Pros: guarantees to sort array of N items in time proportional to (N log N). Cons: uses extra space proportional to N.

## Abstract In-Place Merge Sort
Divide the array into two sorted parts, compare entries of each half and copy the lesser value back to itself, using a auxiliary array to keep original value.

Pros: Good to know for understanding merge sort's extension. Cons: Essentially pointless on its own. 

## Top-Down Merge Sort
An extension to abstract in-place merge sort, that sort fully un-sorted array. A famously known example of "divide and conquer", recursively performing an abstract in-place merge sort while dividing the array to halves.

To further extend this method, we can 1) use insertion sort for small sub-arrays, 2) skip sub-arrays that are already in order, and 3) Recursive trickery to eliminate the copy to the auxiliary (TODO: add more info when I truly understand).

## Bottom-Up Merge Sort
Also a recursive algorithm similar to top-down approach. Rather than dividing the array over and over then merge sorting the sub-arrays, bottom-up approach merge sort sub-sections (usually smallest portion possible) of the array with multiple passes increasing the sub-section size (usually double the previous size, for in-place merge sort) until the whole array is sorted.

Method of choice when sorting *linked-list* data structure, where one can sort the list *in-place* by creating pointer to node instead of having a seperate auxiliary array.

> All sorting above are compare-based sorting algorithm.

# Quicksort
See follow.

## Basic
A divide and conquer method. First partitioning an array to two subarrays, then sort each indepentently. In comparsion to merge sort, which require an additional sort of the final two sorted subarrays, quicksort rearrange the array such that when the two subarrays are sorted, the whole array are ordered.

> Partitioning is the key to quicksort.

## 3-Way Partitioning Partitioning Quick Sort
An extension to basic quicksort, by tracking additional index that are equal to the value that we are tracking, in turns the left and right subarrays will be smaller given that there are large number of duplicate and increasing efficiency by lowering number of possible compares.

# Priority Queues
In comparison to queues (removing the oldest) and stacks (removing the newest), priority queue instead provides two major function 1) remove the maximum and 2) insert. Used when keys correspond to event times, to be processed in chronological order.

## Binary Heap Based Priority Queues
Items are kept in an array. Each key is guaranteed to be larger than or equal to the keys at two other specific positions, in turn, each of those keys must be larger than or equal to two additional keys.

Binary heap guarantee a O(1) access to minimum or maximum, depending on the binary tree type.

## Array Implementation
The binary heap tree structure is represented using an array. Unlike a linked list, there are no pointer information stored at each node, instead node parents and child are accessed by fortunately simple arithmetics.

Index goes up by each tree depth increase and from left to right. To calculate parent index, while it is not at root (index 0), it is done by `(index - 1) / 2`. To access left and right child index, it is done by `(2 * index) + 1` for left child node and `(2 * index) + 2` for right child node.

## More on Heap
If a node violate heap property, the tree will need to be reheapify. There are two main type of reheapifying methods, 1) Bottom-up (aka. swim) and 2) top-down (aka. sink).

Bottom-up (swim) traverse the tree and check for any node that are larger than (in case of maximum heap) that node's parent's key, if true then exchange the node with its parent, and repeat the process until the node reaches a larger key, or "swim" to the root.

Top-down (sink) also traverse the tree but instead checks if node's left or right child is smaller than (also in case of maximum heap), if true then exchange the node with its larger child, and repeat the process until the node is larger than its children, or it "sinks" to the bottom.

## Heap-sort and Sort-down
For any array, heapify it and sort-down the maximum or minimum value (depending on the type of tree) to produce a sorted array.

It shines at very large data sets, while similar to selection sort, it does not require to process all data like selection sort and work on a subset. Using less compares.

# How to run the code

This project uses rustc 1.83.0 (90b35a623 2024-11-26) and was developed on Ubuntu 22.04/ MacOS 15.2. Make sure rustc is properly installed before continuing.

No main is written for the code so feel free to create your own array and test out each function. Run

```
cargo doc --open
```

to see whats available. And run

```
cargo run
```

to actually run your code.

Although limited, test are written for each (sorting) function to ensure correct functionality. Run

```
cargo test
```

to see for yourself!
