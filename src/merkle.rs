use std::hash::{DefaultHasher, Hash, Hasher};

/* Internal struct explanations
We have a vector that stores all hashes and all
*/
struct MerkleTree {
    hashes: Vec<Vec<u64>>,
    levels: usize,
    count: usize,
}

// Private hash function to not have to repeate myself with the
// defaulthasher boilerplater
fn hash<T: Hash>(left: T, right: Option<T>) -> u64 {
    let mut hasher = DefaultHasher::new();
    left.hash(&mut hasher);
    if let Some(val) = right {
        val.hash(&mut hasher)
    }
    hasher.finish()
}

impl MerkleTree {
    fn max_count(&self) -> usize {
        2_usize.pow(self.levels as u32)
    }

    // A Merkle Tree can be built out of an array.
    pub fn from_array<T: Hash, const count: usize>(array: [T; count]) -> Self {
        let mut levels: usize = 1;
        let mut hashes: Vec<Vec<u64>> = vec![Vec::new()];

        for element in array {
            hashes[0].push(hash(element, None));
        }

        while hashes[levels - 1].len() > 1 {
            hashes.push(Vec::new());

            let mut index = 0;

            while index < hashes[levels - 1].len() {
                let left = hashes[levels - 1][index];
                let right = match hashes[levels - 1].get(index + 1) {
                    Some(value) => *value,
                    None => left,
                };
                hashes[levels].push(hash(left, Some(right)));
                index += 2;
            }

            levels += 1;
        }

        MerkleTree {
            hashes,
            levels,
            count,
        }
    }

    // A Merkle Tree can be dynamic, this means that elements can be added once it is built.
    //Inserts an element and returns the hash and place where it has been inserted
    pub fn insert_element<T: Hash>(&self, element: T) -> (u64, usize) {
        todo!()
    }

    // Returns the root hash of the tree
    pub fn root(&self) -> u64 {
        return self.hashes[self.levels - 1][0];
    }

    // A Merkle Tree can generate a proof that it contains an element.
    // A proof consists of the other hashes that are required to compare with the
    // root of the tree
    pub fn generate_proof(&self, index: usize) -> Vec<u64> {
        todo!()
    }

    // A Merkle Tree can verify that a given hash is contained in it.
    pub fn verify_proof(&self, index: usize) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_proper_sizes() {
        let array2 = [10_u64; 2];
        let array4 = [10_u64; 4];
        let array6 = [10_u64; 6];
        let array8 = [10_u64; 8];

        assert_eq!(MerkleTree::from_array(array2).levels, 2);
        assert_eq!(MerkleTree::from_array(array4).levels, 3);
        assert_eq!(MerkleTree::from_array(array6).levels, 4);
        assert_eq!(MerkleTree::from_array(array8).levels, 4);
    }

    #[test]
    fn check_base_layer() {
        let array = [10_u64; 2];
        let tree = MerkleTree::from_array(array);

        let mut hasher = DefaultHasher::new();
        10_u64.hash(&mut hasher);
        let hash = hasher.finish();
        assert_eq!(tree.hashes[0][0], hash);
        assert_eq!(tree.hashes[0][1], hash);
    }

    #[test]
    fn check_merged_hash() {
        let array = [10_u64; 2];
        let tree = MerkleTree::from_array(array);

        let mut hasher = DefaultHasher::new();
        10_u64.hash(&mut hasher);
        let hash = hasher.finish();
        let mut hasher = DefaultHasher::new();
        hash.hash(&mut hasher);
        hash.hash(&mut hasher);
        let hash = hasher.finish();
        assert_eq!(tree.hashes[1][0], hash);
    }

    #[test]
    fn check_unbalanced_tree() {
        let array = [10_u64; 6];
        let tree = MerkleTree::from_array(array);

        assert_eq!(tree.hashes.len(), 4);
        assert_eq!(tree.hashes[0].len(), 6);
        assert_eq!(tree.hashes[1].len(), 3);
        assert_eq!(tree.hashes[2].len(), 2);
        assert_eq!(tree.hashes[3].len(), 1);
    }

    #[test]
    fn check_unbalanced_tree_hashes() {
        let array: [u64; 5] = [0, 1, 2, 3, 4];
        let tree = MerkleTree::from_array(array);

        let mut hasher = DefaultHasher::new();
        4_u64.hash(&mut hasher);
        let hash = hasher.finish();
        let mut hasher = DefaultHasher::new();
        hash.hash(&mut hasher);
        hash.hash(&mut hasher);
        let hash = hasher.finish();
        assert_eq!(tree.hashes[1][2], hash);
    }

    #[test]
    fn check_hash_ordering() {
        let array: [u64; 2] = [0, 1];
        let tree = MerkleTree::from_array(array);

        let hash_0 = hash(0_u64, None);
        let hash_1 = hash(1_u64, None);
        let hash_correct = hash(hash_0, Some(hash_1));
        let hash_wrong = hash(hash_1, Some(hash_0));

        assert_eq!(tree.hashes[1][0], hash_correct);
        assert_ne!(tree.hashes[1][0], hash_wrong);
    }
}
