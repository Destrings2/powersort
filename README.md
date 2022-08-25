This is a [PowerSort](https://www.wild-inter.net/publications/munro-wild-2018) implementation in Rust.

# PowerSort
PowerSort is a run-adaptive mergesort, relating the problem of finding a good merge order to the problem of building optimal binary search trees. 
Specifically, it adapts an heuristic that build nearly optimal binary search trees. PowerSort is similar to TimSort: proceeding from left to right over the input
detecting already existing runs and deciding if a merge is useful.

TimSort uses a quite complex set of rules to decide when to merge, the rules themselves where empirically derived and require looking at the top three elements of
the run stack to maintain the invariants. PowerSort on the other hands only requires the calculationg of a single integer between runs which decides when
it is beneficial to merge. This integer is called the *power* of the run and related to the depth the node would have in the resulting merge tree following
the bisection heuristic.
