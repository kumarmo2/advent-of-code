#[cfg(test)]
mod kmp {

    use crate::kmp;

    #[test]
    fn basic_test() {
        let mut lps: [usize; 100] = [0; 100];
        let occurences = kmp::find_occurences("aa", "a", &mut lps[..]);
        let Some(occurences) = occurences else {
            panic!("expected it to have value");
        };
        assert_eq!(2, occurences.indexes.len());
    }
}
