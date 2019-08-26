use crate::conversion::gazetteer_entities::convert_to_slot_value;
use crate::errors::*;
use failure::{format_err, ResultExt};
pub use gazetteer_entity_parser::{
    EntityValue, Parser as EntityParser, ParserBuilder as EntityParserBuilder,
};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use snips_nlu_ontology::{BuiltinEntity, BuiltinGazetteerEntityKind, IntoBuiltinEntityKind};
use snips_nlu_utils::string::substring_with_char_range;
use std::fmt::Debug;
use std::fs;
use std::fs::File;
use std::ops::Range;
use std::path::Path;

pub trait EntityIdentifier:
    Clone + Debug + PartialEq + Serialize + DeserializeOwned + Sized
{
    fn try_from_identifier(identifier: String) -> Result<Self>;
    fn into_identifier(self) -> String;
}

impl EntityIdentifier for String {
    fn try_from_identifier(identifier: String) -> Result<Self> {
        Ok(identifier)
    }

    fn into_identifier(self) -> String {
        self
    }
}

impl EntityIdentifier for BuiltinGazetteerEntityKind {
    fn try_from_identifier(identifier: String) -> Result<Self> {
        BuiltinGazetteerEntityKind::from_identifier(&identifier)
    }

    fn into_identifier(self) -> String {
        self.identifier().to_string()
    }
}

#[derive(PartialEq, Debug)]
pub struct GazetteerParser<T>
where
    T: EntityIdentifier,
{
    entity_parsers: Vec<GazetteerEntityParser<T>>,
}

impl GazetteerParser<BuiltinGazetteerEntityKind> {
    pub fn extend_gazetteer_entity(
        &mut self,
        entity_kind: BuiltinGazetteerEntityKind,
        entity_values: impl Iterator<Item = EntityValue>,
    ) -> Result<()> {
        self.entity_parsers
            .iter_mut()
            .find(|entity_parser| entity_parser.entity_identifier == entity_kind)
            .map(|entity_parser| entity_parser.parser.prepend_values(entity_values.collect()))
            .ok_or_else(|| {
                format_err!(
                    "Cannot find gazetteer parser for entity '{:?}'",
                    entity_kind
                )
            })
    }
}

