import Foundation
import NaturalLanguage

@available(macOS 14.0, *)
private func nlContextualEmbedding(_ handle: UnsafeMutableRawPointer?) -> NLContextualEmbedding? {
    nlBorrow(handle)
}

@available(macOS 14.0, *)
private func nlContextualEmbeddingResultObject(_ handle: UnsafeMutableRawPointer?) -> NLContextualEmbeddingResult? {
    nlBorrow(handle)
}

@_cdecl("nl_contextual_embedding_with_model_identifier")
public func nl_contextual_embedding_with_model_identifier(_ identifier: UnsafePointer<CChar>?) -> UnsafeMutableRawPointer? {
    guard let identifier else { return nil }
    if #available(macOS 14.0, *) {
        guard let embedding = NLContextualEmbedding(modelIdentifier: String(cString: identifier)) else {
            return nil
        }
        return nlRetain(embedding)
    }
    return nil
}

@_cdecl("nl_contextual_embeddings_for_query")
public func nl_contextual_embeddings_for_query(
    _ languages: UnsafePointer<UnsafePointer<CChar>?>?,
    _ languageCount: Int,
    _ scripts: UnsafePointer<UnsafePointer<CChar>?>?,
    _ scriptCount: Int,
    _ hasRevision: Bool,
    _ revision: Int,
    _ outArray: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ outCount: UnsafeMutablePointer<Int>,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    if #available(macOS 14.0, *) {
        var query: [NLContextualEmbeddingKey: Any] = [:]
        let languageValues = nlCStringArray(languages, languageCount).map(NLLanguage.init(rawValue:))
        if !languageValues.isEmpty { query[.languages] = languageValues }
        let scriptValues = nlCStringArray(scripts, scriptCount).map(NLScript.init(rawValue:))
        if !scriptValues.isEmpty { query[.scripts] = scriptValues }
        if hasRevision { query[.revision] = revision }
        let handles = NLContextualEmbedding.contextualEmbeddings(forValues: query).map(nlRetain)
        let (array, count) = nlHandleArray(handles)
        outArray.pointee = array
        outCount.pointee = count
        return NL_OK
    }
    nlSetError(outError, "NLContextualEmbedding requires macOS 14+")
    return NL_UNSUPPORTED
}

@_cdecl("nl_contextual_embedding_with_language")
public func nl_contextual_embedding_with_language(_ language: UnsafePointer<CChar>?) -> UnsafeMutableRawPointer? {
    guard let language else { return nil }
    if #available(macOS 14.0, *) {
        return NLContextualEmbedding(language: NLLanguage(rawValue: String(cString: language))).map(nlRetain)
    }
    return nil
}

@_cdecl("nl_contextual_embedding_with_script")
public func nl_contextual_embedding_with_script(_ script: UnsafePointer<CChar>?) -> UnsafeMutableRawPointer? {
    guard let script else { return nil }
    if #available(macOS 14.0, *) {
        return NLContextualEmbedding(script: NLScript(rawValue: String(cString: script))).map(nlRetain)
    }
    return nil
}

@_cdecl("nl_contextual_embedding_model_identifier")
public func nl_contextual_embedding_model_identifier(
    _ handle: UnsafeMutableRawPointer?,
    _ outIdentifier: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    if #available(macOS 14.0, *) {
        guard let embedding = nlContextualEmbedding(handle) else {
            nlSetError(outError, "invalid contextual embedding handle")
            return NL_INVALID_ARGUMENT
        }
        outIdentifier.pointee = nlFfiString(embedding.modelIdentifier)
        return NL_OK
    }
    nlSetError(outError, "NLContextualEmbedding requires macOS 14+")
    return NL_UNSUPPORTED
}

