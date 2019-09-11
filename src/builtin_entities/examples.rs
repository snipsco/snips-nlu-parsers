use snips_nlu_ontology::{BuiltinEntityKind, IntoBuiltinEntityKind};

pub fn de_examples<T: IntoBuiltinEntityKind>(entity_kind: &T) -> &'static [&'static str] {
    match entity_kind.into_builtin_kind() {
        BuiltinEntityKind::AmountOfMoney => &["10$", "ungefähr 5€", "zwei tausend Dollar"],
        BuiltinEntityKind::Duration => &[
            "2stdn",
            "drei monate",
            "ein halbe Stunde",
            "8 Jahre und zwei Tage",
        ],
        BuiltinEntityKind::Number => &[
            "2001",
            "einundzwanzig",
            "zwei tausend",
            "zwei tausend und drei",
        ],
        BuiltinEntityKind::Ordinal => &["Erste", "der zweite", "zwei und zwanzigster"],
        BuiltinEntityKind::Temperature => &[
            "70K",
            "3°C",
            "Dreiundzwanzig Grad",
            "zweiunddreißig Grad Fahrenheit",
        ],
        BuiltinEntityKind::Datetime => &[
            "Heute",
            "16.30 Uhr",
            "in 1 Stunde",
            "dritter Dienstag im Juni",
        ],
        BuiltinEntityKind::Date => &[],
        BuiltinEntityKind::Time => &[],
        BuiltinEntityKind::DatePeriod => &[],
        BuiltinEntityKind::TimePeriod => &[],
        BuiltinEntityKind::Percentage => &[
            "25%",
            "zwanzig Prozent",
            "zwei tausend und fünfzig Prozent",
        ],
        BuiltinEntityKind::MusicAlbum => &["Discovery"],
        BuiltinEntityKind::MusicArtist => &["Daft Punk"],
        BuiltinEntityKind::MusicTrack => &["Harder Better Faster Stronger"],
        BuiltinEntityKind::City => &["Berlin", "Essen", "Zürich", "Paris"],
        BuiltinEntityKind::Country => &["Frankreich"],
        BuiltinEntityKind::Region => &["Bayern", "Tirol"],
    }
}

pub fn en_examples<T: IntoBuiltinEntityKind>(entity_kind: &T) -> &'static [&'static str] {
    match entity_kind.into_builtin_kind() {
        BuiltinEntityKind::AmountOfMoney => &[
            "$10",
            "six euros",
            "around 5€",
            "ten dollars and five cents",
        ],
        BuiltinEntityKind::Duration => &[
            "1h",
            "during two minutes",
            "for 20 seconds",
            "3 months",
            "half an hour",
            "8 years and two days",
        ],
        BuiltinEntityKind::Number => &["2001", "twenty one", "three hundred and four"],
        BuiltinEntityKind::Ordinal => &["1st", "the second", "the twenty third"],
        BuiltinEntityKind::Temperature => &[
            "70K",
            "3°C",
            "Twenty three degrees",
            "one hundred degrees fahrenheit",
        ],
        BuiltinEntityKind::Datetime => &[
            "tomorrow at 9pm",
            "today",
            "on october 1st at 10am",
            "at 8 a.m.",
            "4:30 pm",
            "in 1 hour",
            "the 3rd tuesday of June",
        ],
        BuiltinEntityKind::Date => &[
            "today",
            "on Wednesday",
            "March 26th",
            "saturday january 19",
            "monday 15th april 2019",
            "the day after tomorrow",
        ],
        BuiltinEntityKind::Time => &[
            "now",
            "at noon",
            "at 8 a.m.",
            "4:30 pm",
            "in one hour",
            "for ten o'clock",
            "at ten in the evening",
        ],
        BuiltinEntityKind::DatePeriod => &[
            "january",
            "2019",
            "from monday to friday",
            "from wednesday 27th to saturday 30th",
            "this week",
        ],
        BuiltinEntityKind::TimePeriod => &[
            // "tonight" currently not interpreted as a TimePeriod because intersected with
            // today's date, which makes it interpreted as a date+time (will be fixed)
            //"tonight",
            // "this morning" currently not interpreted as a TimePeriod (same reason)
            // "this morning",
            "until dinner",
            "from five to ten",
            // This is currently bugged + interpreted as TimePeriod (same reason, with "this")
            // "this evening after eight thirty",
            "by the end of the day",
        ],
        BuiltinEntityKind::Percentage => {
            &["25%", "twenty percent", "two hundred and fifty percents"]
        }
        BuiltinEntityKind::MusicAlbum => &["Discovery"],
        BuiltinEntityKind::MusicArtist => &["Daft Punk"],
        BuiltinEntityKind::MusicTrack => &["Harder Better Faster Stronger"],
        BuiltinEntityKind::City => &["San Francisco", "Los Angeles", "Beijing", "Paris"],
        BuiltinEntityKind::Country => &["France"],
        BuiltinEntityKind::Region => &["California", "Washington"],
    }
}

