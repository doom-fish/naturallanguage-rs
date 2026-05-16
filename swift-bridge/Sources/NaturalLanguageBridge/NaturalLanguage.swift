// NaturalLanguage Bridge
//
// @_cdecl wrappers around Apple's NaturalLanguage framework so Rust can
// drive language detection, tokenisation, and named-entity recognition
// without depending on any Swift type at the FFI boundary.

import Foundation
import NaturalLanguage

// MARK: - String helpers

@_cdecl("nl_string_free")
public func nl_string_free(_ s: UnsafeMutablePointer<CChar>?) {
    guard let s = s else { return }
    free(s)
}

// MARK: - Layout-compatible structs (mirror Rust ffi/mod.rs)

public struct NLLanguageHypothesisRaw {
    public var language: UnsafeMutablePointer<CChar>?
    public var confidence: Double
}

public struct NLTokenRaw {
    /// UTF-16 character offset of the token start.
    public var start: Int
    /// UTF-16 character length of the token.
    public var length: Int
    /// NUL-terminated UTF-8 substring; caller frees via nl_tokens_free.
    public var text: UnsafeMutablePointer<CChar>?
}

public struct NLNamedEntityRaw {
    public var start: Int
    public var length: Int
    public var text: UnsafeMutablePointer<CChar>?
    /// One of: "PersonalName", "PlaceName", "OrganizationName", "Other"
    public var tag: UnsafeMutablePointer<CChar>?
}

// MARK: - Language detection

/// Single-shot dominant-language detection.
///
/// Returns a NUL-terminated BCP-47 language identifier (e.g. "en", "sv")
/// via `out_language`, or NL_NO_DOMINANT_LANGUAGE when the recognizer
/// can't decide.
@_cdecl("nl_dominant_language")
public func nl_dominant_language(
    _ text: UnsafePointer<CChar>,
    _ outLanguage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>
) -> Int32 {
    let s = String(cString: text)
    guard let lang = NLLanguageRecognizer.dominantLanguage(for: s) else {
        return NL_NO_DOMINANT_LANGUAGE
    }
    outLanguage.pointee = nlFfiString(lang.rawValue)
    return NL_OK
}

/// Multi-hypothesis variant that returns up to `maxHypotheses` ranked
/// (language, confidence) pairs.
///
/// The output array is a flat C array of `NLLanguageHypothesisRaw`,
/// allocated by Swift; Rust must call `nl_language_hypotheses_free` to
/// release it.
@_cdecl("nl_language_hypotheses")
public func nl_language_hypotheses(
    _ text: UnsafePointer<CChar>,
    _ maxHypotheses: Int,
    _ outArray: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ outCount: UnsafeMutablePointer<Int>
) -> Int32 {
    let s = String(cString: text)
    let recognizer = NLLanguageRecognizer()
    recognizer.processString(s)
    let hypotheses = recognizer.languageHypotheses(withMaximum: maxHypotheses)
    if hypotheses.isEmpty {
        outArray.pointee = nil
        outCount.pointee = 0
        return NL_OK
    }
    // Stable order: descending confidence.
    let sorted = hypotheses.sorted { $0.value > $1.value }
    let count = sorted.count
    let buffer = UnsafeMutablePointer<NLLanguageHypothesisRaw>.allocate(capacity: count)
    for (i, (lang, conf)) in sorted.enumerated() {
        buffer.advanced(by: i).initialize(to: NLLanguageHypothesisRaw(
            language: nlFfiString(lang.rawValue),
            confidence: conf
        ))
    }
    outArray.pointee = UnsafeMutableRawPointer(buffer)
    outCount.pointee = count
    return NL_OK
}

@_cdecl("nl_language_hypotheses_free")
public func nl_language_hypotheses_free(_ array: UnsafeMutableRawPointer?, _ count: Int) {
    guard let array = array else { return }
    let typed = array.assumingMemoryBound(to: NLLanguageHypothesisRaw.self)
    for i in 0..<count {
        if let p = typed.advanced(by: i).pointee.language { free(p) }
    }
    typed.deallocate()
}

// MARK: - Tokenization

private func tokenUnit(_ raw: Int32) -> NLTokenUnit {
    switch raw {
    case 0: return .word
    case 1: return .sentence
    case 2: return .paragraph
    case 3: return .document
    default: return .word
    }
}

