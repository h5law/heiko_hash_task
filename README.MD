# Heiko Hash Task

Here at Heiko we are working a lot with hashes in Rust. We have come up with a
way to aggregate hashes together, treating them similar to integers and adding
them together. Any overflowing bits are discarded.

For that we have created a function called `aggregate_hashes` which takes an
array of hashes and adds them to each other one by one using the `append_hash`
function. This one uses 64 bit addition of parts of the hashes and tracks any
carry bits to be added to higher parts.

We have a suspicion that performance of this function could be improved if it
added hashes together slightly differently. Specifically, if a single part from
all hashes is added together, tracking any carry bits, and then the algorithm
moves on to the higher parts.

We want to investigate if there is indeed any merit to these suspicions. There
is already a sample benchmark for the `aggregate_hashes` function available. We
would like you to create a new function in the aggregator that sums hashes in
the proposed manner and create benchmarks to compare its performance to the old
version. Feel free to modify or add any code necessary, but try to keep the
current version of the `aggregate_hashes` function unchanged. Any additional
suggestions for where performance could be improved is welcome, as well as any
additional relevant tests.

### Command cheat sheet
Build project:
`cargo build`

Run tests:
`cargo test`

Run benchmarks:
`cargo bench`
