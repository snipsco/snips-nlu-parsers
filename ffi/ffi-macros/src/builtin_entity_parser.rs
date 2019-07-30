use crate::Result;
use failure::ResultExt;
use ffi_utils::{convert_to_c_string, CReprOf, CStringArray, RawPointerConverter};
use snips_nlu_ontology::{BuiltinEntity, BuiltinEntityKind, BuiltinGazetteerEntityKind};
use snips_nlu_ontology_ffi_macros::{CBuiltinEntity, CBuiltinEntityArray};
use snips_nlu_parsers::{BuiltinEntityParser, BuiltinEntityParserLoader, EntityValue};
use std::ffi::CStr;
use std::slice;

#[repr(C)]
pub struct CBuiltinEntityParser(*const libc::c_void);

macro_rules! get_parser {
    ($opaque:ident) => {{
        let container: &$crate::CBuiltinEntityParser = unsafe { &*$opaque };
        let x = container.0 as *const BuiltinEntityParser;
        unsafe { &*x }
    }};
}

macro_rules! get_parser_mut {
    ($opaque:ident) => {{
        let container: &$crate::CBuiltinEntityParser = unsafe { &*$opaque };
        let x = container.0 as *mut BuiltinEntityParser;
        unsafe { &mut *x }
    }};
}

pub fn create_builtin_entity_parser(
    ptr: *mut *const CBuiltinEntityParser,
    json_config: *const libc::c_char,
) -> Result<()> {
    let json_config = unsafe { CStr::from_ptr(json_config) }.to_str()?;
    let parser_loader: BuiltinEntityParserLoader = serde_json::from_str(json_config)?;
    let parser = parser_loader.load()?;

    let c_parser = CBuiltinEntityParser(parser.into_raw_pointer() as _).into_raw_pointer();

    unsafe {
        *ptr = c_parser;
    }
    Ok(())
}

pub fn extend_gazetteer_entity_json(
    ptr: *const CBuiltinEntityParser,
    entity_name: *const libc::c_char,
    entity_values_json: *const libc::c_char,
) -> Result<()> {
    let parser = get_parser_mut!(ptr);
    let entity_identifier = unsafe { CStr::from_ptr(entity_name) }.to_str()?;
    let entity_kind = BuiltinGazetteerEntityKind::from_identifier(entity_identifier)?;
    let entity_values_json_str = unsafe { CStr::from_ptr(entity_values_json) }.to_str()?;
    let entity_values: Vec<EntityValue> = serde_json::from_str(entity_values_json_str)?;

    parser.extend_gazetteer_entity(entity_kind, entity_values.into_iter())?;
    Ok(())
}

pub fn persist_builtin_entity_parser(
    ptr: *const CBuiltinEntityParser,
    path: *const libc::c_char,
) -> Result<()> {
    let parser = get_parser!(ptr);
    let parser_path = unsafe { CStr::from_ptr(path) }.to_str()?;
    parser.persist(parser_path)?;
    Ok(())
}

pub fn load_builtin_entity_parser(
    ptr: *mut *const CBuiltinEntityParser,
    path: *const libc::c_char,
) -> Result<()> {
    let parser_path = unsafe { CStr::from_ptr(path) }.to_str()?;
    let builtin_entity_parser = BuiltinEntityParser::from_path(parser_path)?;
    let c_parser =
        CBuiltinEntityParser(builtin_entity_parser.into_raw_pointer() as _).into_raw_pointer();

    unsafe {
        *ptr = c_parser;
    }
    Ok(())
}

pub fn extract_builtin_entity_c(
    ptr: *const CBuiltinEntityParser,
    sentence: *const libc::c_char,
    filter_entity_kinds: *const CStringArray,
    results: *mut *const CBuiltinEntityArray,
) -> Result<()> {
    let c_entities = extract_builtin_entity(ptr, sentence, filter_entity_kinds)?
        .into_iter()
        .map(CBuiltinEntity::from)
        .collect::<Vec<_>>();
    let c_entities = CBuiltinEntityArray::from(c_entities).into_raw_pointer();

    unsafe {
        *results = c_entities;
    }

    Ok(())
}

pub fn extract_builtin_entity_json(
    ptr: *const CBuiltinEntityParser,
    sentence: *const libc::c_char,
    filter_entity_kinds: *const CStringArray,
    results: *mut *const libc::c_char,
) -> Result<()> {
    let entities = extract_builtin_entity(ptr, sentence, filter_entity_kinds)?;
    let json = ::serde_json::to_string(&entities)?;

    let cs = convert_to_c_string!(json);
    unsafe { *results = cs }

    Ok(())
}

pub fn extract_builtin_entity(
    ptr: *const CBuiltinEntityParser,
    sentence: *const libc::c_char,
    filter_entity_kinds: *const CStringArray,
) -> Result<Vec<BuiltinEntity>> {
    let parser = get_parser!(ptr);
    let sentence = unsafe { CStr::from_ptr(sentence) }.to_str()?;

    let opt_filters: Option<Vec<_>> = if !filter_entity_kinds.is_null() {
        let filters = unsafe {
            let array = &*filter_entity_kinds;
            slice::from_raw_parts(array.data, array.size as usize)
        }
        .into_iter()
        .map(|&ptr| {
            Ok(unsafe { CStr::from_ptr(ptr) }
                .to_str()
                .map_err(::failure::Error::from)
                .and_then(|s| {
                    Ok(BuiltinEntityKind::from_identifier(s)
                        .with_context(|_| format!("`{}` isn't a known builtin entity kind", s))?)
                })?)
        })
        .collect::<Result<Vec<_>>>()?;
        Some(filters)
    } else {
        None
    };
    let opt_filters = opt_filters.as_ref().map(|vec| vec.as_slice());

    parser.extract_entities(sentence, opt_filters)
}

pub fn destroy_builtin_entity_parser(ptr: *mut CBuiltinEntityParser) -> Result<()> {
    unsafe {
        let parser = CBuiltinEntityParser::from_raw_pointer(ptr)?.0;
        let _ = BuiltinEntityParser::from_raw_pointer(parser as _);
    }
    Ok(())
}
