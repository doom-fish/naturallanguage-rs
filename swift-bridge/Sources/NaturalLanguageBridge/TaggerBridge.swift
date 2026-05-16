import Foundation
import NaturalLanguage

private func taggerStringAndRange(_ tagger: NLTagger, _ range: NLTextRangeRaw) -> (String, Range<String.Index>)? {
    guard let string = tagger.string else { return nil }
    let ns = nsRange(from: range)
    guard nlValidRange(ns, in: string), let swiftRange = Range(ns, in: string) else { return nil }
    return (string, swiftRange)
}

private func taggerTaggedSpans(
    _ tagger: NLTagger,
    range: NLTextRangeRaw,
    unit: NLTokenUnit,
    scheme: NLTagScheme,
    options: NLTagger.Options
) -> [(NSRange, String, String?)] {
    guard let (string, swiftRange) = taggerStringAndRange(tagger, range) else { return [] }
    var values: [(NSRange, String, String?)] = []
    tagger.enumerateTags(in: swiftRange, unit: unit, scheme: scheme, options: options) { tag, tokenRange in
        let ns = NSRange(tokenRange, in: string)
        values.append((ns, String(string[tokenRange]), tag?.rawValue))
        return true
    }
    return values
}

@_cdecl("nl_tagger_create")
public func nl_tagger_create(
    _ schemes: UnsafePointer<UnsafePointer<CChar>?>?,
    _ count: Int,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard let schemes, count >= 0 else {
        nlSetError(outError, "invalid tag scheme array")
        return nil
    }
    var values: [NLTagScheme] = []
    values.reserveCapacity(count)
    for index in 0..<count {
        guard let scheme = nlString(schemes.advanced(by: index).pointee) else { continue }
        values.append(NLTagScheme(rawValue: scheme))
    }
    return nlRetain(NLTagger(tagSchemes: values))
}

@_cdecl("nl_tagger_tag_schemes")
public func nl_tagger_tag_schemes(
    _ handle: UnsafeMutableRawPointer?,
    _ outArray: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ outCount: UnsafeMutablePointer<Int>,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let tagger: NLTagger = nlBorrow(handle) else {
        nlSetError(outError, "invalid tagger handle")
        return NL_INVALID_ARGUMENT
    }
    let (array, count) = nlStringArray(tagger.tagSchemes.map(\.rawValue))
    outArray.pointee = array
    outCount.pointee = count
    return NL_OK
}

@_cdecl("nl_tagger_string")
public func nl_tagger_string(
    _ handle: UnsafeMutableRawPointer?,
    _ outString: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let tagger: NLTagger = nlBorrow(handle) else {
        nlSetError(outError, "invalid tagger handle")
        return NL_INVALID_ARGUMENT
    }
    outString.pointee = tagger.string.map(nlFfiString) ?? nil
    return NL_OK
}

@_cdecl("nl_tagger_set_string")
public func nl_tagger_set_string(
    _ handle: UnsafeMutableRawPointer?,
    _ string: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let tagger: NLTagger = nlBorrow(handle) else {
        nlSetError(outError, "invalid tagger handle")
        return NL_INVALID_ARGUMENT
    }
    tagger.string = string.map { String(cString: $0) }
    return NL_OK
}

@_cdecl("nl_tagger_available_tag_schemes")
public func nl_tagger_available_tag_schemes(
    _ unit: Int32,
    _ language: UnsafePointer<CChar>?,
    _ outArray: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ outCount: UnsafeMutablePointer<Int>,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let language else {
        nlSetError(outError, "language is required")
        return NL_INVALID_ARGUMENT
    }
    let values = NLTagger.availableTagSchemes(for: nlTokenUnit(unit), language: NLLanguage(rawValue: String(cString: language)))
    let (array, count) = nlStringArray(values.map(\.rawValue))
    outArray.pointee = array
    outCount.pointee = count
    return NL_OK
}