/// Tokenize `text` into the requested unit (0=word 1=sentence 2=paragraph
/// 3=document). Returns a flat array of `NLTokenRaw`; Rust frees with
/// `nl_tokens_free`.
@_cdecl("nl_tokenize")
public func nl_tokenize(
    _ text: UnsafePointer<CChar>,
    _ unitRaw: Int32,
    _ outArray: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ outCount: UnsafeMutablePointer<Int>
) -> Int32 {
    let s = String(cString: text)
    let tokenizer = NLTokenizer(unit: tokenUnit(unitRaw))
    tokenizer.string = s
    var tokens: [(start: Int, length: Int, text: String)] = []
    tokenizer.enumerateTokens(in: s.startIndex..<s.endIndex) { range, _ in
        let start = s.utf16.distance(from: s.utf16.startIndex, to: range.lowerBound.samePosition(in: s.utf16) ?? s.utf16.startIndex)
        let end = s.utf16.distance(from: s.utf16.startIndex, to: range.upperBound.samePosition(in: s.utf16) ?? s.utf16.endIndex)
        tokens.append((start: start, length: end - start, text: String(s[range])))
        return true
    }
    if tokens.isEmpty {
        outArray.pointee = nil
        outCount.pointee = 0
        return NL_OK
    }
    let buffer = UnsafeMutablePointer<NLTokenRaw>.allocate(capacity: tokens.count)
    for (i, t) in tokens.enumerated() {
        buffer.advanced(by: i).initialize(to: NLTokenRaw(
            start: t.start,
            length: t.length,
            text: nlFfiString(t.text)
        ))
    }
    outArray.pointee = UnsafeMutableRawPointer(buffer)
    outCount.pointee = tokens.count
    return NL_OK
}

@_cdecl("nl_tokens_free")
public func nl_tokens_free(_ array: UnsafeMutableRawPointer?, _ count: Int) {
    guard let array = array else { return }
    let typed = array.assumingMemoryBound(to: NLTokenRaw.self)
    for i in 0..<count {
        if let p = typed.advanced(by: i).pointee.text { free(p) }
    }
    typed.deallocate()
}

// MARK: - Named entity recognition

/// Named-entity recognition using NLTagger with the `.nameType` scheme.
/// Returns entities tagged as PersonalName / PlaceName / OrganizationName.
/// Other-tagged tokens are filtered out.
@_cdecl("nl_named_entities")
public func nl_named_entities(
    _ text: UnsafePointer<CChar>,
    _ outArray: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ outCount: UnsafeMutablePointer<Int>
) -> Int32 {
    let s = String(cString: text)
    let tagger = NLTagger(tagSchemes: [.nameType])
    tagger.string = s
    let options: NLTagger.Options = [.omitPunctuation, .omitWhitespace, .joinNames]

    var entities: [(start: Int, length: Int, text: String, tag: String)] = []
    tagger.enumerateTags(
        in: s.startIndex..<s.endIndex,
        unit: .word,
        scheme: .nameType,
        options: options
    ) { tag, range in
        guard let tag = tag,
              [NLTag.personalName, .placeName, .organizationName].contains(tag) else {
            return true
        }
        let start = s.utf16.distance(from: s.utf16.startIndex, to: range.lowerBound.samePosition(in: s.utf16) ?? s.utf16.startIndex)
        let end = s.utf16.distance(from: s.utf16.startIndex, to: range.upperBound.samePosition(in: s.utf16) ?? s.utf16.endIndex)
        entities.append((
            start: start,
            length: end - start,
            text: String(s[range]),
            tag: tag.rawValue
        ))
        return true
    }
    if entities.isEmpty {
        outArray.pointee = nil
        outCount.pointee = 0
        return NL_OK
    }
    let buffer = UnsafeMutablePointer<NLNamedEntityRaw>.allocate(capacity: entities.count)
    for (i, e) in entities.enumerated() {
        buffer.advanced(by: i).initialize(to: NLNamedEntityRaw(
            start: e.start,
            length: e.length,
            text: nlFfiString(e.text),
            tag: nlFfiString(e.tag)
        ))
    }
    outArray.pointee = UnsafeMutableRawPointer(buffer)
    outCount.pointee = entities.count
    return NL_OK
}

@_cdecl("nl_named_entities_free")
public func nl_named_entities_free(_ array: UnsafeMutableRawPointer?, _ count: Int) {
    guard let array = array else { return }
    let typed = array.assumingMemoryBound(to: NLNamedEntityRaw.self)
    for i in 0..<count {
        if let p = typed.advanced(by: i).pointee.text { free(p) }
        if let p = typed.advanced(by: i).pointee.tag { free(p) }
    }
    typed.deallocate()
}

// MARK: - Word embeddings (v0.2)

private var embeddingStore: [UnsafeMutableRawPointer: NLEmbedding] = [:]
private let embeddingStoreLock = NSLock()

@_cdecl("nl_word_embedding_for_language")
public func nl_word_embedding_for_language(
    _ language: UnsafePointer<CChar>
) -> UnsafeMutableRawPointer? {
    let langStr = String(cString: language)
    let lang = NLLanguage(rawValue: langStr)
    guard let emb = NLEmbedding.wordEmbedding(for: lang) else { return nil }
    let key = Unmanaged.passRetained(emb).toOpaque()
    embeddingStoreLock.lock()
    embeddingStore[key] = emb
    embeddingStoreLock.unlock()
    return key
}

