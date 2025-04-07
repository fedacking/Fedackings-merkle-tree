# Fedackings-merkle-tree

A rust implementation of a Merkle Tree data structure. The merkle tree is a binary tree where each of the nodes is a hash. The leafs refer to outside blocks of data, while the parents are hashes of the children nodes. This structure allows us to compare different blocks of data to not only see if it changed but in which section of the data it changed. Useful for checking and redownloading sections of files after downloading.

## MVP Features

Implemented features:

    A Merkle Tree can be built out of an array.
    A Merkle Tree can be dynamic, this means that elements can be added once it is built.
    A Merkle Tree can generate a proof that it contains an element.
    A Merkle Tree can verify that a given hash is contained in it.

These have been implemented with the following methods:

    MerkleTree::from_array(array: [T; count]) -> Self
    MerkleTree::insert_element(&mut self, element: T) -> usize
    MerkleTree::generate_proof(&self, index: usize) -> Vec<u64>
    MerkleTree::verify_proof(mut element_hash: u64, mut index: usize, proof: Vec<u64>, root: u64) -> bool

The hashes are represented as u64 numbers given out by our hash method, that takes up to two parameters. Every element inserted needs to implement the Hash trait for our hashing.

## Structure

The Merkle tree is stored in a vector of vectors. The hashes[0] is the vector of hashes of the base elements and each subsequent level is the hash of the elements below. In case of an unbalanced tree the nodes without brothers just hash with themselves to create a parent.

## Running

You can add the library to your dependencies like so in Cargo.toml:

```
[dependencies]
fedackings-merkle-tree = { git = "https://github.com/fedacking/Fedackings-merkle-tree.git", branch = "feature/make-library" }
```

Tests can be executed with:

```
make test
```
