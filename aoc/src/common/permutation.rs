struct PermutationIndices {
    indices: Vec<usize>,
    swaps: Vec<usize>,
    i: usize,
}

impl PermutationIndices {
    fn new(size: usize) -> Self {
        Self {
            indices: (0..size).collect(),
            swaps: vec![0; size],
            i: 0,
        }
    }
}

/// Heap's algorithm
impl Iterator for PermutationIndices {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i > 0 {
            loop {
                if self.i >= self.swaps.len() {
                    return None;
                }

                if self.swaps[self.i] < self.i {
                    break;
                }

                self.swaps[self.i] = 0;
                self.i += 1;
            }

            self.indices.swap(self.i, (self.i & 1) * self.swaps[self.i]);
            self.swaps[self.i] += 1;
        }

        self.i = 1;
        Some(self.indices.clone())
    }
}

pub struct Permutations<T> {
    data: Vec<T>,
    indices: PermutationIndices,
}

impl<T> Iterator for Permutations<T>
where T: Clone
{
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(indices) = self.indices.next() {
            Some(
                indices
                    .into_iter()
                    .map(|i| self.data[i].clone())
                    .collect()
            )
        } else {
            None
        }
    }
}

pub trait PermutationsExt<T>
where T: Clone
{
    /// Creates iterator that generates all permutations of items in this collection.
    ///
    /// Uses Heap's algorithm.
    fn permutations(&self) -> Permutations<T>;
}

macro_rules! perms_ext_impl {
    ($ty:ty) => {
        impl<T> PermutationsExt<T> for $ty
        where T: Clone
        {
            fn permutations(&self) -> Permutations<T> {
                Permutations {
                    data: self.to_vec(),
                    indices: PermutationIndices::new(self.len()),
                }
            }
        }
    };

    ($ty:ty, $($tys:ty),+) => {
        perms_ext_impl!($ty);
        perms_ext_impl!($($tys),+);
    };
}

perms_ext_impl!(Vec<T>, &[T], [T]);

pub struct LexicographicPermutations<T> {
    data: Vec<T>,
    size: usize,
    first: bool,
}

impl<T> Iterator for LexicographicPermutations<T>
where T: Clone + PartialOrd
{
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.first {
            self.first = false;
        } else {
            if self.size < 2 {
                return None;
            }

            let mut i = self.size - 1;
            while i > 0 && self.data[i - 1] >= self.data[i] {
                i -= 1;
            }

            if i == 0 {
                return None;
            }

            let mut j = self.size - 1;
            while j >= i && self.data[j] <= self.data[i - 1] {
                j -= 1;
            }

            self.data.swap(j, i - 1);
            let _ = &self.data[i..].reverse();
        }

        Some(self.data.clone())
    }
}

pub trait LexicographicPermutationsExt<T>
where T: Clone + PartialOrd
{
    /// Creates iterator that generates lexicographic ordering of items in this collection.
    fn lexicographic_permutations(&self) -> LexicographicPermutations<T>;
}

macro_rules! lex_perms_ext_impl {
    ($ty:ty) => {
        impl<T> LexicographicPermutationsExt<T> for $ty
        where T: Clone + PartialOrd
        {
            fn lexicographic_permutations(&self) -> LexicographicPermutations<T> {
                LexicographicPermutations {
                    data: self.to_vec(),
                    size: self.len(),
                    first: true,
                }
            }
        }
    };

    ($ty:ty, $($tys:ty),+) => {
        lex_perms_ext_impl!($ty);
        lex_perms_ext_impl!($($tys),+);
    };
}

lex_perms_ext_impl!(Vec<T>, &[T], [T]);

#[cfg(test)]
mod tests {
    #[test]
    fn test_permutation_indices() {
        let mut perms = super::PermutationIndices::new(3);
        assert_eq!(perms.next(), Some(vec![0, 1, 2]));
        assert_eq!(perms.next(), Some(vec![1, 0, 2]));
        assert_eq!(perms.next(), Some(vec![2, 0, 1]));
        assert_eq!(perms.next(), Some(vec![0, 2, 1]));
        assert_eq!(perms.next(), Some(vec![1, 2, 0]));
        assert_eq!(perms.next(), Some(vec![2, 1, 0]));
        assert_eq!(perms.next(), None);
    }

    #[test]
    fn test_permutations() {
        use super::PermutationsExt as _;

        let data = &['a', 'b', 'c', 'd'][..];
        let mut perms = data.permutations();
        assert_eq!(perms.next(), Some(vec!['a', 'b', 'c', 'd']));
        assert_eq!(perms.next(), Some(vec!['b', 'a', 'c', 'd']));
        assert_eq!(perms.next(), Some(vec!['c', 'a', 'b', 'd']));
        assert_eq!(perms.next(), Some(vec!['a', 'c', 'b', 'd']));
        assert_eq!(perms.next(), Some(vec!['b', 'c', 'a', 'd']));
        assert_eq!(perms.next(), Some(vec!['c', 'b', 'a', 'd']));
        assert_eq!(perms.next(), Some(vec!['d', 'b', 'a', 'c']));
        assert_eq!(perms.next(), Some(vec!['b', 'd', 'a', 'c']));
        assert_eq!(perms.next(), Some(vec!['a', 'd', 'b', 'c']));
        assert_eq!(perms.next(), Some(vec!['d', 'a', 'b', 'c']));
        assert_eq!(perms.next(), Some(vec!['b', 'a', 'd', 'c']));
        assert_eq!(perms.next(), Some(vec!['a', 'b', 'd', 'c']));
        assert_eq!(perms.next(), Some(vec!['a', 'c', 'd', 'b']));
        assert_eq!(perms.next(), Some(vec!['c', 'a', 'd', 'b']));
        assert_eq!(perms.next(), Some(vec!['d', 'a', 'c', 'b']));
        assert_eq!(perms.next(), Some(vec!['a', 'd', 'c', 'b']));
        assert_eq!(perms.next(), Some(vec!['c', 'd', 'a', 'b']));
        assert_eq!(perms.next(), Some(vec!['d', 'c', 'a', 'b']));
        assert_eq!(perms.next(), Some(vec!['d', 'c', 'b', 'a']));
        assert_eq!(perms.next(), Some(vec!['c', 'd', 'b', 'a']));
        assert_eq!(perms.next(), Some(vec!['b', 'd', 'c', 'a']));
        assert_eq!(perms.next(), Some(vec!['d', 'b', 'c', 'a']));
        assert_eq!(perms.next(), Some(vec!['c', 'b', 'd', 'a']));
        assert_eq!(perms.next(), Some(vec!['b', 'c', 'd', 'a']));
        assert_eq!(perms.next(), None);
    }

    #[test]
    fn test_lexicographic_permutations() {
        use super::LexicographicPermutationsExt as _;

        let data = &[1, 2, 3][..];
        let mut perms = data.lexicographic_permutations();
        assert_eq!(perms.next(), Some(vec![1, 2, 3]));
        assert_eq!(perms.next(), Some(vec![1, 3, 2]));
        assert_eq!(perms.next(), Some(vec![2, 1, 3]));
        assert_eq!(perms.next(), Some(vec![2, 3, 1]));
        assert_eq!(perms.next(), Some(vec![3, 1, 2]));
        assert_eq!(perms.next(), Some(vec![3, 2, 1]));
        assert_eq!(perms.next(), None);
    }
}