pub fn es_examples<T: IntoBuiltinEntityKind>(entity_kind: &T) -> &'static [&'static str] {
    match entity_kind.into_builtin_kind() {
        BuiltinEntityKind::AmountOfMoney => &[
            "$10",
            "15€",
            "cinco euros",
            "16,65 €",
            "diez dólares y cinco centavos",
            "treinta y tres mil millones de rupias",
            "ocho cientos bitcoins",
            "noventa coronas danesas",
            "845584 francos suizos",
        ],
        BuiltinEntityKind::Duration => &[
            "1h",
            "3 meses",
            "diez minutos",
            "media hora",
            "ciento dos minutos",
            "8 años y dos dias",
            "un año catorce semanas y tres horas",
            "tres cuartos de hora",
            // TODO: Add these examples when they are supported by the BuiltinEntityParser
            //"durante los próximos dos años",
        ],
        BuiltinEntityKind::Number => &[
            "2001",
            "dieciocho",
            "ciento dos",
            "tres mil nueve",
            "ciento cuarenta y nueve",
            "cuatro cientos dieciséis",
            "quinientos noventa y uno",
            "mil novecientos cuarenta y cuatro",
        ],
        BuiltinEntityKind::Ordinal => &[
            "primer",
            "decima",
            // TODO: Add these examples when they are supported by the BuiltinEntityParser
            // "vigésimo primero",
        ],
        BuiltinEntityKind::Temperature => &[
            "70 grados kelvin",
            "3°C",
            "veintitrés grados",
            "tres mil grados fahrenheit",
            "veinte grados centígrados",
            "setecientos ochenta y nueve kelvin",
            "quince grados bajo cero",
            "-459,67 °F",
        ],
        BuiltinEntityKind::Datetime => &[
            "hoy",
            "esta noche",
            "a la 1:30",
            "el primer jueves de junio",
            "el 30 de julio por la tarde",
            "la primera semana de la primavera",
            "de cinco a ocho de la tarde",
            // TODO: Add these examples when they are supported by the BuiltinEntityParser
            // "las próximas navidades",
        ],
        BuiltinEntityKind::Date => &[],
        BuiltinEntityKind::Time => &[],
        BuiltinEntityKind::DatePeriod => &[],
        BuiltinEntityKind::TimePeriod => &[],
        BuiltinEntityKind::Percentage => &[
            "25%",
            "quince por ciento",
            "20 por ciento",
            "tres por ciento",
            "veinte por ciento",
            "tres mil por ciento",
            "cien por cien",
            "setenta y cinco por ciento",
        ],
        BuiltinEntityKind::MusicAlbum => &["Discovery"],
        BuiltinEntityKind::MusicArtist => &["Daft Punk"],
        BuiltinEntityKind::MusicTrack => &["Harder Better Faster Stronger"],
        BuiltinEntityKind::City => &["Madrid", "Barcelona", "Bilbao", "Paris"],
        BuiltinEntityKind::Country => &["Francia"],
        BuiltinEntityKind::Region => &["Andalusia", "Catalonia"],
    }
}

