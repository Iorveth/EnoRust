use grammar_regex::*;
use messages::*;
use parser::ContextValues;
use regex::Regex;
use std::collections::HashMap;
use std::collections::LinkedList;
pub trait Copy: Clone {}

#[derive(Clone)]
pub struct Tokenizer {
    pub input: &'static str,
    pub context: HashMap<&'static str, ContextValues>,
    pub line: usize,
    pub index: usize,
    pub instructions: LinkedList<HashMap<&'static str, InstructionValues>>,
}
#[derive(Clone)]
pub enum InstructionValues {
    Index(usize),
    Line(usize),
    Type(&'static str),
    Comment(&'static str),
    Name(&'static str),
    Ranges(HashMap<&'static str, RangesValues>),
    Value(&'static str),
    Separator(&'static str),
    Depth(usize),
    Template(&'static str),
    DeepCopy(bool),
    Length(usize),
    ContentRange(Vec<usize>),
}
#[derive(Clone)]
pub enum RangesValues {
    Name(Vec<usize>),
    NameOperator(Vec<usize>),
    EscapeBeginOperator(Vec<usize>),
    EscapeEndOperator(Vec<usize>),
    BlockOperator(Vec<usize>),
    ItemOperator(Vec<usize>),
    Value(Vec<usize>),
    EntryOperator(Vec<usize>),
    NewlineContinuationOperator(Vec<usize>),
    LineContinuationOperator(Vec<usize>),
    Template(Vec<usize>),
    SectionOperator(Vec<usize>),
    CopyOperator(Vec<usize>),
    DeepCopyOperator(Vec<usize>),
    Comment(Vec<usize>),
    Content(Vec<usize>),
    CommentOperator(Vec<usize>),
}

impl InstructionValues {
    pub fn get_numeric(&self) -> Option<&usize> {
        match self {
            InstructionValues::Index(c) => Some(c),
            InstructionValues::Line(c) => Some(c),
            InstructionValues::Depth(c) => Some(c),
            InstructionValues::Length(c) => Some(c),
            _ => None,
        }
    }
    pub fn get_str(&self) -> Option<&'static str> {
        match self {
            InstructionValues::Type(c) => Some(c),
            InstructionValues::Comment(c) => Some(c),
            InstructionValues::Name(c) => Some(c),
            InstructionValues::Value(c) => Some(c),
            InstructionValues::Separator(c) => Some(c),
            InstructionValues::Template(c) => Some(c),
            _ => None,
        }
    }
}

impl Tokenizer {
    pub fn new(contextt: HashMap<&'static str, ContextValues>) -> Tokenizer {
        let mut k = Tokenizer {
            context: contextt.clone(),
            input: contextt.get("Input").unwrap().get_str().unwrap(),
            line: 0,
            index: 0,
            instructions: LinkedList::new(),
        };
        k.context.insert(
            "Instructions",
            ContextValues::Instruction(LinkedList::new()),
        );
        k
    }
    pub fn tokenize(&mut self) {
        #[derive(Clone, Copy)]
        let mut ranges = HashMap::new();
        loop {
            let reg = Regex::new(REGEX).unwrap();
            let r = reg.find(self.input);
            let capture = reg.captures(self.input).unwrap();
            let mut instruction = HashMap::new();
            if (r.is_none()) || (r.unwrap().start() != self.index) {
                instruction = self.tokenize_error_context();
                Tokenization::error_msg("invalid_line", &self.context, &instruction);
            }
            instruction.insert("Index", InstructionValues::Index(self.index));
            instruction.insert("Line", InstructionValues::Index(self.line));
            let mut block = false;
            if capture.get(EMPTY_LINE_INDEX) != None {
                instruction.insert("Type", InstructionValues::Type("EMPTY_LINE"));
            } else if capture.get(NAME_OPERATOR_INDEX).is_some() {
                let unescaped_name = capture.get(NAME_UNESCAPED_INDEX).unwrap().as_str();

                if !(unescaped_name.is_empty()) {
                    instruction.insert("Name", InstructionValues::Name(unescaped_name));
                    let name_column =
                        capture.get(NAME_UNESCAPED_INDEX).unwrap().start() - self.index;
                    let name_operator_column =
                        capture.get(NAME_OPERATOR_INDEX).unwrap().start() - self.index;
                    instruction.insert(
                        "Ranges",
                        InstructionValues::Ranges({
                            ranges.insert(
                                "Name",
                                RangesValues::Name(
                                    (name_column..(name_column + unescaped_name.len())).collect(),
                                ),
                            );
                            ranges.insert(
                                "NameOperator",
                                RangesValues::NameOperator(
                                    (name_operator_column..(name_column + 1)).collect(),
                                ),
                            );
                            ranges.clone()
                        }),
                    );
                } else {
                    ranges = HashMap::new();
                    let escaped_name = capture.get(NAME_ESCAPED_INDEX).unwrap().as_str();
                    instruction.insert("Name", InstructionValues::Name(escaped_name));

                    let escape_operator = capture.get(NAME_ESCAPE_BEGIN_OPERATOR_INDEX).unwrap();
                    let escape_begin_operator_column = capture
                        .get(NAME_ESCAPE_BEGIN_OPERATOR_INDEX)
                        .unwrap()
                        .start()
                        - self.index;
                    let name_column = capture.get(NAME_ESCAPED_INDEX).unwrap().start() - self.index;
                    let escape_end_operator_column =
                        capture.get(NAME_ESCAPE_END_OPERATOR_INDEX).unwrap().start() - self.index;
                    let name_operator_column =
                        capture.get(NAME_OPERATOR_INDEX).unwrap().start() - self.index;
                    instruction.insert(
                        "Ranges",
                        InstructionValues::Ranges({
                            ranges.insert(
                                "EscapeBeginOperator",
                                RangesValues::EscapeBeginOperator(
                                    (escape_begin_operator_column
                                        ..(escape_begin_operator_column
                                            + escape_operator.as_str().len()))
                                        .collect(),
                                ),
                            );
                            ranges.insert(
                                "EscapeEndOperator",
                                RangesValues::EscapeEndOperator(
                                    (escape_end_operator_column
                                        ..(escape_end_operator_column
                                            + escape_operator.as_str().len()))
                                        .collect(),
                                ),
                            );
                            ranges.insert(
                                "NameOperator",
                                RangesValues::NameOperator(
                                    (name_operator_column..(name_operator_column + 1)).collect(),
                                ),
                            );
                            ranges.insert(
                                "Name",
                                RangesValues::Name(
                                    (name_column..(name_column + escaped_name.len())).collect(),
                                ),
                            );
                            ranges.clone()
                        }),
                    );
                }
                let value = capture.get(FIELD_VALUE_INDEX).unwrap();
                if capture.get(FIELD_VALUE_INDEX).is_some() {
                    instruction.insert("Type", InstructionValues::Type("FIELD"));
                    instruction.insert("Value", InstructionValues::Value(value.as_str()));

                    let value_column = capture.get(FIELD_VALUE_INDEX).unwrap().start() - self.index;
                    instruction.insert(
                        "Ranges",
                        InstructionValues::Ranges({
                            ranges.insert(
                                "Value",
                                RangesValues::Value(
                                    (value_column..value_column + value.as_str().len()).collect(),
                                ),
                            );
                            ranges.clone()
                        }),
                    );
                } else {
                    instruction.insert("Type", InstructionValues::Type("Name"));
                }
            } else if capture.get(LIST_ITEM_OPERATOR_INDEX).is_some() {
                instruction.insert("Type", InstructionValues::Type("LIST_ITEM"));
                let operator_column =
                    capture.get(LIST_ITEM_OPERATOR_INDEX).unwrap().start() - self.index;
                instruction.insert(
                    "Ranges",
                    InstructionValues::Ranges({
                        ranges.insert(
                            "ItemOperator",
                            RangesValues::ItemOperator(
                                (operator_column..(operator_column + 1)).collect(),
                            ),
                        );
                        ranges.clone()
                    }),
                );
                let value = capture.get(LIST_ITEM_VALUE_INDEX).unwrap();
                instruction.insert("Value", InstructionValues::Value(value.as_str()));
                if capture.get(LIST_ITEM_VALUE_INDEX).is_some() {
                    let value_column =
                        capture.get(LIST_ITEM_VALUE_INDEX).unwrap().start() - self.index;
                    instruction.insert(
                        "Ranges",
                        InstructionValues::Ranges({
                            ranges.insert(
                                "Value",
                                RangesValues::Value(
                                    (value_column..(value_column + value.as_str().len())).collect(),
                                ),
                            );
                            ranges.clone()
                        }),
                    );
                }
            } else if capture.get(FIELDSET_ENTRY_OPERATOR_INDEX).is_some() {
                let unescaped_name = capture.get(NAME_UNESCAPED_INDEX).unwrap();
                if capture.get(NAME_UNESCAPED_INDEX).is_some() {
                    instruction.insert("Name", InstructionValues::Name(unescaped_name.as_str()));
                    let name_column =
                        capture.get(NAME_UNESCAPED_INDEX).unwrap().start() - self.index;
                    let entry_operator_column =
                        capture.get(FIELDSET_ENTRY_OPERATOR_INDEX).unwrap().start() - self.index;

                    instruction.insert(
                        "Ranges",
                        InstructionValues::Ranges({
                            ranges.insert(
                                "EntryOperator",
                                RangesValues::EntryOperator(
                                    (entry_operator_column..(entry_operator_column + 1)).collect(),
                                ),
                            );
                            ranges.insert(
                                "Name",
                                RangesValues::Name(
                                    (name_column..(name_column + unescaped_name.as_str().len()))
                                        .collect(),
                                ),
                            );
                            ranges.clone()
                        }),
                    );
                } else {
                    let escaped_name = capture.get(NAME_ESCAPED_INDEX).unwrap();
                    instruction.insert("Name", InstructionValues::Name(escaped_name.as_str()));

                    let escape_operator = capture.get(NAME_ESCAPE_BEGIN_OPERATOR_INDEX).unwrap();
                    let escape_begin_operator_column = capture
                        .get(NAME_ESCAPE_BEGIN_OPERATOR_INDEX)
                        .unwrap()
                        .start()
                        - self.index;
                    let name_column = capture.get(NAME_ESCAPED_INDEX).unwrap().start() - self.index;
                    let escape_end_operator_column =
                        capture.get(NAME_ESCAPE_END_OPERATOR_INDEX).unwrap().start() - self.index;
                    let entry_operator_column =
                        capture.get(FIELDSET_ENTRY_OPERATOR_INDEX).unwrap().start() - self.index;

                    instruction.insert(
                        "Ranges",
                        InstructionValues::Ranges({
                            ranges.insert(
                                "EscapeBeginOperator",
                                RangesValues::EscapeBeginOperator(
                                    (escape_begin_operator_column
                                        ..(escape_begin_operator_column
                                            + escape_operator.as_str().len()))
                                        .collect(),
                                ),
                            );
                            ranges.insert(
                                "EscapeEndOperator",
                                RangesValues::EscapeEndOperator(
                                    (escape_end_operator_column
                                        ..(escape_end_operator_column
                                            + escape_operator.as_str().len()))
                                        .collect(),
                                ),
                            );
                            ranges.insert(
                                "EntryOperator",
                                RangesValues::NameOperator(
                                    (entry_operator_column..(entry_operator_column + 1)).collect(),
                                ),
                            );
                            ranges.insert(
                                "Name",
                                RangesValues::Name(
                                    (name_column..(name_column + escaped_name.as_str().len()))
                                        .collect(),
                                ),
                            );
                            ranges.clone()
                        }),
                    );
                }
                instruction.insert("Type", InstructionValues::Type("FIELDSET_ENTRY"));
                let value = capture.get(FIELDSET_ENTRY_VALUE_INDEX).unwrap();
                instruction.insert("Value", InstructionValues::Value(value.as_str()));
                if capture.get(FIELDSET_ENTRY_VALUE_INDEX).is_some() {
                    let value_column =
                        capture.get(FIELDSET_ENTRY_VALUE_INDEX).unwrap().start() - self.index;
                    instruction.insert(
                        "Ranges",
                        InstructionValues::Ranges({
                            ranges.insert(
                                "Value",
                                RangesValues::Value(
                                    (value_column..(value_column + value.as_str().len())).collect(),
                                ),
                            );
                            ranges.clone()
                        }),
                    );
                }
            } else if capture.get(LINE_CONTINUATION_OPERATOR_INDEX).is_some() {
                instruction.insert("Separator", InstructionValues::Separator(" "));
                instruction.insert("Type", InstructionValues::Type("CONTINUATION"));

                let value = capture.get(LINE_CONTINUATION_VALUE_INDEX).unwrap();
                instruction.insert("Value", InstructionValues::Value(value.as_str()));

                let operator_column = capture
                    .get(LINE_CONTINUATION_OPERATOR_INDEX)
                    .unwrap()
                    .start() - self.index;
                instruction.insert(
                    "Ranges",
                    InstructionValues::Ranges({
                        ranges.insert(
                            "LineContinuationOperator",
                            RangesValues::LineContinuationOperator(
                                (operator_column..(operator_column + 1)).collect(),
                            ),
                        );
                        ranges.clone()
                    }),
                );
                if capture.get(LINE_CONTINUATION_VALUE_INDEX).is_some() {
                    let value_column =
                        capture.get(LINE_CONTINUATION_VALUE_INDEX).unwrap().start() - self.index;
                    instruction.insert(
                        "Ranges",
                        InstructionValues::Ranges({
                            ranges.insert(
                                "Value",
                                RangesValues::Value(
                                    (value_column..(value_column + value.as_str().len())).collect(),
                                ),
                            );
                            ranges.clone()
                        }),
                    );
                }
            } else if capture.get(NEWLINE_CONTINUATION_OPERATOR_INDEX).is_some() {
                instruction.insert("Separator", InstructionValues::Separator("\n"));
                instruction.insert("Type", InstructionValues::Type("CONTINUATION"));

                let value = capture.get(NEWLINE_CONTINUATION_VALUE_INDEX).unwrap();
                instruction.insert("Value", InstructionValues::Value(value.as_str()));

                let operator_column = capture
                    .get(NEWLINE_CONTINUATION_OPERATOR_INDEX)
                    .unwrap()
                    .start() - self.index;
                instruction.insert(
                    "Ranges",
                    InstructionValues::Ranges({
                        ranges.insert(
                            "NewLineContinuationOperator",
                            RangesValues::NewlineContinuationOperator(
                                (operator_column..(operator_column + 1)).collect(),
                            ),
                        );
                        ranges.clone()
                    }),
                );
                if capture.get(NEWLINE_CONTINUATION_VALUE_INDEX).is_some() {
                    let value_column = capture
                        .get(NEWLINE_CONTINUATION_VALUE_INDEX)
                        .unwrap()
                        .start() - self.index;
                    instruction.insert(
                        "Ranges",
                        InstructionValues::Ranges({
                            ranges.insert(
                                "Value",
                                RangesValues::Value(
                                    (value_column..(value_column + value.as_str().len())).collect(),
                                ),
                            );
                            ranges.clone()
                        }),
                    );
                }
            } else if capture.get(SECTION_HASHES_INDEX).is_some() {
                let section_operator = capture.get(SECTION_HASHES_INDEX).unwrap();
                instruction.insert(
                    "Depth",
                    InstructionValues::Depth(section_operator.as_str().len()),
                );
                instruction.insert("Type", InstructionValues::Type("SECTION"));
                let section_operator_column =
                    capture.get(SECTION_HASHES_INDEX).unwrap().start() - self.index;
                let unescaped_name = capture.get(SECTION_NAME_UNESCAPED_INDEX).unwrap();

                if capture.get(SECTION_NAME_UNESCAPED_INDEX).is_some() {
                    instruction.insert("Name", InstructionValues::Name(unescaped_name.as_str()));
                    let name_column =
                        capture.get(SECTION_NAME_UNESCAPED_INDEX).unwrap().start() - self.index;
                    let name_end_column = name_column + unescaped_name.as_str().len();
                    instruction.insert(
                        "Ranges",
                        InstructionValues::Ranges({
                            ranges.insert(
                                "Name",
                                RangesValues::LineContinuationOperator(
                                    (name_column..name_end_column).collect(),
                                ),
                            );
                            ranges.insert(
                                "SectionOperator",
                                RangesValues::LineContinuationOperator(
                                    (section_operator_column
                                        ..(section_operator_column
                                            + section_operator.as_str().len()))
                                        .collect(),
                                ),
                            );
                            ranges.clone()
                        }),
                    );
                } else {
                    let escaped_name = capture.get(SECTION_NAME_ESCAPED_INDEX).unwrap();
                    instruction.insert("Name", InstructionValues::Name(escaped_name.as_str()));

                    let escape_operator = capture
                        .get(SECTION_NAME_ESCAPE_BEGIN_OPERATOR_INDEX)
                        .unwrap();
                    let escape_begin_operator_column = capture
                        .get(SECTION_NAME_ESCAPE_BEGIN_OPERATOR_INDEX)
                        .unwrap()
                        .start()
                        - self.index;
                    let name_column =
                        capture.get(SECTION_NAME_ESCAPED_INDEX).unwrap().start() - self.index;
                    let escape_end_operator_column = capture
                        .get(SECTION_NAME_ESCAPE_END_OPERATOR_INDEX)
                        .unwrap()
                        .start() - self.index;
                    let name_end_column =
                        escape_end_operator_column + escape_operator.as_str().len();

                    instruction.insert(
                        "Ranges",
                        InstructionValues::Ranges({
                            ranges.insert(
                                "EscapeBeginOperator",
                                RangesValues::EscapeBeginOperator(
                                    (escape_begin_operator_column
                                        ..(escape_begin_operator_column
                                            + escape_operator.as_str().len()))
                                        .collect(),
                                ),
                            );
                            ranges.insert(
                                "EscapeEndOperator",
                                RangesValues::EscapeEndOperator(
                                    (escape_end_operator_column
                                        ..(escape_end_operator_column
                                            + escape_operator.as_str().len()))
                                        .collect(),
                                ),
                            );
                            ranges.insert(
                                "Name",
                                RangesValues::Name(
                                    (name_column..(name_column + escaped_name.as_str().len()))
                                        .collect(),
                                ),
                            );
                            ranges.insert(
                                "SectionOperator",
                                RangesValues::SectionOperator(
                                    (section_operator_column
                                        ..(section_operator_column
                                            + section_operator.as_str().len()))
                                        .collect(),
                                ),
                            );
                            ranges.clone()
                        }),
                    );
                }
                let template = capture.get(SECTION_TEMPLATE_INDEX).unwrap();

                if capture.get(SECTION_TEMPLATE_INDEX).is_some() {
                    instruction.insert("Template", InstructionValues::Template(template.as_str()));

                    let copy_operator = capture.get(SECTION_COPY_OPERATOR_INDEX).unwrap();
                    let copy_operator_column =
                        capture.get(SECTION_COPY_OPERATOR_INDEX).unwrap().start() - self.index;
                    let template_column =
                        capture.get(SECTION_TEMPLATE_INDEX).unwrap().start() - self.index;

                    if copy_operator.as_str() == "<" {
                        instruction.insert("DeepCopy", InstructionValues::DeepCopy(false));
                        instruction.insert(
                            "Ranges",
                            InstructionValues::Ranges({
                                ranges.insert(
                                    "CopyOperator",
                                    RangesValues::CopyOperator(
                                        (copy_operator_column..(copy_operator_column + 1))
                                            .collect(),
                                    ),
                                );
                                ranges.clone()
                            }),
                        );
                    } else {
                        instruction.insert("DeepCopy", InstructionValues::DeepCopy(true));
                        instruction.insert(
                            "Ranges",
                            InstructionValues::Ranges({
                                ranges.insert(
                                    "DeepCopyOperator",
                                    RangesValues::DeepCopyOperator(
                                        (copy_operator_column..(copy_operator_column + 2))
                                            .collect(),
                                    ),
                                );
                                ranges.clone()
                            }),
                        );
                    }
                    instruction.insert(
                        "Ranges",
                        InstructionValues::Ranges({
                            ranges.insert(
                                "Template",
                                RangesValues::Template(
                                    (template_column..(template_column + template.as_str().len()))
                                        .collect(),
                                ),
                            );
                            ranges.clone()
                        }),
                    );
                }
            } else if capture.get(BLOCK_DASHES_INDEX).is_some() {
                let operator = capture.get(BLOCK_DASHES_INDEX).unwrap();
                let name = capture.get(BLOCK_NAME_INDEX).unwrap();

                instruction.insert("Name", InstructionValues::Name(name.as_str()));
                instruction.insert("Type", InstructionValues::Type("BLOCK"));

                let operator_column = capture.get(BLOCK_DASHES_INDEX).unwrap().start() - self.index;
                let name_column = capture.get(BLOCK_NAME_INDEX).unwrap().start() - self.index;
                instruction.insert(
                    "Length",
                    InstructionValues::Length(r.unwrap().end() - self.index),
                );

                instruction.insert(
                    "Ranges",
                    InstructionValues::Ranges({
                        ranges.insert(
                            "Template",
                            RangesValues::Template(
                                (operator_column..(operator_column + operator.as_str().len()))
                                    .collect(),
                            ),
                        );
                        ranges.clone()
                    }),
                );

                instruction.insert(
                    "Ranges",
                    InstructionValues::Ranges({
                        ranges.insert(
                            "BlockOperator",
                            RangesValues::BlockOperator(
                                (operator_column..(operator_column + operator.as_str().len()))
                                    .collect(),
                            ),
                        );
                        ranges.insert(
                            "Name",
                            RangesValues::Name(
                                (name_column..(name_column + name.as_str().len())).collect(),
                            ),
                        );
                        ranges.clone()
                    }),
                );
                self.context
                    .get("Instructions")
                    .unwrap()
                    .get_instruction()
                    .as_mut()
                    .unwrap()
                    .push_back(instruction.clone());

                self.index = r.unwrap().end();

                let terminator_str =
                    "\\n[^\\S\\n]*({operator})[^\\S\\n]*({re.escape(name)})[^\\S\\n]*(?=\\n|$)";
                let terminator_re = Regex::new(terminator_str).unwrap();
                let terminator_match = terminator_re.find_at(self.input, self.index).unwrap();
                //TODO start match from self.index
                let terminator_capture = terminator_re.captures(self.input).unwrap();

                self.index += 1;
                self.line += 1;

                if terminator_re.find_at(self.input, self.index).is_some() {
                    self.tokenize_error_context();
                    Tokenization::error_msg("unterminated_block", &self.context, &instruction);
                }

                let end_of_block_index = terminator_match.start();
                if end_of_block_index != self.index - 1 {
                    let mut instr = HashMap::new();
                    instruction.insert(
                        "ContentRange",
                        InstructionValues::ContentRange(
                            (self.index..(end_of_block_index - 1)).collect(),
                        ),
                    );
                    loop {
                        instr = HashMap::new();
                        ranges = HashMap::new();
                        let end_of_line_index = (&self.input[self.index..end_of_block_index])
                            .find("\n")
                            .unwrap();
                        if (&self.input[self.index..end_of_block_index])
                            .find("\n")
                            .is_none()
                        {
                            self.instructions.push_back({
                                instr.insert("Index", InstructionValues::Index(self.index));
                                instr.insert(
                                    "Length",
                                    InstructionValues::Length(end_of_block_index - self.index),
                                );
                                instr.insert("Line", InstructionValues::Line(self.line));
                                instr.insert(
                                    "Rages",
                                    InstructionValues::Ranges({
                                        ranges.insert(
                                            "Content",
                                            RangesValues::Content(
                                                (0..(end_of_block_index - self.index)).collect(),
                                            ),
                                        );
                                        ranges.clone()
                                    }),
                                );
                                instr.insert("Type", InstructionValues::Type("BLOCK_CONTENT"));
                                instr.clone()
                            });

                            self.index = end_of_block_index + 1;
                            self.line += 1;

                            break;
                        } else {
                            self.instructions.push_back({
                                instr.insert("Index", InstructionValues::Index(self.index));
                                instr.insert(
                                    "Length",
                                    InstructionValues::Length(end_of_line_index - self.index),
                                );
                                instr.insert("Line", InstructionValues::Line(self.line));
                                instr.insert(
                                    "Rages",
                                    InstructionValues::Ranges({
                                        ranges.insert(
                                            "Content",
                                            RangesValues::Content(
                                                (0..(end_of_line_index - self.index)).collect(),
                                            ),
                                        );
                                        ranges.clone()
                                    }),
                                );
                                instr.insert("Type", InstructionValues::Type("BLOCK_CONTENT"));
                                instr.clone()
                            });

                            self.index = end_of_line_index + 1;
                            self.line += 1;
                        }
                    }
                }
                ranges = HashMap::new();
                instruction.insert("Index", InstructionValues::Index(self.index));
                instruction.insert(
                    "Length",
                    InstructionValues::Length(terminator_match.end() - self.index),
                );
                instruction.insert("Line", InstructionValues::Line(self.line));
                instruction.insert(
                    "Ranges",
                    InstructionValues::Ranges({
                        ranges.insert(
                            "BlockOperator",
                            RangesValues::BlockOperator(
                                ((terminator_capture.get(1).unwrap().start() - self.index)
                                    ..(terminator_capture.get(1).unwrap().end() - self.index))
                                    .collect(),
                            ),
                        );
                        ranges.insert(
                            "Name",
                            RangesValues::BlockOperator(
                                ((terminator_capture.get(2).unwrap().start() - self.index)
                                    ..(terminator_capture.get(2).unwrap().end() - self.index))
                                    .collect(),
                            ),
                        );
                        ranges.clone()
                    }),
                );
                instruction.insert("Type", InstructionValues::Type("BLOCK_TERMINATOR"));

                self.instructions.push_back(instruction);

                self.index = terminator_match.end() + 1;
                self.line += 1;

                block = true;
            } else if capture.get(COMMENT_OPERATOR_INDEX).is_some() {
                let comment = capture.get(COMMENT_TEXT_INDEX);
                let comment_operator_column =
                    capture.get(COMMENT_OPERATOR_INDEX).unwrap().start() - self.index;

                instruction.insert("Type", InstructionValues::Type("COMMENT"));
                instruction.insert(
                    "Comment",
                    InstructionValues::Comment(comment.unwrap().as_str()),
                );
                instruction.insert(
                    "Ranges",
                    InstructionValues::Ranges({
                        ranges.insert(
                            "CommentOperator",
                            RangesValues::CommentOperator(
                                (comment_operator_column..(comment_operator_column + 1)).collect(),
                            ),
                        );
                        ranges.clone()
                    }),
                );

                if comment.is_some() {
                    let comment_column =
                        capture.get(COMMENT_TEXT_INDEX).unwrap().start() - self.index;
                    instruction.insert(
                        "Ranges",
                        InstructionValues::Ranges({
                            ranges.insert(
                                "Comment",
                                RangesValues::Comment(
                                    (comment_column
                                        ..(comment_column + comment.unwrap().as_str().len()))
                                        .collect(),
                                ),
                            );
                            ranges.clone()
                        }),
                    );
                }
            } else if capture.get(COPY_OPERATOR_INDEX).is_some() {
                let operator = capture.get(COPY_OPERATOR_INDEX);
                let template = capture.get(TEMPLATE_INDEX);
                let unescaped_name = capture.get(NAME_UNESCAPED_INDEX);

                if unescaped_name.is_some() {
                    instruction.insert(
                        "Name",
                        InstructionValues::Name(unescaped_name.unwrap().as_str()),
                    );

                    let name_column =
                        capture.get(NAME_UNESCAPED_INDEX).unwrap().start() - self.index;
                    let operator_column =
                        capture.get(COPY_OPERATOR_INDEX).unwrap().start() - self.index;

                    instruction.insert(
                        "Ranges",
                        InstructionValues::Ranges({
                            ranges.insert(
                                "CopyOperator",
                                RangesValues::CopyOperator(
                                    (operator_column
                                        ..operator_column + operator.unwrap().as_str().len())
                                        .collect(),
                                ),
                            );
                            ranges.insert(
                                "Name",
                                RangesValues::Name(
                                    (name_column
                                        ..name_column + unescaped_name.unwrap().as_str().len())
                                        .collect(),
                                ),
                            );
                            ranges.clone()
                        }),
                    );
                } else {
                    let escaped_name = capture.get(NAME_ESCAPED_INDEX);
                    instruction.insert(
                        "Name",
                        InstructionValues::Name(escaped_name.unwrap().as_str()),
                    );

                    let escape_operator = capture.get(NAME_ESCAPE_BEGIN_OPERATOR_INDEX);
                    let escape_begin_operator_column = capture
                        .get(NAME_ESCAPE_BEGIN_OPERATOR_INDEX)
                        .unwrap()
                        .start()
                        - self.index;
                    let name_column = capture.get(NAME_ESCAPED_INDEX).unwrap().start() - self.index;
                    let escape_end_operator_column =
                        capture.get(NAME_ESCAPE_END_OPERATOR_INDEX).unwrap().start() - self.index;
                    let operator_column =
                        capture.get(COPY_OPERATOR_INDEX).unwrap().start() - self.index;

                    instruction.insert(
                        "Ranges",
                        InstructionValues::Ranges({
                            ranges.insert(
                                "CopyOperator",
                                RangesValues::CopyOperator(
                                    (operator_column
                                        ..(operator_column + operator.unwrap().as_str().len()))
                                        .collect(),
                                ),
                            );
                            ranges.insert(
                                "EscapeBeginOperator",
                                RangesValues::EscapeBeginOperator(
                                    (escape_begin_operator_column
                                        ..(escape_begin_operator_column
                                            + escape_operator.unwrap().as_str().len()))
                                        .collect(),
                                ),
                            );
                            ranges.insert(
                                "EscapeEndOperator",
                                RangesValues::EscapeEndOperator(
                                    (escape_end_operator_column
                                        ..(escape_end_operator_column
                                            + escape_operator.unwrap().as_str().len()))
                                        .collect(),
                                ),
                            );
                            ranges.insert(
                                "Name",
                                RangesValues::Name(
                                    (name_column
                                        ..(name_column + escaped_name.unwrap().as_str().len()))
                                        .collect(),
                                ),
                            );
                            ranges.clone()
                        }),
                    );
                }

                instruction.insert(
                    "Template",
                    InstructionValues::Template(template.unwrap().as_str()),
                );
                instruction.insert("Type", InstructionValues::Type("NAME"));

                let template_column = capture.get(TEMPLATE_INDEX).unwrap().start() - self.index;
                instruction.insert(
                    "Ranges",
                    InstructionValues::Ranges({
                        ranges.insert(
                            "Template",
                            RangesValues::Template(
                                (template_column
                                    ..template_column + template.unwrap().as_str().len())
                                    .collect(),
                            ),
                        );
                        ranges.clone()
                    }),
                );

                if !(block) {
                    instruction.insert(
                        "Length",
                        InstructionValues::Length(r.unwrap().end() - self.index),
                    );
                    self.index = r.unwrap().end() + 1;
                    self.context
                        .get("Instructions")
                        .unwrap()
                        .get_instruction()
                        .unwrap()
                        .push_back(instruction.clone());
                    self.line += 1;
                }

                let mut instr = HashMap::new();
                if self.index >= self.input.len() {
                    if self.input.len() > 0
                        && self.input.chars().nth(self.input.len() - 1).unwrap() == '\n'
                    {
                        self.context
                            .get("Instructions")
                            .unwrap()
                            .get_instruction()
                            .unwrap()
                            .push_back({
                                instr.insert("Index", InstructionValues::Index(self.input.len()));
                                instr.insert("Length", InstructionValues::Length(0));
                                instr.insert("Line", InstructionValues::Line(self.line));
                                instr.insert("Type", InstructionValues::Type("EMPTY_LINE"));
                                instr.clone()
                            });
                        break;
                    }
                }
            }
        }
    }

    pub fn tokenize_error_context(&mut self) -> HashMap<&'static str, InstructionValues> {
        let mut first_instruction = HashMap::new();
        loop {
            let end_of_line_index = &self.input[self.index..self.input.len()].find("\n");
            let mut instruction = HashMap::new();
            if end_of_line_index.is_none() {
                instruction.insert("Index", InstructionValues::Index(self.index));
                instruction.insert(
                    "Length",
                    InstructionValues::Length(self.input.len() - self.index),
                );
                instruction.insert("Line", InstructionValues::Line(self.line));
                for (key, val) in self.context.iter_mut() {
                    if key == &"Instructions" {
                        val.get_instruction()
                            .as_mut()
                            .unwrap()
                            .push_back(instruction.clone());
                    }
                }
                if first_instruction.is_empty() {
                    return instruction.clone();
                } else {
                    return first_instruction;
                }
            } else {
                instruction.insert("Index", InstructionValues::Index(self.index));
                instruction.insert(
                    "Length",
                    InstructionValues::Length(end_of_line_index.unwrap() - self.index),
                );
                instruction.insert("Line", InstructionValues::Line(self.line));
                for (key, val) in self.context.iter_mut() {
                    if key == &"Instructions" {
                        val.get_instruction().unwrap().push_back(instruction.clone());
                    }
                }
                if first_instruction.is_empty() {
                    first_instruction = instruction.clone();

                    self.index = end_of_line_index.unwrap() + 1;
                    self.line += 1;
                }
            }
        }
    }
}
