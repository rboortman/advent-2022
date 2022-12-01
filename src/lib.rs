use std::fmt::Display;

use reqwest::header::{COOKIE, USER_AGENT};

pub trait Assignment {
    type Input;
    type Output: Display;

    fn parse_input(&self, input: String) -> Self::Input;

    fn silver(&self, input: &Self::Input) -> Self::Output;
    fn gold(&self, input: &Self::Input) -> Self::Output;

    // fn run(&self, input: String, is_debug: bool) -> (String, String);
    fn run(&self, input: String) -> (Self::Output, Self::Output) {
        let parsed = self.parse_input(input);
        (self.silver(&parsed), self.gold(&parsed))
    }
}
