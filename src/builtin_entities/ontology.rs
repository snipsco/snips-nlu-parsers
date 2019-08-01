use crate::parsable::{BuiltinEntityKindDetails, ParsableEntityKind, ParsableLanguage};
use serde::{Deserialize, Serialize};
use snips_nlu_ontology::Language;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LanguageBuiltinEntityOntology {
    language: String,
    entities: Vec<BuiltinEntityKindDetails>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CompleteBuiltinEntityOntology(Vec<LanguageBuiltinEntityOntology>);

pub fn complete_entity_ontology() -> CompleteBuiltinEntityOntology {
    let language_ontologies = Language::all()
        .iter()
        .map(|language| language_entity_ontology(*language))
        .collect();
    CompleteBuiltinEntityOntology(language_ontologies)
}

pub fn language_entity_ontology(language: Language) -> LanguageBuiltinEntityOntology {
    let entities = language
        .supported_entity_kinds()
        .iter()
        .map(|entity_kind| entity_kind.ontology_details(language))
        .collect();
    LanguageBuiltinEntityOntology {
        language: language.to_string(),
        entities,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_complete_entities_ontology() {
        assert!(serde_json::to_string_pretty(&complete_entity_ontology()).is_ok())
    }

    #[test]
    fn test_entities_ontology() {
        for language in Language::all() {
            assert!(serde_json::to_string_pretty(&language_entity_ontology(*language)).is_ok())
        }
    }
}
