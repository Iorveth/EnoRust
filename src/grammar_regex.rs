pub const REGEX: &'static str = "[^\\S\\n]*(?:{EMPTY}|{NOT_EMPTY})[^\\S\\n]*(?=\\n|$)";
//

pub const OPTIONAL: &'static str = "([^\\n]+?)?";
pub const REQUIRED: &'static str = "(\\S[^\\n]*?)";

//
pub const EMPTY: &'static str = "()";
pub const EMPTY_LINE_INDEX: usize = 1;

// | Value
pub const NEWLINE_CONTINUATION: &'static str = "(\\|)[^\\S\\n]*{OPTIONAL}";
pub const NEWLINE_CONTINUATION_OPERATOR_INDEX: usize = 2;
pub const NEWLINE_CONTINUATION_VALUE_INDEX: usize = 3;

// \ Value
pub const LINE_CONTINUATION: &'static str = "(\\\\)[^\\S\\n]*{OPTIONAL}";
pub const LINE_CONTINUATION_OPERATOR_INDEX: usize = 4;
pub const LINE_CONTINUATION_VALUE_INDEX: usize = 5;

pub const CONTINUATION: &'static str = "{NEWLINE_CONTINUATION}|{LINE_CONTINUATION}";

// > Comment
pub const COMMENT: &'static str = "(>)[^\\S\\n]*{OPTIONAL}";
pub const COMMENT_OPERATOR_INDEX: usize = 6;
pub const COMMENT_TEXT_INDEX: usize = 7;

// - Value
pub const LIST_ITEM: &'static str = "(-)(?!-)[^\\S\\n]*{OPTIONAL}";
pub const LIST_ITEM_OPERATOR_INDEX: usize = 8;
pub const LIST_ITEM_VALUE_INDEX: usize = 9;
// -- Name
pub const BLOCK: &'static str = "(-{{2,}})[^\\S\\n]*{REQUIRED}";
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
pub const SECTION_NAME_ESCAPED: &'static str =
    "(`+)[^\\S\\n]*(\\S[^\\n]*?)[^\\S\\n]*(\\{SECTION_NAME_ESCAPE_BEGIN_OPERATOR_INDEX})";
pub const SECTION_NAME_ESCAPED_INDEX: usize = 15;
pub const SECTION_NAME_ESCAPE_END_OPERATOR_INDEX: usize = 16;

//# Name < Template
// # `Escaped section name` < Template
pub const SECTION_NAME: &'static str = "(?:{SECTION_NAME_UNESCAPED}|{SECTION_NAME_ESCAPED})";
pub const SECTION_TEMPLATE: &'static str = "(?:(<(?!<)|<<)[^\\S\\n]*{REQUIRED})?";
pub const SECTION: &'static str = "{SECTION_HASHES}\\s*{SECTION_NAME}[^\\S\\n]*{SECTION_TEMPLATE}";
pub const SECTION_COPY_OPERATOR_INDEX: usize = 17;
pub const SECTION_TEMPLATE_INDEX: usize = 18;

pub const EARLY_DETERMINED: &'static str = "{CONTINUATION}|{COMMENT}|{LIST_ITEM}|{BLOCK}|{SECTION}";

// Name
pub const NAME_UNESCAPED: &'static str = "(?![>#\\-`\\\\|])([^\\s:=<][^:=<]*?)";
pub const NAME_UNESCAPED_INDEX: usize = 19;

//`Name`
pub const NAME_ESCAPE_BEGIN_OPERATOR_INDEX: usize = 20;
pub const NAME_ESCAPED: &'static str =
    "(`+)[^\\S\\n]*(\\S[^\\n]*?)[^\\S\\n]*(\\{NAME_ESCAPE_BEGIN_OPERATOR_INDEX})";
pub const NAME_ESCAPED_INDEX: usize = 21;
pub const NAME_ESCAPE_END_OPERATOR_INDEX: usize = 22;

pub const NAME: &'static str = "(?:{NAME_UNESCAPED}|{NAME_ESCAPED})";

// :
// : Value
pub const FIELD_OR_NAME: &'static str = "(:)[^\\S\\n]*{OPTIONAL}";
pub const NAME_OPERATOR_INDEX: usize = 23;
pub const FIELD_VALUE_INDEX: usize = 24;

// =
// = Value
pub const FIELDSET_ENTRY: &'static str = "(=)[^\\S\\n]*{OPTIONAL}";
pub const FIELDSET_ENTRY_OPERATOR_INDEX: usize = 25;
pub const FIELDSET_ENTRY_VALUE_INDEX: usize = 26;

// < Template
// << Template
pub const COPY: &'static str = "(<(?!<)|<<)\\s*{REQUIRED}";
pub const COPY_OPERATOR_INDEX: usize = 27;
pub const TEMPLATE_INDEX: usize = 28;

pub const LATE_DETERMINED: &'static str = "{NAME}\\s*(?:{FIELD_OR_NAME}|{FIELDSET_ENTRY}|{COPY})";

pub const NOT_EMPTY: &'static str = "(?:{EARLY_DETERMINED}|{LATE_DETERMINED})";
