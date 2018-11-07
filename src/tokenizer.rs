use env_logger;
use grammar_regex::*;
use messages::*;
use parser::ContextValues;
use onig;
use regex;
use std::collections::HashMap;
use std::collections::LinkedList;
pub trait Copy: Clone {}

#[derive(Clone, Debug)]
pub struct Tokenizer {
    pub input: &'static str,
    pub context: HashMap<&'static str, ContextValues>,
    pub line: usize,
    pub index: usize,
    pub instructions: LinkedList<HashMap<&'static str, InstructionValues>>,
}
#[derive(Clone, Debug)]
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
#[derive(Clone, Debug)]
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
    pub fn new(context: &mut HashMap<&'static str, ContextValues>) -> Tokenizer {
        let mut k = Tokenizer {
            context: context.clone(),
            input: context.get("Input").unwrap().get_str().unwrap(),
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
        env_logger::init();
        info!("starting up");
        let mut ranges = HashMap::new();
        let mut block;
        loop {
            let reg = onig::Regex::new(get_regex().as_str()).unwrap();
            let r = reg.find(self.input);
            let capture = reg.captures(self.input).unwrap();
            println!("{:?}", capture);
            println!("{:?}", r);
            let mut instruction = HashMap::new();
            if (r.is_none()) || (r.unwrap().0 != self.index) {
                instruction = self.tokenize_error_context();
                Tokenization::error_msg("invalid_line", &self.context, &instruction);
            }
            instruction.insert("Index", InstructionValues::Index(self.index));
            instruction.insert("Line", InstructionValues::Index(self.line));
            block = false;

            if capture.at(EMPTY_LINE_INDEX).is_some() {
                instruction.insert("Type", InstructionValues::Type("EMPTY_LINE"));
            } else if capture.at(NAME_OPERATOR_INDEX).is_some() {
                info!("moved here");
                let unescaped_name = capture.at(NAME_UNESCAPED_INDEX).unwrap();

                if !(unescaped_name.is_empty()) {
                    instruction.insert("Name", InstructionValues::Name(unescaped_name));
                    let name_column =
                        capture.pos(NAME_UNESCAPED_INDEX).unwrap().0 - self.index;
                    let name_operator_column =
                        capture.pos(NAME_OPERATOR_INDEX).unwrap().0 - self.index;
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
                    let escaped_name = capture.at(NAME_ESCAPED_INDEX).unwrap();
                    instruction.insert("Name", InstructionValues::Name(escaped_name));

                    let escape_operator = capture.at(NAME_ESCAPE_BEGIN_OPERATOR_INDEX).unwrap();
                    let escape_begin_operator_column = capture
                        .pos(NAME_ESCAPE_BEGIN_OPERATOR_INDEX)
                        .unwrap()
                        .0
                        - self.index;
                    let name_column = capture.pos(NAME_ESCAPED_INDEX).unwrap().0 - self.index;
                    let escape_end_operator_column =
                        capture.pos(NAME_ESCAPE_END_OPERATOR_INDEX).unwrap().0 - self.index;
                    let name_operator_column =
                        capture.pos(NAME_OPERATOR_INDEX).unwrap().0 - self.index;
                    instruction.insert(
                        "Ranges",
                        InstructionValues::Ranges({
                            ranges.insert(
                                "EscapeBeginOperator",
                                RangesValues::EscapeBeginOperator(
                                    (escape_begin_operator_column
                                        ..(escape_begin_operator_column
                                        + escape_operator.len()))
                                        .collect(),
                                ),
                            );
                            ranges.insert(
                                "EscapeEndOperator",
                                RangesValues::EscapeEndOperator(
                                    (escape_end_operator_column
                                        ..(escape_end_operator_column
                                        + escape_operator.len()))
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
                let value = capture.at(FIELD_VALUE_INDEX).unwrap();
                if capture.at(FIELD_VALUE_INDEX).is_some() {
                    instruction.insert("Type", InstructionValues::Type("FIELD"));
                    instruction.insert("Value", InstructionValues::Value(value));

                    let value_column = capture.pos(FIELD_VALUE_INDEX).unwrap().0 - self.index;
                    instruction.insert(
                        "Ranges",
                        InstructionValues::Ranges({
                            ranges.insert(
                                "Value",
                                RangesValues::Value(
                                    (value_column..value_column + value.len()).collect(),
                                ),
                            );
                            ranges.clone()
                        }),
                    );
                } else {
                    instruction.insert("Type", InstructionValues::Type("Name"));
                }
            } else if capture.at(LIST_ITEM_OPERATOR_INDEX).is_some() {
                info!("moved here");
                instruction.insert("Type", InstructionValues::Type("LIST_ITEM"));
                let operator_column =
                    capture.pos(LIST_ITEM_OPERATOR_INDEX).unwrap().0 - self.index;
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
                let value = capture.at(LIST_ITEM_VALUE_INDEX).unwrap();
                instruction.insert("Value", InstructionValues::Value(value));
                if capture.at(LIST_ITEM_VALUE_INDEX).is_some() {
                    let value_column =
                        capture.pos(LIST_ITEM_VALUE_INDEX).unwrap().0 - self.index;
                    instruction.insert(
                        "Ranges",
                        InstructionValues::Ranges({
                            ranges.insert(
                                "Value",
                                RangesValues::Value(
                                    (value_column..(value_column + value.len())).collect(),
                                ),
                            );
                            ranges.clone()
                        }),
                    );
                }
            } else if capture.at(FIELDSET_ENTRY_OPERATOR_INDEX).is_some() {
                info!("moved here");
                let unescaped_name = capture.at(NAME_UNESCAPED_INDEX).unwrap();
                if capture.at(NAME_UNESCAPED_INDEX).is_some() {
                    instruction.insert("Name", InstructionValues::Name(unescaped_name));
                    let name_column =
                        capture.pos(NAME_UNESCAPED_INDEX).unwrap().0 - self.index;
                    let entry_operator_column =
                        capture.pos(FIELDSET_ENTRY_OPERATOR_INDEX).unwrap().0 - self.index;

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
                                    (name_column..(name_column + unescaped_name.len()))
                                        .collect(),
                                ),
                            );
                            ranges.clone()
                        }),
                    );
                } else {
                    let escaped_name = capture.at(NAME_ESCAPED_INDEX).unwrap();
                    instruction.insert("Name", InstructionValues::Name(escaped_name));

                    let escape_operator = capture.at(NAME_ESCAPE_BEGIN_OPERATOR_INDEX).unwrap();
                    let escape_begin_operator_column = capture
                        .pos(NAME_ESCAPE_BEGIN_OPERATOR_INDEX)
                        .unwrap()
                        .0
                        - self.index;
                    let name_column = capture.pos(NAME_ESCAPED_INDEX).unwrap().0 - self.index;
                    let escape_end_operator_column =
                        capture.pos(NAME_ESCAPE_END_OPERATOR_INDEX).unwrap().0 - self.index;
                    let entry_operator_column =
                        capture.pos(FIELDSET_ENTRY_OPERATOR_INDEX).unwrap().0 - self.index;

                    instruction.insert(
                        "Ranges",
                        InstructionValues::Ranges({
                            ranges.insert(
                                "EscapeBeginOperator",
                                RangesValues::EscapeBeginOperator(
                                    (escape_begin_operator_column
                                        ..(escape_begin_operator_column
                                        + escape_operator.len()))
                                        .collect(),
                                ),
                            );
                            ranges.insert(
                                "EscapeEndOperator",
                                RangesValues::EscapeEndOperator(
                                    (escape_end_operator_column
                                        ..(escape_end_operator_column
                                        + escape_operator.len()))
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
                                    (name_column..(name_column + escaped_name.len()))
                                        .collect(),
                                ),
                            );
                            ranges.clone()
                        }),
                    );
                }
                instruction.insert("Type", InstructionValues::Type("FIELDSET_ENTRY"));
                let value = capture.at(FIELDSET_ENTRY_VALUE_INDEX).unwrap();
                instruction.insert("Value", InstructionValues::Value(value));
                if capture.at(FIELDSET_ENTRY_VALUE_INDEX).is_some() {
                    let value_column =
                        capture.pos(FIELDSET_ENTRY_VALUE_INDEX).unwrap().0 - self.index;
                    instruction.insert(
                        "Ranges",
                        InstructionValues::Ranges({
                            ranges.insert(
                                "Value",
                                RangesValues::Value(
                                    (value_column..(value_column + value.len())).collect(),
                                ),
                            );
                            ranges.clone()
                        }),
                    );
                }
            } else if capture.at(LINE_CONTINUATION_OPERATOR_INDEX).is_some() {
                info!("moved here");
                instruction.insert("Separator", InstructionValues::Separator(" "));
                instruction.insert("Type", InstructionValues::Type("CONTINUATION"));

                let value = capture.at(LINE_CONTINUATION_VALUE_INDEX).unwrap();
                instruction.insert("Value", InstructionValues::Value(value));

                let operator_column = capture
                    .pos(LINE_CONTINUATION_OPERATOR_INDEX)
                    .unwrap()
                    .0 - self.index;
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
                if capture.at(LINE_CONTINUATION_VALUE_INDEX).is_some() {
                    let value_column =
                        capture.pos(LINE_CONTINUATION_VALUE_INDEX).unwrap().0 - self.index;
                    instruction.insert(
                        "Ranges",
                        InstructionValues::Ranges({
                            ranges.insert(
                                "Value",
                                RangesValues::Value(
                                    (value_column..(value_column + value.len())).collect(),
                                ),
                            );
                            ranges.clone()
                        }),
                    );
                }
            } else if capture.at(NEWLINE_CONTINUATION_OPERATOR_INDEX).is_some() {
                info!("moved here");
                instruction.insert("Separator", InstructionValues::Separator("\n"));
                instruction.insert("Type", InstructionValues::Type("CONTINUATION"));

                let value = capture.at(NEWLINE_CONTINUATION_VALUE_INDEX).unwrap();
                instruction.insert("Value", InstructionValues::Value(value));

                let operator_column = capture
                    .pos(NEWLINE_CONTINUATION_OPERATOR_INDEX)
                    .unwrap()
                    .0 - self.index;
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
                if capture.at(NEWLINE_CONTINUATION_VALUE_INDEX).is_some() {
                    let value_column = capture
                        .pos(NEWLINE_CONTINUATION_VALUE_INDEX)
                        .unwrap()
                        .0 - self.index;
                    instruction.insert(
                        "Ranges",
                        InstructionValues::Ranges({
                            ranges.insert(
                                "Value",
                                RangesValues::Value(
                                    (value_column..(value_column + value.len())).collect(),
                                ),
                            );
                            ranges.clone()
                        }),
                    );
                }
            } else if capture.at(SECTION_HASHES_INDEX).is_some() {
                info!("moved here");
                let section_operator = capture.at(SECTION_HASHES_INDEX).unwrap();
                instruction.insert(
                    "Depth",
                    InstructionValues::Depth(section_operator.len()),
                );
                instruction.insert("Type", InstructionValues::Type("SECTION"));
                let section_operator_column =
                    capture.pos(SECTION_HASHES_INDEX).unwrap().0 - self.index;
                let unescaped_name = capture.at(SECTION_NAME_UNESCAPED_INDEX).unwrap();

                if capture.pos(SECTION_NAME_UNESCAPED_INDEX).is_some() {
                    instruction.insert("Name", InstructionValues::Name(unescaped_name));
                    let name_column =
                        capture.pos(SECTION_NAME_UNESCAPED_INDEX).unwrap().0 - self.index;
                    let name_end_column = name_column + unescaped_name.len();
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
                                        + section_operator.len()))
                                        .collect(),
                                ),
                            );
                            ranges.clone()
                        }),
                    );
                } else {
                    let escaped_name = capture.at(SECTION_NAME_ESCAPED_INDEX).unwrap();
                    instruction.insert("Name", InstructionValues::Name(escaped_name));

                    let escape_operator = capture
                        .at(SECTION_NAME_ESCAPE_BEGIN_OPERATOR_INDEX)
                        .unwrap();
                    let escape_begin_operator_column = capture
                        .pos(SECTION_NAME_ESCAPE_BEGIN_OPERATOR_INDEX)
                        .unwrap()
                        .0
                        - self.index;
                    let name_column =
                        capture.pos(SECTION_NAME_ESCAPED_INDEX).unwrap().0 - self.index;
                    let escape_end_operator_column = capture
                        .pos(SECTION_NAME_ESCAPE_END_OPERATOR_INDEX)
                        .unwrap()
                        .0 - self.index;
                    let _name_end_column = escape_end_operator_column + escape_operator.len();

                    instruction.insert(
                        "Ranges",
                        InstructionValues::Ranges({
                            ranges.insert(
                                "EscapeBeginOperator",
                                RangesValues::EscapeBeginOperator(
                                    (escape_begin_operator_column
                                        ..(escape_begin_operator_column
                                        + escape_operator.len()))
                                        .collect(),
                                ),
                            );
                            ranges.insert(
                                "EscapeEndOperator",
                                RangesValues::EscapeEndOperator(
                                    (escape_end_operator_column
                                        ..(escape_end_operator_column
                                        + escape_operator.len()))
                                        .collect(),
                                ),
                            );
                            ranges.insert(
                                "Name",
                                RangesValues::Name(
                                    (name_column..(name_column + escaped_name.len()))
                                        .collect(),
                                ),
                            );
                            ranges.insert(
                                "SectionOperator",
                                RangesValues::SectionOperator(
                                    (section_operator_column
                                        ..(section_operator_column
                                        + section_operator.len()))
                                        .collect(),
                                ),
                            );
                            ranges.clone()
                        }),
                    );
                }
                let template = capture.at(SECTION_TEMPLATE_INDEX).unwrap();

                if capture.at(SECTION_TEMPLATE_INDEX).is_some() {
                    instruction.insert("Template", InstructionValues::Template(template));

                    let copy_operator = capture.at(SECTION_COPY_OPERATOR_INDEX).unwrap();
                    let copy_operator_column =
                        capture.pos(SECTION_COPY_OPERATOR_INDEX).unwrap().0 - self.index;
                    let template_column =
                        capture.pos(SECTION_TEMPLATE_INDEX).unwrap().0 - self.index;

                    if copy_operator == "<" {
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
                                    (template_column..(template_column + template.len()))
                                        .collect(),
                                ),
                            );
                            ranges.clone()
                        }),
                    );
                }
            } else if capture.at(BLOCK_DASHES_INDEX).is_some() {
                info!("moved here");
                let operator = capture.at(BLOCK_DASHES_INDEX).unwrap();
                let name = capture.at(BLOCK_NAME_INDEX).unwrap();

                instruction.insert("Name", InstructionValues::Name(name));
                instruction.insert("Type", InstructionValues::Type("BLOCK"));

                let operator_column = capture.pos(BLOCK_DASHES_INDEX).unwrap().0 - self.index;
                let name_column = capture.pos(BLOCK_NAME_INDEX).unwrap().0 - self.index;
                instruction.insert(
                    "Length",
                    InstructionValues::Length(r.unwrap().1 - self.index),
                );

                instruction.insert(
                    "Ranges",
                    InstructionValues::Ranges({
                        ranges.insert(
                            "Template",
                            RangesValues::Template(
                                (operator_column..(operator_column + operator.len()))
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
                                (operator_column..(operator_column + operator.len()))
                                    .collect(),
                            ),
                        );
                        ranges.insert(
                            "Name",
                            RangesValues::Name(
                                (name_column..(name_column + name.len())).collect(),
                            ),
                        );
                        ranges.clone()
                    }),
                );
                self.context
                    .get_mut("Instructions")
                    .unwrap()
                    .get_instruction_mut()
                    .unwrap()
                    .push_back(instruction.clone());

                self.index = r.unwrap().1;

                let terminator_str = format!("\\n[^\\S\\n]*({})[^\\S\\n]*({})[^\\S\\n]*(?=\\n|$)",operator, regex::escape(name));
                let terminator_re = onig::Regex::new(terminator_str.as_str()).unwrap();
                //TODO start match from self.index
                let terminator_match = terminator_re.find_at(self.input, self.index).unwrap();
                let terminator_capture = terminator_re.captures_at(self.input, self.index).unwrap();

                self.index += 1;
                self.line += 1;

                if terminator_re.find_at(self.input, self.index).is_some() {
                    self.tokenize_error_context();
                    Tokenization::error_msg("unterminated_block", &self.context, &instruction);
                }

                let end_of_block_index = terminator_match.0;
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
                    InstructionValues::Length(terminator_match.1 - self.index),
                );
                instruction.insert("Line", InstructionValues::Line(self.line));
                instruction.insert(
                    "Ranges",
                    InstructionValues::Ranges({
                        ranges.insert(
                            "BlockOperator",
                            RangesValues::BlockOperator(
                                ((terminator_capture.pos(1).unwrap().0 - self.index)
                                    ..(terminator_capture.pos(1).unwrap().1 - self.index))
                                    .collect(),
                            ),
                        );
                        ranges.insert(
                            "Name",
                            RangesValues::BlockOperator(
                                ((terminator_capture.pos(2).unwrap().0 - self.index)
                                    ..(terminator_capture.pos(2).unwrap().1 - self.index))
                                    .collect(),
                            ),
                        );
                        ranges.clone()
                    }),
                );
                instruction.insert("Type", InstructionValues::Type("BLOCK_TERMINATOR"));

                self.instructions.push_back(instruction);

                self.index = terminator_match.1 + 1;
                self.line += 1;

                block = true;
            } else if capture.at(COMMENT_OPERATOR_INDEX).is_some() {
                info!("moved here");
                let comment = capture.at(COMMENT_TEXT_INDEX);
                let comment_operator_column =
                    capture.pos(COMMENT_OPERATOR_INDEX).unwrap().0 - self.index;

                instruction.insert("Type", InstructionValues::Type("COMMENT"));
                instruction.insert(
                    "Comment",
                    InstructionValues::Comment(comment.unwrap()),
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
                        capture.pos(COMMENT_TEXT_INDEX).unwrap().0 - self.index;
                    instruction.insert(
                        "Ranges",
                        InstructionValues::Ranges({
                            ranges.insert(
                                "Comment",
                                RangesValues::Comment(
                                    (comment_column
                                        ..(comment_column + comment.unwrap().len()))
                                        .collect(),
                                ),
                            );
                            ranges.clone()
                        }),
                    );
                }
            } else if capture.at(COPY_OPERATOR_INDEX).is_some() {
                info!("moved here");
                let operator = capture.at(COPY_OPERATOR_INDEX);
                let template = capture.at(TEMPLATE_INDEX);
                let unescaped_name = capture.at(NAME_UNESCAPED_INDEX);

                if unescaped_name.is_some() {
                    instruction.insert(
                        "Name",
                        InstructionValues::Name(unescaped_name.unwrap()),
                    );

                    let name_column =
                        capture.pos(NAME_UNESCAPED_INDEX).unwrap().0 - self.index;
                    let operator_column =
                        capture.pos(COPY_OPERATOR_INDEX).unwrap().0 - self.index;

                    instruction.insert(
                        "Ranges",
                        InstructionValues::Ranges({
                            ranges.insert(
                                "CopyOperator",
                                RangesValues::CopyOperator(
                                    (operator_column
                                        ..operator_column + operator.unwrap().len())
                                        .collect(),
                                ),
                            );
                            ranges.insert(
                                "Name",
                                RangesValues::Name(
                                    (name_column
                                        ..name_column + unescaped_name.unwrap().len())
                                        .collect(),
                                ),
                            );
                            ranges.clone()
                        }),
                    );
                } else {
                    let escaped_name = capture.at(NAME_ESCAPED_INDEX);
                    instruction.insert(
                        "Name",
                        InstructionValues::Name(escaped_name.unwrap()),
                    );

                    let escape_operator = capture.at(NAME_ESCAPE_BEGIN_OPERATOR_INDEX);
                    let escape_begin_operator_column = capture
                        .pos(NAME_ESCAPE_BEGIN_OPERATOR_INDEX)
                        .unwrap()
                        .0
                        - self.index;
                    let name_column = capture.pos(NAME_ESCAPED_INDEX).unwrap().0 - self.index;
                    let escape_end_operator_column =
                        capture.pos(NAME_ESCAPE_END_OPERATOR_INDEX).unwrap().0 - self.index;
                    let operator_column =
                        capture.pos(COPY_OPERATOR_INDEX).unwrap().0 - self.index;

                    instruction.insert(
                        "Ranges",
                        InstructionValues::Ranges({
                            ranges.insert(
                                "CopyOperator",
                                RangesValues::CopyOperator(
                                    (operator_column
                                        ..(operator_column + operator.unwrap().len()))
                                        .collect(),
                                ),
                            );
                            ranges.insert(
                                "EscapeBeginOperator",
                                RangesValues::EscapeBeginOperator(
                                    (escape_begin_operator_column
                                        ..(escape_begin_operator_column
                                        + escape_operator.unwrap().len()))
                                        .collect(),
                                ),
                            );
                            ranges.insert(
                                "EscapeEndOperator",
                                RangesValues::EscapeEndOperator(
                                    (escape_end_operator_column
                                        ..(escape_end_operator_column
                                        + escape_operator.unwrap().len()))
                                        .collect(),
                                ),
                            );
                            ranges.insert(
                                "Name",
                                RangesValues::Name(
                                    (name_column
                                        ..(name_column + escaped_name.unwrap().len()))
                                        .collect(),
                                ),
                            );
                            ranges.clone()
                        }),
                    );
                }

                instruction.insert(
                    "Template",
                    InstructionValues::Template(template.unwrap()),
                );
                instruction.insert("Type", InstructionValues::Type("NAME"));

                let template_column = capture.pos(TEMPLATE_INDEX).unwrap().0 - self.index;
                instruction.insert(
                    "Ranges",
                    InstructionValues::Ranges({
                        ranges.insert(
                            "Template",
                            RangesValues::Template(
                                (template_column
                                    ..template_column + template.unwrap().len())
                                    .collect(),
                            ),
                        );
                        ranges.clone()
                    }),
                );

                if !(block) {
                    instruction.insert(
                        "Length",
                        InstructionValues::Length(r.unwrap().1 - self.index),
                    );
                    self.index = r.unwrap().1 + 1;
                    self.context
                        .get_mut("Instructions")
                        .unwrap()
                        .get_instruction_mut()
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
                                .get_mut("Instructions")
                                .unwrap()
                                .get_instruction_mut()
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
                        val.get_instruction_mut()
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
                        val.get_instruction_mut().unwrap().push_back(instruction.clone());
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