@_cdecl("nl_sentence_embedding_for_language")
public func nl_sentence_embedding_for_language(
    _ language: UnsafePointer<CChar>
) -> UnsafeMutableRawPointer? {
    let langStr = String(cString: language)
    let lang = NLLanguage(rawValue: langStr)
    guard let emb = NLEmbedding.sentenceEmbedding(for: lang) else { return nil }
    let key = Unmanaged.passRetained(emb).toOpaque()
    embeddingStoreLock.lock()
    embeddingStore[key] = emb
    embeddingStoreLock.unlock()
    return key
}

@_cdecl("nl_embedding_release")
public func nl_embedding_release(_ handle: UnsafeMutableRawPointer?) {
    guard let handle = handle else { return }
    embeddingStoreLock.lock()
    embeddingStore.removeValue(forKey: handle)
    embeddingStoreLock.unlock()
    Unmanaged<NLEmbedding>.fromOpaque(handle).release()
}

@_cdecl("nl_embedding_dimension")
public func nl_embedding_dimension(_ handle: UnsafeMutableRawPointer?) -> Int {
    guard let handle = handle else { return 0 }
    let emb = Unmanaged<NLEmbedding>.fromOpaque(handle).takeUnretainedValue()
    return emb.dimension
}

@_cdecl("nl_embedding_vocabulary_size")
public func nl_embedding_vocabulary_size(_ handle: UnsafeMutableRawPointer?) -> Int {
    guard let handle = handle else { return 0 }
    let emb = Unmanaged<NLEmbedding>.fromOpaque(handle).takeUnretainedValue()
    return emb.vocabularySize
}

/// Fill `out_buf` (length = embedding.dimension) with the vector for
/// `word`. Returns `true` on success.
@_cdecl("nl_embedding_vector_for_string")
public func nl_embedding_vector_for_string(
    _ handle: UnsafeMutableRawPointer?,
    _ word: UnsafePointer<CChar>,
    _ out_buf: UnsafeMutablePointer<Double>,
    _ out_len: Int
) -> Bool {
    guard let handle = handle else { return false }
    let emb = Unmanaged<NLEmbedding>.fromOpaque(handle).takeUnretainedValue()
    let wordStr = String(cString: word)
    guard let vec = emb.vector(for: wordStr) else { return false }
    let n = min(vec.count, out_len)
    for i in 0..<n { out_buf[i] = vec[i] }
    return true
}

/// Cosine distance between `a` and `b` per Apple's
/// `NLDistanceTypeCosine` (= 0). Returns `-1.0` if either string is
/// missing from the embedding.
@_cdecl("nl_embedding_distance")
public func nl_embedding_distance(
    _ handle: UnsafeMutableRawPointer?,
    _ a: UnsafePointer<CChar>,
    _ b: UnsafePointer<CChar>
) -> Double {
    guard let handle = handle else { return -1.0 }
    let emb = Unmanaged<NLEmbedding>.fromOpaque(handle).takeUnretainedValue()
    let aStr = String(cString: a)
    let bStr = String(cString: b)
    guard emb.contains(aStr), emb.contains(bStr) else { return -1.0 }
    return emb.distance(between: aStr, and: bStr, distanceType: .cosine)
}

/// Find up to `maxCount` nearest neighbours of `word`. Emits `n`
/// `NLEmbeddingNeighborRaw` rows into a newly-allocated buffer.
@_cdecl("nl_embedding_neighbors_for_string")
public func nl_embedding_neighbors_for_string(
    _ handle: UnsafeMutableRawPointer?,
    _ word: UnsafePointer<CChar>,
    _ maxCount: Int,
    _ out_array: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ out_count: UnsafeMutablePointer<Int>
) -> Bool {
    guard let handle = handle else { return false }
    let emb = Unmanaged<NLEmbedding>.fromOpaque(handle).takeUnretainedValue()
    let wordStr = String(cString: word)
    var names: [String] = []
    var dists: [Double] = []
    emb.enumerateNeighbors(for: wordStr, maximumCount: maxCount, distanceType: .cosine) {
        neighbor, dist in
        names.append(neighbor)
        dists.append(dist)
        return true
    }
    let n = names.count
    if n == 0 { out_array.pointee = nil; out_count.pointee = 0; return true }
    let buf = UnsafeMutablePointer<NLEmbeddingNeighborRaw>.allocate(capacity: n)
    for i in 0..<n {
        buf.advanced(by: i).initialize(to: NLEmbeddingNeighborRaw(
            word: strdup(names[i]),
            distance: dists[i]
        ))
    }
    out_array.pointee = UnsafeMutableRawPointer(buf)
    out_count.pointee = n
    return true
}

public struct NLEmbeddingNeighborRaw {
    public var word: UnsafeMutablePointer<CChar>?
    public var distance: Double
}

@_cdecl("nl_embedding_neighbors_free")
public func nl_embedding_neighbors_free(_ array: UnsafeMutableRawPointer?, _ count: Int) {
    guard let array = array else { return }
    let typed = array.assumingMemoryBound(to: NLEmbeddingNeighborRaw.self)
    for i in 0..<count { if let s = typed.advanced(by: i).pointee.word { free(s) } }
    typed.deallocate()
}
