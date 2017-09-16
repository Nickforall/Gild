#[cfg(test)]
mod tests {
    use ValidationChain;
    use validators;

    #[test]
    fn max_size() {
        let cases = vec![
            ("a", 3, true),
            ("hello", 3, false)
        ];

        for (input, size, expect) in cases {
            let res = ValidationChain::new()
                .add(validators::MaxSize::new(size))
                .validate(String::from(input))
                .is_ok();

            assert_eq!(
                ValidationChain::new()
                    .add(validators::MaxSize::new(size))
                    .validate(String::from(input))
                    .is_ok(),
                expect
            )
        }
    }
}
