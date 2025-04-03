# Fedackings-merkle-tree

A rust implementation of a Merkle Tree data structure. The merkle tree is a binary tree where each of the nodes is a hash. The leafs refer to outside blocks of data, while the parents are hashes of the children nodes. This structure allows us to compare different blocks of data to not only see if it changed but in which section of the data it changed. Useful for checking and redownloading sections of files after downloading.

## MVP Features

Necesary features:

    A Merkle Tree can be built out of an array.
    A Merkle Tree can be dynamic, this means that elements can be added once it is built.
    A Merkle Tree can generate a proof that it contains an element.
    A Merkle Tree can verify that a given hash is contained in it.

## Structure

The Merkle tree is stored in a vector of vectos. The hashes[0] is the vector of hashes of the base elements and each subsequent level is the hash of the elements below. In case of an unbalanced tree the nodes without brothers just hash with themselves to create a parent.

## Running

You can run the test program by using:

```
make run
```

Tests can be executed with:

```
make test
```
