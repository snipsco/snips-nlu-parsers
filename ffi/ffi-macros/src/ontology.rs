use crate::Result;
use ffi_utils::{point_to_string, CReprOf, CStringArray, RawPointerConverter};
use snips_nlu_ontology::{
    BuiltinEntityKind, BuiltinGazetteerEntityKind, GrammarEntityKind, IntoBuiltinEntityKind,
    Language,
};
use snips_nlu_parsers::ontology::*;
use snips_nlu_parsers::parsable::ParsableEntityKind;
use std::ffi::CStr;
use std::str::FromStr;

pub fn get_supported_builtin_entities(
    language: *const libc::c_char,
    results: *mut *const CStringArray,
) -> Result<()> {
    let language_str = unsafe { CStr::from_ptr(language) }.to_str()?;
    let language = Language::from_str(&*language_str.to_uppercase())?;
    let entities = BuiltinEntityKind::all()
        .iter()
        .filter(|e| e.supported_languages().contains(&language))
        .map(|e| e.identifier().to_string())
        .collect::<Vec<_>>();
    let c_entities = CStringArray::c_repr_of(entities)?.into_raw_pointer();
    unsafe {
        *results = c_entities;
    }
    Ok(())
}

pub fn get_supported_grammar_entities(
    language: *const libc::c_char,
    results: *mut *const CStringArray,
) -> Result<()> {
    let language_str = unsafe { CStr::from_ptr(language) }.to_str()?;
    let language = Language::from_str(&*language_str.to_uppercase())?;
    let entities = GrammarEntityKind::all()
        .iter()
        .filter(|e| e.supported_languages().contains(&language))
        .map(|e| e.identifier().to_string())
        .collect::<Vec<_>>();
    let c_entities = CStringArray::c_repr_of(entities)?.into_raw_pointer();
    unsafe {
        *results = c_entities;
    }
    Ok(())
}

pub fn get_supported_builtin_gazetteer_entities(
    language: *const libc::c_char,
    results: *mut *const CStringArray,
) -> Result<()> {
    let language_str = unsafe { CStr::from_ptr(language) }.to_str()?;
    let language = Language::from_str(&*language_str.to_uppercase())?;
    let entities = BuiltinGazetteerEntityKind::all()
        .iter()
        .filter(|e| e.supported_languages().contains(&language))
        .map(|e| e.identifier().to_string())
        .collect::<Vec<_>>();
    let c_entities = CStringArray::c_repr_of(entities)?.into_raw_pointer();
    unsafe {
        *results = c_entities;
    }
    Ok(())
}

pub fn get_builtin_entity_examples(
    builtin_entity_kind: *const libc::c_char,
    language: *const libc::c_char,
    results: *mut *const CStringArray,
) -> Result<()> {
    let entity_kind_str = unsafe { CStr::from_ptr(builtin_entity_kind) }.to_str()?;
    let entity_kind = BuiltinEntityKind::from_identifier(&*entity_kind_str)?;
    let language_str = unsafe { CStr::from_ptr(language) }.to_str()?;
    let language = Language::from_str(&*language_str.to_uppercase())?;
    let examples = entity_kind
        .examples(language)
        .into_iter()
        .map(|example| example.to_string())
        .collect::<Vec<_>>();
    let c_examples = CStringArray::c_repr_of(examples)?.into_raw_pointer();
    unsafe {
        *results = c_examples;
    }
    Ok(())
}

pub fn get_complete_entity_ontology_json(ontology_result: *mut *const libc::c_char) -> Result<()> {
    let ontology = serde_json::to_string_pretty(&complete_entity_ontology())?;
    point_to_string(ontology_result, ontology)
}

pub fn get_language_entity_ontology_json(
    language: *const libc::c_char,
    ontology_result: *mut *const libc::c_char,
) -> Result<()> {
    let language_str = unsafe { CStr::from_ptr(language) }.to_str()?;
    let language = Language::from_str(&*language_str.to_uppercase())?;
    let ontology = serde_json::to_string_pretty(&language_entity_ontology(language))?;
    point_to_string(ontology_result, ontology)
}