@_cdecl("nl_tagger_token_range_at_index")
public func nl_tagger_token_range_at_index(
    _ handle: UnsafeMutableRawPointer?,
    _ characterIndex: Int,
    _ unit: Int32,
    _ outRange: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let tagger: NLTagger = nlBorrow(handle), let outRange, let string = tagger.string else {
        nlSetError(outError, "invalid tagger handle or out range")
        return NL_INVALID_ARGUMENT
    }
    guard characterIndex >= 0 && characterIndex < string.utf16.count else {
        nlSetError(outError, "character index out of bounds")
        return NL_INVALID_ARGUMENT
    }
    let index = nlIndex(characterIndex, in: string)
    let tokenRange = tagger.tokenRange(at: index, unit: nlTokenUnit(unit))
    outRange.assumingMemoryBound(to: NLTextRangeRaw.self).pointee = nlRange(from: tokenRange, in: string)
    return NL_OK
}

@_cdecl("nl_tagger_token_range_for_range")
public func nl_tagger_token_range_for_range(
    _ handle: UnsafeMutableRawPointer?,
    _ rangeStart: Int,
    _ rangeLength: Int,
    _ unit: Int32,
    _ outRange: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    let range = NLTextRangeRaw(start: rangeStart, length: rangeLength)
    guard let tagger: NLTagger = nlBorrow(handle), let outRange, let string = tagger.string,
          let swiftRange = Range(nsRange(from: range), in: string) else {
        nlSetError(outError, "invalid tagger handle or range")
        return NL_INVALID_ARGUMENT
    }
    let tokenRange = tagger.tokenRange(for: swiftRange, unit: nlTokenUnit(unit))
    outRange.assumingMemoryBound(to: NLTextRangeRaw.self).pointee = nlRange(from: tokenRange, in: string)
    return NL_OK
}

@_cdecl("nl_tagger_dominant_language")
public func nl_tagger_dominant_language(
    _ handle: UnsafeMutableRawPointer?,
    _ outLanguage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let tagger: NLTagger = nlBorrow(handle) else {
        nlSetError(outError, "invalid tagger handle")
        return NL_INVALID_ARGUMENT
    }
    guard let language = tagger.dominantLanguage else {
        outLanguage.pointee = nil
        return NL_NO_DOMINANT_LANGUAGE
    }
    outLanguage.pointee = nlFfiString(language.rawValue)
    return NL_OK
}

@_cdecl("nl_tagger_tags_in_range")
public func nl_tagger_tags_in_range(
    _ handle: UnsafeMutableRawPointer?,
    _ rangeStart: Int,
    _ rangeLength: Int,
    _ unit: Int32,
    _ scheme: UnsafePointer<CChar>?,
    _ options: UInt64,
    _ outArray: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ outCount: UnsafeMutablePointer<Int>,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let tagger: NLTagger = nlBorrow(handle), let scheme else {
        nlSetError(outError, "invalid tagger handle or scheme")
        return NL_INVALID_ARGUMENT
    }
    let spans = taggerTaggedSpans(
        tagger,
        range: NLTextRangeRaw(start: rangeStart, length: rangeLength),
        unit: nlTokenUnit(unit),
        scheme: NLTagScheme(rawValue: String(cString: scheme)),
        options: NLTagger.Options(rawValue: UInt(options))
    )
    guard !spans.isEmpty else {
        outArray.pointee = nil
        outCount.pointee = 0
        return NL_OK
    }
    let buffer = UnsafeMutablePointer<NLTagSpanRaw>.allocate(capacity: spans.count)
    for (index, span) in spans.enumerated() {
        buffer.advanced(by: index).initialize(to: NLTagSpanRaw(
            start: span.0.location,
            length: span.0.length,
            text: nlFfiString(span.1),
            tag: span.2.map(nlFfiString) ?? nil
        ))
    }
    outArray.pointee = UnsafeMutableRawPointer(buffer)
    outCount.pointee = spans.count
    return NL_OK
}

@_cdecl("nl_tagger_tag_at_index")
public func nl_tagger_tag_at_index(
    _ handle: UnsafeMutableRawPointer?,
    _ characterIndex: Int,
    _ unit: Int32,
    _ scheme: UnsafePointer<CChar>?,
    _ outSpan: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let tagger: NLTagger = nlBorrow(handle), let scheme, let outSpan, let string = tagger.string else {
        nlSetError(outError, "invalid tagger handle or scheme")
        return NL_INVALID_ARGUMENT
    }
    guard characterIndex >= 0 && characterIndex < string.utf16.count else {
        nlSetError(outError, "character index out of bounds")
        return NL_INVALID_ARGUMENT
    }
    let index = nlIndex(characterIndex, in: string)
    let (tag, tokenRange) = tagger.tag(at: index, unit: nlTokenUnit(unit), scheme: NLTagScheme(rawValue: String(cString: scheme)))
    let range = NSRange(tokenRange, in: string)
    outSpan.assumingMemoryBound(to: NLTagSpanRaw.self).pointee = NLTagSpanRaw(
        start: range.location,
        length: range.length,
        text: nlFfiString(String(string[tokenRange])),
        tag: tag.map { nlFfiString($0.rawValue) } ?? nil
    )
    return NL_OK
}

