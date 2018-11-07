use std::collections::HashMap;
use std::collections::LinkedList;
use tokenizer::*;

#[derive(Clone, Debug)]
pub enum ContextValues {
    Indexing(usize),
    Input(&'static str),
    Instruction(LinkedList<HashMap<&'static str, InstructionValues>>),
}

impl ContextValues {
    pub fn get_str(&self) -> Option<&'static str> {
        match self {
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

    pub fn get_instruction(&self) -> Option<&LinkedList<HashMap<&'static str, InstructionValues>>> {
        if let ContextValues::Instruction(c) = &self {
            Some(c)
        } else {
            None
        }
    }
    pub fn get_instruction_mut(&mut self) -> Option<&mut LinkedList<HashMap<&'static str, InstructionValues>>> {
        match self {
            ContextValues::Instruction(c) => Some(c),
            _ => None,
        }
    }
}

pub fn parse(input: &'static str,  zero_indexing: bool) -> Tokenizer {
    let mut context = HashMap::new();
    context.insert(
        "Indexing",
        ContextValues::Indexing(if zero_indexing { 0 } else { 1 }),
    );
    context.insert("Input", ContextValues::Input(input));
    //println!("{}", context.get("Input").unwrap().getStr().unwrap());
    let mut tokenizer = Tokenizer::new(&mut context);
    tokenizer.tokenize();
    tokenizer
}
