use crate::conversion::*;
use crate::errors::*;
use crate::gazetteer_parser::GazetteerParser;
use crate::utils::{get_ranges_mapping, NON_SPACE_REGEX, NON_SPACE_SEPARATED_LANGUAGES};
use failure::{format_err, ResultExt};
pub use gazetteer_entity_parser::EntityValue;
use itertools::Itertools;
use rustling_ontology::{build_parser, OutputKind, Parser as RustlingParser, ResolverContext};
use serde_derive::{Deserialize, Serialize};
use serde_json;
use snips_nlu_ontology::*;
use snips_nlu_utils::string::{convert_to_byte_range, convert_to_char_index};
use std::fs;
use std::ops::Range;
use std::path::{Path, PathBuf};
use std::str::FromStr;

pub struct BuiltinEntityParser {
    gazetteer_parser: Option<GazetteerParser<BuiltinGazetteerEntityKind>>,
    rustling_parser: RustlingParser,
    language: Language,
    rustling_entity_kinds: Vec<BuiltinEntityKind>,
}

#[derive(Serialize, Deserialize)]
pub struct BuiltinEntityParserLoader {
    language: Language,
    gazetteer_parser_path: Option<PathBuf>,
}

impl BuiltinEntityParserLoader {
    pub fn new(language: Language) -> Self {
        BuiltinEntityParserLoader {
            language,
            gazetteer_parser_path: None,
        }
    }

    pub fn use_gazetter_parser<P: AsRef<Path>>(&mut self, parser_path: P) -> &mut Self {
        self.gazetteer_parser_path = Some(parser_path.as_ref().to_path_buf());
        self
    }

    pub fn load(&self) -> Result<BuiltinEntityParser> {
        let supported_entity_kinds = BuiltinEntityKind::supported_entity_kinds(self.language);
        let ordered_entity_kinds = OutputKind::all()
            .iter()
            .map(|output_kind| output_kind.ontology_into())
            .filter(|builtin_entity_kind| supported_entity_kinds.contains(&builtin_entity_kind))
            .collect();
        let rustling_parser = build_parser(self.language.ontology_into()).map_err(|_| {
            format_err!(
                "Cannot create Rustling Parser for language {:?}",
                self.language
            )
        })?;
        let gazetteer_parser = match &self.gazetteer_parser_path {
            Some(parser_path) => Some(GazetteerParser::from_path(parser_path)?),
            None => None,
        };
        Ok(BuiltinEntityParser {
            gazetteer_parser,
            rustling_parser,
            language: self.language,
            rustling_entity_kinds: ordered_entity_kinds,
        })
    }
}

impl BuiltinEntityParser {
    pub fn extract_entities(
        &self,
        sentence: &str,
        filter_entity_kinds: Option<&[BuiltinEntityKind]>,
    ) -> Result<Vec<BuiltinEntity>> {
        if NON_SPACE_SEPARATED_LANGUAGES.contains(&self.language) {
            self._extract_entities_for_non_space_separated(sentence, filter_entity_kinds)
        } else {
            self._extract_entities(sentence, filter_entity_kinds)
        }
    }

    fn _extract_entities(
        &self,
        sentence: &str,
        filter_entity_kinds: Option<&[BuiltinEntityKind]>,
    ) -> Result<Vec<BuiltinEntity>> {
        let context = ResolverContext::default();
        let rustling_output_kinds = self
            .rustling_entity_kinds
            .iter()
            .filter(|entity_kind| {
                filter_entity_kinds
                    .map(|kinds| kinds.contains(&entity_kind))
                    .unwrap_or(true)
            })
            .flat_map(|kind| kind.try_ontology_into().ok())
            .collect::<Vec<OutputKind>>();
        let rustling_entities = if rustling_output_kinds.is_empty() {
            vec![]
        } else {
            self.rustling_parser
                .parse_with_kind_order(&sentence.to_lowercase(), &context, &rustling_output_kinds)
                .unwrap_or_else(|_| vec![])
                .into_iter()
                .map(|parser_match| rustling::convert_to_builtin(sentence, parser_match))
                .sorted_by(|a, b| Ord::cmp(&a.range.start, &b.range.start))
        };

        let mut gazetteer_entities = match &self.gazetteer_parser {
            Some(gazetteer_parser) => {
                let gazetteer_entity_kinds: Option<Vec<BuiltinGazetteerEntityKind>> =
                    filter_entity_kinds.map(|kinds| {
                        kinds
                            .into_iter()
                            .flat_map(|kind| kind.try_into_gazetteer_kind().ok())
                            .collect()
                    });
                gazetteer_parser.extract_builtin_entities(
                    sentence,
                    gazetteer_entity_kinds.as_ref().map(|kinds| &**kinds),
                )?
            }
            None => vec![],
        };

        let mut entities = rustling_entities;
        entities.append(&mut gazetteer_entities);
        Ok(entities)
    }

