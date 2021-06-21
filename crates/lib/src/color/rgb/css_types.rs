use rug::Float;

use crate::color::rgb::srgb::DEFAULT_SRGB_PRECISION;
use crate::error::ParsingError;

/// Parses a CSS number (e.g. '1.2' as a float 1.2).
// https://www.w3.org/TR/css-values-3/#number
pub(crate) fn parse_number(seq: &str) -> Result<Float, ParsingError> {
    Ok(Float::with_val(DEFAULT_SRGB_PRECISION, Float::parse(seq)?))
}

/// Formats a float as a CSS number (e.g. 0.6 as '0.6').
pub(crate) fn format_number(val: Float) -> String {
    format!("{}", val.to_f32())
}


/// Checks if something can be parsed as a CSS percentage.
pub(crate) fn is_percentage(seq: &str) -> bool {
    seq.ends_with('%')
}

/// Parses a CSS percentage (e.g. '60%' as a float 0.6).
// https://www.w3.org/TR/css-values-3/#percentage-value
pub(crate) fn parse_percentage(seq: &str) -> Result<Float, ParsingError> {
    debug_assert!(is_percentage(seq));

    let index_of_percentage_sign = seq.rfind('%').unwrap();
    let percentage_number = parse_number(&seq[..index_of_percentage_sign])?;
    Ok(percentage_number / 100)
}

/// Formats a float as a CSS percentage (e.g. 0.6 as '60%').
pub(crate) fn format_percentage(val: Float) -> String {
    let tmp: Float = val * 100;
    format!("{}%", tmp.to_f32())
}
