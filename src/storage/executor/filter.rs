use std::fmt;

use crate::constants as Constants;
use crate::storage::executor::unit_content::UnitContent;

use regex::Error as RegexError;
use regex::Regex;

#[derive(Debug)]
pub enum FilterError {
    RegexError(RegexError),
    ParseFilterError,
}

impl From<RegexError> for FilterError {
    fn from(err: RegexError) -> FilterError {
        FilterError::RegexError(err)
    }
}

pub type FilterResult<T> = Result<T, FilterError>;

#[derive(Debug)]
pub enum FilterOperator {
    Equal,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    NotEqual,
}

impl fmt::Display for FilterOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string = match self {
            FilterOperator::Equal => "=",
            FilterOperator::Greater => ">",
            FilterOperator::GreaterEqual => ">=",
            FilterOperator::Less => "<",
            FilterOperator::LessEqual => "<=",
            FilterOperator::NotEqual => "!=",
        };
        write!(f, "{}", string)
    }
}

#[derive(Debug)]
pub struct FilterOperands {
    pub map_key: String,
    pub unit_content: UnitContent,
}

#[derive(Debug)]
pub struct FilterUnit {
    pub operands: FilterOperands,
    pub operator: FilterOperator,
}

impl fmt::Display for FilterUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (left_operand, right_operand) = (&self.operands.map_key, &self.operands.unit_content);
        write!(
            f,
            "{}{}{}",
            left_operand,
            &self.operator,
            right_operand.to_string()
        )
    }
}

#[derive(Debug, PartialEq)]
pub enum LogicalOperator {
    Or,
    And,
}

impl fmt::Display for LogicalOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string = match self {
            LogicalOperator::Or => "||",
            LogicalOperator::And => "&&",
        };
        write!(f, "{}", string)
    }
}

#[derive(Debug)]
pub struct Filter {
    pub filter_units: Vec<FilterUnit>,
    pub logical_operators: Vec<LogicalOperator>,
}

impl fmt::Display for Filter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::new();
        let mut filter_index = 0;
        let mut operator_index = 0;

        while &filter_index < &self.filter_units.len() {
            if filter_index == &self.filter_units.len() - 1 {
                result.push_str(&self.filter_units[filter_index].to_string());
                break;
            }

            let filter_unit_string = &self.filter_units[filter_index];
            let operator_string = &self.logical_operators[operator_index];

            result.push_str(&format!("{}{}", filter_unit_string, operator_string));

            filter_index += 1;
            operator_index += 1;
        }

        write!(f, "{}", result)
    }
}

fn get_operands_and_operators(regex: &Regex, text: &str) -> (Vec<String>, Vec<String>) {
    let mut current_pos = 0;
    let matches = regex.find_iter(text);

    let mut operands_str_vec = vec![];
    let mut operator_str_vec = vec![];

    for pos in matches.into_iter() {
        let start = pos.start();
        let end = pos.end();

        let left_part = text[current_pos..start].to_string();
        operands_str_vec.push(left_part);

        let operator = text[start..end].to_string();
        operator_str_vec.push(operator);

        current_pos = end;
    }

    let last_part = text[current_pos..].to_string();
    operands_str_vec.push(last_part);

    return (operands_str_vec, operator_str_vec);
}

pub fn parse_filter_string(filter_string: String) -> FilterResult<Filter> {
    let logical_re = Regex::new(r"&&|\|\|")?;
    let (filter_units_str_vec, logical_operators_str_vec) =
        get_operands_and_operators(&logical_re, &filter_string);

    if filter_units_str_vec.len() != logical_operators_str_vec.len() + 1 {
        return Err(FilterError::ParseFilterError);
    }

    let filter_unit_operator_re = Regex::new(r">=|<=|!=|>|=|<")?;

    let filter_units: FilterResult<Vec<FilterUnit>> = filter_units_str_vec
        .iter()
        .map(|filter_unit_str| {
            let (operands, operators) =
                get_operands_and_operators(&filter_unit_operator_re, &filter_unit_str);
            if operands.len() != operators.len() + 1 && operators.len() != 1 {
                return Err(FilterError::ParseFilterError);
            }

            let left_operand = &operands[0];
            let right_operand = &operands[1];

            let unit_content = if let Ok(number) = right_operand.parse::<f64>() {
                UnitContent::Float64(number)
            } else {
                UnitContent::String(right_operand.to_owned())
            };

            let filter_operands = FilterOperands {
                map_key: left_operand.to_owned(),
                unit_content,
            };

            let operator = match operators[0].as_str() {
                Constants::FILTER_GREATER_EQUAL => FilterOperator::GreaterEqual,
                Constants::FILTER_LESS_EQUAL => FilterOperator::LessEqual,
                Constants::FILTER_NOT_EQUAL => FilterOperator::NotEqual,
                Constants::FILTER_GREATER => FilterOperator::Greater,
                Constants::FILTER_LESS => FilterOperator::Less,
                Constants::FILTER_EQUAL => FilterOperator::Equal,
                _ => return Err(FilterError::ParseFilterError),
            };

            let filter_unit = FilterUnit {
                operands: filter_operands,
                operator,
            };

            return Ok(filter_unit);
        })
        .collect();

    let logical_operators: FilterResult<Vec<LogicalOperator>> = logical_operators_str_vec
        .iter()
        .map(|operator_str| match operator_str.as_str() {
            "&&" => return Ok(LogicalOperator::And),
            "||" => return Ok(LogicalOperator::Or),
            _ => return Err(FilterError::ParseFilterError),
        })
        .collect();

    let filter = Filter {
        filter_units: filter_units?,
        logical_operators: logical_operators?,
    };

    return Ok(filter);
}