@_cdecl("nl_tagger_tag_hypotheses_at_index")
public func nl_tagger_tag_hypotheses_at_index(
    _ handle: UnsafeMutableRawPointer?,
    _ characterIndex: Int,
    _ unit: Int32,
    _ scheme: UnsafePointer<CChar>?,
    _ maximumCount: Int,
    _ outRange: UnsafeMutableRawPointer?,
    _ outArray: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ outCount: UnsafeMutablePointer<Int>,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let tagger: NLTagger = nlBorrow(handle), let scheme, let outRange, let string = tagger.string else {
        nlSetError(outError, "invalid tagger handle or scheme")
        return NL_INVALID_ARGUMENT
    }
    guard characterIndex >= 0 && characterIndex < string.utf16.count else {
        nlSetError(outError, "character index out of bounds")
        return NL_INVALID_ARGUMENT
    }
    let index = nlIndex(characterIndex, in: string)
    let (hypotheses, tokenRange) = tagger.tagHypotheses(
        at: index,
        unit: nlTokenUnit(unit),
        scheme: NLTagScheme(rawValue: String(cString: scheme)),
        maximumCount: maximumCount
    )
    let pairs = hypotheses.map { ($0.key, $0.value) }.sorted { $0.1 > $1.1 }
    let (array, count) = nlStringDoubleArray(pairs)
    outArray.pointee = array
    outCount.pointee = count
    outRange.assumingMemoryBound(to: NLTextRangeRaw.self).pointee = nlRange(from: tokenRange, in: string)
    return NL_OK
}

@_cdecl("nl_tagger_set_language")
public func nl_tagger_set_language(
    _ handle: UnsafeMutableRawPointer?,
    _ language: UnsafePointer<CChar>?,
    _ rangeStart: Int,
    _ rangeLength: Int,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    let range = NLTextRangeRaw(start: rangeStart, length: rangeLength)
    guard let tagger: NLTagger = nlBorrow(handle), let language, let string = tagger.string,
          let swiftRange = Range(nsRange(from: range), in: string) else {
        nlSetError(outError, "invalid tagger handle or range")
        return NL_INVALID_ARGUMENT
    }
    tagger.setLanguage(NLLanguage(rawValue: String(cString: language)), range: swiftRange)
    return NL_OK
}

@_cdecl("nl_tagger_set_orthography")
public func nl_tagger_set_orthography(
    _ handle: UnsafeMutableRawPointer?,
    _ dominantScript: UnsafePointer<CChar>?,
    _ entries: UnsafeMutableRawPointer?,
    _ entryCount: Int,
    _ rangeStart: Int,
    _ rangeLength: Int,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    let range = NLTextRangeRaw(start: rangeStart, length: rangeLength)
    guard let tagger: NLTagger = nlBorrow(handle), let string = tagger.string,
          let swiftRange = Range(nsRange(from: range), in: string) else {
        nlSetError(outError, "invalid tagger handle or range")
        return NL_INVALID_ARGUMENT
    }
    var languageMap: [String: [String]] = [:]
    if let entries, entryCount > 0 {
        let typed = entries.assumingMemoryBound(to: NLOrthographyEntryRefRaw.self)
        for index in 0..<entryCount {
            let row = typed.advanced(by: index).pointee
            guard let script = nlString(row.script), let language = nlString(row.language) else { continue }
            languageMap[script, default: []].append(language)
        }
    }
    let orthography = NSOrthography(
        dominantScript: nlString(dominantScript) ?? "",
        languageMap: languageMap
    )
    tagger.setOrthography(orthography, range: swiftRange)
    return NL_OK
}

