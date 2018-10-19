use parser::ContextValues;
use regex::Regex;
use std::collections::HashMap;
use tokenizer::InstructionValues;

#[derive(Clone, Copy, Debug)]
pub struct Messages {
    elements: Elements,
    tokenization: Tokenization,
    analysis: Analysis,
    resolution: Resolution,
    validation: Validation,
    loaders: Loaders,
}
#[derive(Clone, Copy, Debug)]
pub struct Elements {
    document: &'static str,
    empty: &'static str,
    field: &'static str,
    fieldset: &'static str,
    fieldset_entry: &'static str,
    list: &'static str,
    list_item: &'static str,
    section: &'static str,
    value: &'static str,
}

pub const UNTERMINATED_ESCAPED_NAME: &'static str = "^\\s*(`+)(?!`)((?:(?!\\1).)+)$";
#[derive(Clone, Copy, Debug)]
pub struct Tokenization {
    invalid_line: &'static str,
    unterminated_block: &'static str,
    unterminated_escaped_name: &'static str,
}

#[derive(Clone, Copy, Debug)]
pub struct Analysis {
    duplicate_fieldset_entry_name: &'static str,
    fieldset_entry_in_field: &'static str,
    fieldset_entry_in_list: &'static str,
    list_item_in_field: &'static str,
    list_item_in_fieldset: &'static str,
    missing_element_for_continuation: &'static str,
    missing_name_for_fieldset_entry: &'static str,
    missing_name_for_list_item: &'static str,
    section_hierarchy_layer_skip: &'static str,
}
#[derive(Clone, Copy, Debug)]
pub struct Resolution {
    copying_block_into_fieldset: &'static str,
    copying_block_into_list: &'static str,
    copying_block_into_section: &'static str,
    copying_field_into_fieldset: &'static str,
    copying_field_into_list: &'static str,
    copying_field_into_section: &'static str,
    copying_fieldset_into_field: &'static str,
    copying_fieldset_into_list: &'static str,
    copying_fieldset_into_section: &'static str,
    copying_list_into_field: &'static str,
    copying_list_into_fieldset: &'static str,
    copying_list_into_section: &'static str,
    copying_section_into_empty: &'static str,
    copying_section_into_field: &'static str,
    copying_section_into_fieldset: &'static str,
    copying_section_into_list: &'static str,
    cyclic_dependency: &'static str,
    multiple_templates_found: &'static str,
    template_not_found: &'static str,
}
#[derive(Clone, Copy, Debug)]
pub struct Validation {
    exact_count_not_met: &'static str,
    excess_name: &'static str,
    expected_element_got_elements: &'static str,
    expected_field_got_fields: &'static str,
    expected_field_got_fieldset: &'static str,
    expected_field_got_list: &'static str,
    expected_field_got_section: &'static str,
    expected_fields_got_fieldset: &'static str,
    expected_fields_got_list: &'static str,
    expected_fields_got_section: &'static str,
    expected_fieldset_got_field: &'static str,
    expected_fieldset_got_fieldsets: &'static str,
    expected_fieldset_got_list: &'static str,
    expected_fieldset_got_section: &'static str,
    expected_fieldsets_got_field: &'static str,
    expected_fieldsets_got_list: &'static str,
    expected_fieldsets_got_section: &'static str,
    expected_list_got_field: &'static str,
    expected_list_got_fieldset: &'static str,
    expected_list_got_lists: &'static str,
    expected_list_got_section: &'static str,
    expected_lists_got_field: &'static str,
    expected_lists_got_fieldset: &'static str,
    expected_lists_got_section: &'static str,
    expected_section_got_empty: &'static str,
    expected_section_got_field: &'static str,
    expected_section_got_fieldset: &'static str,
    expected_section_got_list: &'static str,
    expected_section_got_sections: &'static str,
    expected_sections_got_empty: &'static str,
    expected_sections_got_field: &'static str,
    expected_sections_got_fieldset: &'static str,
    expected_sections_got_list: &'static str,
    generic_error: &'static str,
    max_count_not_met: &'static str,
    min_count_not_met: &'static str,
    missing_element: &'static str,
    missing_field: &'static str,
    missing_field_value: &'static str,
    missing_fieldset: &'static str,
    missing_fieldset_entry: &'static str,
    missing_fieldset_entry_value: &'static str,
    missing_list: &'static str,
    missing_list_item_value: &'static str,
    missing_section: &'static str,
}
#[derive(Clone, Copy, Debug)]
pub struct Loaders {
    invalid_boolean: &'static str,
    invalid_color: &'static str,
    invalid_date: &'static str,
    invalid_datetime: &'static str,
    invalid_email: &'static str,
    invalid_float: &'static str,
    invalid_integer: &'static str,
    invalid_json: &'static str,
    invalid_lat_lng: &'static str,
    invalid_url: &'static str,
}