pub fn fr_examples<T: IntoBuiltinEntityKind>(entity_kind: &T) -> &'static [&'static str] {
    match entity_kind.into_builtin_kind() {
        BuiltinEntityKind::AmountOfMoney => &[
            "10$",
            "environ 5€",
            "six euros",
            "dix dollars et cinq centimes",
        ],
        BuiltinEntityKind::Duration => &[
            "1h",
            "pendant vingt minutes",
            "durant 3 secondes",
            "3 mois",
            "une demi heure",
            "8 ans et deux jours",
        ],
        BuiltinEntityKind::Number => &[
            "2001",
            "vingt deux",
            "deux cent trois",
            "quatre vingt dix neuf",
        ],
        BuiltinEntityKind::Ordinal => &[
            "1er",
            "43ème",
            "le deuxième",
            "cinq centième",
            "vingt et unieme",
        ],
        BuiltinEntityKind::Temperature => &[
            "70K",
            "3°C",
            "vingt trois degrés",
            "45 degrés celsius",
            "deux cent degrés Fahrenheit",
        ],
        BuiltinEntityKind::Datetime => &[
            "Aujourd'hui",
            "à 14:30",
            "demain matin",
            "hier vers 10 heures",
            "dans 1 heure",
            "le premier jeudi de Juin",
        ],
        BuiltinEntityKind::Date => &[
            "aujourd'hui",
            "mercredi",
            "le 26 mars",
            "samedi 19 janvier",
            "lundi 15 avril 2019",
            "après demain",
        ],
        BuiltinEntityKind::Time => &[
            "maintenant",
            "à midi",
            "à 8h00",
            "16h30",
            "dans une heure",
            "à 10h du soir",
        ],
        BuiltinEntityKind::DatePeriod => &[
            "janvier",
            "2019",
            "de lundi à vendredi",
            "de mercredi 27 à samedi 30",
            "cette semaine",
        ],
        BuiltinEntityKind::TimePeriod => &[
            "ce soir",
            "jusqu'au diner",
            "ce matin",
            "de 5 à 6",
            "jusqu'à 16h",
        ],
        BuiltinEntityKind::Percentage => &["25%", "20 pourcents", "quatre vingt dix pourcents"],
        BuiltinEntityKind::MusicAlbum => &["Discovery"],
        BuiltinEntityKind::MusicArtist => &["Daft Punk"],
        BuiltinEntityKind::MusicTrack => &["Harder Better Faster Stronger"],
        BuiltinEntityKind::City => &["Paris", "Brest", "Bruxelles", "Pékin", "Londres"],
        BuiltinEntityKind::Country => &["France"],
        BuiltinEntityKind::Region => &["Bretagne", "Corse", "Province de Liège"],
    }
}

pub fn it_examples<T: IntoBuiltinEntityKind>(entity_kind: &T) -> &'static [&'static str] {
    match entity_kind.into_builtin_kind() {
        BuiltinEntityKind::AmountOfMoney => &[
            "$10",
            "15€",
            "cinque euro",
            "sei mila euro",
            "quattordici franchi svizzeri",
            "cinquanta sette dollari australiani",
            "dieci dollari e cinque centesimi",
            "cento diciotto mila corone danesi",
            "sessant uno euro e novanta nove centesimi",
        ],
        BuiltinEntityKind::Duration => &[
            // TODO: Add these examples when they are supported by the BuiltinEntityParser
            // "1h",
            "per un mese",
            "durante tre settimane",
            "durante un quarto d'ora",
            "per tre anni e mezzo",
            "per quattro ore e venti due minuti",
            "3 mesi",
            "dieci minuti",
            "cento due minuti",
            "8 anni e due giorni",
        ],
        BuiltinEntityKind::Number => &[
            "otto",
            "sedici",
            "cento",
            "venti due",
            "sei mila",
            "cento quaranta nove",
            "tre mila cinque cento",
            "due cento novanta tré",
            "mille otto cento cinquanta sei",
            "un milione sette cento dodici mila",
            "sessanta due mila tre cento ottanta nove",
        ],
        BuiltinEntityKind::Ordinal => &[
            "primo",
            "decima",
            // TODO: Add these examples when they are supported by the BuiltinEntityParser
            // "vent unesimo",
            // "novanta quattresima",
            // "tre cento settantesima",
        ],
        BuiltinEntityKind::Temperature => &[
            "3°C",
            "tre gradi",
            "quindici gradi celsius",
            "settant uno fahrenheit",
            "due cento novanta cinque gradi kelvin",
        ],
        BuiltinEntityKind::Datetime => &[
            "domattina",
            "giovedì prossimo",
            "a febbraio",
            "tra quindici giorni",
            "il dodici marzo 2020",
            "dopodomani a mezzanotte e dieci",
            "alle sette e mezza di sera",
            "alle 1:30",
            "il primo giovedí di giugno",
        ],
        BuiltinEntityKind::Date => &[],
        BuiltinEntityKind::Time => &[],
        BuiltinEntityKind::DatePeriod => &[],
        BuiltinEntityKind::TimePeriod => &[],
        BuiltinEntityKind::Percentage => &[
            "25%",
            "due percento",
            "cento percento",
            "20 percento",
            "tre mila percento",
            "sessanta sei percento",
            "diciotto per cento",
            "venti nove per cento",
        ],
        BuiltinEntityKind::MusicAlbum => &["Discovery"],
        BuiltinEntityKind::MusicArtist => &["Daft Punk"],
        BuiltinEntityKind::MusicTrack => &["Harder Better Faster Stronger"],
        BuiltinEntityKind::City => &["San Francisco", "Roma", "Agrigente"],
        BuiltinEntityKind::Country => &["Italia"],
        BuiltinEntityKind::Region => &["Sardinia", "Sicilia"],
    }
}

