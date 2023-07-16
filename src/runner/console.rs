use dialoguer::{console::Term, theme::ColorfulTheme, Select};
use std::fmt::Display;
use eyre::Result;
use crate::runner::options::Choice;

pub struct Console {
    current_choice: Choice,
}

impl Console {
    pub fn new() -> Self {
        Self {
            current_choice: Choice::Wallet,
        }
    }

    pub fn display<T: Display + Clone>(self, elements: Vec<T>, prompt: &str) -> Result<T> 
    { 
        let selection = Select::with_theme(&ColorfulTheme::default())
            .items(&elements)
            .with_prompt(prompt)
            .default(0)
            .interact_on_opt(&Term::stderr())
            .unwrap();

        match selection {
            Some(index) => Ok(elements[index].clone()),
            None => self.display(elements, prompt),
        }
    }

    pub fn next_choice(&mut self) -> Option<Choice> {
        match self.current_choice {
            Choice::Wallet => {
                self.current_choice = Choice::ChainType;
                Some(Choice::Wallet)
            },
            Choice::ChainType => {
                self.current_choice = Choice::Type;
                Some(Choice::ChainType)
            },
            Choice::Type => {
                self.current_choice = Choice::Wallet;
                Some(Choice::Type)
            },
        }
    }
}