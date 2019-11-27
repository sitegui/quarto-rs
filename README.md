# quarto-rs
A Q-learning algorithm for Quarto implemented in Rust

# How to run

1. [Install rust](https://www.rust-lang.org/learn/get-started)
2. Run in release mode with: `cargo run --release`

# Current results

It goes out of memory after 27 milion episodes:

```
== Cycle 27/1000 ==
avg train score = 67.570656, avg eval score = 100, avg eval random score = 27.9
Eval random stats: Some(QLearningStats { total_actions: 5318, random_actions: 0, dummy_actions: 3647, learned_actions: 1671 })
Q-table size = 14154939, epsilon = 0.1
Stats = Some(QLearningStats { total_actions: 4408656, random_actions: 440880, dummy_actions: 435909, learned_actions: 3531867 })
depth | learned actions | avg. visits | num. states
    0 |             240 |    13500000 |           1
    1 |               5 |        5626 |           7
    1 |               6 |        5640 |           6
    1 |               7 |        5595 |           6
    1 |               8 |        5621 |           6
    1 |               9 |        5649 |           7
    1 |              10 |        5655 |          16
    1 |              11 |        5616 |          13
    1 |              12 |        5649 |          12
    1 |              13 |        5633 |           7
    1 |              14 |        5626 |          13
    1 |              15 |        5623 |          10
    1 |              16 |        5642 |           9
    1 |              17 |        5599 |          12
    1 |              18 |        5606 |           6
    1 |              19 |        5669 |           9
    1 |              20 |        5642 |           6
    1 |              21 |        5601 |           8
    1 |              22 |        5652 |           3
    1 |              23 |        5640 |           2
    1 |              24 |        5657 |           5
    1 |              25 |        5722 |           1
    1 |              26 |        5666 |           4
    1 |              27 |        5695 |           2
    1 |              28 |        5581 |           3
    1 |              29 |        5700 |           5
    1 |              30 |        5605 |           3
    1 |              31 |        5581 |           3
    1 |              32 |        5626 |           2
    1 |              33 |        5618 |           2
    1 |              34 |        5643 |           3
    1 |              35 |        5623 |           2
    1 |              36 |        5635 |           5
    1 |              37 |        5617 |           4
    1 |              41 |        5608 |           1
    1 |              42 |        5665 |           2
    1 |              43 |        5629 |           2
    1 |              44 |        5578 |           2
    1 |              45 |        5735 |           1
    1 |              47 |        5597 |           1
    1 |              51 |        5664 |           1
    1 |              59 |        5686 |           1
    1 |              69 |        5565 |           1
    1 |             209 |      455504 |           2
    1 |             210 |      474311 |          24
```