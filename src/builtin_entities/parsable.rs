use crate::builtin_entities::examples::*;
use crate::builtin_entities::supported_languages::supported_languages;
use serde_derive::{Deserialize, Serialize};
use snips_nlu_ontology::{BuiltinEntityKind, IntoBuiltinEntityKind, Language};

pub trait ParsableEntityKind {
    fn examples(&self, language: Language) -> &'static [&'static str];
    fn supported_languages(&self) -> &'static [Language];
    fn ontology_details(&self, language: Language) -> BuiltinEntityKindDetails;
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BuiltinEntityKindDetails {
    name: String,
    label: String,
    description: String,
    examples: Vec<String>,
    result_description: String,
    supported_languages: Vec<String>,
}

impl<T> ParsableEntityKind for T
where
    T: IntoBuiltinEntityKind,
{
    fn examples(&self, language: Language) -> &'static [&'static str] {
        match language {
            Language::DE => de_examples(self),
            Language::EN => en_examples(self),
            Language::ES => es_examples(self),
            Language::FR => fr_examples(self),
            Language::JA => ja_examples(self),
            Language::IT => it_examples(self),
            Language::PT_PT => pt_pt_examples(self),
            Language::PT_BR => pt_br_examples(self),
            Language::KO => ko_examples(self),
        }
    }

    fn supported_languages(&self) -> &'static [Language] {
        supported_languages(self)
    }

    fn ontology_details(&self, language: Language) -> BuiltinEntityKindDetails {
        BuiltinEntityKindDetails {
            name: self.into_builtin_kind().to_string(),
            label: self.identifier().to_string(),
            description: self.description().to_string(),
            examples: self
                .examples(language)
                .into_iter()
                .map(|ex| ex.to_string())
                .collect(),
            result_description: self.result_description(),
            supported_languages: self
                .supported_languages()
                .into_iter()
                .map(|l| l.to_string())
                .collect(),
        }
    }
}

pub trait ParsableLanguage {
    fn supported_entity_kinds(&self) -> Vec<BuiltinEntityKind>;
}

impl ParsableLanguage for Language {
    fn supported_entity_kinds(&self) -> Vec<BuiltinEntityKind> {
        BuiltinEntityKind::all()
            .into_iter()
            .filter(|e| e.supported_languages().contains(self))
            .cloned()
            .collect()
    }
}
