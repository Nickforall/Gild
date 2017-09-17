# Gild

Gild is a simple validation library in Rust, you can chain multiple conditions and
check whether the input is valid. You can also write your own validators, if our
set of validator conditions isn't enough.

Using it is as easy as this:

```rust

ValidationChain::new()
    .add(validators::MaxSize::new(1)) // false
    .add(validators::MinSize::new(5)) // true
    .validate(String::from("Hello, World"))
    .is_ok();

```

## Writing custom rules

We make use of Rust's great typesystem to create custom validators

```rust
struct MyCustomValidator;

impl ValidatorCondition for MyCustomValidator {
    fn validate(&self, input: String) -> bool {
        if input == String::from("cool") {
            return true
        }

        return false
    }

    fn get_err_message(&self) -> String {
        format!("Input is not cool...")
    }
}
```

**If you wrote a validator that you think the community might want to use,
feel free to open a PR**

## Validation rules

We currently have a small set of rules available:

* `MaxSize`
* `MinSize`
* `Contains`
* `Empty`
* `NotContains`
