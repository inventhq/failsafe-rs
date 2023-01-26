use crate::policies::fallback::FallbackAble;
use crate::policies::timeout::Interruptable;
use crate::Runnable;
use rand::distributions::{Alphanumeric, DistString};
use rand::random;
/// a structure to test failsafe
use std::any::Any;
use std::thread::sleep;
use std::time::Duration;
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

    pub fn from_any(other: &Box<dyn Any>) -> &Self {
        other.downcast_ref::<PersonError>().unwrap()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Person {
    name: Option<String>,
    // the followings are for helping test
    _always_fail: bool,
    _fail_pattern: Option<Vec<bool>>,
    _bk_fail_pattern: Option<Vec<bool>>,
    _wait_for: Option<Duration>,
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
            _always_fail: false,
            _fail_pattern: None,
            _bk_fail_pattern: None,
            _wait_for: None,
        }
    }

    pub fn with_name(name: &str) -> Self {
        Person {
            name: Some(name.to_string()),
            _always_fail: false,
            _fail_pattern: None,
            _bk_fail_pattern: None,
            _wait_for: None,
        }
    }

    pub fn name(&self) -> String {
        self.name.clone().unwrap()
    }

    pub fn set_always_fail(&mut self, always_fail: bool) {
        self._always_fail = always_fail;
    }

    pub fn set_fail_pattern(&mut self, fail_pattern: Vec<bool>) {
        if fail_pattern.len() == 0 {
            self._fail_pattern = None;
            self._bk_fail_pattern = None;
            return;
        }
        let mut k = fail_pattern.clone();
        k.reverse();
        self._bk_fail_pattern = Some(k.clone());
        self._fail_pattern = Some(k);
    }

    pub fn set_wait_for(&mut self, d: Duration) {
        self._wait_for = Some(d);
    }
}

impl Runnable for Person {
    fn run(&mut self) -> Result<(), Box<dyn Any>> {
        println!("I am a person, getting my name!");
        let name = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
        // Followings are for testing only
        {
            let error: bool = if self._fail_pattern != None {
                if self._bk_fail_pattern.as_ref().unwrap().is_empty() {
                    self._bk_fail_pattern = self._fail_pattern.clone();
                }
                self._bk_fail_pattern
                    .as_mut()
                    .and_then(|v| v.pop())
                    .unwrap()
            } else if self._always_fail {
                true
            } else {
                random()
            };
            if self._wait_for.is_some() {
                self._wait_for.as_ref().and_then(|v| {
                    sleep(*v);
                    Some(())
                });
            }
            println!("{}", error);
            if error {
                println!("Couldn't get a name!");
                return Err(Box::new(PersonError::as_any(
                    &PersonError::NameFindingError,
                )));
            }
        }
        println!("Got a name! {}", name);
        self.name = Some(name);
        Ok(())
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

impl Interruptable for Person {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