impl Elements {
    pub fn msg(el: Elements, el_name: &'static str) -> &'static str {
        match el_name {
            "document" => el.document,
            "empty" => el.empty,
            "field" => el.field,
            "fieldset" => el.fieldset,
            "fieldset_entry" => el.fieldset_entry,
            "list" => el.list,
            "list_item" => el.list_item,
            "section" => el.section,
            "value" => el.value,
            _ => "Error",
        }
    }
}

impl Tokenization {
    pub fn error_msg(
        el_name: &'static str,
        context: &HashMap<&'static str, ContextValues>,
        instruction: &HashMap<&'static str, InstructionValues>,
    ) -> String {
        match el_name {
            "invalid_line" => {
                let line = &context.get("Input").unwrap().get_str().unwrap()
                    [*instruction.get("Index").unwrap().get_numeric().unwrap()
                        ..(instruction.get("Index").unwrap().get_numeric().unwrap()
                            + instruction.get("Length").unwrap().get_numeric().unwrap())];
                //match() instead of search()
                let reg = Regex::new(UNTERMINATED_ESCAPED_NAME).unwrap().find(line);

                if reg.is_some() {
                    return unterminated_escaped_name(context, instruction);
                }
                return rt_format!(
                    context
                        .get("Messages")
                        .unwrap()
                        .get_messages()
                        .unwrap()
                        .tokenization
                        .invalid_line,
                    (instruction.get("Line").unwrap().get_numeric().unwrap()
                        + context.get("Indexing").unwrap().get_indexing().unwrap())
                ).unwrap();
            }
            "unterminated_escaped_name" => unterminated_escaped_name(context, instruction),
            "unterminated_block" => rt_format!(
                context
                    .get("Messages")
                    .unwrap()
                    .get_messages()
                    .unwrap()
                    .tokenization
                    .unterminated_block,
                instruction.get("Name").unwrap().get_str().unwrap(),
                (instruction.get("Line").unwrap().get_numeric().unwrap()
                    + context.get("Indexing").unwrap().get_indexing().unwrap())
            ).unwrap(),
            _ => "Error".to_string(),
        }
    }
}

pub fn unterminated_escaped_name(
    context: &HashMap<&'static str, ContextValues>,
    instruction: &HashMap<&'static str, InstructionValues>,
) -> String {
    rt_format!(
        context
            .get("Messages")
            .unwrap()
            .get_messages()
            .unwrap()
            .tokenization
            .unterminated_escaped_name,
        (instruction.get("Line").unwrap().get_numeric().unwrap()
            + context.get("Indexing").unwrap().get_indexing().unwrap())
    ).unwrap()
}

impl Analysis {
    pub fn msg_with_one_value(el: Analysis, el_name: &'static str, line: i32) -> String {
        match el_name {
            "fieldset_entry_in_field" => rt_format!(el.fieldset_entry_in_field, line).unwrap(),
            "fieldset_entry_in_list" => rt_format!(el.fieldset_entry_in_list, line).unwrap(),
            "list_item_in_field" => rt_format!(el.list_item_in_field, line).unwrap(),
            "list_item_in_fieldset" => rt_format!(el.list_item_in_fieldset, line).unwrap(),
            "missing_element_for_continuation" => {
                rt_format!(el.missing_element_for_continuation, line).unwrap()
            }
            "missing_name_for_fieldset_entry" => {
                rt_format!(el.missing_name_for_fieldset_entry, line).unwrap()
            }
            "missing_name_for_list_item" => {
                rt_format!(el.missing_name_for_list_item, line).unwrap()
            }
            "section_hierarchy_layer_skip" => {
                rt_format!(el.section_hierarchy_layer_skip, line).unwrap()
            }
            _ => "Error".to_string(),
        }
    }
    pub fn msg_with_two_values(
        el: Analysis,
        el_name: &'static str,
        fieldset_name: &'static str,
        entry_name: &'static str,
    ) -> String {
        match el_name {
            "duplicate_fieldset_entry_name" => {
                rt_format!(el.duplicate_fieldset_entry_name, fieldset_name, entry_name).unwrap()
            }
            _ => "Error".to_string(),
        }
    }
}

impl Resolution {
    pub fn msg_with_one_value(el: Resolution, el_name: &'static str, line: i32) -> String {
        match el_name {
            "copying_block_into_fieldset" => {
                rt_format!(el.copying_block_into_fieldset, line).unwrap()
            }
            "copying_block_into_list" => rt_format!(el.copying_block_into_list, line).unwrap(),
            "copying_block_into_section" => {
                rt_format!(el.copying_block_into_section, line).unwrap()
            }
            "copying_field_into_fieldset" => {
                rt_format!(el.copying_field_into_fieldset, line).unwrap()
            }
            "copying_field_into_list" => rt_format!(el.copying_field_into_list, line).unwrap(),
            "copying_field_into_section" => {
                rt_format!(el.copying_field_into_section, line).unwrap()
            }
            "copying_fieldset_into_field" => {
                rt_format!(el.copying_fieldset_into_field, line).unwrap()
            }
            "copying_fieldset_into_list" => rt_format!(el.copying_field_into_list, line).unwrap(),
            "copying_fieldset_into_section" => {
                rt_format!(el.copying_field_into_section, line).unwrap()
            }
            "copying_list_into_field" => rt_format!(el.copying_list_into_field, line).unwrap(),
            "copying_list_into_fieldset" => {
                rt_format!(el.copying_list_into_fieldset, line).unwrap()
            }
            "copying_list_into_section" => rt_format!(el.copying_list_into_section, line).unwrap(),
            "copying_section_into_empty" => {
                rt_format!(el.copying_section_into_empty, line).unwrap()
            }
            "copying_section_into_field" => {
                rt_format!(el.copying_section_into_field, line).unwrap()
            }
            "copying_section_into_fieldset" => {
                rt_format!(el.copying_section_into_fieldset, line).unwrap()
            }
            "copying_section_into_list" => rt_format!(el.copying_section_into_list, line).unwrap(),
            _ => "Error".to_string(),
        }
    }
    pub fn msg_with_two_values(
        el: Resolution,
        el_name: &'static str,
        line: i32,
        name: &'static str,
    ) -> String {
        match el_name {
            "cyclic_dependency" => rt_format!(el.cyclic_dependency, line, name).unwrap(),
            "template_not_found" => rt_format!(el.template_not_found, line, name).unwrap(),
            "multiple_templates_found" => {
                rt_format!(el.multiple_templates_found, line, name).unwrap()
            }
            _ => "Error".to_string(),
        }
    }
}
impl Validation {
    pub fn msg_with_one_value(el: Validation, el_name: &'static str, name: &'static str) -> String {
        match el_name {
            "excess_name" => rt_format!(el.excess_name, name).unwrap(),
            "expected_element_got_elements" => {
                rt_format!(el.expected_element_got_elements, name).unwrap()
            }
            "expected_field_got_fields" => rt_format!(el.expected_field_got_fields, name).unwrap(),
            "expected_field_got_fieldset" => {
                rt_format!(el.expected_field_got_fieldset, name).unwrap()
            }
            "expected_field_got_list" => rt_format!(el.expected_field_got_list, name).unwrap(),
            "expected_field_got_section" => {
                rt_format!(el.expected_field_got_section, name).unwrap()
            }
            "expected_fields_got_fieldset" => {
                rt_format!(el.expected_fields_got_fieldset, name).unwrap()
            }
            "expected_fields_got_list" => rt_format!(el.expected_fields_got_list, name).unwrap(),
            "expected_fields_got_section" => {
                rt_format!(el.expected_field_got_section, name).unwrap()
            }
            "expected_fieldset_got_field" => {
                rt_format!(el.expected_fieldsets_got_field, name).unwrap()
            }
            "expected_fieldset_got_fieldsets" => {
                rt_format!(el.expected_fieldset_got_fieldsets, name).unwrap()
            }
            "expected_fieldset_got_list" => {
                rt_format!(el.expected_fieldset_got_list, name).unwrap()
            }
            "expected_fieldset_got_section" => {
                rt_format!(el.expected_fieldset_got_section, name).unwrap()
            }
            "expected_fieldsets_got_field" => {
                rt_format!(el.expected_fieldsets_got_field, name).unwrap()
            }
            "expected_fieldsets_got_list" => {
                rt_format!(el.expected_fieldsets_got_list, name).unwrap()
            }
            "expected_fieldsets_got_section" => {
                rt_format!(el.expected_fieldsets_got_section, name).unwrap()
            }
            "expected_list_got_field" => rt_format!(el.expected_list_got_field, name).unwrap(),
            "expected_list_got_fieldset" => {
                rt_format!(el.expected_list_got_fieldset, name).unwrap()
            }
            "expected_list_got_lists" => rt_format!(el.expected_list_got_lists, name).unwrap(),
            "expected_list_got_section" => rt_format!(el.expected_list_got_section, name).unwrap(),
            "expected_lists_got_field" => rt_format!(el.expected_lists_got_field, name).unwrap(),
            "expected_lists_got_fieldset" => {
                rt_format!(el.expected_lists_got_fieldset, name).unwrap()
            }
            "expected_lists_got_section" => {
                rt_format!(el.expected_lists_got_section, name).unwrap()
            }
            "expected_section_got_empty" => {
                rt_format!(el.expected_section_got_empty, name).unwrap()
            }
            "expected_section_got_field" => {
                rt_format!(el.expected_section_got_field, name).unwrap()
            }
            "expected_section_got_fieldset" => {
                rt_format!(el.expected_section_got_fieldset, name).unwrap()
            }
            "expected_section_got_list" => rt_format!(el.expected_section_got_list, name).unwrap(),
            "expected_section_got_sections" => {
                rt_format!(el.expected_section_got_sections, name).unwrap()
            }
            "expected_sections_got_empty" => {
                rt_format!(el.expected_sections_got_empty, name).unwrap()
            }
            "expected_sections_got_field" => {
                rt_format!(el.expected_sections_got_field, name).unwrap()
            }
            "expected_sections_got_fieldset" => {
                rt_format!(el.expected_sections_got_fieldset, name).unwrap()
            }
            "expected_sections_got_list" => {
                rt_format!(el.expected_sections_got_list, name).unwrap()
            }
            "generic_error" => rt_format!(el.generic_error, name).unwrap(),
            "missing_element" => rt_format!(el.missing_element, name).unwrap(),
            "missing_field" => rt_format!(el.missing_field, name).unwrap(),
            "missing_field_value" => rt_format!(el.missing_field_value, name).unwrap(),
            "missing_fieldset" => rt_format!(el.missing_fieldset, name).unwrap(),
            "missing_fieldset_entry" => rt_format!(el.missing_fieldset_entry, name).unwrap(),
            "missing_fieldset_entry_value" => {
                rt_format!(el.missing_fieldset_entry_value, name).unwrap()
            }
            "missing_list" => rt_format!(el.missing_list, name).unwrap(),
            "missing_list_item_value" => rt_format!(el.missing_list_item_value, name).unwrap(),
            "missing_section" => rt_format!(el.missing_section, name).unwrap(),
            _ => "Error".to_string(),
        }
    }
    pub fn msg_with_three_values(
        el: Validation,
        el_name: &'static str,
        name: &'static str,
        actual: i32,
        expected: i32,
    ) -> String {
        match el_name {
            "exact_count_not_met" => {
                rt_format!(el.exact_count_not_met, name, actual, expected).unwrap()
            }
            "max_count_not_met" => {
                rt_format!(el.max_count_not_met, name, actual, expected).unwrap()
            }
            "min_count_not_met" => {
                rt_format!(el.min_count_not_met, name, actual, expected).unwrap()
            }
            _ => "Error".to_string(),
        }
    }
}

impl Loaders {
    pub fn msg_with_one_value(el: Loaders, el_name: &'static str, name: &'static str) -> String {
        match el_name {
            "invalid_boolean" => rt_format!(el.invalid_boolean, name).unwrap(),
            "invalid_color" => rt_format!(el.invalid_color, name).unwrap(),
            "invalid_date" => rt_format!(el.invalid_date, name).unwrap(),
            "invalid_datetime" => rt_format!(el.invalid_datetime, name).unwrap(),
            "invalid_email" => rt_format!(el.invalid_email, name).unwrap(),
            "invalid_float" => rt_format!(el.invalid_float, name).unwrap(),
            "invalid_integer" => rt_format!(el.invalid_integer, name).unwrap(),
            "invalid_lat_lng" => rt_format!(el.invalid_lat_lng, name).unwrap(),
            "invalid_url" => rt_format!(el.invalid_url, name).unwrap(),
            _ => "Error".to_string(),
        }
    }
    pub fn masg_with_two_values(
        el: Loaders,
        el_name: &'static str,
        name: &'static str,
        error: &'static str,
    ) -> String {
        match el_name {
            "invalid_json" => rt_format!(el.invalid_json, name, error).unwrap(),
            _ => "Error".to_string(),
        }
    }
}
impl Messages {
    pub fn get_messages(locale: &'static str) -> Option<Messages> {
        match locale {
            "en" => Some(get_en_messages()),
            _ => None,
        }
    }
}

fn get_en_messages() -> Messages {
    let msg = Messages {
        elements: Elements {
            document: "Document",
            empty: "Empty Element",
            field: "Field",
            fieldset: "Fieldset",
            fieldset_entry: "Fieldset Entry",
            list: "List",
            list_item: "List Item",
            section: "Section",
            value: "Value",
        },

        tokenization: Tokenization {
            invalid_line: "Line {} does not follow any specified pattern.",
            unterminated_block: "The block '{}' starting in line {} is not terminated until the end of the document.",
            unterminated_escaped_name: "In line {} the name of an element is escaped, but the escape sequence is not terminated until the end of the line."
        },

        analysis: Analysis {
            duplicate_fieldset_entry_name: "The fieldset '{}' contains two entries named '{}'.",
            fieldset_entry_in_field: "Line {} contains a fieldset entry inside a field.",
            fieldset_entry_in_list: "Line {} contains a fieldset entry inside a list.",
            list_item_in_field: "Line {} contains a list item inside a field.",
            list_item_in_fieldset: "Line {} contains a list item inside a fieldset.",
            missing_element_for_continuation: "Line {} contains a continuation without any continuable element being specified before.",
            missing_name_for_fieldset_entry:  "Line {} contains a fieldset entry without a name for a fieldset being specified before.",
            missing_name_for_list_item: "Line {} contains a list item without a name for a list being specified before.",
            section_hierarchy_layer_skip: "Line {} starts a section that is more than one level deeper than the current one."
        },

        resolution: Resolution {
            copying_block_into_fieldset: "In line {} a block is copied into a fieldset.",
            copying_block_into_list: "In line {} a block is copied into a list.",
            copying_block_into_section: "In line {} a block is copied into a section.",
            copying_field_into_fieldset: "In line {} a field is copied into a fieldset.",
            copying_field_into_list: "In line {} a field is copied into a list.",
            copying_field_into_section: "In line {} a field is copied into a section.",
            copying_fieldset_into_field: "In line {} a fieldset is copied into a field.",
            copying_fieldset_into_list: "In line {} a fieldset is copied into a list.",
            copying_fieldset_into_section: "In line {} a fieldset is copied into a section.",
            copying_list_into_field: "In line {} a list is copied into a field.",
            copying_list_into_fieldset: "In line {} a list is copied into a fieldset.",
            copying_list_into_section: "In line {} a list is copied into a section.",
            copying_section_into_empty: "In line {} a section is copied into an empty element.",
            copying_section_into_field: "In line {} a section is copied into a field.",
            copying_section_into_fieldset: "In line {} a section is copied into a fieldset.",
            copying_section_into_list: "In line {} a section is copied into a list.",
            cyclic_dependency: "In line {} '{}' is copied into itself.",
            multiple_templates_found: "In line {} it is not clear which of the elements named '{}' should be copied.",
            template_not_found: "In line {} the element '{}' should be copied, but it was not found."
        },

        validation: Validation {
            exact_count_not_met:  "The list '{}' contains {actual} items, but must contain exactly {expected} items.",
            excess_name: "An excess element named '{}' was found, is it possibly a typo?",
            expected_element_got_elements: "Instead of the expected single element '{}' several elements with this name were found.",
            expected_field_got_fields: "Instead of the expected single field '{}' several fields with this name were found.",
            expected_field_got_fieldset: "Instead of the expected field '{}' a fieldset with this name was found.",
            expected_field_got_list:  "Instead of the expected field '{}' a list with this name was found.",
            expected_field_got_section: "Instead of the expected field '{}' a section with this name was found.",
            expected_fields_got_fieldset: "Only fields with the name '{}' were expected, but a fieldset with this name was found.",
            expected_fields_got_list: "Only fields with the name '{}' were expected, but a list with this name was found.",
            expected_fields_got_section: "Only fields with the name '{}' were expected, but a section with this name was found.",
            expected_fieldset_got_field: "Instead of the expected fieldset '{}' a field with this name was found.",
            expected_fieldset_got_fieldsets: "Instead of the expected single fieldset '{}' several fieldsets with this name were found.",
            expected_fieldset_got_list: "Instead of the expected fieldset '{}' a list with this name was found.",
            expected_fieldset_got_section: "Instead of the expected fieldset '{}' a section with this name was found.",
            expected_fieldsets_got_field: "Only fieldsets with the name '{}' were expected, but a field with this name was found.",
            expected_fieldsets_got_list: "Only fieldsets with the name '{}' were expected, but a list with this name was found.",
            expected_fieldsets_got_section: "Only fieldsets with the name '{}' were expected, but a section with this name was found.",
            expected_list_got_field: "Instead of the expected list '{}' a field with this name was found.",
            expected_list_got_fieldset: "Instead of the expected list '{}' a fieldset with this name was found.",
            expected_list_got_lists: "Instead of the expected single list '{}' several lists with this name were found.",
            expected_list_got_section: "Instead of the expected list '{}' a section with this name was found.",
            expected_lists_got_field: "Only lists with the name '{}' were expected, but a field with this name was found.",
            expected_lists_got_fieldset: "Only lists with the name '{}' were expected, but a fieldset with this name was found.",
            expected_lists_got_section: "Only lists with the name '{}' were expected, but a section with this name was found.",
            expected_section_got_empty: "Instead of the expected section '{}' an empty element with this name was found.",
            expected_section_got_field: "Instead of the expected section '{}' a field with this name was found.",
            expected_section_got_fieldset: "Instead of the expected section '{}' a fieldset with this name was found.",
            expected_section_got_list: "Instead of the expected section '{}' a list with this name was found.",
            expected_section_got_sections: "Instead of the expected single section '{}' several sections with this name were found.",
            expected_sections_got_empty: "Only sections with the name '{}' were expected, but an empty element with this name was found.",
            expected_sections_got_field: "Only sections with the name '{}' were expected, but a field with this name was found.",
            expected_sections_got_fieldset: "Only sections with the name '{}' were expected, but a fieldset with this name was found.",
            expected_sections_got_list: "Only sections with the name '{}' were expected, but a list with this name was found.",
            generic_error: "There is a problem with the value of the element '{}'.",
            max_count_not_met: "The list '{}' contains {actual} items, but may only contain a maximum of {} items.",
            min_count_not_met: "The list '{}' contains {actual} items, but must contain at least {} items.",
            missing_element: "The element '{}' is missing - in case it has been specified look for typos and also check for correct capitalization.",
            missing_field: "The field '{}' is missing - in case it has been specified look for typos and also check for correct capitalization.",
            missing_field_value: "The field '{}' must contain a value.",
            missing_fieldset: "The fieldset '{}' is missing - in case it has been specified look for typos and also check for correct capitalization.",
            missing_fieldset_entry: "The fieldset entry '{}' is missing - in case it has been specified look for typos and also check for correct capitalization.",
            missing_fieldset_entry_value: "The fieldset entry '{}' must contain a value.",
            missing_list: "The list '{}' is missing - in case it has been specified look for typos and also check for correct capitalization.",
            missing_list_item_value: "The list '{}' may not contain empty items.",
            missing_section: "The section '{}' is missing - in case it has been specified look for typos and also check for correct capitalization."
        },

        loaders: Loaders {
            invalid_boolean: "'{}' must contain a boolean - allowed values are 'true', 'false', 'yes' and 'no'.",
            invalid_color: "'{}' must contain a color, for instance '#B6D918', '#fff' or '#01b'.",
            invalid_date: "'{}' must contain a valid date, for instance '1993-11-18'.",
            invalid_datetime: "'{}' must contain a valid date or date and time, for instance '1961-01-22' or '1989-11-09T19:17Z' (see https://www.w3.org/TR/NOTE-datetime).",
            invalid_email: "'{}' must contain a valid email address, for instance 'jane.doe@eno-lang.org'.",
            invalid_float: "'{}' must contain a decimal number, for instance '13.0', '-9.159' or '42'.",
            invalid_integer: "'{}' must contain an integer, for instance '42' or '-21'.",
            invalid_json: "'{}' must contain valid JSON - the parser returned: '{}'.",
            invalid_lat_lng: "'{}' must contain a valid latitude/longitude coordinate pair, for instance '48.2093723, 16.356099'.",
            invalid_url: "'{}' must contain a valid URL, for instance 'https://eno-lang.org'."
        }
    };
    msg
}