#[derive(PartialEq, Debug)]
struct GazetteerEntityParser<T>
where
    T: EntityIdentifier,
{
    entity_identifier: T,
    parser: EntityParser,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GazetteerParserBuilder {
    pub entity_parsers: Vec<GazetteerEntityParserBuilder>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GazetteerEntityParserBuilder {
    pub entity_identifier: String,
    pub entity_parser: EntityParserBuilder,
}

impl GazetteerParserBuilder {
    pub fn build<T>(self) -> Result<GazetteerParser<T>>
    where
        T: EntityIdentifier,
    {
        let entity_parsers = self
            .entity_parsers
            .into_iter()
            .map(|parser_builder| parser_builder.build())
            .collect::<Result<_>>()?;
        Ok(GazetteerParser { entity_parsers })
    }
}

impl GazetteerEntityParserBuilder {
    fn build<T>(self) -> Result<GazetteerEntityParser<T>>
    where
        T: EntityIdentifier,
    {
        Ok(GazetteerEntityParser {
            entity_identifier: T::try_from_identifier(self.entity_identifier)?,
            parser: self.entity_parser.build()?,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct GazetteerEntityMatch<T>
where
    T: EntityIdentifier,
{
    pub value: String,
    pub resolved_value: String,
    pub alternative_resolved_values: Vec<String>,
    pub range: Range<usize>,
    pub entity_identifier: T,
}

impl<T> GazetteerParser<T>
where
    T: EntityIdentifier,
{
    pub fn extract_entities(
        &self,
        sentence: &str,
        filter_entities: Option<&[T]>,
    ) -> Result<Vec<GazetteerEntityMatch<T>>> {
        Ok(self
            .entity_parsers
            .iter()
            .filter(|&parser| {
                filter_entities
                    .map(|kinds| kinds.contains(&parser.entity_identifier))
                    .unwrap_or(true)
            })
            .map(|parser| {
                Ok(parser
                    .parser
                    .run(&sentence.to_lowercase())?
                    .into_iter()
                    .map(|parsed_value| GazetteerEntityMatch {
                        value: substring_with_char_range(sentence.to_string(), &parsed_value.range),
                        range: parsed_value.range,
                        resolved_value: parsed_value.resolved_value.resolved,
                        alternative_resolved_values: parsed_value
                            .alternatives
                            .into_iter()
                            .map(|v| v.resolved)
                            .collect(),
                        entity_identifier: parser.entity_identifier.clone(),
                    })
                    .collect::<Vec<_>>())
            })
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .flat_map(|v| v)
            .collect())
    }
}

impl GazetteerParser<BuiltinGazetteerEntityKind> {
    pub fn extract_builtin_entities(
        &self,
        sentence: &str,
        filter_entities: Option<&[BuiltinGazetteerEntityKind]>,
    ) -> Result<Vec<BuiltinEntity>> {
        Ok(self
            .extract_entities(sentence, filter_entities)?
            .into_iter()
            .map(|entity_match| {
                let entity_identifier = entity_match.entity_identifier;
                let alternatives = entity_match
                    .alternative_resolved_values
                    .into_iter()
                    .map(|alternative| convert_to_slot_value(alternative, entity_identifier))
                    .collect();
                BuiltinEntity {
                    value: entity_match.value,
                    range: entity_match.range,
                    entity: convert_to_slot_value(entity_match.resolved_value, entity_identifier),
                    alternatives,
                    entity_kind: entity_identifier.into_builtin_kind(),
                }
            })
            .collect())
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct GazetteerParserMetadata {
    pub parsers_metadata: Vec<EntityParserMetadata>,
}

#[derive(Serialize, Deserialize)]
pub struct EntityParserMetadata {
    pub entity_identifier: String,
    pub entity_parser: String,
}

impl<T> GazetteerParser<T>
where
    T: EntityIdentifier,
{
    pub fn persist<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        fs::create_dir(path.as_ref()).with_context(|_| {
            format!(
                "Cannot create gazetteer parser directory at path: {:?}",
                path.as_ref()
            )
        })?;
        let mut gazetteer_parser_metadata = GazetteerParserMetadata::default();
        for (index, entity_parser) in self.entity_parsers.iter().enumerate() {
            let parser_directory = format!("parser_{}", index + 1);
            let parser_path = path.as_ref().join(&parser_directory);
            let entity_identifier = entity_parser.entity_identifier.clone().into_identifier();
            entity_parser.parser.dump(parser_path).with_context(|_| {
                format!(
                    "Cannot dump entity parser for entity '{}'",
                    &entity_identifier
                )
            })?;
            gazetteer_parser_metadata
                .parsers_metadata
                .push(EntityParserMetadata {
                    entity_identifier,
                    entity_parser: parser_directory,
                })
        }
        let metadata_path = path.as_ref().join("metadata.json");
        let metadata_file = File::create(&metadata_path).with_context(|_| {
            format!(
                "Cannot create metadata file for gazetteer parser at path: {:?}",
                metadata_path
            )
        })?;
        serde_json::to_writer_pretty(metadata_file, &gazetteer_parser_metadata)
            .with_context(|_| "Cannot serialize gazetteer parser metadata")?;
        Ok(())
    }
}

impl<T> GazetteerParser<T>
where
    T: EntityIdentifier,
{
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self> {
        let metadata_path = path.as_ref().join("metadata.json");
        let metadata_file = File::open(&metadata_path).with_context(|_| {
            format!(
                "Cannot open metadata file for gazetteer parser at path: {:?}",
                metadata_path
            )
        })?;
        let metadata: GazetteerParserMetadata = serde_json::from_reader(metadata_file)
            .with_context(|_| "Cannot deserialize gazetteer parser metadata")?;
        let entity_parsers = metadata
            .parsers_metadata
            .into_iter()
            .map(|entity_parser_metadata| {
                let parser = EntityParser::from_folder(
                    path.as_ref().join(&entity_parser_metadata.entity_parser),
                )
                .with_context(|_| {
                    format!(
                        "Cannot create entity parser from path: {}",
                        entity_parser_metadata.entity_parser
                    )
                })?;
                Ok(GazetteerEntityParser {
                    entity_identifier: T::try_from_identifier(
                        entity_parser_metadata.entity_identifier,
                    )?,
                    parser,
                })
            })
            .collect::<Result<_>>()?;
        Ok(Self { entity_parsers })
    }
}

#[cfg(test)]
mod test {
    use gazetteer_entity_parser::{gazetteer, EntityValue, Gazetteer, LicenseInfo, ParserBuilder};
    use snips_nlu_ontology::{
        BuiltinEntityKind, BuiltinGazetteerEntityKind, SlotValue, StringValue,
    };
    use tempfile::tempdir;

    use crate::test_utils::test_path;

    use super::*;

    fn get_test_custom_gazetteer_parser() -> GazetteerParser<String> {
        let artist_entity_parser_builder =
            get_music_artist_parser_builder().license_info(LicenseInfo {
                filename: "LICENSE".to_string(),
                content: "Some license content\nhere\n".to_string(),
            });
        let track_entity_parser_builder = get_music_track_parser_builder();
        let gazetteer_parser_builder = GazetteerParserBuilder {
            entity_parsers: vec![
                GazetteerEntityParserBuilder {
                    entity_identifier: "music_artist".to_string(),
                    entity_parser: artist_entity_parser_builder,
                },
                GazetteerEntityParserBuilder {
                    entity_identifier: "music_track".to_string(),
                    entity_parser: track_entity_parser_builder,
                },
            ],
        };
        gazetteer_parser_builder.build().unwrap()
    }

    fn get_test_builtin_gazetteer_parser() -> GazetteerParser<BuiltinGazetteerEntityKind> {
        let artist_entity_parser_builder = get_music_artist_parser_builder();
        let track_entity_parser_builder = get_music_track_parser_builder();
        let gazetteer_parser_builder = GazetteerParserBuilder {
            entity_parsers: vec![
                GazetteerEntityParserBuilder {
                    entity_identifier: "snips/musicArtist".to_string(),
                    entity_parser: artist_entity_parser_builder,
                },
                GazetteerEntityParserBuilder {
                    entity_identifier: "snips/musicTrack".to_string(),
                    entity_parser: track_entity_parser_builder,
                },
            ],
        };
        gazetteer_parser_builder.build().unwrap()
    }

    fn get_music_track_parser_builder() -> ParserBuilder {
        let track_entity_parser_builder = EntityParserBuilder::default()
            .minimum_tokens_ratio(0.7)
            .gazetteer(gazetteer!(
                (
                    "harder better faster stronger",
                    "Harder Better Faster Stronger"
                ),
                ("what s my age again", "What's my age again"),
            ));
        track_entity_parser_builder
    }

    fn get_music_artist_parser_builder() -> ParserBuilder {
        EntityParserBuilder::default()
            .minimum_tokens_ratio(0.6)
            .gazetteer(gazetteer!(
                ("the rolling stones", "The Rolling Stones"),
                ("blink one eight two", "Blink 182"),
            ))
    }

    fn get_ambiguous_music_artist_parser_builder() -> ParserBuilder {
        EntityParserBuilder::default()
            .minimum_tokens_ratio(0.6)
            .gazetteer(gazetteer!(
                ("the rolling stones", "The Rolling Stones"),
                ("the crying stones", "The Crying Stones"),
                ("blink one eight two", "Blink 182"),
            ))
    }

    #[test]
    fn test_should_parse_above_threshold() {
        // Given
        let gazetteer_parser = get_test_custom_gazetteer_parser();

        // When
        let input = "I want to listen to the track harder better faster please";
        let result = gazetteer_parser.extract_entities(input, None);

        // Then
        let expected_match = GazetteerEntityMatch {
            value: "harder better faster".to_string(),
            resolved_value: "Harder Better Faster Stronger".to_string(),
            alternative_resolved_values: vec![],
            range: 30..50,
            entity_identifier: "music_track".to_string(),
        };
        assert_eq!(Some(vec![expected_match]), result.ok());
    }

    #[test]
    fn test_should_not_parse_below_threshold() {
        // Given
        let gazetteer_parser = get_test_custom_gazetteer_parser();

        // When
        let input = "I want to listen to the track harder better please";
        let result = gazetteer_parser.extract_entities(input, None);

        // Then
        assert_eq!(Some(vec![]), result.ok());
    }

    #[test]
    fn test_should_parse_using_scope() {
        // Given
        let gazetteer_parser = get_test_custom_gazetteer_parser();

        // When
        let input = "I want to listen to what s my age again by blink one eight two";
        let artist_scope: &[String] = &["music_artist".to_string()];
        let result_artist = gazetteer_parser.extract_entities(input, Some(artist_scope));
        let track_scope: &[String] = &["music_track".to_string()];
        let result_track = gazetteer_parser.extract_entities(input, Some(track_scope));

        // Then
        let expected_artist_match = GazetteerEntityMatch {
            value: "blink one eight two".to_string(),
            resolved_value: "Blink 182".to_string(),
            alternative_resolved_values: vec![],
            range: 43..62,
            entity_identifier: "music_artist".to_string(),
        };

        let expected_track_match = GazetteerEntityMatch {
            value: "what s my age again".to_string(),
            resolved_value: "What's my age again".to_string(),
            alternative_resolved_values: vec![],
            range: 20..39,
            entity_identifier: "music_track".to_string(),
        };
        assert_eq!(Some(vec![expected_artist_match]), result_artist.ok());
        assert_eq!(Some(vec![expected_track_match]), result_track.ok());
    }

    #[test]
    fn test_should_parse_with_alternatives() {
        // Given
        let gazetteer_parser = GazetteerParserBuilder {
            entity_parsers: vec![GazetteerEntityParserBuilder {
                entity_identifier: "music_artist".to_string(),
                entity_parser: get_ambiguous_music_artist_parser_builder(),
            }],
        }
        .build()
        .unwrap();

        // When
        let input = "I want to listen to the stones";
        let result = gazetteer_parser.extract_entities(input, None);

        // Then
        let expected_match = GazetteerEntityMatch {
            value: "the stones".to_string(),
            resolved_value: "The Rolling Stones".to_string(),
            alternative_resolved_values: vec!["The Crying Stones".to_string()],
            range: 20..30,
            entity_identifier: "music_artist".to_string(),
        };
        assert_eq!(Some(vec![expected_match]), result.ok());
    }

    #[test]
    fn test_should_parse_with_builtin_entities() {
        // Given
        let builtin_gazetteer_parser = get_test_builtin_gazetteer_parser();

        // When
        let input = "I want to listen to the track harder better faster please";
        let result = builtin_gazetteer_parser.extract_builtin_entities(input, None);

        // Then
        let expected_match = BuiltinEntity {
            value: "harder better faster".to_string(),
            entity: SlotValue::MusicTrack(StringValue {
                value: "Harder Better Faster Stronger".to_string(),
            }),
            alternatives: vec![],
            range: 30..50,
            entity_kind: BuiltinEntityKind::MusicTrack,
        };
        assert_eq!(Some(vec![expected_match]), result.ok());
    }

    #[test]
    fn test_should_persist_custom_gazetteer_parser() {
        // Given
        let gazetteer_parser = get_test_custom_gazetteer_parser();
        let temp_dir = tempdir().unwrap();
        let parser_dir = temp_dir.path().join("custom_gazetteer_parser");

        // When
        gazetteer_parser.persist(&parser_dir).unwrap();
        let loaded_gazetteer_parser = GazetteerParser::from_path(&parser_dir).unwrap();

        // Then
        let expected_license_path = parser_dir.join("parser_1").join("LICENSE");
        assert!(expected_license_path.exists());
        assert_eq!(gazetteer_parser, loaded_gazetteer_parser);
    }

    #[test]
    fn test_should_load_custom_gazetteer_parser_from_path() {
        // Given
        let path = test_path().join("custom_gazetteer_parser");

        // When
        let parser = GazetteerParser::from_path(path);

        // Then
        let expected_parser = get_test_custom_gazetteer_parser();
        assert_eq!(Some(expected_parser), parser.ok());
    }

    #[test]
    fn test_should_persist_builtin_gazetteer_parser() {
        // Given
        let gazetteer_parser = get_test_builtin_gazetteer_parser();
        let temp_dir = tempdir().unwrap();
        let parser_dir = temp_dir.path().join("builtin_gazetteer_parser");

        // When
        gazetteer_parser.persist(&parser_dir).unwrap();
        let loaded_gazetteer_parser = GazetteerParser::from_path(&parser_dir).unwrap();

        // Then
        assert_eq!(gazetteer_parser, loaded_gazetteer_parser);
    }

    #[test]
    fn test_should_load_builtin_gazetteer_parser_from_path() {
        // Given
        let path = test_path().join("builtin_gazetteer_parser");

        // When
        let parser = GazetteerParser::from_path(path);

        // Then
        let expected_parser = get_test_builtin_gazetteer_parser();
        assert_eq!(Some(expected_parser), parser.ok());
    }
}
