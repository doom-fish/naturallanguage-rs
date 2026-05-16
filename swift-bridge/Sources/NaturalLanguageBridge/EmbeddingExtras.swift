import Foundation
import NaturalLanguage

private func nlEmbedding(_ handle: UnsafeMutableRawPointer?) -> NLEmbedding? {
    nlBorrow(handle)
}

private func nlNeighborArray(_ values: [(String, Double)]) -> (UnsafeMutableRawPointer?, Int) {
    guard !values.isEmpty else { return (nil, 0) }
    let buffer = UnsafeMutablePointer<NLEmbeddingNeighborRaw>.allocate(capacity: values.count)
    for (index, value) in values.enumerated() {
        buffer.advanced(by: index).initialize(to: NLEmbeddingNeighborRaw(
            word: nlFfiString(value.0),
            distance: value.1
        ))
    }
    return (UnsafeMutableRawPointer(buffer), values.count)
}

@_cdecl("nl_word_embedding_for_language_revision")
public func nl_word_embedding_for_language_revision(
    _ language: UnsafePointer<CChar>?,
    _ revision: Int
) -> UnsafeMutableRawPointer? {
    guard let language else { return nil }
    let lang = NLLanguage(rawValue: String(cString: language))
    guard let embedding = NLEmbedding.wordEmbedding(for: lang, revision: revision) else { return nil }
    return nlRetain(embedding)
}

@_cdecl("nl_sentence_embedding_for_language_revision")
public func nl_sentence_embedding_for_language_revision(
    _ language: UnsafePointer<CChar>?,
    _ revision: Int
) -> UnsafeMutableRawPointer? {
    guard let language else { return nil }
    let lang = NLLanguage(rawValue: String(cString: language))
    guard let embedding = NLEmbedding.sentenceEmbedding(for: lang, revision: revision) else { return nil }
    return nlRetain(embedding)
}

@_cdecl("nl_embedding_with_contents_of_url")
public func nl_embedding_with_contents_of_url(
    _ path: UnsafePointer<CChar>?,
    _ outHandle: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let path else {
        nlSetError(outError, "path is required")
        return NL_INVALID_ARGUMENT
    }
    do {
        let embedding = try NLEmbedding(contentsOf: URL(fileURLWithPath: String(cString: path)))
        outHandle.pointee = nlRetain(embedding)
        return NL_OK
    } catch {
        nlSetError(outError, error)
        outHandle.pointee = nil
        return nlNSErrorCode(error)
    }
}

@_cdecl("nl_embedding_contains_string")
public func nl_embedding_contains_string(
    _ handle: UnsafeMutableRawPointer?,
    _ word: UnsafePointer<CChar>?
) -> Bool {
    guard let embedding = nlEmbedding(handle), let word else { return false }
    return embedding.contains(String(cString: word))
}

@_cdecl("nl_embedding_language")
public func nl_embedding_language(
    _ handle: UnsafeMutableRawPointer?,
    _ outLanguage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let embedding = nlEmbedding(handle) else {
        nlSetError(outError, "invalid embedding handle")
        return NL_INVALID_ARGUMENT
    }
    outLanguage.pointee = embedding.language.map { nlFfiString($0.rawValue) } ?? nil
    return NL_OK
}

@_cdecl("nl_embedding_revision")
public func nl_embedding_revision(_ handle: UnsafeMutableRawPointer?) -> Int {
    guard let embedding = nlEmbedding(handle) else { return 0 }
    return embedding.revision
}

@_cdecl("nl_embedding_distance_with_type")
public func nl_embedding_distance_with_type(
    _ handle: UnsafeMutableRawPointer?,
    _ a: UnsafePointer<CChar>?,
    _ b: UnsafePointer<CChar>?,
    _ distanceType: Int32
) -> Double {
    guard let embedding = nlEmbedding(handle), let a, let b else { return -1.0 }
    let first = String(cString: a)
    let second = String(cString: b)
    guard embedding.contains(first), embedding.contains(second) else { return -1.0 }
    return embedding.distance(between: first, and: second, distanceType: nlDistanceType(distanceType))
}

@_cdecl("nl_embedding_neighbors_for_string_with_limit")
public func nl_embedding_neighbors_for_string_with_limit(
    _ handle: UnsafeMutableRawPointer?,
    _ word: UnsafePointer<CChar>?,
    _ maxCount: Int,
    _ maxDistance: Double,
    _ distanceType: Int32,
    _ outArray: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ outCount: UnsafeMutablePointer<Int>
) -> Bool {
    guard let embedding = nlEmbedding(handle), let word else { return false }
    let query = String(cString: word)
    var values: [(String, Double)] = []
    embedding.enumerateNeighbors(
        for: query,
        maximumCount: maxCount,
        distanceType: nlDistanceType(distanceType)
    ) { neighbor, distance in
        if maxDistance < 0 || distance <= maxDistance {
            values.append((neighbor, distance))
        }
        return true
    }
    let (array, count) = nlNeighborArray(values)
    outArray.pointee = array
    outCount.pointee = count
    return true
}