@_cdecl("nl_contextual_embedding_languages")
public func nl_contextual_embedding_languages(
    _ handle: UnsafeMutableRawPointer?,
    _ outArray: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ outCount: UnsafeMutablePointer<Int>,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    if #available(macOS 14.0, *) {
        guard let embedding = nlContextualEmbedding(handle) else {
            nlSetError(outError, "invalid contextual embedding handle")
            return NL_INVALID_ARGUMENT
        }
        let (array, count) = nlStringArray(embedding.languages.map(\.rawValue))
        outArray.pointee = array
        outCount.pointee = count
        return NL_OK
    }
    nlSetError(outError, "NLContextualEmbedding requires macOS 14+")
    return NL_UNSUPPORTED
}

@_cdecl("nl_contextual_embedding_scripts")
public func nl_contextual_embedding_scripts(
    _ handle: UnsafeMutableRawPointer?,
    _ outArray: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ outCount: UnsafeMutablePointer<Int>,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    if #available(macOS 14.0, *) {
        guard let embedding = nlContextualEmbedding(handle) else {
            nlSetError(outError, "invalid contextual embedding handle")
            return NL_INVALID_ARGUMENT
        }
        let (array, count) = nlStringArray(embedding.scripts.map(\.rawValue))
        outArray.pointee = array
        outCount.pointee = count
        return NL_OK
    }
    nlSetError(outError, "NLContextualEmbedding requires macOS 14+")
    return NL_UNSUPPORTED
}

@_cdecl("nl_contextual_embedding_revision")
public func nl_contextual_embedding_revision(
    _ handle: UnsafeMutableRawPointer?,
    _ outRevision: UnsafeMutablePointer<Int>,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    if #available(macOS 14.0, *) {
        guard let embedding = nlContextualEmbedding(handle) else {
            nlSetError(outError, "invalid contextual embedding handle")
            return NL_INVALID_ARGUMENT
        }
        outRevision.pointee = embedding.revision
        return NL_OK
    }
    nlSetError(outError, "NLContextualEmbedding requires macOS 14+")
    return NL_UNSUPPORTED
}

@_cdecl("nl_contextual_embedding_dimension")
public func nl_contextual_embedding_dimension(
    _ handle: UnsafeMutableRawPointer?,
    _ outDimension: UnsafeMutablePointer<Int>,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    if #available(macOS 14.0, *) {
        guard let embedding = nlContextualEmbedding(handle) else {
            nlSetError(outError, "invalid contextual embedding handle")
            return NL_INVALID_ARGUMENT
        }
        outDimension.pointee = embedding.dimension
        return NL_OK
    }
    nlSetError(outError, "NLContextualEmbedding requires macOS 14+")
    return NL_UNSUPPORTED
}

@_cdecl("nl_contextual_embedding_maximum_sequence_length")
public func nl_contextual_embedding_maximum_sequence_length(
    _ handle: UnsafeMutableRawPointer?,
    _ outValue: UnsafeMutablePointer<Int>,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    if #available(macOS 14.0, *) {
        guard let embedding = nlContextualEmbedding(handle) else {
            nlSetError(outError, "invalid contextual embedding handle")
            return NL_INVALID_ARGUMENT
        }
        outValue.pointee = embedding.maximumSequenceLength
        return NL_OK
    }
    nlSetError(outError, "NLContextualEmbedding requires macOS 14+")
    return NL_UNSUPPORTED
}

@_cdecl("nl_contextual_embedding_load")
public func nl_contextual_embedding_load(
    _ handle: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    if #available(macOS 14.0, *) {
        guard let embedding = nlContextualEmbedding(handle) else {
            nlSetError(outError, "invalid contextual embedding handle")
            return NL_INVALID_ARGUMENT
        }
        do {
            try embedding.load()
            return NL_OK
        } catch {
            nlSetError(outError, error)
            return nlNSErrorCode(error)
        }
    }
    nlSetError(outError, "NLContextualEmbedding requires macOS 14+")
    return NL_UNSUPPORTED
}