pub fn content_satisfied_filter(content: &UnitContent, filter: &Filter) -> bool {
    let logical_operators = &filter.logical_operators;
    let mut logical_operators_index = 0;
    let filter_units = &filter.filter_units;
    let mut current_operator: Option<&LogicalOperator> = None;
    let mut last_result;
    let mut current_result = true;

    for filter_unit in filter_units.iter() {
        if !current_result && current_operator != Some(&LogicalOperator::Or) {
            break;
        }

        last_result = current_result;
        current_result = content_satisfied_filter_unit(&content, &filter_unit);

        if let Some(operator) = current_operator {
            match operator {
                LogicalOperator::Or => {
                    if !(last_result || current_result) {
                        return false;
                    } else {
                        current_result = true;
                    }
                }
                LogicalOperator::And => {
                    if !(last_result && current_result) {
                        return false;
                    } else {
                        current_result = true;
                    }
                }
            }
        }

        if &logical_operators_index < &logical_operators.len() {
            current_operator = Some(&logical_operators[logical_operators_index]);
            logical_operators_index += 1;
        }
    }

    return current_result;
}

pub fn content_satisfied_filter_unit(content: &UnitContent, filter_unit: &FilterUnit) -> bool {
    let filter_map_key = &filter_unit.operands.map_key;
    let filter_content = &filter_unit.operands.unit_content;
    let operator = &filter_unit.operator;

    match operator {
        FilterOperator::Equal => match content {
            UnitContent::Map(map) => match map.get(filter_map_key) {
                Some(content) => {
                    if content == filter_content {
                        return true;
                    } else {
                        return false;
                    }
                }
                None => return false,
            },
            _ => return false,
        },
        FilterOperator::NotEqual => match content {
            UnitContent::Map(map) => match map.get(filter_map_key) {
                Some(content) => {
                    if content != filter_content {
                        return true;
                    } else {
                        return false;
                    }
                }
                None => return false,
            },
            _ => return false,
        },
        FilterOperator::Less => match content {
            UnitContent::Map(map) => match map.get(filter_map_key) {
                Some(content) => match content {
                    UnitContent::Float64(number) => match filter_content {
                        UnitContent::Float64(filter_number) => {
                            if number < filter_number {
                                return true;
                            } else {
                                return false;
                            }
                        }
                        _ => return false,
                    },
                    UnitContent::String(string) => match filter_content {
                        UnitContent::String(filter_string) => {
                            if string < filter_string {
                                return true;
                            } else {
                                return false;
                            }
                        }
                        _ => return false,
                    },
                    _ => return false,
                },
                None => return false,
            },
            _ => return false,
        },
        FilterOperator::LessEqual => match content {
            UnitContent::Map(map) => match map.get(filter_map_key) {
                Some(content) => match content {
                    UnitContent::Float64(number) => match filter_content {
                        UnitContent::Float64(filter_number) => {
                            if number <= filter_number {
                                return true;
                            } else {
                                return false;
                            }
                        }
                        _ => return false,
                    },
                    UnitContent::String(string) => match filter_content {
                        UnitContent::String(filter_string) => {
                            if string <= filter_string {
                                return true;
                            } else {
                                return false;
                            }
                        }
                        _ => return false,
                    },
                    _ => return false,
                },
                None => return false,
            },
            _ => return false,
        },
        FilterOperator::Greater => match content {
            UnitContent::Map(map) => match map.get(filter_map_key) {
                Some(content) => match content {
                    UnitContent::Float64(number) => match filter_content {
                        UnitContent::Float64(filter_number) => {
                            if number > filter_number {
                                return true;
                            } else {
                                return false;
                            }
                        }
                        _ => return false,
                    },
                    UnitContent::String(string) => match filter_content {
                        UnitContent::String(filter_string) => {
                            if string > filter_string {
                                return true;
                            } else {
                                return false;
                            }
                        }
                        _ => return false,
                    },
                    _ => return false,
                },
                None => return false,
            },
            _ => return false,
        },
        FilterOperator::GreaterEqual => match content {
            UnitContent::Map(map) => match map.get(filter_map_key) {
                Some(content) => match content {
                    UnitContent::Float64(number) => match filter_content {
                        UnitContent::Float64(filter_number) => {
                            if number >= filter_number {
                                return true;
                            } else {
                                return false;
                            }
                        }
                        _ => return false,
                    },
                    UnitContent::String(string) => match filter_content {
                        UnitContent::String(filter_string) => {
                            if string >= filter_string {
                                return true;
                            } else {
                                return false;
                            }
                        }
                        _ => return false,
                    },
                    _ => return false,
                },
                None => return false,
            },
            _ => return false,
        },
    }
}
