//! `NLLanguage` typed string constants.

use crate::string_enum::string_extensible_enum;

string_extensible_enum! {
    /// A BCP-47 language tag used by Apple's `NaturalLanguage` framework.
    pub struct Language {
        /// Undetermined language.
        UNDETERMINED = "und";
        AMHARIC = "am";
        ARABIC = "ar";
        ARMENIAN = "hy";
        BENGALI = "bn";
        BULGARIAN = "bg";
        BURMESE = "my";
        CATALAN = "ca";
        CHEROKEE = "chr";
        CROATIAN = "hr";
        CZECH = "cs";
        DANISH = "da";
        DUTCH = "nl";
        ENGLISH = "en";
        FINNISH = "fi";
        FRENCH = "fr";
        GEORGIAN = "ka";
        GERMAN = "de";
        GREEK = "el";
        GUJARATI = "gu";
        HEBREW = "he";
        HINDI = "hi";
        HUNGARIAN = "hu";
        ICELANDIC = "is";
        INDONESIAN = "id";
        ITALIAN = "it";
        JAPANESE = "ja";
        KANNADA = "kn";
        KHMER = "km";
        KOREAN = "ko";
        LAO = "lo";
        MALAY = "ms";
        MALAYALAM = "ml";
        MARATHI = "mr";
        MONGOLIAN = "mn";
        NORWEGIAN = "nb";
        ORIYA = "or";
        PERSIAN = "fa";
        POLISH = "pl";
        PORTUGUESE = "pt";
        PUNJABI = "pa";
        ROMANIAN = "ro";
        RUSSIAN = "ru";
        SIMPLIFIED_CHINESE = "zh-Hans";
        SINHALESE = "si";
        SLOVAK = "sk";
        SPANISH = "es";
        SWEDISH = "sv";
        TAMIL = "ta";
        TELUGU = "te";
        THAI = "th";
        TIBETAN = "bo";
        TRADITIONAL_CHINESE = "zh-Hant";
        TURKISH = "tr";
        UKRAINIAN = "uk";
        URDU = "ur";
        VIETNAMESE = "vi";
        /// Added in macOS 13.
        KAZAKH = "kk";
    }
}
