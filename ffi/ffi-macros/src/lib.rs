pub use builtin_entity_parser::*;
pub use gazetteer_entity_parser::*;
pub use ontology::*;

mod builtin_entity_parser;
mod gazetteer_entity_parser;
mod ontology;

type Result<T> = ::std::result::Result<T, ::failure::Error>;

#[macro_export]
macro_rules! export_nlu_parsers_c_symbols {
    () => {
        #[no_mangle]
        pub extern "C" fn snips_nlu_parsers_create_builtin_entity_parser(
            ptr: *mut *const $crate::CBuiltinEntityParser,
            json_config: *const ::libc::c_char,
        ) -> ::ffi_utils::SNIPS_RESULT {
            wrap!($crate::create_builtin_entity_parser(ptr, json_config))
        }

        #[no_mangle]
        pub extern "C" fn snips_nlu_parsers_persist_builtin_entity_parser(
            ptr: *const $crate::CBuiltinEntityParser,
            path: *const ::libc::c_char,
        ) -> ::ffi_utils::SNIPS_RESULT {
            wrap!($crate::persist_builtin_entity_parser(ptr, path))
        }

        #[no_mangle]
        pub extern "C" fn snips_nlu_parsers_load_builtin_entity_parser(
            ptr: *mut *const $crate::CBuiltinEntityParser,
            parser_path: *const ::libc::c_char,
        ) -> ::ffi_utils::SNIPS_RESULT {
            wrap!($crate::load_builtin_entity_parser(ptr, parser_path))
        }

        #[no_mangle]
        pub extern "C" fn snips_nlu_parsers_extend_gazetteer_entity_json(
            ptr: *const $crate::CBuiltinEntityParser,
            entity_name: *const libc::c_char,
            entity_values_json: *const libc::c_char,
        ) -> ::ffi_utils::SNIPS_RESULT {
            wrap!($crate::extend_gazetteer_entity_json(
                ptr,
                entity_name,
                entity_values_json
            ))
        }

        #[no_mangle]
        pub extern "C" fn snips_nlu_parsers_extract_builtin_entities(
            ptr: *const $crate::CBuiltinEntityParser,
            sentence: *const ::libc::c_char,
            filter_entity_kinds: *const ::ffi_utils::CStringArray,
            results: *mut *const snips_nlu_ontology_ffi_macros::CBuiltinEntityArray,
        ) -> ::ffi_utils::SNIPS_RESULT {
            wrap!($crate::extract_builtin_entity_c(
                ptr,
                sentence,
                filter_entity_kinds,
                results
            ))
        }

        #[no_mangle]
        pub extern "C" fn snips_nlu_parsers_extract_builtin_entities_json(
            ptr: *const $crate::CBuiltinEntityParser,
            sentence: *const ::libc::c_char,
            filter_entity_kinds: *const ::ffi_utils::CStringArray,
            results: *mut *const ::libc::c_char,
        ) -> ::ffi_utils::SNIPS_RESULT {
            wrap!($crate::extract_builtin_entity_json(
                ptr,
                sentence,
                filter_entity_kinds,
                results
            ))
        }

        #[no_mangle]
        pub extern "C" fn snips_nlu_parsers_destroy_builtin_entity_array(
            ptr: *mut ::snips_nlu_ontology_ffi_macros::CBuiltinEntityArray,
        ) -> ::ffi_utils::SNIPS_RESULT {
            use ffi_utils::RawPointerConverter;
            use snips_nlu_ontology_ffi_macros::CBuiltinEntityArray;
            wrap!(unsafe { CBuiltinEntityArray::from_raw_pointer(ptr) })
        }

        #[no_mangle]
        pub extern "C" fn snips_nlu_parsers_destroy_builtin_entity_parser(
            ptr: *mut $crate::CBuiltinEntityParser,
        ) -> ::ffi_utils::SNIPS_RESULT {
            wrap!($crate::destroy_builtin_entity_parser(ptr))
        }

        #[no_mangle]
        pub extern "C" fn snips_nlu_parsers_build_gazetteer_entity_parser(
            ptr: *mut *const $crate::CGazetteerEntityParser,
            json_config: *const ::libc::c_char,
        ) -> ::ffi_utils::SNIPS_RESULT {
            wrap!($crate::build_gazetteer_entity_parser(ptr, json_config))
        }

        #[no_mangle]
        pub extern "C" fn snips_nlu_parsers_load_gazetteer_entity_parser(
            ptr: *mut *const $crate::CGazetteerEntityParser,
            parser_path: *const ::libc::c_char,
        ) -> ::ffi_utils::SNIPS_RESULT {
            wrap!($crate::load_gazetteer_entity_parser(ptr, parser_path))
        }

        #[no_mangle]
        pub extern "C" fn snips_nlu_parsers_persist_gazetteer_entity_parser(
            ptr: *const $crate::CGazetteerEntityParser,
            path: *const ::libc::c_char,
        ) -> ::ffi_utils::SNIPS_RESULT {
            wrap!($crate::persist_gazetteer_entity_parser(ptr, path))
        }

        #[no_mangle]
        pub extern "C" fn snips_nlu_parsers_extract_gazetteer_entities_json(
            ptr: *const $crate::CGazetteerEntityParser,
            sentence: *const ::libc::c_char,
            filter_entity_kinds: *const ::ffi_utils::CStringArray,
            results: *mut *const ::libc::c_char,
        ) -> ::ffi_utils::SNIPS_RESULT {
            wrap!($crate::extract_gazetteer_entity_json(
                ptr,
                sentence,
                filter_entity_kinds,
                results
            ))
        }

        #[no_mangle]
        pub extern "C" fn snips_nlu_parsers_destroy_gazetteer_entity_parser(
            ptr: *mut $crate::CGazetteerEntityParser,
        ) -> ::ffi_utils::SNIPS_RESULT {
            wrap!($crate::destroy_gazetteer_entity_parser(ptr))
        }

        #[no_mangle]
        pub extern "C" fn snips_nlu_parsers_supported_builtin_entities(
            language: *const libc::c_char,
            results: *mut *const ::ffi_utils::CStringArray,
        ) -> ::ffi_utils::SNIPS_RESULT {
            wrap!($crate::get_supported_builtin_entities(language, results))
        }

        #[no_mangle]
        pub extern "C" fn snips_nlu_parsers_supported_grammar_entities(
            language: *const libc::c_char,
            results: *mut *const ::ffi_utils::CStringArray,
        ) -> ::ffi_utils::SNIPS_RESULT {
            wrap!($crate::get_supported_grammar_entities(language, results))
        }

        #[no_mangle]
        pub extern "C" fn snips_nlu_parsers_supported_builtin_gazetteer_entities(
            language: *const libc::c_char,
            results: *mut *const ::ffi_utils::CStringArray,
        ) -> ::ffi_utils::SNIPS_RESULT {
            wrap!($crate::get_supported_builtin_gazetteer_entities(
                language, results
            ))
        }

        #[no_mangle]
        pub extern "C" fn snips_nlu_parsers_builtin_entity_examples(
            builtin_entity_kind: *const libc::c_char,
            language: *const libc::c_char,
            results: *mut *const ::ffi_utils::CStringArray,
        ) -> ::ffi_utils::SNIPS_RESULT {
            wrap!($crate::get_builtin_entity_examples(
                builtin_entity_kind,
                language,
                results
            ))
        }

        #[no_mangle]
        pub extern "C" fn snips_nlu_parsers_complete_entity_ontology_json(
            result: *mut *const libc::c_char,
        ) -> ::ffi_utils::SNIPS_RESULT {
            wrap!($crate::get_complete_entity_ontology_json(result))
        }

        #[no_mangle]
        pub extern "C" fn snips_nlu_parsers_language_entity_ontology_json(
            language: *const libc::c_char,
            result: *mut *const libc::c_char,
        ) -> ::ffi_utils::SNIPS_RESULT {
            wrap!($crate::get_language_entity_ontology_json(language, result))
        }
    };
}
