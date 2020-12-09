#[cfg(test)]
mod tests {

    fn call_recursive(is: Vec<i32>) {
        return recursive(is.into_iter());
        fn recursive<I: Iterator<Item = i32> + Clone>(is: I) {
            let _ = is.clone().collect::<Vec<_>>();
            recursive(is.map(|i| i))
        }
    }

    #[test]
    fn build_hangs() {
        call_recursive(Vec::new());
    }
}
