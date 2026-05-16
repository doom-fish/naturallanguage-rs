# NaturalLanguage.framework coverage audit

Audit target: `naturallanguage` `v0.4.0` against `MacOSX26.2.sdk` plus `cargo expand --lib > target/cargo-expand-lib.rs`.

Legend: ✅ implemented · 🟡 partial · ⏭️ skipped

## Notes

- `NLDataAsset` is not present in the current macOS `NaturalLanguage.framework` headers.
- `NLEmbedding.getVector:forString:` is `NS_SWIFT_UNAVAILABLE` in Apple's headers.
- `NLContextualEmbedding.init` and `NLContextualEmbeddingResult.init` are `NS_UNAVAILABLE` in Apple's headers.

## NLLanguage constants

| Symbol | Status | Notes |
| --- | --- | --- |
| `AMHARIC` | ✅ implemented |  |
| `ARABIC` | ✅ implemented |  |
| `ARMENIAN` | ✅ implemented |  |
| `BENGALI` | ✅ implemented |  |
| `BULGARIAN` | ✅ implemented |  |
| `BURMESE` | ✅ implemented |  |
| `CATALAN` | ✅ implemented |  |
| `CHEROKEE` | ✅ implemented |  |
| `CROATIAN` | ✅ implemented |  |
| `CZECH` | ✅ implemented |  |
| `DANISH` | ✅ implemented |  |
| `DUTCH` | ✅ implemented |  |
| `ENGLISH` | ✅ implemented |  |
| `FINNISH` | ✅ implemented |  |
| `FRENCH` | ✅ implemented |  |
| `GEORGIAN` | ✅ implemented |  |
| `GERMAN` | ✅ implemented |  |
| `GREEK` | ✅ implemented |  |
| `GUJARATI` | ✅ implemented |  |
| `HEBREW` | ✅ implemented |  |
| `HINDI` | ✅ implemented |  |
| `HUNGARIAN` | ✅ implemented |  |
| `ICELANDIC` | ✅ implemented |  |
| `INDONESIAN` | ✅ implemented |  |
| `ITALIAN` | ✅ implemented |  |
| `JAPANESE` | ✅ implemented |  |
| `KANNADA` | ✅ implemented |  |
| `KAZAKH` | ✅ implemented |  |
| `KHMER` | ✅ implemented |  |
| `KOREAN` | ✅ implemented |  |
| `LAO` | ✅ implemented |  |
| `MALAY` | ✅ implemented |  |
| `MALAYALAM` | ✅ implemented |  |
| `MARATHI` | ✅ implemented |  |
| `MONGOLIAN` | ✅ implemented |  |
| `NORWEGIAN` | ✅ implemented |  |
| `ORIYA` | ✅ implemented |  |
| `PERSIAN` | ✅ implemented |  |
| `POLISH` | ✅ implemented |  |
| `PORTUGUESE` | ✅ implemented |  |
| `PUNJABI` | ✅ implemented |  |
| `ROMANIAN` | ✅ implemented |  |
| `RUSSIAN` | ✅ implemented |  |
| `SIMPLIFIED_CHINESE` | ✅ implemented |  |
| `SINHALESE` | ✅ implemented |  |
| `SLOVAK` | ✅ implemented |  |
| `SPANISH` | ✅ implemented |  |
| `SWEDISH` | ✅ implemented |  |
| `TAMIL` | ✅ implemented |  |
| `TELUGU` | ✅ implemented |  |
| `THAI` | ✅ implemented |  |
| `TIBETAN` | ✅ implemented |  |
| `TRADITIONAL_CHINESE` | ✅ implemented |  |
| `TURKISH` | ✅ implemented |  |
| `UKRAINIAN` | ✅ implemented |  |
| `UNDETERMINED` | ✅ implemented |  |
| `URDU` | ✅ implemented |  |
| `VIETNAMESE` | ✅ implemented |  |

## NLScript constants

