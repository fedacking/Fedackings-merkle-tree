# Fedackings-merkle-tree

A rust implementation of a Merkle Tree data structure. The merkle tree is a binary tree where each of the nodes is a hash. The leafs refer to outside blocks of data, while the parents are hashes of the children nodes. This structure allows us to compare different blocks of data to not only see if it changed but in which section of the data it changed. Useful for checking and redownloading sections of files after downloading.

## Running

You can run the test program by using:

```
make run
```

Tests can be executed with:

```
make test
```
