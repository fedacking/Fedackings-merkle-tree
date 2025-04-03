use std::hash::{DefaultHasher, Hash, Hasher};

/* Internal struct explanations
We have a vector that stores all vectors with the corresponding hashes
level 0 is all of the hashes
*/
struct MerkleTree {
    hashes: Vec<Vec<u64>>,
    levels: usize,
    count: usize,
}

// Private hash function to not have to repeat myself with the
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
        1 << (self.levels - 1)
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
    //Inserts an element and returns the index where it has been inserted
    pub fn insert_element<T: Hash>(&mut self, element: T) -> usize {
        let mut index = self.count;
        self.count += 1;
        if self.count > self.max_count() {
            self.hashes.push(vec![0]);
            self.levels += 1;
        }
        let mut new_hash = hash(element, None);
        self.hashes[0].push(new_hash);
        for i in 1..self.levels {
            if index % 2 != 0 {
                new_hash = hash(self.hashes[i - 1][index - 1], Some(new_hash));
                index /= 2;
                self.hashes[i][index] = new_hash;
            } else {
                new_hash = hash(new_hash, Some(new_hash));
                index /= 2;
                self.hashes[i].push(new_hash);
            };
        }

        self.count - 1
    }

    // Returns the root hash of the tree
    pub fn root(&self) -> Option<&u64> {
        self.hashes[self.levels - 1].first()
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

    #[test]
    fn check_empty_tree_root() {
        let tree = MerkleTree::from_array([0_u64; 0]);

        assert_eq!(tree.root(), None);
    }

    #[test]
    fn check_add_to_empty_tree() {
        let mut tree = MerkleTree::from_array([0_u64; 0]);
        tree.insert_element(5_u64);
        let hash_5 = hash(5_u64, None);

        assert_eq!(*tree.root().unwrap(), hash_5);
    }

    #[test]
    fn check_add_changes_root() {
        let mut tree = MerkleTree::from_array([4_u64; 1]);
        let root = *tree.root().unwrap();
        tree.insert_element(5_u64);
        let root_hash = hash(hash(4_u64, None), Some(hash(5_u64, None)));

        assert_ne!(*tree.root().unwrap(), root);
        assert_eq!(*tree.root().unwrap(), root_hash);
    }

    #[test]
    fn check_add_changes_root_large() {
        let mut tree = MerkleTree::from_array([4_u64; 8]);
        let levels = tree.levels;
        let root = *tree.root().unwrap();
        tree.insert_element(5_u64);

        assert_ne!(*tree.root().unwrap(), root);
        assert_ne!(tree.levels, levels);
    }
}