| Symbol | Status | Notes |
| --- | --- | --- |
| `ARABIC` | ✅ implemented |  |
| `ARMENIAN` | ✅ implemented |  |
| `BENGALI` | ✅ implemented |  |
| `CANADIAN_ABORIGINAL_SYLLABICS` | ✅ implemented |  |
| `CHEROKEE` | ✅ implemented |  |
| `CYRILLIC` | ✅ implemented |  |
| `DEVANAGARI` | ✅ implemented |  |
| `ETHIOPIC` | ✅ implemented |  |
| `GEORGIAN` | ✅ implemented |  |
| `GREEK` | ✅ implemented |  |
| `GUJARATI` | ✅ implemented |  |
| `GURMUKHI` | ✅ implemented |  |
| `HEBREW` | ✅ implemented |  |
| `JAPANESE` | ✅ implemented |  |
| `KANNADA` | ✅ implemented |  |
| `KHMER` | ✅ implemented |  |
| `KOREAN` | ✅ implemented |  |
| `LAO` | ✅ implemented |  |
| `LATIN` | ✅ implemented |  |
| `MALAYALAM` | ✅ implemented |  |
| `MONGOLIAN` | ✅ implemented |  |
| `MYANMAR` | ✅ implemented |  |
| `ORIYA` | ✅ implemented |  |
| `SIMPLIFIED_CHINESE` | ✅ implemented |  |
| `SINHALA` | ✅ implemented |  |
| `TAMIL` | ✅ implemented |  |
| `TELUGU` | ✅ implemented |  |
| `THAI` | ✅ implemented |  |
| `TIBETAN` | ✅ implemented |  |
| `TRADITIONAL_CHINESE` | ✅ implemented |  |
| `UNDETERMINED` | ✅ implemented |  |

## NLTagScheme constants

| Symbol | Status | Notes |
| --- | --- | --- |
| `LANGUAGE` | ✅ implemented |  |
| `LEMMA` | ✅ implemented |  |
| `LEXICAL_CLASS` | ✅ implemented |  |
| `NAME_TYPE` | ✅ implemented |  |
| `NAME_TYPE_OR_LEXICAL_CLASS` | ✅ implemented |  |
| `SCRIPT` | ✅ implemented |  |
| `SENTIMENT_SCORE` | ✅ implemented |  |
| `TOKEN_TYPE` | ✅ implemented |  |

## NLTag constants

| Symbol | Status | Notes |
| --- | --- | --- |
| `ADJECTIVE` | ✅ implemented |  |
| `ADVERB` | ✅ implemented |  |
| `CLASSIFIER` | ✅ implemented |  |
| `CLOSE_PARENTHESIS` | ✅ implemented |  |
| `CLOSE_QUOTE` | ✅ implemented |  |
| `CONJUNCTION` | ✅ implemented |  |
| `DASH` | ✅ implemented |  |
| `DETERMINER` | ✅ implemented |  |
| `IDIOM` | ✅ implemented |  |
| `INTERJECTION` | ✅ implemented |  |
| `NOUN` | ✅ implemented |  |
| `NUMBER` | ✅ implemented |  |
| `OPEN_PARENTHESIS` | ✅ implemented |  |
| `OPEN_QUOTE` | ✅ implemented |  |
| `ORGANIZATION_NAME` | ✅ implemented |  |
| `OTHER` | ✅ implemented |  |
| `OTHER_PUNCTUATION` | ✅ implemented |  |
| `OTHER_WHITESPACE` | ✅ implemented |  |
| `OTHER_WORD` | ✅ implemented |  |
| `PARAGRAPH_BREAK` | ✅ implemented |  |
| `PARTICLE` | ✅ implemented |  |
| `PERSONAL_NAME` | ✅ implemented |  |
| `PLACE_NAME` | ✅ implemented |  |
| `PREPOSITION` | ✅ implemented |  |
| `PRONOUN` | ✅ implemented |  |
| `PUNCTUATION` | ✅ implemented |  |
| `SENTENCE_TERMINATOR` | ✅ implemented |  |
| `VERB` | ✅ implemented |  |
| `WHITESPACE` | ✅ implemented |  |
| `WORD` | ✅ implemented |  |
| `WORD_JOINER` | ✅ implemented |  |

## NLTokenUnit

| Symbol | Status | Notes |
| --- | --- | --- |
| `Document` | ✅ implemented |  |
| `Paragraph` | ✅ implemented |  |
| `Sentence` | ✅ implemented |  |
| `Word` | ✅ implemented |  |

## NLTokenizerAttributes

| Symbol | Status | Notes |
| --- | --- | --- |
| `EMOJI` | ✅ implemented |  |
| `NUMERIC` | ✅ implemented |  |
| `SYMBOLIC` | ✅ implemented |  |

## NLTaggerOptions

| Symbol | Status | Notes |
| --- | --- | --- |
| `JOIN_CONTRACTIONS` | ✅ implemented |  |
| `JOIN_NAMES` | ✅ implemented |  |
| `OMIT_OTHER` | ✅ implemented |  |
| `OMIT_PUNCTUATION` | ✅ implemented |  |
| `OMIT_WHITESPACE` | ✅ implemented |  |
| `OMIT_WORDS` | ✅ implemented |  |

## NLTaggerAssetsResult

| Symbol | Status | Notes |
| --- | --- | --- |
| `Available` | ✅ implemented |  |
| `Error` | ✅ implemented |  |
| `NotAvailable` | ✅ implemented |  |