@_cdecl("nl_embedding_neighbors_for_vector")
public func nl_embedding_neighbors_for_vector(
    _ handle: UnsafeMutableRawPointer?,
    _ values: UnsafePointer<Double>?,
    _ len: Int,
    _ maxCount: Int,
    _ distanceType: Int32,
    _ outArray: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ outCount: UnsafeMutablePointer<Int>
) -> Bool {
    nl_embedding_neighbors_for_vector_with_limit(handle, values, len, maxCount, -1.0, distanceType, outArray, outCount)
}

@_cdecl("nl_embedding_neighbors_for_vector_with_limit")
public func nl_embedding_neighbors_for_vector_with_limit(
    _ handle: UnsafeMutableRawPointer?,
    _ values: UnsafePointer<Double>?,
    _ len: Int,
    _ maxCount: Int,
    _ maxDistance: Double,
    _ distanceType: Int32,
    _ outArray: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ outCount: UnsafeMutablePointer<Int>
) -> Bool {
    guard let embedding = nlEmbedding(handle), let values, len >= 0 else { return false }
    let vector = Array(UnsafeBufferPointer(start: values, count: len))
    var result: [(String, Double)] = []
    embedding.enumerateNeighbors(
        for: vector,
        maximumCount: maxCount,
        distanceType: nlDistanceType(distanceType)
    ) { neighbor, distance in
        if maxDistance < 0 || distance <= maxDistance {
            result.append((neighbor, distance))
        }
        return true
    }
    let (array, count) = nlNeighborArray(result)
    outArray.pointee = array
    outCount.pointee = count
    return true
}

@_cdecl("nl_embedding_supported_revisions_for_language")
public func nl_embedding_supported_revisions_for_language(
    _ language: UnsafePointer<CChar>?,
    _ outArray: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ outCount: UnsafeMutablePointer<Int>
) -> Int32 {
    guard let language else { return NL_INVALID_ARGUMENT }
    let revisions = Array(NLEmbedding.supportedRevisions(for: NLLanguage(rawValue: String(cString: language))))
    let (array, count) = nlUsizeArray(revisions)
    outArray.pointee = array
    outCount.pointee = count
    return NL_OK
}

@_cdecl("nl_embedding_current_revision_for_language")
public func nl_embedding_current_revision_for_language(_ language: UnsafePointer<CChar>?) -> Int {
    guard let language else { return 0 }
    return NLEmbedding.currentRevision(for: NLLanguage(rawValue: String(cString: language)))
}

@_cdecl("nl_embedding_supported_sentence_revisions_for_language")
public func nl_embedding_supported_sentence_revisions_for_language(
    _ language: UnsafePointer<CChar>?,
    _ outArray: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ outCount: UnsafeMutablePointer<Int>
) -> Int32 {
    guard let language else { return NL_INVALID_ARGUMENT }
    let revisions = Array(NLEmbedding.supportedSentenceEmbeddingRevisions(for: NLLanguage(rawValue: String(cString: language))))
    let (array, count) = nlUsizeArray(revisions)
    outArray.pointee = array
    outCount.pointee = count
    return NL_OK
}

@_cdecl("nl_embedding_current_sentence_revision_for_language")
public func nl_embedding_current_sentence_revision_for_language(_ language: UnsafePointer<CChar>?) -> Int {
    guard let language else { return 0 }
    return NLEmbedding.currentSentenceEmbeddingRevision(for: NLLanguage(rawValue: String(cString: language)))
}

@_cdecl("nl_embedding_write_dictionary")
public func nl_embedding_write_dictionary(
    _ entries: UnsafeMutableRawPointer?,
    _ count: Int,
    _ language: UnsafePointer<CChar>?,
    _ revision: Int,
    _ path: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let entries, let path else {
        nlSetError(outError, "entries and path are required")
        return NL_INVALID_ARGUMENT
    }
    var dictionary: [String: [Double]] = [:]
    let typed = entries.assumingMemoryBound(to: NLEmbeddingVectorEntryRefRaw.self)
    for index in 0..<count {
        let row = typed.advanced(by: index).pointee
        guard let word = nlString(row.word), let values = row.values else { continue }
        let vector = Array(UnsafeBufferPointer(start: values, count: row.len))
        dictionary[word] = vector
    }
    do {
        try NLEmbedding.write(
            dictionary,
            language: language.map { NLLanguage(rawValue: String(cString: $0)) },
            revision: revision,
            to: URL(fileURLWithPath: String(cString: path))
        )
        return NL_OK
    } catch {
        nlSetError(outError, error)
        return nlNSErrorCode(error)
    }
}
