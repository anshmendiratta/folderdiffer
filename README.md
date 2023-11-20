# diff_folders

# What?

A rust cli tool to "diff" files, that is, to find files that only exist in one of two given folders, and to dump them in a third folder.

# How?

To build the binary, compile the code with `cargo r` or `cargo rust`. Then, dig into `target/debug/` and find `diff_folders` -- this is your binary file. You can then copy that file anywhere you like (like in your $PATH).

## Usage

To now use the script, run it using `./difF_folders` and pass in three arguments: the first two folders you wish to diff, and the third folder you wish to dump all the diff'd files into.

A use case might look like `./diff_folders t1/ t2/ dump/`.

# What is `ZmodTwo`?

Similar to $\mathbb{Z}_2$ from math, meaning "all integers mod two" = ${0, 1}$, ZmodTwo is meant to be interpreted as the collection of all objects that occur only once between two vectors (stored in ZmodTwo). So, if we have `a = ZmodTwo { items: vec![1, 2] }` and `b = ZmodTwo { items: vec![2, 3] }`, then `c = a + b` (addition is defined in `lib.rs`) is also equal to `ZmodTwo { items: vec![1, 3] }` since `1` only occurs once in `a` and `3` only once in `b`, but `2` occurs twice, once in `a` and once in `b`. In this sense, `2` has been "modulo 2'd" $\equiv$ 0.