    pub fn _extract_entities_for_non_space_separated(
        &self,
        sentence: &str,
        filter_entity_kinds: Option<&[BuiltinEntityKind]>,
    ) -> Result<Vec<BuiltinEntity>> {
        let original_tokens_bytes_ranges: Vec<Range<usize>> = NON_SPACE_REGEX
            .find_iter(sentence)
            .map(|m| m.start()..m.end())
            .collect();

        let joined_sentence = original_tokens_bytes_ranges
            .iter()
            .map(|r| &sentence[r.clone()])
            .join("");

        if original_tokens_bytes_ranges.is_empty() {
            return Ok(vec![]);
        }

        let ranges_mapping = get_ranges_mapping(&original_tokens_bytes_ranges);

        Ok(self
            ._extract_entities(&*joined_sentence, filter_entity_kinds)?
            .into_iter()
            .filter_map(|ent| {
                let byte_range = convert_to_byte_range(&*joined_sentence, &ent.range);
                let start = byte_range.start;
                let end = byte_range.end;
                // Check if match range correspond to original tokens otherwise skip the entity
                if (start == 0 as usize || ranges_mapping.contains_key(&start))
                    && (ranges_mapping.contains_key(&end))
                {
                    let start_token_index = if start == 0 as usize {
                        0 as usize
                    } else {
                        ranges_mapping[&start] + 1
                    };
                    let end_token_index = ranges_mapping[&end];

                    let original_start = original_tokens_bytes_ranges[start_token_index].start;
                    let original_end = original_tokens_bytes_ranges[end_token_index].end;
                    let value = sentence[original_start..original_end].to_string();

                    let original_ent = BuiltinEntity {
                        value,
                        range: convert_to_char_index(&sentence, original_start)
                            ..convert_to_char_index(&sentence, original_end),
                        entity: ent.entity,
                        entity_kind: ent.entity_kind,
                    };
                    Some(original_ent)
                } else {
                    None
                }
            })
            .collect())
    }
}

impl BuiltinEntityParser {
    pub fn extend_gazetteer_entity(
        &mut self,
        entity_kind: BuiltinGazetteerEntityKind,
        entity_values: impl Iterator<Item = EntityValue>,
    ) -> Result<()> {
        self.gazetteer_parser
            .as_mut()
            .map(|gazetteer_parser| {
                gazetteer_parser.extend_gazetteer_entity(entity_kind, entity_values)
            })
            .transpose()?
            .ok_or_else(|| format_err!("No gazetteer parser found for entity '{:?}'", entity_kind))
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BuiltinParserMetadata {
    pub language: String,
    pub gazetteer_parser: Option<String>,
}

impl BuiltinEntityParser {
    pub fn persist<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        fs::create_dir(path.as_ref()).with_context(|_| {
            format!(
                "Cannot create builtin entity parser directory at path: {:?}",
                path.as_ref()
            )
        })?;
        let gazetteer_parser_directory = if let Some(ref gazetteer_parser) = self.gazetteer_parser {
            let gazetteer_parser_path = path.as_ref().join("gazetteer_entity_parser");
            gazetteer_parser.persist(gazetteer_parser_path)?;
            Some("gazetteer_entity_parser".to_string())
        } else {
            None
        };
        let gazetteer_parser_metadata = BuiltinParserMetadata {
            language: self.language.to_string(),
            gazetteer_parser: gazetteer_parser_directory,
        };
        let metadata_path = path.as_ref().join("metadata.json");
        let metadata_file = fs::File::create(&metadata_path).with_context(|_| {
            format!("Cannot create metadata file at path: {:?}", metadata_path)
        })?;
        serde_json::to_writer_pretty(metadata_file, &gazetteer_parser_metadata)
            .with_context(|_| "Cannot serialize builtin parser metadata")?;
        Ok(())
    }

    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self> {
        let metadata_path = path.as_ref().join("metadata.json");
        let metadata_file = fs::File::open(&metadata_path).with_context(|_| {
            format!(
                "Cannot open builtin parser metadata file at path: {:?}",
                metadata_path
            )
        })?;
        let metadata: BuiltinParserMetadata = serde_json::from_reader(metadata_file)
            .with_context(|_| "Cannot deserialize builtin parser metadata")?;
        let language = Language::from_str(&metadata.language)?;
        let mut parser_loader = BuiltinEntityParserLoader::new(language);
        if let Some(gazetteer_parser_dir) = metadata.gazetteer_parser {
            let gazetteer_parser_path = path.as_ref().join(&gazetteer_parser_dir);
            parser_loader.use_gazetter_parser(gazetteer_parser_path);
        }
        parser_loader.load()
    }
}

#[cfg(test)]
mod test {
    use snips_nlu_ontology::language::Language;
    use snips_nlu_ontology::IntoBuiltinEntityKind;
    use snips_nlu_ontology::SlotValue::InstantTime;
    use tempfile::tempdir;

