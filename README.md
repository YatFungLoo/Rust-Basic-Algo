# Rust-Basic-Algo
Simple algorithms implemented with Rust, as self-learning.

*Goal: a sorting a day until i finish chapter 2 of the book on sorting.*

All implemented are referencing from Algorithms 4th Edition by Robert Sedgewick, Kevin Wayne.

[![Build](https://github.com/YatFungLoo/Rust-Basic-Algo/actions/workflows/rust.yml/badge.svg)](https://github.com/YatFungLoo/Rust-Basic-Algo/actions/workflows/rust.yml)

<!-- markdown-toc start - Don't edit this section. Run M-x markdown-toc-refresh-toc -->
**Table of Contents**

- [Rust-Basic-Algo](#rust-basic-algo)
- [Elementary Sorts](#elementary-sorts)
  - [Selection Sort](#selection-sort)
  - [Insertion Sort](#insertion-sort)
  - [Shell Sort](#shell-sort)
- [Merge Sorts](#merge-sorts)
  - [Abstract In-Place Merge Sort](#abstract-in-place-merge-sort)
  - [Top-Down Merge Sort](#top-down-merge-sort)
  - [Bottom-Up Merge Sort](#bottom-up-merge-sort)
- [Quicksort](#quicksort)
  - [Basic](#basic)
- [How to run the code](#how-to-run-the-code)

<!-- markdown-toc end -->

# Elementary Sorts
Doesn't require extra space, only ever swapping in-place. Good for partially sorted array, the more partially sorted an array is the better.

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
Also a recursive algorithm similar to top-down approach. Rather than dividing the array over and over then merge sorting the sub-arrays, bottom-up approach merge sort sub-sections (usually smallest portion possible) of the array with multiple pass increasing the sub-section size (usually double the previous size, for in-place merge sort) until the whole array is sorted.

Method of choice when sorting *linked-list* data structure, where one can sort the list *in-place* by creating pointer to node instead of having a seperate auxiliary array.

> All sorting above are compare-based sorting algorithm.

# Quicksort

## Basic
A divide and conquer method. First partitioning an array to two subarrays, then sort each indepentently. In comparsion to merge sort, which require an additional sort of the final two sorted subarrays, quicksort rearrange the array such that when the two subarrays are sorted, the whole array are ordered.

> Partitioning is the key to quicksort.

# How to run the code

This project uses rustc 1.83.0 (90b35a623 2024-11-26) and was developed on Ubuntu 22.04. Make sure rustc is properly installed before continuing.

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
