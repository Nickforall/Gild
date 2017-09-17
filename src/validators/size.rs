use ValidatorCondition;

/// Validator that checks whether the given is a given maximum length
///
/// # Examples
///
/// ```
/// ValidationChain::new()
///    .add(validators::MaxSize::new(1000))
///    .validate(String::from("This string is very big, but not that big."))
///    .is_ok(),
/// ```
pub struct MaxSize {
    max: usize
}

impl MaxSize {
    pub fn new(size: usize) -> Box<MaxSize> {
        Box::new(MaxSize { max: size })
    }
}

impl ValidatorCondition for MaxSize {
    fn validate(&self, input: String) -> bool {
        if input.len() > self.max {
            return false
        }

        true
    }

    fn get_err_message(&self) -> String {
        format!("Input can not be longer than {} characters", self.max)
    }
}

/// Validator that checks whether the given is a given maximum length
///
/// # Examples
///
/// ```
/// ValidationChain::new()
///    .add(validators::MinSize::new(10))
///    .validate(String::from("This string is at least 10 characters."))
///    .is_ok(),
/// ```
pub struct MinSize {
    max: usize
}

impl MinSize {
    pub fn new(size: usize) -> Box<MinSize> {
        Box::new(MinSize { max: size })
    }
}

impl ValidatorCondition for MinSize {
    fn validate(&self, input: String) -> bool {
        if input.len() < self.max {
            return false
        }

        true
    }

    fn get_err_message(&self) -> String {
        format!("Input should be longer than {} characters", self.max)
    }
}