    use crate::test_utils::test_path;

    use super::*;

    #[test]
    fn test_entities_extraction() {
        let parser = BuiltinEntityParserLoader::new(Language::EN).load().unwrap();
        assert_eq!(
            vec![BuiltinEntityKind::Number, BuiltinEntityKind::Date],
            parser
                .extract_entities("Book me a restaurant for two people tomorrow", None)
                .unwrap()
                .iter()
                .map(|e| e.entity_kind)
                .collect_vec()
        );

        assert_eq!(
            vec![BuiltinEntityKind::Datetime],
            parser
                .extract_entities(
                    "Book me a restaurant for tomorrow",
                    Some(&[BuiltinEntityKind::Datetime])
                )
                .unwrap()
                .iter()
                .map(|e| e.entity_kind)
                .collect_vec()
        );

        assert_eq!(
            vec![BuiltinEntityKind::Datetime],
            parser
                .extract_entities("Book me a restaurant for tomorrow at 8pm", None)
                .unwrap()
                .iter()
                .map(|e| e.entity_kind)
                .collect_vec()
        );

        assert_eq!(
            vec![BuiltinEntityKind::Date],
            parser
                .extract_entities(
                    "Book me a restaurant for tomorrow",
                    Some(&[BuiltinEntityKind::Date])
                )
                .unwrap()
                .iter()
                .map(|e| e.entity_kind)
                .collect_vec()
        );

        assert_eq!(
            vec![BuiltinEntityKind::TimePeriod],
            parser
                .extract_entities("Book the meeting room from 10am to 11am", None)
                .unwrap()
                .iter()
                .map(|e| e.entity_kind)
                .collect_vec()
        );

        assert_eq!(
            vec![BuiltinEntityKind::Time, BuiltinEntityKind::Time],
            parser
                .extract_entities(
                    "Book the meeting room from 10am to 11am",
                    Some(&[BuiltinEntityKind::Time])
                )
                .unwrap()
                .iter()
                .map(|e| e.entity_kind)
                .collect_vec()
        );

        assert_eq!(
            vec![BuiltinEntityKind::Duration],
            parser
                .extract_entities("The weather during two weeks", None)
                .unwrap()
                .iter()
                .map(|e| e.entity_kind)
                .collect_vec()
        );

        assert_eq!(
            vec![BuiltinEntityKind::Percentage],
            parser
                .extract_entities("Set light to ten percents", None)
                .unwrap()
                .iter()
                .map(|e| e.entity_kind)
                .collect_vec()
        );

        assert_eq!(
            vec![BuiltinEntityKind::AmountOfMoney],
            parser
                .extract_entities(
                    "I would like to do a bank transfer of ten euros for my friends",
                    None,
                )
                .unwrap()
                .iter()
                .map(|e| e.entity_kind)
                .collect_vec()
        );
    }

