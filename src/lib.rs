/* Library source code */

mod math {
    pub fn sum_all<T>(v: &Vec<T>) -> T {}

    pub fn multiply_all<T>(v: &Vec<T>) -> T {}
}

mod misc {
    pub fn in_vector<T>(t: &T, v: &Vec<T>) -> bool {}

    pub fn is_sorted<T>(v: &Vec<T>) -> bool {}
}

/* Tests source code */

#[cfg(test)]
mod tests {
    use super::{math, misc};

    #[test]
    fn test_in_vector_present() {
        assert_eq!(in_vector(&1, &vec![1, 2, 3]), true);
    }

    #[test]
    fn test_in_vector_not_present() {
        assert_eq!(in_vector(&1, &vec![2, 2]), false);
    }

    #[test]
    fn test_in_vector_empty() {
        assert_eq!(in_vector(&1, &vec![]), false);
    }

    #[test]
    fn test_is_sorted() {
        assert_eq!(is_sorted(&vec![1, 2, 3, 4, 5]), true);
    }

    #[test]
    fn test_is_not_sorted() {
        assert_ne!(is_sorted(&vec![5, 4]), true);
    }
}