## NLDistanceType

| Symbol | Status | Notes |
| --- | --- | --- |
| `Cosine` | ✅ implemented |  |

## NLDistance

| Symbol | Status | Notes |
| --- | --- | --- |
| `NLDistance` | ✅ implemented | Rust alias: naturallanguage::Distance |

## NLModelType

| Symbol | Status | Notes |
| --- | --- | --- |
| `Classifier` | ✅ implemented |  |
| `Sequence` | ✅ implemented |  |

## NLContextualEmbeddingAssetsResult

| Symbol | Status | Notes |
| --- | --- | --- |
| `Available` | ✅ implemented |  |
| `Error` | ✅ implemented |  |
| `NotAvailable` | ✅ implemented |  |

## NLContextualEmbeddingKey constants

| Symbol | Status | Notes |
| --- | --- | --- |
| `languages` | ✅ implemented | Available via ContextualEmbeddingQuery |
| `revision` | ✅ implemented | Available via ContextualEmbeddingQuery |
| `scripts` | ✅ implemented | Available via ContextualEmbeddingQuery |

## NLLanguageRecognizer

| Symbol | Status | Notes |
| --- | --- | --- |
| `dominantLanguage` | ✅ implemented |  |
| `dominantLanguageForString:` | ✅ implemented |  |
| `init` | ✅ implemented |  |
| `languageConstraints` | ✅ implemented |  |
| `languageHints` | ✅ implemented |  |
| `languageHypothesesWithMaximum:` | ✅ implemented |  |
| `processString:` | ✅ implemented |  |
| `reset` | ✅ implemented |  |

## NLTokenizer

| Symbol | Status | Notes |
| --- | --- | --- |
| `enumerateTokensInRange:usingBlock:` | ✅ implemented |  |
| `initWithUnit:` | ✅ implemented |  |
| `setLanguage:` | ✅ implemented |  |
| `string` | ✅ implemented |  |
| `tokenRangeAtIndex:` | ✅ implemented |  |
| `tokenRangeForRange:` | ✅ implemented |  |
| `tokensForRange:` | ✅ implemented |  |
| `unit` | ✅ implemented |  |

## NLTagger

| Symbol | Status | Notes |
| --- | --- | --- |
| `availableTagSchemesForUnit:language:` | ✅ implemented |  |
| `dominantLanguage` | ✅ implemented |  |
| `enumerateTagsInRange:unit:scheme:options:usingBlock:` | ✅ implemented |  |
| `gazetteersForTagScheme:` | ✅ implemented |  |
| `initWithTagSchemes:` | ✅ implemented |  |
| `modelsForTagScheme:` | ✅ implemented |  |
| `requestAssetsForLanguage:tagScheme:completionHandler:` | ✅ implemented |  |
| `setGazetteers:forTagScheme:` | ✅ implemented |  |
| `setLanguage:range:` | ✅ implemented |  |
| `setModels:forTagScheme:` | ✅ implemented |  |
| `setOrthography:range:` | ✅ implemented |  |
| `string` | ✅ implemented |  |
| `tagAtIndex:unit:scheme:tokenRange:` | ✅ implemented |  |
| `tagHypothesesAtIndex:unit:scheme:maximumCount:tokenRange:` | ✅ implemented |  |
| `tagSchemes` | ✅ implemented |  |
| `tagsInRange:unit:scheme:options:tokenRanges:` | ✅ implemented |  |
| `tokenRangeAtIndex:unit:` | ✅ implemented |  |
| `tokenRangeForRange:unit:` | ✅ implemented |  |

## NLEmbedding