    #[test]
    fn test_entities_extraction_with_empty_scope() {
        let parser = BuiltinEntityParserLoader::new(Language::EN).load().unwrap();
        let entities = parser
            .extract_entities("tomorrow morning", Some(&[]))
            .unwrap();
        assert_eq!(Vec::<BuiltinEntity>::new(), entities);
    }

    #[test]
    fn test_entities_extraction_with_gazetteer_entities() {
        // Given
        let language = Language::FR;
        let parser = BuiltinEntityParserLoader::new(language)
            .use_gazetter_parser(test_path().join("builtin_gazetteer_parser"))
            .load()
            .unwrap();

        // When
        let above_threshold_entity = parser
            .extract_entities("Je voudrais écouter the stones s'il vous plaît", None)
            .unwrap();
        let below_threshold_entity = parser
            .extract_entities("Je voudrais écouter les stones", None)
            .unwrap();

        // Then
        let expected_entity = BuiltinEntity {
            value: "the stones".to_string(),
            range: 20..30,
            entity: SlotValue::MusicArtist(StringValue {
                value: "The Rolling Stones".to_string(),
            }),
            entity_kind: BuiltinEntityKind::MusicArtist,
        };
        assert_eq!(vec![expected_entity], above_threshold_entity);
        assert_eq!(Vec::<BuiltinEntity>::new(), below_threshold_entity);
    }

    #[test]
    fn test_entities_extraction_with_extended_gazetteer_entities() {
        // Given
        let language = Language::EN;
        let mut parser = BuiltinEntityParserLoader::new(language)
            .use_gazetter_parser(test_path().join("builtin_gazetteer_parser"))
            .load()
            .unwrap();
        parser
            .extend_gazetteer_entity(
                BuiltinGazetteerEntityKind::MusicArtist,
                vec![EntityValue {
                    raw_value: "my extended artist".to_string(),
                    resolved_value: "My resolved extended artist".to_string(),
                }]
                .into_iter(),
            )
            .unwrap();

        // When
        let entities = parser
            .extract_entities("I want to listen to my extended artist please", None)
            .unwrap();

        // Then
        let expected_entity = BuiltinEntity {
            value: "my extended artist".to_string(),
            range: 20..38,
            entity: SlotValue::MusicArtist(StringValue {
                value: "My resolved extended artist".to_string(),
            }),
            entity_kind: BuiltinEntityKind::MusicArtist,
        };
        assert_eq!(vec![expected_entity], entities);
    }

    #[test]
    fn test_should_not_allow_extension_for_missing_entity() {
        // Given
        let language = Language::EN;
        let mut parser = BuiltinEntityParserLoader::new(language).load().unwrap();

        // When
        let extension_result = parser.extend_gazetteer_entity(
            BuiltinGazetteerEntityKind::MusicArtist,
            vec![EntityValue {
                raw_value: "my extended artist".to_string(),
                resolved_value: "My resolved extended artist".to_string(),
            }]
            .into_iter(),
        );

        // Then
        assert!(extension_result.is_err());
    }

    #[test]
    fn test_entities_extraction_for_non_space_separated_languages() {
        let parser = BuiltinEntityParserLoader::new(Language::JA).load().unwrap();
        let expected_datetime_value = InstantTimeValue {
            value: "2013-02-10 00:00:00 +01:00".to_string(),
            grain: Grain::Day,
            precision: Precision::Exact,
        };

        let expected_entity = BuiltinEntity {
            value: "二 千 十三 年二 月十 日".to_string(),
            range: 10..24,
            entity_kind: BuiltinEntityKind::Datetime,
            entity: InstantTime(expected_datetime_value.clone()),
        };

        let parsed_entities = parser
            .extract_entities(
                " の カリフォル  二 千 十三 年二 月十 日  ニア州の天気予報は？",
                None,
            )
            .unwrap();
        assert_eq!(1, parsed_entities.len());
        let parsed_entity = &parsed_entities[0];
        assert_eq!(expected_entity.value, parsed_entity.value);
        assert_eq!(expected_entity.range, parsed_entity.range);
        assert_eq!(expected_entity.entity_kind, parsed_entity.entity_kind);

        if let SlotValue::InstantTime(ref parsed_datetime) = parsed_entity.entity {
            assert_eq!(expected_datetime_value.grain, parsed_datetime.grain);
            assert_eq!(expected_datetime_value.precision, parsed_datetime.precision);
        } else {
            panic!("")
        }

        assert_eq!(
            Vec::<BuiltinEntity>::new(),
            parser
                .extract_entities(
                    "二 千 十三 年二 月十 日の カリフォルニア州の天気予報は？",
                    None,
                )
                .unwrap()
        );
    }