pub fn pt_br_examples<T: IntoBuiltinEntityKind>(entity_kind: &T) -> &'static [&'static str] {
    match entity_kind.into_builtin_kind() {
        BuiltinEntityKind::AmountOfMoney => &[
            "10$",
            "15€",
            "cinco euros",
            "16,65 €",
            "dois euros e cinco centavos",
            "dez libras esterlinas",
            "845584 francos suíços",
        ],
        BuiltinEntityKind::Datetime => &["hoje"],
        BuiltinEntityKind::Date => &[],
        BuiltinEntityKind::Time => &[],
        BuiltinEntityKind::DatePeriod => &[],
        BuiltinEntityKind::TimePeriod => &[],
        BuiltinEntityKind::Duration => &[
            "1 hora",
            "3 meses",
            "dez minutos",
            "meia hora",
            "oito anos e dois semanas",
            "um ano quatro semanas e tres horas",
        ],
        BuiltinEntityKind::Number => &["2001"],
        BuiltinEntityKind::Ordinal => &["primeira"],
        BuiltinEntityKind::Temperature => &[
            "70 graus kelvin",
            "3°C",
            "dez graus",
            "quatro graus centígrados",
            "-459,67 °F",
        ],
        BuiltinEntityKind::Percentage => &["25%"],
        BuiltinEntityKind::MusicAlbum => &["Discovery"],
        BuiltinEntityKind::MusicArtist => &["Daft Punk"],
        BuiltinEntityKind::MusicTrack => &["Harder Better Faster Stronger"],
        BuiltinEntityKind::City => &["São Paulo", "Rio", "Los Angeles", "Paris"],
        BuiltinEntityKind::Country => &["Brasil"],
        BuiltinEntityKind::Region => &["Bahia", "Amazonas"],
    }
}

pub fn pt_pt_examples<T: IntoBuiltinEntityKind>(entity_kind: &T) -> &'static [&'static str] {
    match entity_kind.into_builtin_kind() {
        BuiltinEntityKind::AmountOfMoney => &[
            "10$",
            "15€",
            "cinco euros",
            "16,65 €",
            "dois euros e cinco centavos",
            "dez libras esterlinas",
            "845584 francos suíços",
        ],
        BuiltinEntityKind::Datetime => &["hoje"],
        BuiltinEntityKind::Date => &[],
        BuiltinEntityKind::Time => &[],
        BuiltinEntityKind::DatePeriod => &[],
        BuiltinEntityKind::TimePeriod => &[],
        BuiltinEntityKind::Duration => &[
            "1 hora",
            "3 meses",
            "dez minutos",
            "meia hora",
            "oito anos e dois semanas",
            "um ano quatro semanas e tres horas",
        ],
        BuiltinEntityKind::Number => &["2001"],
        BuiltinEntityKind::Ordinal => &["primeira"],
        BuiltinEntityKind::Temperature => &[
            "70 graus kelvin",
            "3°C",
            "dez graus",
            "quatro graus centígrados",
            "-459,67 °F",
        ],
        BuiltinEntityKind::Percentage => &["25%"],
        BuiltinEntityKind::MusicAlbum => &["Discovery"],
        BuiltinEntityKind::MusicArtist => &["Daft Punk"],
        BuiltinEntityKind::MusicTrack => &["Harder Better Faster Stronger"],
        BuiltinEntityKind::City => &["Liboa", "Porto", "Amadora"],
        BuiltinEntityKind::Country => &["Portugal", "Espanha"],
        BuiltinEntityKind::Region => &["Norte", "Alentejo"],
    }
}