@_cdecl("nl_contextual_embedding_unload")
public func nl_contextual_embedding_unload(
    _ handle: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    if #available(macOS 14.0, *) {
        guard let embedding = nlContextualEmbedding(handle) else {
            nlSetError(outError, "invalid contextual embedding handle")
            return NL_INVALID_ARGUMENT
        }
        embedding.unload()
        return NL_OK
    }
    nlSetError(outError, "NLContextualEmbedding requires macOS 14+")
    return NL_UNSUPPORTED
}

@_cdecl("nl_contextual_embedding_result_for_string")
public func nl_contextual_embedding_result_for_string(
    _ handle: UnsafeMutableRawPointer?,
    _ text: UnsafePointer<CChar>?,
    _ language: UnsafePointer<CChar>?,
    _ outResult: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let text else {
        nlSetError(outError, "text is required")
        return NL_INVALID_ARGUMENT
    }
    if #available(macOS 14.0, *) {
        guard let embedding = nlContextualEmbedding(handle) else {
            nlSetError(outError, "invalid contextual embedding handle")
            return NL_INVALID_ARGUMENT
        }
        do {
            let result = try embedding.embeddingResult(
                for: String(cString: text),
                language: language.map { NLLanguage(rawValue: String(cString: $0)) }
            )
            outResult.pointee = nlRetain(result)
            return NL_OK
        } catch {
            nlSetError(outError, error)
            outResult.pointee = nil
            return nlNSErrorCode(error)
        }
    }
    nlSetError(outError, "NLContextualEmbedding requires macOS 14+")
    return NL_UNSUPPORTED
}

@_cdecl("nl_contextual_embedding_has_available_assets")
public func nl_contextual_embedding_has_available_assets(
    _ handle: UnsafeMutableRawPointer?,
    _ outValue: UnsafeMutablePointer<Bool>,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    if #available(macOS 14.0, *) {
        guard let embedding = nlContextualEmbedding(handle) else {
            nlSetError(outError, "invalid contextual embedding handle")
            return NL_INVALID_ARGUMENT
        }
        outValue.pointee = embedding.hasAvailableAssets
        return NL_OK
    }
    nlSetError(outError, "NLContextualEmbedding requires macOS 14+")
    return NL_UNSUPPORTED
}

@_cdecl("nl_contextual_embedding_request_assets")
public func nl_contextual_embedding_request_assets(
    _ handle: UnsafeMutableRawPointer?,
    _ outResult: UnsafeMutablePointer<Int32>,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    if #available(macOS 14.0, *) {
        guard let embedding = nlContextualEmbedding(handle) else {
            nlSetError(outError, "invalid contextual embedding handle")
            return NL_INVALID_ARGUMENT
        }
        let sem = DispatchSemaphore(value: 0)
        var result = NLContextualEmbedding.AssetsResult.error
        var completionError: Error?
        embedding.requestAssets { value, error in
            result = value
            completionError = error
            sem.signal()
        }
        _ = sem.wait(timeout: .now() + .seconds(30))
        outResult.pointee = Int32(result.rawValue)
        if let completionError, result == .error {
            nlSetError(outError, completionError)
        }
        return NL_OK
    }
    nlSetError(outError, "NLContextualEmbedding requires macOS 14+")
    return NL_UNSUPPORTED
}

@_cdecl("nl_contextual_embedding_result_string")
public func nl_contextual_embedding_result_string(
    _ handle: UnsafeMutableRawPointer?,
    _ outString: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    if #available(macOS 14.0, *) {
        guard let result = nlContextualEmbeddingResultObject(handle) else {
            nlSetError(outError, "invalid contextual embedding result handle")
            return NL_INVALID_ARGUMENT
        }
        outString.pointee = nlFfiString(result.string)
        return NL_OK
    }
    nlSetError(outError, "NLContextualEmbedding requires macOS 14+")
    return NL_UNSUPPORTED
}

