struct CombinationIndices {
    c: Vec<usize>,
    n: usize,
    k: usize,
    first: bool,
}

impl CombinationIndices {
    fn new(n: usize, k: usize) -> Self {
        assert!(n >= k, "k ({}) cannot be greater than n ({})", k, n);
        assert!(k > 0, "k ({}) must not be 0", k);

        let mut c = vec![];
        for i in 0..k {
            c.push(i);
        }

        Self {
            c,
            n,
            k,
            first: true,
        }
    }
}

impl Iterator for CombinationIndices {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.first {
            self.first = false;
        } else {
            let Self {
                ref mut c,
                ref n,
                ref k,
                ..
            } = self;

            let mut i = k - 1;
            c[i] += 1;

            if c[i] > n - 1 {
                while c[i] >= n - k + i {
                    if i == 0 {
                        return None;
                    }

                    i -= 1;
                }

                c[i] += 1;
                while i < k - 1 {
                    c[i + 1] = c[i] + 1;
                    i += 1;
                }
            }
        }

        Some(self.c.clone())
    }
}

pub struct Combinations<T> {
    data: Vec<T>,
    indices: CombinationIndices,
}

impl<T> Iterator for Combinations<T>
where T: Clone
{
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.indices.next() {
            None => None,
            Some(indices) => Some(
                indices
                    .into_iter()
                    .map(|i| self.data[i].clone())
                    .collect()
            ),
        }
    }
}

pub trait CombinationsExt<T>
where T: Clone
{
    /// Creates iterator that generates all combinations of `k` items in this collection.
    fn combinations(&self, k: usize) -> Combinations<T>;
}

impl<I, T> CombinationsExt<T> for I
where
    I: Iterator<Item = T> + Clone + ?Sized,
    T: Clone,
{
    fn combinations(&self, k: usize) -> Combinations<T> {
        let data = self.clone().collect::<Vec<_>>();
        let n = data.len();

        Combinations {
            data,
            indices: CombinationIndices::new(n, k),
        }
    }
}

struct CombinationsWithRepetitionIndices {
    c: Vec<usize>,
    n: usize,
    k: usize,
    first: bool,
}

impl CombinationsWithRepetitionIndices {
    fn new(n: usize, k: usize) -> Self {
        assert!(n >= k, "k ({}) cannot be greater than n ({})", k, n);
        assert!(k > 0, "k ({}) must not be 0", k);

        Self {
            c: vec![0; k],
            n,
            k,
            first: true,
        }
    }
}

impl Iterator for CombinationsWithRepetitionIndices {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.first {
            self.first = false;
        } else {
            let Self {
                ref mut c,
                ref n,
                ref k,
                ..
            } = self;
            let mut i = k - 1;

            loop {
                if c[i] < n - 1 {
                    let level = c[i] + 1;
                    for j in i..*k {
                        c[j] = level;
                    }

                    break;
                }

                i = match i.overflowing_sub(1) {
                    (_, true) => return None,
                    (i, _) => i,
                };
            }
        }

        Some(self.c.clone())
    }
}

pub struct CombinationsWithRepetition<T> {
    data: Vec<T>,
    indices: CombinationsWithRepetitionIndices,
}

impl<T> Iterator for CombinationsWithRepetition<T>
where T: Clone
{
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.indices.next() {
            None => None,
            Some(indices) => Some(
                indices
                    .into_iter()
                    .map(|i| self.data[i].clone())
                    .collect()
            ),
        }
    }
}

pub trait CombinationsWithRepetitionExt<T>
where T: Clone
{
    /// Creates iterator that generates all combinations of `k` items in this
    /// collection, with repetition of items.
    fn combinations_with_repetition(&self, k: usize) -> CombinationsWithRepetition<T>;
}

impl<I, T> CombinationsWithRepetitionExt<T> for I
where
    I: Iterator<Item = T> + Clone + ?Sized,
    T: Clone,
{
    fn combinations_with_repetition(&self, k: usize) -> CombinationsWithRepetition<T> {
        let data = self.clone().collect::<Vec<_>>();
        let n = data.len();

        CombinationsWithRepetition {
            data,
            indices: CombinationsWithRepetitionIndices::new(n, k),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn combination_indices() {
        let mut iter = super::CombinationIndices::new(4, 2);
        assert_eq!(iter.next(), Some(vec![0, 1]));
        assert_eq!(iter.next(), Some(vec![0, 2]));
        assert_eq!(iter.next(), Some(vec![0, 3]));
        assert_eq!(iter.next(), Some(vec![1, 2]));
        assert_eq!(iter.next(), Some(vec![1, 3]));
        assert_eq!(iter.next(), Some(vec![2, 3]));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn combinations() {
        use super::CombinationsExt as _;

        let data = vec!['a', 'b', 'c', 'd'];
        let mut iter = data.into_iter().combinations(2);
        assert_eq!(iter.next(), Some(vec!['a', 'b']));
        assert_eq!(iter.next(), Some(vec!['a', 'c']));
        assert_eq!(iter.next(), Some(vec!['a', 'd']));
        assert_eq!(iter.next(), Some(vec!['b', 'c']));
        assert_eq!(iter.next(), Some(vec!['b', 'd']));
        assert_eq!(iter.next(), Some(vec!['c', 'd']));
        assert_eq!(iter.next(), None);

        let data = vec!['p', 'q', 'r', 's'];
        let mut iter = data.into_iter().combinations(3);
        assert_eq!(iter.next(), Some(vec!['p', 'q', 'r']));
        assert_eq!(iter.next(), Some(vec!['p', 'q', 's']));
        assert_eq!(iter.next(), Some(vec!['p', 'r', 's']));
        assert_eq!(iter.next(), Some(vec!['q', 'r', 's']));
        assert_eq!(iter.next(), None);

        let data = 1..=4;
        let mut iter = data.combinations(4);
        assert_eq!(iter.next(), Some(vec![1, 2, 3, 4]));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn combinations_with_repetition() {
        use super::CombinationsWithRepetitionExt as _;

        let data = vec!["iced", "jam", "plain"];
        let mut iter = data.into_iter().combinations_with_repetition(2);
        assert_eq!(iter.next(), Some(vec!["iced", "iced"]));
        assert_eq!(iter.next(), Some(vec!["iced", "jam"]));
        assert_eq!(iter.next(), Some(vec!["iced", "plain"]));
        assert_eq!(iter.next(), Some(vec!["jam", "jam"]));
        assert_eq!(iter.next(), Some(vec!["jam", "plain"]));
        assert_eq!(iter.next(), Some(vec!["plain", "plain"]));
        assert_eq!(iter.next(), None);
    }
}
