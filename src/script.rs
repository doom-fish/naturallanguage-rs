//! `NLScript` typed string constants.

use crate::string_enum::string_extensible_enum;

string_extensible_enum! {
    /// A BCP-47 script tag used by Apple's `NaturalLanguage` framework.
    pub struct Script {
        /// Undetermined script.
        UNDETERMINED = "Zzzz";
        ARABIC = "Arab";
        ARMENIAN = "Armn";
        BENGALI = "Beng";
        CANADIAN_ABORIGINAL_SYLLABICS = "Cans";
        CHEROKEE = "Cher";
        CYRILLIC = "Cyrl";
        DEVANAGARI = "Deva";
        ETHIOPIC = "Ethi";
        GEORGIAN = "Geor";
        GREEK = "Grek";
        GUJARATI = "Gujr";
        GURMUKHI = "Guru";
        HEBREW = "Hebr";
        JAPANESE = "Jpan";
        KANNADA = "Knda";
        KHMER = "Khmr";
        KOREAN = "Kore";
        LAO = "Laoo";
        LATIN = "Latn";
        MALAYALAM = "Mlym";
        MONGOLIAN = "Mong";
        MYANMAR = "Mymr";
        ORIYA = "Orya";
        SIMPLIFIED_CHINESE = "Hans";
        SINHALA = "Sinh";
        TAMIL = "Taml";
        TELUGU = "Telu";
        THAI = "Thai";
        TIBETAN = "Tibt";
        TRADITIONAL_CHINESE = "Hant";
    }
}
