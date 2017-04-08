pub fn analyze_ecb(bytes: &[u8], keysize: &usize) -> bool {
    let mut left = bytes.chunks(*keysize).collect::<Vec<&[u8]>>();
    left.sort();

    // Shift and zip with itself to compare
    // consecutive values
    let mut right = left.to_vec(); // cloning
    right.remove(0);

    left.iter()
        .zip(right.iter())
        .any(|(left, right)| {
            left.iter().eq(right.iter())
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analyze_ecb_positive() {
        assert!(analyze_ecb(&[1, 2, 3, 4, 3, 4, 5, 6], &(2_usize)));
    }

    #[test]
    fn test_analyze_ecb_negative() {
        assert!(!analyze_ecb(&[1, 2, 3, 4, 5, 6], &(2_usize)));
    }
}
