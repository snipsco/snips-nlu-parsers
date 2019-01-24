use ffi_utils::*;
use snips_nlu_ontology_ffi_macros::export_nlu_ontology_c_symbols;
use snips_nlu_parsers_ffi_macros::export_nlu_parsers_c_symbols;

generate_error_handling!(snips_nlu_parsers_get_last_error);

export_nlu_ontology_c_symbols!();

export_nlu_parsers_c_symbols!();
