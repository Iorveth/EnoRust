use messages::*;
use std::collections::HashMap;
use std::collections::LinkedList;
use tokenizer::*;
#[derive(Clone)]

pub enum ContextValues {
    Indexing(usize),
    Input(&'static str),
    Messages(Messages),
    Locale(&'static str),
    Instruction(LinkedList<HashMap<&'static str, InstructionValues>>),
}

impl ContextValues {
    pub fn get_str(&self) -> Option<&'static str> {
        match self {
            ContextValues::Locale(c) => Some(c),
            ContextValues::Input(c) => Some(c),
            _ => None,
        }
    }
    pub fn get_indexing(&self) -> Option<&usize> {
        match self {
            ContextValues::Indexing(c) => Some(c),
            _ => None,
        }
    }
    pub fn get_messages(&self) -> Option<&Messages> {
        if let ContextValues::Messages(c) = &self {
            Some(c)
        } else {
            None
        }
    }

    pub fn get_instruction(&self) -> Option<&LinkedList<HashMap<&'static str, InstructionValues>>> {
        if let ContextValues::Instruction(c) = &self {
            Some(c)
        } else {
            None
        }
    }
}

pub fn parse(input: &'static str, locale: &'static str, zero_indexing: bool) {
    let mut context = HashMap::new();
    context.insert(
        "Indexing",
        ContextValues::Indexing(if zero_indexing { 0 } else { 1 }),
    );
    context.insert("Input", ContextValues::Input(input));
    context.insert(
        "Messages",
        ContextValues::Messages(Messages::get_messages(locale).unwrap()),
    );
    context.insert("Locale", ContextValues::Locale(locale));
    //println!("{}", context.get("Input").unwrap().getStr().unwrap());
    let mut tokenizer = Tokenizer::new(context);
    tokenizer.tokenize();
}