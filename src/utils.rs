use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;
use std::ops::Range;

use lazy_static::lazy_static;
use regex::Regex;
use snips_nlu_ontology::Language;

lazy_static! {
    pub static ref NON_SPACE_REGEX: Regex = Regex::new(r"[^\s]+").unwrap();
}

lazy_static! {
    pub static ref NON_SPACE_SEPARATED_LANGUAGES: HashSet<Language> =
        [Language::JA].into_iter().cloned().collect();
}

pub fn get_ranges_mapping(tokens_ranges: &Vec<Range<usize>>) -> HashMap<usize, usize> {
    /* Given tokens ranges returns a mapping of byte index to a token index
    The byte indexes corresponds to indexes of the end of tokens in string given by joining all
    the tokens. The tokens index gives the index of the tokens preceding the byte index.

    For instance, if range_mapping[65] -> 5, this means that the token of index 6 starts at the
    65th byte in the joined string
    */
    let ranges_mapping = HashMap::<usize, usize>::from_iter(tokens_ranges.iter().enumerate().fold(
        vec![],
        |mut acc: Vec<(usize, usize)>, (token_index, ref original_range)| {
            let previous_end = if token_index == 0 {
                0 as usize
            } else {
                acc[acc.len() - 1].0
            };
            acc.push((
                previous_end + original_range.end - original_range.start,
                token_index,
            ));
            acc
        },
    ));
    ranges_mapping
}
