
pub const OPTIONAL: &'static str = "([^\\n]+?)?";
pub const REQUIRED: &'static str = "(\\S[^\\n]*?)";

//
pub const EMPTY: &'static str = "()";
pub const EMPTY_LINE_INDEX: usize = 1;

// | Value
pub const NEWLINE_CONTINUATION_OPERATOR_INDEX: usize = 2;
pub const NEWLINE_CONTINUATION_VALUE_INDEX: usize = 3;

// \ Value
pub const LINE_CONTINUATION_OPERATOR_INDEX: usize = 4;
pub const LINE_CONTINUATION_VALUE_INDEX: usize = 5;


// > Comment
pub const COMMENT_OPERATOR_INDEX: usize = 6;
pub const COMMENT_TEXT_INDEX: usize = 7;

// - Value
pub const LIST_ITEM_OPERATOR_INDEX: usize = 8;
pub const LIST_ITEM_VALUE_INDEX: usize = 9;
// -- Name
pub const BLOCK_DASHES_INDEX: usize = 10;
pub const BLOCK_NAME_INDEX: usize = 11;

// #
pub const SECTION_HASHES: &'static str = "(#+)(?!#)";
pub const SECTION_HASHES_INDEX: usize = 12;

// # Name
pub const SECTION_NAME_UNESCAPED: &'static str = "(?!`)([^\\s<][^<\\n]*?)";
pub const SECTION_NAME_UNESCAPED_INDEX: usize = 13;

// # `Name`
pub const SECTION_NAME_ESCAPE_BEGIN_OPERATOR_INDEX: usize = 14;
//# TODO: Should this exclude the backreference inside the quotes? (as in ((?:(?!\\1).)+) ) here and elsewhere (probably not because it's not greedy.?)
pub const SECTION_NAME_ESCAPED_INDEX: usize = 15;
pub const SECTION_NAME_ESCAPE_END_OPERATOR_INDEX: usize = 16;

//# Name < Template
// # `Escaped section name` < Template
pub const SECTION_COPY_OPERATOR_INDEX: usize = 17;
pub const SECTION_TEMPLATE_INDEX: usize = 18;


// Name
pub const NAME_UNESCAPED: &'static str = "(?![>#\\-`\\\\|])([^\\s:=<][^:=<]*?)";
pub const NAME_UNESCAPED_INDEX: usize = 19;

//`Name`
pub const NAME_ESCAPE_BEGIN_OPERATOR_INDEX: usize = 20;
pub const NAME_ESCAPED_INDEX: usize = 21;
pub const NAME_ESCAPE_END_OPERATOR_INDEX: usize = 22;


// :
// : Value
pub const NAME_OPERATOR_INDEX: usize = 23;
pub const FIELD_VALUE_INDEX: usize = 24;

// =
// = Value
pub const FIELDSET_ENTRY_OPERATOR_INDEX: usize = 25;
pub const FIELDSET_ENTRY_VALUE_INDEX: usize = 26;

// < Template
// << Template
pub const COPY_OPERATOR_INDEX: usize = 27;
pub const TEMPLATE_INDEX: usize = 28;


pub struct GrammarRegex {
    pub new_line_continuation: String,
    pub line_continuation: String,
    pub continuation: String,
    pub comment: String,
    pub list_item: String,
    pub  block: String,
    pub section_name: String,
    pub section_name_escaped: String,
    pub  section_template: String,
    pub section: String,
    pub early_determined: String,
    pub  name_escaped: String,
    pub  name: String,
    pub  field_or_name: String,
    pub  field_set_entry: String,
    pub  copy: String,
    pub  late_determined: String,
    pub  not_empty: String,
    pub  regex: String
}

impl GrammarRegex{
    pub fn initialize() -> GrammarRegex{
        // | Value
        let new_line_continuation = format!("(\\|)[^\\S\\n]*{}", OPTIONAL);
        // \ Value
        let line_continuation = format!("(\\\\)[^\\S\\n]*{}", OPTIONAL);
        let continuation = format!("{}|{}", new_line_continuation, line_continuation);
        // # `Name`
        let section_name_escaped = format!("(`+)[^\\S\\n]*(\\S[^\\n]*?)[^\\S\\n]*(\\{})", SECTION_NAME_ESCAPE_BEGIN_OPERATOR_INDEX);
        //# Name < Template
        // # `Escaped section name` < Template
        let section_template = format!("(?:(<(?!<)|<<)[^\\S\\n]*{})?", REQUIRED);
        let section_name = format!("(?:{}|{})", SECTION_NAME_UNESCAPED, section_name_escaped);
        let section = format!("{}\\s*{}[^\\S\\n]*{}", SECTION_HASHES, section_name, section_template);
        // > Comment
        let comment = format!("(>)[^\\S\\n]*{}", OPTIONAL);
        // - Value
        let list_item = format!("(-)(?!-)[^\\S\\n]*{}", OPTIONAL);
        // -- Name
        let block =  format!("(-{{2,}})[^\\S\\n]*{}", REQUIRED);
        let early_determined = format!("{}|{}|{}|{}|{}", continuation, comment, list_item, block, section);
        //`Name`
        let name_escaped = format!("(`+)[^\\S\\n]*(\\S[^\\n]*?)[^\\S\\n]*(\\{})", NAME_ESCAPE_BEGIN_OPERATOR_INDEX);
        let name = format!("(?:{}|{})", name_escaped, NAME_ESCAPED_INDEX);
        // :
        // : Value
        let field_or_name = format!("(:)[^\\S\\n]*{}", OPTIONAL);
        // =
        // = Value
        let field_set_entry  = format!("(=)[^\\S\\n]*{}", OPTIONAL);
        // < Template
        // << Template
        let copy = format!("(<(?!<)|<<)\\s*{}", REQUIRED);
        let late_determined = format!("{}\\s*(?:{}|{}|{})", name, field_or_name,field_set_entry,copy);
        let not_empty = format!("(?:{}|{})", early_determined, late_determined);

        let regex = format!("[^\\S\\n]*(?:{}|{})[^\\S\\n]*(?=\\n|$)", EMPTY, not_empty);
        GrammarRegex { new_line_continuation, line_continuation, continuation, comment, list_item, block,
            section_name_escaped, section_template, section_name, section, early_determined, name_escaped, name,
            field_or_name, field_set_entry, copy, late_determined, not_empty, regex
        }
    }
}
