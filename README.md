This is code to count the number of equivalence classes of a circle of n bits
where you can flip 3 consecutive 1s or 0s. The problem of counting these was
posed by `NuKuYul` on [mathoverflow][1]. Counting the equivalence classes is
done by considering it as a graph problem where the bitstrings are nodes,
flipping consecutive bits creates edges and equivalence classes are the
connected components of the graph.

There is a python implementation in `count.py` and a rust implementation under
`rust-version`.

## Output
```
  3 : 7
  4 : 8
  5 : 13
  6 : 27
  7 : 32
  8 : 51
  9 : 98
 10 : 130
 11 : 210
 12 : 374
 13 : 542
 14 : 872
 15 : 1505
 16 : 2268
 19 : 9552
 21 : 25880
 23 : 65146
 24 : 108588
 25 : 170266
 26 : 275296
 27 : 457119
 28 : 719864
... etc.
```

[1]: https://mathoverflow.net/questions/357250