@_cdecl("nl_contextual_embedding_result_language")
public func nl_contextual_embedding_result_language(
    _ handle: UnsafeMutableRawPointer?,
    _ outLanguage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    if #available(macOS 14.0, *) {
        guard let result = nlContextualEmbeddingResultObject(handle) else {
            nlSetError(outError, "invalid contextual embedding result handle")
            return NL_INVALID_ARGUMENT
        }
        outLanguage.pointee = nlFfiString(result.language.rawValue)
        return NL_OK
    }
    nlSetError(outError, "NLContextualEmbedding requires macOS 14+")
    return NL_UNSUPPORTED
}

@_cdecl("nl_contextual_embedding_result_sequence_length")
public func nl_contextual_embedding_result_sequence_length(_ handle: UnsafeMutableRawPointer?) -> Int {
    if #available(macOS 14.0, *) {
        return nlContextualEmbeddingResultObject(handle)?.sequenceLength ?? 0
    }
    return 0
}

@_cdecl("nl_contextual_embedding_result_token_vectors_in_range")
public func nl_contextual_embedding_result_token_vectors_in_range(
    _ handle: UnsafeMutableRawPointer?,
    _ rangeStart: Int,
    _ rangeLength: Int,
    _ outArray: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ outCount: UnsafeMutablePointer<Int>,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    if #available(macOS 14.0, *) {
        let range = NLTextRangeRaw(start: rangeStart, length: rangeLength)
        guard let result = nlContextualEmbeddingResultObject(handle), let swiftRange = Range(nsRange(from: range), in: result.string) else {
            nlSetError(outError, "invalid contextual embedding result handle or range")
            return NL_INVALID_ARGUMENT
        }
        var values: [(NSRange, [Double])] = []
        result.enumerateTokenVectors(in: swiftRange) { tokenVector, tokenRange in
            values.append((NSRange(tokenRange, in: result.string), tokenVector))
            return true
        }
        guard !values.isEmpty else {
            outArray.pointee = nil
            outCount.pointee = 0
            return NL_OK
        }
        let buffer = UnsafeMutablePointer<NLTokenVectorRaw>.allocate(capacity: values.count)
        for (index, value) in values.enumerated() {
            buffer.advanced(by: index).initialize(to: NLTokenVectorRaw(
                start: value.0.location,
                length: value.0.length,
                values: nlCopyDoubles(value.1),
                len: value.1.count
            ))
        }
        outArray.pointee = UnsafeMutableRawPointer(buffer)
        outCount.pointee = values.count
        return NL_OK
    }
    nlSetError(outError, "NLContextualEmbedding requires macOS 14+")
    return NL_UNSUPPORTED
}

@_cdecl("nl_contextual_embedding_result_token_vector_at_index")
public func nl_contextual_embedding_result_token_vector_at_index(
    _ handle: UnsafeMutableRawPointer?,
    _ characterIndex: Int,
    _ outVector: UnsafeMutableRawPointer?,
    _ outFound: UnsafeMutablePointer<Bool>,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    if #available(macOS 14.0, *) {
        guard let result = nlContextualEmbeddingResultObject(handle), let outVector else {
            nlSetError(outError, "invalid contextual embedding result handle")
            return NL_INVALID_ARGUMENT
        }
        guard characterIndex >= 0 && characterIndex < result.string.utf16.count else {
            nlSetError(outError, "character index out of bounds")
            return NL_INVALID_ARGUMENT
        }
        let index = nlIndex(characterIndex, in: result.string)
        if let (vector, tokenRange) = result.tokenVector(at: index) {
            outFound.pointee = true
            outVector.assumingMemoryBound(to: NLTokenVectorRaw.self).pointee = NLTokenVectorRaw(
                start: NSRange(tokenRange, in: result.string).location,
                length: NSRange(tokenRange, in: result.string).length,
                values: nlCopyDoubles(vector),
                len: vector.count
            )
        } else {
            outFound.pointee = false
        }
        return NL_OK
    }
    nlSetError(outError, "NLContextualEmbedding requires macOS 14+")
    return NL_UNSUPPORTED
}
