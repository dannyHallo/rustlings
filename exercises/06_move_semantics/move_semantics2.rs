fn fill_vec(vec: &[i32]) -> Vec<i32> {
    let mut res = vec.to_vec();
    res.push(88);
    res
}

fn main() {
    // You can optionally experiment here.
    
    let test_vec = vec![22, 44, 66];
    let vec1 = fill_vec(&test_vec);
    println!("{:?}", vec1);
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: Make both vectors `vec0` and `vec1` accessible at the same time to
    // fix the compiler error in the test.
    #[test]
    fn move_semantics2() {
        let vec0 = vec![22, 44, 66];

        let vec1 = fill_vec(&vec0[..]);

        assert_eq!(vec0, [22, 44, 66]);
        assert_eq!(vec1, [22, 44, 66, 88]);
    }
}
