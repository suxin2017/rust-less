use cssparser::{CowRcStr, ParseError, Parser, ParserState};
use crate::error::ParserError;
use crate::parser::ParserOptions;
use crate::properties::custom::TokenList;
use crate::rules::Location;
use crate::rules::variable::VariableDefined;
use crate::values::string::CowArcStr;


pub fn parse_variable_defined<'i, 't>(name: CowRcStr<'i>, input: &mut Parser<'i, 't>, options: &ParserOptions<'_, 'i>) -> Result<VariableDefined<'i>, ParseError<'i, ParserError<'i>>> {
    let token_list = TokenList::parse_valueable_token_list(input, options, 0)?;

    Ok(VariableDefined {
        name: CowArcStr::from(&name),
        value: token_list,
    })
}
