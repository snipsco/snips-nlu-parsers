use snips_nlu_ontology::{BuiltinEntityKind, IntoBuiltinEntityKind, Language};

#[rustfmt::skip::macros(language_support)]
pub fn supported_languages<T: IntoBuiltinEntityKind>(entity_kind: &T) -> &'static [Language] {
    macro_rules! language_support {
        ($(($entity_kind:ident, [$($language:ident),*]),)*) => {
            match entity_kind.into_builtin_kind() {
                $(
                    BuiltinEntityKind::$entity_kind => &[$(Language::$language,)*],
                )*
            }
        }
    }

    language_support!(
        (AmountOfMoney, [DE, EN, ES, FR, IT, JA, KO, PT_BR, PT_PT]),
        (Duration,      [DE, EN, ES, FR, IT, JA, KO, PT_BR, PT_PT]),
        (Number,        [DE, EN, ES, FR, IT, JA, KO, PT_BR, PT_PT]),
        (Ordinal,       [DE, EN, ES, FR, IT, JA, KO, PT_BR, PT_PT]),
        (Temperature,   [DE, EN, ES, FR, IT, JA, KO, PT_BR, PT_PT]),
        (Datetime,      [DE, EN, ES, FR, IT, JA, KO, PT_BR, PT_PT]),
        (Date,          [EN, FR]),
        (Time,          [EN, FR]),
        (DatePeriod,    [EN, FR]),
        (TimePeriod,    [EN, FR]),
        (Percentage,    [DE, EN, ES, FR, IT, JA, PT_BR, PT_PT]),
        (MusicAlbum,    [DE, EN, ES, FR, IT, JA, PT_BR, PT_PT]),
        (MusicArtist,   [DE, EN, ES, FR, IT, JA, PT_BR, PT_PT]),
        (MusicTrack,    [DE, EN, ES, FR, IT, JA, PT_BR, PT_PT]),
        (City,          [DE, EN, ES, FR, IT, JA, PT_BR, PT_PT]),
        (Country,       [DE, EN, ES, FR, IT, JA, PT_BR, PT_PT]),
        (Region,        [DE, EN, ES, FR, IT, JA, PT_BR, PT_PT]),
    )
}
