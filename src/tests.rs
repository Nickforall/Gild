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
            assert_eq!(
                ValidationChain::new()
                    .add(validators::MaxSize::new(size))
                    .validate(String::from(input))
                    .is_ok(),
                expect
            )
        }
    }

    #[test]
    fn min_size() {
        let cases = vec![
            ("a", 3, false),
            ("hello", 3, true)
        ];

        for (input, size, expect) in cases {
            assert_eq!(
                ValidationChain::new()
                    .add(validators::MinSize::new(size))
                    .validate(String::from(input))
                    .is_ok(),
                expect
            )
        }
    }

    #[test]
    fn empty() {
        let cases = vec![
            ("", true, true),
            ("", false, false),
            ("hello", false, true)
        ];

        for (input, should, expect) in cases {
            assert_eq!(
                ValidationChain::new()
                    .add(validators::Empty::new(should))
                    .validate(String::from(input))
                    .is_ok(),
                expect
            )
        }
    }

    #[test]
    fn contain() {
        let cases = vec![
            ("a", "hello", false),
            ("hello", "hello", true)
        ];

        for (input, contain, expect) in cases {
            assert_eq!(
                ValidationChain::new()
                    .add(validators::Contain::new(String::from(contain)))
                    .validate(String::from(input))
                    .is_ok(),
                expect
            )
        }
    }

    #[test]
    fn not_contain() {
        let cases = vec![
            ("a", "hello", true),
            ("hello", "hello", false)
        ];

        for (input, contain, expect) in cases {
            assert_eq!(
                ValidationChain::new()
                    .add(validators::NotContain::new(String::from(contain)))
                    .validate(String::from(input))
                    .is_ok(),
                expect
            )
        }
    }
}
