# naturallanguage coverage audit (vs MacOSX26.5.sdk)

SDK_PUBLIC_SYMBOLS: 256
VERIFIED: 253
GAPS: 0
EXEMPT: 3
COVERAGE_PCT: 100.0%

Notes:
- `NLDataAsset` is absent from the current `NaturalLanguage.framework` headers, so it is not counted in `SDK_PUBLIC_SYMBOLS`.
- `VERIFIED` entries were cross-checked against the public Rust API (`src/**/*.rs`), Swift bridge thunks (`swift-bridge/Sources/**/*.swift`), `COVERAGE.md`, and `tests/api_coverage.rs`.
- `EXEMPT` entries are public SDK members intentionally skipped because Apple marks them unavailable to the bridge layer.
- The optional `async` feature adds executor-agnostic futures for the one-shot `requestAssets...` completion-handler APIs.

## 🟢 VERIFIED

| Symbol | Kind | Header | Wrapped by |
| --- | --- | --- | --- |
| `AMHARIC` | constant | `NLLanguage.h` | `Language::AMHARIC` |
| `ARABIC` | constant | `NLLanguage.h` | `Language::ARABIC` |
| `ARMENIAN` | constant | `NLLanguage.h` | `Language::ARMENIAN` |
| `BENGALI` | constant | `NLLanguage.h` | `Language::BENGALI` |
| `BULGARIAN` | constant | `NLLanguage.h` | `Language::BULGARIAN` |
| `BURMESE` | constant | `NLLanguage.h` | `Language::BURMESE` |
| `CATALAN` | constant | `NLLanguage.h` | `Language::CATALAN` |
| `CHEROKEE` | constant | `NLLanguage.h` | `Language::CHEROKEE` |
| `CROATIAN` | constant | `NLLanguage.h` | `Language::CROATIAN` |
| `CZECH` | constant | `NLLanguage.h` | `Language::CZECH` |
| `DANISH` | constant | `NLLanguage.h` | `Language::DANISH` |
| `DUTCH` | constant | `NLLanguage.h` | `Language::DUTCH` |
| `ENGLISH` | constant | `NLLanguage.h` | `Language::ENGLISH` |
| `FINNISH` | constant | `NLLanguage.h` | `Language::FINNISH` |
| `FRENCH` | constant | `NLLanguage.h` | `Language::FRENCH` |
| `GEORGIAN` | constant | `NLLanguage.h` | `Language::GEORGIAN` |
| `GERMAN` | constant | `NLLanguage.h` | `Language::GERMAN` |
| `GREEK` | constant | `NLLanguage.h` | `Language::GREEK` |
| `GUJARATI` | constant | `NLLanguage.h` | `Language::GUJARATI` |
| `HEBREW` | constant | `NLLanguage.h` | `Language::HEBREW` |
| `HINDI` | constant | `NLLanguage.h` | `Language::HINDI` |
| `HUNGARIAN` | constant | `NLLanguage.h` | `Language::HUNGARIAN` |
| `ICELANDIC` | constant | `NLLanguage.h` | `Language::ICELANDIC` |
| `INDONESIAN` | constant | `NLLanguage.h` | `Language::INDONESIAN` |
| `ITALIAN` | constant | `NLLanguage.h` | `Language::ITALIAN` |
| `JAPANESE` | constant | `NLLanguage.h` | `Language::JAPANESE` |
| `KANNADA` | constant | `NLLanguage.h` | `Language::KANNADA` |
| `KAZAKH` | constant | `NLLanguage.h` | `Language::KAZAKH` |
| `KHMER` | constant | `NLLanguage.h` | `Language::KHMER` |
| `KOREAN` | constant | `NLLanguage.h` | `Language::KOREAN` |
| `LAO` | constant | `NLLanguage.h` | `Language::LAO` |
| `MALAY` | constant | `NLLanguage.h` | `Language::MALAY` |
| `MALAYALAM` | constant | `NLLanguage.h` | `Language::MALAYALAM` |
| `MARATHI` | constant | `NLLanguage.h` | `Language::MARATHI` |
| `MONGOLIAN` | constant | `NLLanguage.h` | `Language::MONGOLIAN` |
| `NORWEGIAN` | constant | `NLLanguage.h` | `Language::NORWEGIAN` |
| `ORIYA` | constant | `NLLanguage.h` | `Language::ORIYA` |
| `PERSIAN` | constant | `NLLanguage.h` | `Language::PERSIAN` |
| `POLISH` | constant | `NLLanguage.h` | `Language::POLISH` |
| `PORTUGUESE` | constant | `NLLanguage.h` | `Language::PORTUGUESE` |
| `PUNJABI` | constant | `NLLanguage.h` | `Language::PUNJABI` |
| `ROMANIAN` | constant | `NLLanguage.h` | `Language::ROMANIAN` |
| `RUSSIAN` | constant | `NLLanguage.h` | `Language::RUSSIAN` |
| `SIMPLIFIED_CHINESE` | constant | `NLLanguage.h` | `Language::SIMPLIFIED_CHINESE` |
| `SINHALESE` | constant | `NLLanguage.h` | `Language::SINHALESE` |
| `SLOVAK` | constant | `NLLanguage.h` | `Language::SLOVAK` |
| `SPANISH` | constant | `NLLanguage.h` | `Language::SPANISH` |
| `SWEDISH` | constant | `NLLanguage.h` | `Language::SWEDISH` |
| `TAMIL` | constant | `NLLanguage.h` | `Language::TAMIL` |
| `TELUGU` | constant | `NLLanguage.h` | `Language::TELUGU` |
| `THAI` | constant | `NLLanguage.h` | `Language::THAI` |
| `TIBETAN` | constant | `NLLanguage.h` | `Language::TIBETAN` |
| `TRADITIONAL_CHINESE` | constant | `NLLanguage.h` | `Language::TRADITIONAL_CHINESE` |
| `TURKISH` | constant | `NLLanguage.h` | `Language::TURKISH` |
| `UKRAINIAN` | constant | `NLLanguage.h` | `Language::UKRAINIAN` |
| `UNDETERMINED` | constant | `NLLanguage.h` | `Language::UNDETERMINED` |
| `URDU` | constant | `NLLanguage.h` | `Language::URDU` |
| `VIETNAMESE` | constant | `NLLanguage.h` | `Language::VIETNAMESE` |
| `ARABIC` | constant | `NLScript.h` | `Script::ARABIC` |
| `ARMENIAN` | constant | `NLScript.h` | `Script::ARMENIAN` |
| `BENGALI` | constant | `NLScript.h` | `Script::BENGALI` |
| `CANADIAN_ABORIGINAL_SYLLABICS` | constant | `NLScript.h` | `Script::CANADIAN_ABORIGINAL_SYLLABICS` |
| `CHEROKEE` | constant | `NLScript.h` | `Script::CHEROKEE` |
| `CYRILLIC` | constant | `NLScript.h` | `Script::CYRILLIC` |
| `DEVANAGARI` | constant | `NLScript.h` | `Script::DEVANAGARI` |
| `ETHIOPIC` | constant | `NLScript.h` | `Script::ETHIOPIC` |
| `GEORGIAN` | constant | `NLScript.h` | `Script::GEORGIAN` |
| `GREEK` | constant | `NLScript.h` | `Script::GREEK` |
| `GUJARATI` | constant | `NLScript.h` | `Script::GUJARATI` |
| `GURMUKHI` | constant | `NLScript.h` | `Script::GURMUKHI` |
| `HEBREW` | constant | `NLScript.h` | `Script::HEBREW` |
| `JAPANESE` | constant | `NLScript.h` | `Script::JAPANESE` |
| `KANNADA` | constant | `NLScript.h` | `Script::KANNADA` |
| `KHMER` | constant | `NLScript.h` | `Script::KHMER` |
| `KOREAN` | constant | `NLScript.h` | `Script::KOREAN` |
| `LAO` | constant | `NLScript.h` | `Script::LAO` |
| `LATIN` | constant | `NLScript.h` | `Script::LATIN` |
| `MALAYALAM` | constant | `NLScript.h` | `Script::MALAYALAM` |
| `MONGOLIAN` | constant | `NLScript.h` | `Script::MONGOLIAN` |
| `MYANMAR` | constant | `NLScript.h` | `Script::MYANMAR` |
| `ORIYA` | constant | `NLScript.h` | `Script::ORIYA` |
| `SIMPLIFIED_CHINESE` | constant | `NLScript.h` | `Script::SIMPLIFIED_CHINESE` |
| `SINHALA` | constant | `NLScript.h` | `Script::SINHALA` |
| `TAMIL` | constant | `NLScript.h` | `Script::TAMIL` |
| `TELUGU` | constant | `NLScript.h` | `Script::TELUGU` |
| `THAI` | constant | `NLScript.h` | `Script::THAI` |
| `TIBETAN` | constant | `NLScript.h` | `Script::TIBETAN` |
| `TRADITIONAL_CHINESE` | constant | `NLScript.h` | `Script::TRADITIONAL_CHINESE` |
| `UNDETERMINED` | constant | `NLScript.h` | `Script::UNDETERMINED` |
| `LANGUAGE` | constant | `NLTagScheme.h` | `TagScheme::LANGUAGE` |
| `LEMMA` | constant | `NLTagScheme.h` | `TagScheme::LEMMA` |
| `LEXICAL_CLASS` | constant | `NLTagScheme.h` | `TagScheme::LEXICAL_CLASS` |
| `NAME_TYPE` | constant | `NLTagScheme.h` | `TagScheme::NAME_TYPE` |
| `NAME_TYPE_OR_LEXICAL_CLASS` | constant | `NLTagScheme.h` | `TagScheme::NAME_TYPE_OR_LEXICAL_CLASS` |
| `SCRIPT` | constant | `NLTagScheme.h` | `TagScheme::SCRIPT` |
| `SENTIMENT_SCORE` | constant | `NLTagScheme.h` | `TagScheme::SENTIMENT_SCORE` |
| `TOKEN_TYPE` | constant | `NLTagScheme.h` | `TagScheme::TOKEN_TYPE` |
| `ADJECTIVE` | constant | `NLTagScheme.h` | `Tag::ADJECTIVE` |
| `ADVERB` | constant | `NLTagScheme.h` | `Tag::ADVERB` |
| `CLASSIFIER` | constant | `NLTagScheme.h` | `Tag::CLASSIFIER` |
| `CLOSE_PARENTHESIS` | constant | `NLTagScheme.h` | `Tag::CLOSE_PARENTHESIS` |
| `CLOSE_QUOTE` | constant | `NLTagScheme.h` | `Tag::CLOSE_QUOTE` |
| `CONJUNCTION` | constant | `NLTagScheme.h` | `Tag::CONJUNCTION` |
| `DASH` | constant | `NLTagScheme.h` | `Tag::DASH` |
| `DETERMINER` | constant | `NLTagScheme.h` | `Tag::DETERMINER` |
| `IDIOM` | constant | `NLTagScheme.h` | `Tag::IDIOM` |
| `INTERJECTION` | constant | `NLTagScheme.h` | `Tag::INTERJECTION` |
| `NOUN` | constant | `NLTagScheme.h` | `Tag::NOUN` |
| `NUMBER` | constant | `NLTagScheme.h` | `Tag::NUMBER` |
| `OPEN_PARENTHESIS` | constant | `NLTagScheme.h` | `Tag::OPEN_PARENTHESIS` |
| `OPEN_QUOTE` | constant | `NLTagScheme.h` | `Tag::OPEN_QUOTE` |
| `ORGANIZATION_NAME` | constant | `NLTagScheme.h` | `Tag::ORGANIZATION_NAME` |
| `OTHER` | constant | `NLTagScheme.h` | `Tag::OTHER` |
| `OTHER_PUNCTUATION` | constant | `NLTagScheme.h` | `Tag::OTHER_PUNCTUATION` |
| `OTHER_WHITESPACE` | constant | `NLTagScheme.h` | `Tag::OTHER_WHITESPACE` |
| `OTHER_WORD` | constant | `NLTagScheme.h` | `Tag::OTHER_WORD` |
| `PARAGRAPH_BREAK` | constant | `NLTagScheme.h` | `Tag::PARAGRAPH_BREAK` |
| `PARTICLE` | constant | `NLTagScheme.h` | `Tag::PARTICLE` |
| `PERSONAL_NAME` | constant | `NLTagScheme.h` | `Tag::PERSONAL_NAME` |
| `PLACE_NAME` | constant | `NLTagScheme.h` | `Tag::PLACE_NAME` |
| `PREPOSITION` | constant | `NLTagScheme.h` | `Tag::PREPOSITION` |
| `PRONOUN` | constant | `NLTagScheme.h` | `Tag::PRONOUN` |
| `PUNCTUATION` | constant | `NLTagScheme.h` | `Tag::PUNCTUATION` |
| `SENTENCE_TERMINATOR` | constant | `NLTagScheme.h` | `Tag::SENTENCE_TERMINATOR` |
| `VERB` | constant | `NLTagScheme.h` | `Tag::VERB` |
| `WHITESPACE` | constant | `NLTagScheme.h` | `Tag::WHITESPACE` |
| `WORD` | constant | `NLTagScheme.h` | `Tag::WORD` |
| `WORD_JOINER` | constant | `NLTagScheme.h` | `Tag::WORD_JOINER` |
| `Document` | enum case | `NLTokenizer.h` | `TokenUnit::Document` |
| `Paragraph` | enum case | `NLTokenizer.h` | `TokenUnit::Paragraph` |
| `Sentence` | enum case | `NLTokenizer.h` | `TokenUnit::Sentence` |
| `Word` | enum case | `NLTokenizer.h` | `TokenUnit::Word` |
| `EMOJI` | enum case | `NLTokenizer.h` | `TokenizerAttributes::EMOJI` |
| `NUMERIC` | enum case | `NLTokenizer.h` | `TokenizerAttributes::NUMERIC` |
| `SYMBOLIC` | enum case | `NLTokenizer.h` | `TokenizerAttributes::SYMBOLIC` |
| `JOIN_CONTRACTIONS` | enum case | `NLTagger.h` | `TaggerOptions::JOIN_CONTRACTIONS` |
| `JOIN_NAMES` | enum case | `NLTagger.h` | `TaggerOptions::JOIN_NAMES` |
| `OMIT_OTHER` | enum case | `NLTagger.h` | `TaggerOptions::OMIT_OTHER` |
| `OMIT_PUNCTUATION` | enum case | `NLTagger.h` | `TaggerOptions::OMIT_PUNCTUATION` |
| `OMIT_WHITESPACE` | enum case | `NLTagger.h` | `TaggerOptions::OMIT_WHITESPACE` |
| `OMIT_WORDS` | enum case | `NLTagger.h` | `TaggerOptions::OMIT_WORDS` |
| `Available` | enum case | `NLTagger.h` | `TaggerAssetsResult::Available` |
| `Error` | enum case | `NLTagger.h` | `TaggerAssetsResult::Error` |
| `NotAvailable` | enum case | `NLTagger.h` | `TaggerAssetsResult::NotAvailable` |
| `Cosine` | enum case | `NLEmbedding.h` | `DistanceType::Cosine` |
| `NLDistance` | typedef | `NLEmbedding.h` | `naturallanguage::Distance` |
| `Classifier` | enum case | `NLModel.h` | `ModelType::Classifier` |
| `Sequence` | enum case | `NLModel.h` | `ModelType::Sequence` |
| `Available` | enum case | `NLContextualEmbedding.h` | `ContextualEmbeddingAssetsResult::Available` |
| `Error` | enum case | `NLContextualEmbedding.h` | `ContextualEmbeddingAssetsResult::Error` |
| `NotAvailable` | enum case | `NLContextualEmbedding.h` | `ContextualEmbeddingAssetsResult::NotAvailable` |
| `languages` | constant | `NLContextualEmbedding.h` | `ContextualEmbeddingQuery::languages` |
| `revision` | constant | `NLContextualEmbedding.h` | `ContextualEmbeddingQuery::revision` |
| `scripts` | constant | `NLContextualEmbedding.h` | `ContextualEmbeddingQuery::scripts` |
| `dominantLanguage` | property | `NLLanguageRecognizer.h` | `LanguageRecognizer::dominant_language` |
| `dominantLanguageForString:` | method | `NLLanguageRecognizer.h` | `recognizer::dominant_language` |
| `init` | initializer | `NLLanguageRecognizer.h` | `LanguageRecognizer::new` |
| `languageConstraints` | property | `NLLanguageRecognizer.h` | `LanguageRecognizer::language_constraints` / `LanguageRecognizer::set_language_constraints` |
| `languageHints` | property | `NLLanguageRecognizer.h` | `LanguageRecognizer::language_hints` / `LanguageRecognizer::set_language_hints` |
| `languageHypothesesWithMaximum:` | method | `NLLanguageRecognizer.h` | `LanguageRecognizer::language_hypotheses` / `recognizer::language_hypotheses` |
| `processString:` | method | `NLLanguageRecognizer.h` | `LanguageRecognizer::process` |
| `reset` | method | `NLLanguageRecognizer.h` | `LanguageRecognizer::reset` |
| `enumerateTokensInRange:usingBlock:` | method | `NLTokenizer.h` | `Tokenizer::enumerate_tokens_in_range` |
| `initWithUnit:` | initializer | `NLTokenizer.h` | `Tokenizer::new` |
| `setLanguage:` | method | `NLTokenizer.h` | `Tokenizer::set_language` |
| `string` | property | `NLTokenizer.h` | `Tokenizer::string` / `Tokenizer::set_string` |
| `tokenRangeAtIndex:` | method | `NLTokenizer.h` | `Tokenizer::token_range_at_index` |
| `tokenRangeForRange:` | method | `NLTokenizer.h` | `Tokenizer::token_range_for_range` / `Tokenizer::token_ranges_for_range` |
| `tokensForRange:` | method | `NLTokenizer.h` | `Tokenizer::tokens_in_range` / `tokenizer::tokenize` |
| `unit` | property | `NLTokenizer.h` | `Tokenizer::unit` |
| `availableTagSchemesForUnit:language:` | method | `NLTagger.h` | `Tagger::available_tag_schemes` |
| `dominantLanguage` | property | `NLTagger.h` | `Tagger::dominant_language` |
| `enumerateTagsInRange:unit:scheme:options:usingBlock:` | method | `NLTagger.h` | `Tagger::enumerate_tags_in_range` |
| `gazetteersForTagScheme:` | method | `NLTagger.h` | `Tagger::gazetteers_for_tag_scheme` |
| `initWithTagSchemes:` | initializer | `NLTagger.h` | `Tagger::new` |
| `modelsForTagScheme:` | method | `NLTagger.h` | `Tagger::models_for_tag_scheme` |
| `requestAssetsForLanguage:tagScheme:completionHandler:` | method | `NLTagger.h` | `Tagger::request_assets` / `Tagger::request_assets_async` |
| `setGazetteers:forTagScheme:` | method | `NLTagger.h` | `Tagger::set_gazetteers` |
| `setLanguage:range:` | method | `NLTagger.h` | `Tagger::set_language` |
| `setModels:forTagScheme:` | method | `NLTagger.h` | `Tagger::set_models` |
| `setOrthography:range:` | method | `NLTagger.h` | `Tagger::set_orthography` |
| `string` | property | `NLTagger.h` | `Tagger::string` / `Tagger::set_string` |
| `tagAtIndex:unit:scheme:tokenRange:` | method | `NLTagger.h` | `Tagger::tag_at_index` |
| `tagHypothesesAtIndex:unit:scheme:maximumCount:tokenRange:` | method | `NLTagger.h` | `Tagger::tag_hypotheses_at_index` |
| `tagSchemes` | property | `NLTagger.h` | `Tagger::tag_schemes` |
| `tagsInRange:unit:scheme:options:tokenRanges:` | method | `NLTagger.h` | `Tagger::tags_in_range` |
| `tokenRangeAtIndex:unit:` | method | `NLTagger.h` | `Tagger::token_range_at_index` |
| `tokenRangeForRange:unit:` | method | `NLTagger.h` | `Tagger::token_range_for_range` |
| `containsString:` | method | `NLEmbedding.h` | `Embedding::contains_string` |
| `currentRevisionForLanguage:` | method | `NLEmbedding.h` | `Embedding::current_revision_for_language` |
| `currentSentenceEmbeddingRevisionForLanguage:` | method | `NLEmbedding.h` | `Embedding::current_sentence_revision_for_language` |
| `dimension` | property | `NLEmbedding.h` | `Embedding::dimension` |
| `distanceBetweenString:andString:distanceType:` | method | `NLEmbedding.h` | `Embedding::distance_with_type` / `Embedding::distance` |
| `embeddingWithContentsOfURL:error:` | method | `NLEmbedding.h` | `Embedding::from_path` |
| `enumerateNeighborsForString:maximumCount:distanceType:usingBlock:` | method | `NLEmbedding.h` | `Embedding::enumerate_neighbors_for_string` |
| `enumerateNeighborsForString:maximumCount:maximumDistance:distanceType:usingBlock:` | method | `NLEmbedding.h` | `Embedding::enumerate_neighbors_for_string` |
| `enumerateNeighborsForVector:maximumCount:distanceType:usingBlock:` | method | `NLEmbedding.h` | `Embedding::enumerate_neighbors_for_vector` |
| `enumerateNeighborsForVector:maximumCount:maximumDistance:distanceType:usingBlock:` | method | `NLEmbedding.h` | `Embedding::enumerate_neighbors_for_vector` |
| `language` | property | `NLEmbedding.h` | `Embedding::language` |
| `neighborsForString:maximumCount:distanceType:` | method | `NLEmbedding.h` | `Embedding::neighbors` |
| `neighborsForString:maximumCount:maximumDistance:distanceType:` | method | `NLEmbedding.h` | `Embedding::neighbors_with_limit` |
| `neighborsForVector:maximumCount:distanceType:` | method | `NLEmbedding.h` | `Embedding::neighbors_for_vector` |
| `neighborsForVector:maximumCount:maximumDistance:distanceType:` | method | `NLEmbedding.h` | `Embedding::neighbors_for_vector_with_limit` |
| `revision` | property | `NLEmbedding.h` | `Embedding::revision` |
| `sentenceEmbeddingForLanguage:` | method | `NLEmbedding.h` | `Embedding::sentence_for_language` |
| `sentenceEmbeddingForLanguage:revision:` | method | `NLEmbedding.h` | `Embedding::sentence_for_language_revision` |
| `supportedRevisionsForLanguage:` | method | `NLEmbedding.h` | `Embedding::supported_revisions_for_language` |
| `supportedSentenceEmbeddingRevisionsForLanguage:` | method | `NLEmbedding.h` | `Embedding::supported_sentence_revisions_for_language` |
| `vectorForString:` | method | `NLEmbedding.h` | `Embedding::vector_for` |
| `vocabularySize` | property | `NLEmbedding.h` | `Embedding::vocabulary_size` |
| `wordEmbeddingForLanguage:` | method | `NLEmbedding.h` | `Embedding::word_for_language` |
| `wordEmbeddingForLanguage:revision:` | method | `NLEmbedding.h` | `Embedding::word_for_language_revision` |
| `writeEmbeddingForDictionary:language:revision:toURL:error:` | method | `NLEmbedding.h` | `Embedding::write_dictionary` |
| `data` | property | `NLGazetteer.h` | `Gazetteer::data` |
| `gazetteerWithContentsOfURL:error:` | method | `NLGazetteer.h` | `Gazetteer::from_path` |
| `initWithContentsOfURL:error:` | initializer | `NLGazetteer.h` | `Gazetteer::from_path` |
| `initWithData:error:` | initializer | `NLGazetteer.h` | `Gazetteer::from_data` |
| `initWithDictionary:language:error:` | initializer | `NLGazetteer.h` | `Gazetteer::from_dictionary` |
| `labelForString:` | method | `NLGazetteer.h` | `Gazetteer::label_for_string` |
| `language` | property | `NLGazetteer.h` | `Gazetteer::language` |
| `writeGazetteerForDictionary:language:toURL:error:` | method | `NLGazetteer.h` | `Gazetteer::write_dictionary` |
| `configuration` | property | `NLModel.h` | `Model::configuration` |
| `modelWithContentsOfURL:error:` | method | `NLModel.h` | `Model::from_path` |
| `modelWithMLModel:error:` | method | `NLModel.h` | `Model::from_core_ml_model` |
| `predictedLabelForString:` | method | `NLModel.h` | `Model::predicted_label_for_string` |
| `predictedLabelHypothesesForString:maximumCount:` | method | `NLModel.h` | `Model::predicted_label_hypotheses_for_string` |
| `predictedLabelHypothesesForTokens:maximumCount:` | method | `NLModel.h` | `Model::predicted_label_hypotheses_for_tokens` |
| `predictedLabelsForTokens:` | method | `NLModel.h` | `Model::predicted_labels_for_tokens` |
| `currentRevisionForType:` | method | `NLModel.h` | `ModelConfiguration::current_revision_for_type` |
| `language` | property | `NLModel.h` | `ModelConfiguration::language` |
| `revision` | property | `NLModel.h` | `ModelConfiguration::revision` |
| `supportedRevisionsForType:` | method | `NLModel.h` | `ModelConfiguration::supported_revisions_for_type` |
| `type` | property | `NLModel.h` | `ModelConfiguration::model_type` |
| `contextualEmbeddingsForValues:` | method | `NLContextualEmbedding.h` | `ContextualEmbedding::catalog` |
| `contextualEmbeddingWithLanguage:` | method | `NLContextualEmbedding.h` | `ContextualEmbedding::for_language` |
| `contextualEmbeddingWithModelIdentifier:` | method | `NLContextualEmbedding.h` | `ContextualEmbedding::from_model_identifier` |
| `contextualEmbeddingWithScript:` | method | `NLContextualEmbedding.h` | `ContextualEmbedding::for_script` |
| `dimension` | property | `NLContextualEmbedding.h` | `ContextualEmbedding::dimension` |
| `embeddingResultForString:language:error:` | method | `NLContextualEmbedding.h` | `ContextualEmbedding::embedding_result_for_string` |
| `hasAvailableAssets` | property | `NLContextualEmbedding.h` | `ContextualEmbedding::has_available_assets` |
| `languages` | property | `NLContextualEmbedding.h` | `ContextualEmbedding::languages` |
| `loadWithError:` | method | `NLContextualEmbedding.h` | `ContextualEmbedding::load` |
| `maximumSequenceLength` | property | `NLContextualEmbedding.h` | `ContextualEmbedding::maximum_sequence_length` |
| `modelIdentifier` | property | `NLContextualEmbedding.h` | `ContextualEmbedding::model_identifier` |
| `requestEmbeddingAssetsWithCompletionHandler:` | method | `NLContextualEmbedding.h` | `ContextualEmbedding::request_embedding_assets` / `request_embedding_assets_async` |
| `revision` | property | `NLContextualEmbedding.h` | `ContextualEmbedding::revision` |
| `scripts` | property | `NLContextualEmbedding.h` | `ContextualEmbedding::scripts` |
| `unload` | method | `NLContextualEmbedding.h` | `ContextualEmbedding::unload` |
| `enumerateTokenVectorsInRange:usingBlock:` | method | `NLContextualEmbedding.h` | `ContextualEmbeddingResult::enumerate_token_vectors_in_range` |
| `language` | property | `NLContextualEmbedding.h` | `ContextualEmbeddingResult::language` |
| `sequenceLength` | property | `NLContextualEmbedding.h` | `ContextualEmbeddingResult::sequence_length` |
| `string` | property | `NLContextualEmbedding.h` | `ContextualEmbeddingResult::string` |
| `tokenVectorAtIndex:tokenRange:` | method | `NLContextualEmbedding.h` | `ContextualEmbeddingResult::token_vector_at_index` |

## 🔴 GAPS

None.

| Symbol | Kind | Header | Notes |
| --- | --- | --- | --- |

## ⏭️ EXEMPT

| Symbol | Kind | Header | Reason | SDK attribute |
| --- | --- | --- | --- | --- |
| `getVector:forString:` | method | `NLEmbedding.h` | Objective-C selector is `NS_SWIFT_UNAVAILABLE`, so the Swift bridge uses `vectorForString:` instead. | `NS_SWIFT_UNAVAILABLE("Use 'vector(for:)' instead")` |
| `init` | initializer | `NLContextualEmbedding.h` | Unavailable initializer; instances are created via catalog/factory methods only. | `NS_UNAVAILABLE` |
| `init` | initializer | `NLContextualEmbedding.h` | Unavailable initializer; results are produced by `embedding_result_for_string`. | `NS_UNAVAILABLE` |
