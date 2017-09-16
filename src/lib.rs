mod tests;

mod validators;

pub struct ValidationChain {
    chain: Vec<Box<ValidatorCondition>>
}

pub trait ValidatorCondition {
    fn validate(&self, input: String) -> bool {
        unimplemented!()
    }

    fn get_err_message(&self) -> String {
        unimplemented!()
    }
}

impl ValidationChain {
    pub fn new() -> Self {
        ValidationChain { chain: Vec::new() }
    }

    pub fn add(&mut self, condition: Box<ValidatorCondition>) -> &mut Self {
        self.chain.push(condition);

        self
    }

    pub fn validate(&self, s: String) -> Result<String, String> {
        for condition in self.chain.iter() {
            if !condition.validate(s.clone()) {
                return Err(condition.get_err_message())
            }
        }

        Ok(s.clone())
    }
}