    #[test]
    fn test_entity_examples_should_be_parsed() {
        for language in Language::all() {
            let parser = BuiltinEntityParserLoader::new(*language).load().unwrap();
            for entity_kind in GrammarEntityKind::all() {
                for example in (*entity_kind).examples(*language) {
                    let results = parser
                        .extract_entities(example, Some(&[entity_kind.into_builtin_kind()]))
                        .unwrap();
                    assert_eq!(
                        1,
                        results.len(),
                        "Expected 1 result for entity kind '{:?}' in language '{:?}' for example \
                         {:?}, but found: {:?}",
                        entity_kind,
                        language,
                        example,
                        results
                    );
                    assert_eq!(example.to_string(), results[0].value);
                }
            }
        }
    }

    #[test]
    fn test_should_persist_parser() {
        // Given
        let language = Language::FR;
        let parser = BuiltinEntityParserLoader::new(language).load().unwrap();

        let temp_dir = tempdir().unwrap();
        let parser_dir = temp_dir.path().join("builtin_entity_parser");

        // When
        parser.persist(&parser_dir).unwrap();
        let loaded_parser = BuiltinEntityParser::from_path(&parser_dir).unwrap();

        // Then
        assert_eq!(parser.language, loaded_parser.language);
        assert_eq!(None, loaded_parser.gazetteer_parser);
        assert_eq!(
            parser.rustling_entity_kinds,
            loaded_parser.rustling_entity_kinds
        );
    }

    #[test]
    fn test_should_load_parser_from_path() {
        // Given
        let parser_path = test_path().join("builtin_entity_parser_no_gazetteer");

        // When
        let parser = BuiltinEntityParser::from_path(parser_path).unwrap();

        // Then
        let expected_parser = BuiltinEntityParserLoader::new(Language::EN).load().unwrap();
        assert_eq!(expected_parser.language, parser.language);
        assert_eq!(expected_parser.gazetteer_parser, parser.gazetteer_parser);
        assert_eq!(
            expected_parser.rustling_entity_kinds,
            parser.rustling_entity_kinds
        );
    }

    #[test]
    fn test_should_persist_parser_with_gazetteer_entities() {
        // Given
        let language = Language::FR;
        let parser = BuiltinEntityParserLoader::new(language)
            .use_gazetter_parser(test_path().join("builtin_gazetteer_parser"))
            .load()
            .unwrap();

        let temp_dir = tempdir().unwrap();
        let parser_dir = temp_dir.path().join("builtin_entity_parser");

        // When
        parser.persist(&parser_dir).unwrap();
        let loaded_parser = BuiltinEntityParser::from_path(&parser_dir).unwrap();

        // Then
        assert_eq!(parser.language, loaded_parser.language);
        assert_eq!(parser.gazetteer_parser, loaded_parser.gazetteer_parser);
        assert_eq!(
            parser.rustling_entity_kinds,
            loaded_parser.rustling_entity_kinds
        );
    }

    #[test]
    fn test_should_load_parser_with_gazetteer_entities_from_path() {
        // Given
        let parser_path = test_path().join("builtin_entity_parser");

        // When
        let parser = BuiltinEntityParser::from_path(parser_path).unwrap();

        // Then
        let expected_parser = BuiltinEntityParserLoader::new(Language::FR)
            .use_gazetter_parser(test_path().join("builtin_gazetteer_parser"))
            .load()
            .unwrap();
        assert_eq!(expected_parser.language, parser.language);
        assert_eq!(expected_parser.gazetteer_parser, parser.gazetteer_parser);
        assert_eq!(
            expected_parser.rustling_entity_kinds,
            parser.rustling_entity_kinds
        );
    }
}
