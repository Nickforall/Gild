use ValidatorCondition;

pub struct Empty {
    allowed: bool
}

impl Empty {
    pub fn new(allowed: bool) -> Box<Empty> {
        Box::new(Empty { allowed: allowed })
    }
}

impl ValidatorCondition for Empty {
    fn validate(&self, input: String) -> bool {
        input.is_empty() == self.allowed
    }

    fn get_err_message(&self) -> String {
        let mut adverb = "";
        if !self.allowed {
            adverb = "not"
        }

        format!("Input should {} be empty", adverb)
    }
}

pub struct Contain {
    contains: String
}

impl Contain {
    pub fn new(c: String) -> Box<Contain> {
        Box::new(Contain { contains: c })
    }
}

impl ValidatorCondition for Contain {
    fn validate(&self, input: String) -> bool {
        input.contains(self.contains.as_str())
    }

    fn get_err_message(&self) -> String {
        format!("Input should contain {}", self.contains)
    }
}

pub struct NotContain {
    contains: String
}

impl NotContain {
    pub fn new(c: String) -> Box<NotContain> {
        Box::new(NotContain { contains: c })
    }
}

impl ValidatorCondition for NotContain {
    fn validate(&self, input: String) -> bool {
        !input.contains(self.contains.as_str())
    }

    fn get_err_message(&self) -> String {
        format!("Input should not contain {}", self.contains)
    }
}