pub fn ja_examples<T: IntoBuiltinEntityKind>(entity_kind: &T) -> &'static [&'static str] {
    match entity_kind.into_builtin_kind() {
        BuiltinEntityKind::AmountOfMoney => &["八ドル", "五十二アメリカドル"],
        BuiltinEntityKind::Duration => &["一秒間", "五日間", "十ヶ月間"],
        BuiltinEntityKind::Number => &["十二", "二千五", "四千三百二"],
        BuiltinEntityKind::Ordinal => &["十一番目", "九十一番目"],
        BuiltinEntityKind::Temperature => &["五度", "二十五度", "マイナス十度"],
        BuiltinEntityKind::Datetime => &[
            "一昨日",
            "次の水曜日",
            "十三時三十分",
            "二千十三年十二月二十三日",
        ],
        BuiltinEntityKind::Date => &[],
        BuiltinEntityKind::Time => &[],
        BuiltinEntityKind::DatePeriod => &[],
        BuiltinEntityKind::TimePeriod => &[],
        BuiltinEntityKind::Percentage => &["十五%", "五パーセント"],
        BuiltinEntityKind::MusicAlbum => &["Discovery"],
        BuiltinEntityKind::MusicArtist => &["Daft Punk"],
        BuiltinEntityKind::MusicTrack => &["Harder Better Faster Stronger"],
        BuiltinEntityKind::City => &["パリ", "東京", "京都"],
        BuiltinEntityKind::Country => &["日本"],
        BuiltinEntityKind::Region => &["北海道", "関東地方"],
    }
}

pub fn ko_examples<T: IntoBuiltinEntityKind>(entity_kind: &T) -> &'static [&'static str] {
    match entity_kind.into_builtin_kind() {
        BuiltinEntityKind::AmountOfMoney => &["10$", "약 5 유로", "10 달러 5 센트"],
        BuiltinEntityKind::Duration => &["양일", "1시간", "3 개월"],
        BuiltinEntityKind::Number => &["2001", "삼천", "스물 둘", "천 아흔 아홉"],
        BuiltinEntityKind::Ordinal => &["첫", "첫번째"],
        BuiltinEntityKind::Temperature => &["5도", "섭씨 20도", "화씨 백 도"],
        BuiltinEntityKind::Datetime => &["오늘", "14시 30 분에", "5 월 첫째 목요일"],
        BuiltinEntityKind::Date => &[],
        BuiltinEntityKind::Time => &[],
        BuiltinEntityKind::DatePeriod => &[],
        BuiltinEntityKind::TimePeriod => &[],
        BuiltinEntityKind::Percentage => &[],
        BuiltinEntityKind::MusicAlbum => &[],
        BuiltinEntityKind::MusicArtist => &[],
        BuiltinEntityKind::MusicTrack => &[],
        BuiltinEntityKind::City => &[],
        BuiltinEntityKind::Country => &[],
        BuiltinEntityKind::Region => &[],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsable::ParsableEntityKind;
    use snips_nlu_ontology::Language;
    use std::collections::HashSet;

    #[test]
    fn test_entity_examples_should_be_provided_for_all_supported_languages() {
        for entity_kind in BuiltinEntityKind::all() {
            for language in entity_kind.supported_languages() {
                let examples = entity_kind.examples(*language);
                assert!(
                    examples.len() >= 1,
                    "No examples provided for entity '{:?}' in language '{:?}'",
                    entity_kind,
                    language
                )
            }
        }
    }

    #[test]
    fn test_entity_examples_should_not_be_provided_for_non_supported_languages() {
        for entity_kind in BuiltinEntityKind::all() {
            let all_languages: HashSet<&Language> = Language::all().into_iter().collect();
            let supported_languages: HashSet<&Language> =
                entity_kind.supported_languages().into_iter().collect();
            let non_supported_languages: HashSet<&&Language> =
                all_languages.difference(&supported_languages).collect();
            for language in non_supported_languages {
                let examples = entity_kind.examples(**language);
                assert!(
                    examples.is_empty(),
                    "No examples should be provided for entity '{:?}' in language '{:?}', as it is \
                    not supported",
                    entity_kind,
                    language
                )
            }
        }
    }
}