| Symbol | Status | Notes |
| --- | --- | --- |
| `containsString:` | ✅ implemented |  |
| `currentRevisionForLanguage:` | ✅ implemented |  |
| `currentSentenceEmbeddingRevisionForLanguage:` | ✅ implemented |  |
| `dimension` | ✅ implemented |  |
| `distanceBetweenString:andString:distanceType:` | ✅ implemented |  |
| `embeddingWithContentsOfURL:error:` | ✅ implemented |  |
| `enumerateNeighborsForString:maximumCount:distanceType:usingBlock:` | ✅ implemented |  |
| `enumerateNeighborsForString:maximumCount:maximumDistance:distanceType:usingBlock:` | ✅ implemented |  |
| `enumerateNeighborsForVector:maximumCount:distanceType:usingBlock:` | ✅ implemented |  |
| `enumerateNeighborsForVector:maximumCount:maximumDistance:distanceType:usingBlock:` | ✅ implemented |  |
| `getVector:forString:` | ⏭️ skipped | Apple marks this selector NS_SWIFT_UNAVAILABLE; use vectorForString: instead. |
| `language` | ✅ implemented |  |
| `neighborsForString:maximumCount:distanceType:` | ✅ implemented |  |
| `neighborsForString:maximumCount:maximumDistance:distanceType:` | ✅ implemented |  |
| `neighborsForVector:maximumCount:distanceType:` | ✅ implemented |  |
| `neighborsForVector:maximumCount:maximumDistance:distanceType:` | ✅ implemented |  |
| `revision` | ✅ implemented |  |
| `sentenceEmbeddingForLanguage:` | ✅ implemented |  |
| `sentenceEmbeddingForLanguage:revision:` | ✅ implemented |  |
| `supportedRevisionsForLanguage:` | ✅ implemented |  |
| `supportedSentenceEmbeddingRevisionsForLanguage:` | ✅ implemented |  |
| `vectorForString:` | ✅ implemented |  |
| `vocabularySize` | ✅ implemented |  |
| `wordEmbeddingForLanguage:` | ✅ implemented |  |
| `wordEmbeddingForLanguage:revision:` | ✅ implemented |  |
| `writeEmbeddingForDictionary:language:revision:toURL:error:` | ✅ implemented |  |

## NLGazetteer

| Symbol | Status | Notes |
| --- | --- | --- |
| `data` | ✅ implemented |  |
| `gazetteerWithContentsOfURL:error:` | ✅ implemented |  |
| `initWithContentsOfURL:error:` | ✅ implemented |  |
| `initWithData:error:` | ✅ implemented |  |
| `initWithDictionary:language:error:` | ✅ implemented |  |
| `labelForString:` | ✅ implemented |  |
| `language` | ✅ implemented |  |
| `writeGazetteerForDictionary:language:toURL:error:` | ✅ implemented |  |

## NLModel

| Symbol | Status | Notes |
| --- | --- | --- |
| `configuration` | ✅ implemented |  |
| `modelWithContentsOfURL:error:` | ✅ implemented |  |
| `modelWithMLModel:error:` | ✅ implemented |  |
| `predictedLabelForString:` | ✅ implemented |  |
| `predictedLabelHypothesesForString:maximumCount:` | ✅ implemented |  |
| `predictedLabelHypothesesForTokens:maximumCount:` | ✅ implemented |  |
| `predictedLabelsForTokens:` | ✅ implemented |  |

## NLModelConfiguration

| Symbol | Status | Notes |
| --- | --- | --- |
| `currentRevisionForType:` | ✅ implemented |  |
| `language` | ✅ implemented |  |
| `revision` | ✅ implemented |  |
| `supportedRevisionsForType:` | ✅ implemented |  |
| `type` | ✅ implemented |  |

## NLContextualEmbedding

| Symbol | Status | Notes |
| --- | --- | --- |
| `contextualEmbeddingWithLanguage:` | ✅ implemented |  |
| `contextualEmbeddingWithModelIdentifier:` | ✅ implemented |  |
| `contextualEmbeddingWithScript:` | ✅ implemented |  |
| `contextualEmbeddingsForValues:` | ✅ implemented |  |
| `dimension` | ✅ implemented |  |
| `embeddingResultForString:language:error:` | ✅ implemented |  |
| `hasAvailableAssets` | ✅ implemented |  |
| `init` | ⏭️ skipped | Apple marks this initializer NS_UNAVAILABLE; instances come from the catalog/factory methods. |
| `languages` | ✅ implemented |  |
| `loadWithError:` | ✅ implemented |  |
| `maximumSequenceLength` | ✅ implemented |  |
| `modelIdentifier` | ✅ implemented |  |
| `requestEmbeddingAssetsWithCompletionHandler:` | ✅ implemented |  |
| `revision` | ✅ implemented |  |
| `scripts` | ✅ implemented |  |
| `unload` | ✅ implemented |  |

## NLContextualEmbeddingResult

| Symbol | Status | Notes |
| --- | --- | --- |
| `enumerateTokenVectorsInRange:usingBlock:` | ✅ implemented |  |
| `init` | ⏭️ skipped | Apple marks this initializer NS_UNAVAILABLE; results are produced by embeddingResultForString:language:error:. |
| `language` | ✅ implemented |  |
| `sequenceLength` | ✅ implemented |  |
| `string` | ✅ implemented |  |
| `tokenVectorAtIndex:tokenRange:` | ✅ implemented |  |

## NLDataAsset

| Symbol | Status | Notes |
| --- | --- | --- |
| `NLDataAsset` | ⏭️ skipped | No NLDataAsset symbol exists anywhere in the current macOS 26.2 NaturalLanguage.framework headers. |

