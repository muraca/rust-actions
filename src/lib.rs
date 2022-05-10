/* Library source code */

mod math {
    pub fn sum_all<T>(v: &Vec<T>) -> T {
        panic!("Method not implemented");
    }

    pub fn multiply_all<T>(v: &Vec<T>) -> T {
        panic!("Method not implemented");
    }
}

mod misc {
    pub fn in_vector<T>(t: &T, v: &Vec<T>) -> bool {
        panic!("Method not implemented");
    }

    pub fn is_sorted<T>(v: &Vec<T>) -> bool {
        panic!("Method not implemented");
    }
}

/* Tests source code */

#[cfg(test)]
mod tests {
    use super::misc;

    #[test]
    fn test_in_vector_present() {
        assert!(misc::in_vector(&1, &vec![1, 2, 3]));
    }

    #[test]
    fn test_in_vector_not_present() {
        assert_eq!(misc::in_vector(&1, &vec![2, 2]), false);
    }

    #[test]
    fn test_in_vector_empty() {
        assert_ne!(misc::in_vector(&1, &vec![]), true);
    }

    #[test]
    fn test_is_sorted() {
        assert!(misc::is_sorted(&vec![1, 2, 3, 4, 5]));
    }

    #[test]
    #[should_panic(expected = "Array is not sorted!")]
    fn test_is_not_sorted() {
        assert!(misc::is_sorted(&vec![5, 4, 6]), "Array is not sorted!");
    }

    #[test]
    #[should_panic(expected = "Vec is empty!")]
    fn test_is_sorted_empty() {
        let mut v = vec![0];
        v.pop();
        misc::is_sorted(&v);
    }
}
