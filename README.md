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
- [How to run the code](#how-to-run-the-code)

<!-- markdown-toc end -->

# Elementary Sorts
Doesn't require extra space, only ever swaping inplace. Good for paritally sorted array, the more paritally sorted an array is the better.

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
