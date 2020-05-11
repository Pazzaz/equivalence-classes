This is code to count the number of equivalence classes of a circle of n bits
where you can flip 3 consecutive 1s or 0s. The problem of counting these was
posed by 'NuKuYul' on [mathoverflow][1]. Counting the equivalence classes is
done by considering it as a graph problem where the bitstrings are nodes,
flipping consecutive bits creates edges and equivalence classes are the
connected components of the graph.

There is a python implementation in 'count.py' and a rust implementation under
'rust-version'.

[1]: https://mathoverflow.net/questions/357250