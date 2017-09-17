use ValidatorCondition;

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