@_cdecl("nl_tagger_set_models")
public func nl_tagger_set_models(
    _ handle: UnsafeMutableRawPointer?,
    _ models: UnsafePointer<UnsafeMutableRawPointer?>?,
    _ count: Int,
    _ scheme: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let tagger: NLTagger = nlBorrow(handle), let models, let scheme else {
        nlSetError(outError, "invalid tagger handle or model array")
        return NL_INVALID_ARGUMENT
    }
    var values: [NLModel] = []
    values.reserveCapacity(count)
    for index in 0..<count {
        if let model: NLModel = nlBorrow(models.advanced(by: index).pointee) {
            values.append(model)
        }
    }
    tagger.setModels(values, forTagScheme: NLTagScheme(rawValue: String(cString: scheme)))
    return NL_OK
}

@_cdecl("nl_tagger_models_for_tag_scheme")
public func nl_tagger_models_for_tag_scheme(
    _ handle: UnsafeMutableRawPointer?,
    _ scheme: UnsafePointer<CChar>?,
    _ outArray: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ outCount: UnsafeMutablePointer<Int>,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let tagger: NLTagger = nlBorrow(handle), let scheme else {
        nlSetError(outError, "invalid tagger handle or scheme")
        return NL_INVALID_ARGUMENT
    }
    let values = tagger.models(forTagScheme: NLTagScheme(rawValue: String(cString: scheme))).map(nlRetain)
    let (array, count) = nlHandleArray(values)
    outArray.pointee = array
    outCount.pointee = count
    return NL_OK
}

@_cdecl("nl_tagger_set_gazetteers")
public func nl_tagger_set_gazetteers(
    _ handle: UnsafeMutableRawPointer?,
    _ gazetteers: UnsafePointer<UnsafeMutableRawPointer?>?,
    _ count: Int,
    _ scheme: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let tagger: NLTagger = nlBorrow(handle), let gazetteers, let scheme else {
        nlSetError(outError, "invalid tagger handle or gazetteer array")
        return NL_INVALID_ARGUMENT
    }
    if #available(macOS 10.15, *) {
        var values: [NLGazetteer] = []
        values.reserveCapacity(count)
        for index in 0..<count {
            if let gazetteer: NLGazetteer = nlBorrow(gazetteers.advanced(by: index).pointee) {
                values.append(gazetteer)
            }
        }
        tagger.setGazetteers(values, for: NLTagScheme(rawValue: String(cString: scheme)))
        return NL_OK
    }
    nlSetError(outError, "NLGazetteer requires macOS 10.15+")
    return NL_UNSUPPORTED
}

@_cdecl("nl_tagger_gazetteers_for_tag_scheme")
public func nl_tagger_gazetteers_for_tag_scheme(
    _ handle: UnsafeMutableRawPointer?,
    _ scheme: UnsafePointer<CChar>?,
    _ outArray: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ outCount: UnsafeMutablePointer<Int>,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let tagger: NLTagger = nlBorrow(handle), let scheme else {
        nlSetError(outError, "invalid tagger handle or scheme")
        return NL_INVALID_ARGUMENT
    }
    if #available(macOS 10.15, *) {
        let values = tagger.gazetteers(for: NLTagScheme(rawValue: String(cString: scheme))).map(nlRetain)
        let (array, count) = nlHandleArray(values)
        outArray.pointee = array
        outCount.pointee = count
        return NL_OK
    }
    nlSetError(outError, "NLGazetteer requires macOS 10.15+")
    return NL_UNSUPPORTED
}

@_cdecl("nl_tagger_request_assets")
public func nl_tagger_request_assets(
    _ language: UnsafePointer<CChar>?,
    _ scheme: UnsafePointer<CChar>?,
    _ outResult: UnsafeMutablePointer<Int32>,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let language, let scheme else {
        nlSetError(outError, "language and scheme are required")
        return NL_INVALID_ARGUMENT
    }
    if #available(macOS 10.15, *) {
        let sem = DispatchSemaphore(value: 0)
        var result = NLTagger.AssetsResult.error
        var completionError: Error?
        NLTagger.requestAssets(
            for: NLLanguage(rawValue: String(cString: language)),
            tagScheme: NLTagScheme(rawValue: String(cString: scheme))
        ) { value, error in
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
    nlSetError(outError, "asset requests require macOS 10.15+")
    return NL_UNSUPPORTED
}
