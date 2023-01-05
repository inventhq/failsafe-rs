use crate::policies::fallback::FallbackAble;
use crate::Runnable;
use rand::distributions::{Alphanumeric, DistString};
use rand::random;
/// a structure to test failsafe
use std::any::Any;
use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum PersonError {
    #[error("Failed to find name for")]
    NameFindingError,
}

impl PersonError {
    pub fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Person {
    name: Option<String>,
    always_fail: bool,
    fail_pattern: Option<Vec<bool>>,
    _bk_fail_pattern: Option<Vec<bool>>,
}

impl Person {
    pub(crate) fn set_name(&mut self, name: &str) {
        self.name = Some(name.to_string());
    }
}

impl Person {
    pub fn new() -> Self {
        Person {
            name: None,
            always_fail: false,
            fail_pattern: None,
            _bk_fail_pattern: None,
        }
    }

    pub fn with_name(name: &str) -> Self {
        Person {
            name: Some(name.to_string()),
            always_fail: false,
            fail_pattern: None,
            _bk_fail_pattern: None,
        }
    }

    pub fn name(&self) -> String {
        self.name.clone().unwrap()
    }

    pub fn set_always_fail(&mut self, always_fail: bool) {
        self.always_fail = always_fail;
    }

    pub fn set_fail_pattern(&mut self, fail_pattern: Vec<bool>) {
        self._bk_fail_pattern = Some(fail_pattern.clone());
        self.fail_pattern = Some(fail_pattern);
    }
}

impl Runnable for Person {
    fn run(&mut self) -> Result<(), Box<dyn Any>> {
        println!("I am a person, getting my name!");
        let name = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
        let error: bool = if self.fail_pattern != None {
            if self._bk_fail_pattern.as_ref().unwrap().is_empty() {
                self._bk_fail_pattern = self.fail_pattern.clone();
            }
            self._bk_fail_pattern
                .as_mut()
                .and_then(|v| v.pop())
                .unwrap()
        } else if self.always_fail {
            true
        } else {
            random()
        };
        if error {
            println!("Couldn't get a name!");
            return Err(Box::new(PersonError::as_any(
                &PersonError::NameFindingError,
            )));
        }
        println!("Got a name! {}", name);
        self.name = Some(name);
        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn update(&mut self, other: &Box<dyn FallbackAble>) {
        let n: &Person = other.as_any().downcast_ref().unwrap();
        self.name = Some(n.name().clone());
    }
}

impl FallbackAble for Person {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
