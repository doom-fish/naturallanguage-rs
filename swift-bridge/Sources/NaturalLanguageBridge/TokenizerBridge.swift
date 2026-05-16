import Foundation
import NaturalLanguage

private func tokenizerSpans(_ tokenizer: NLTokenizer, range: NSRange) -> [(NSRange, String, UInt64)] {
    guard let string = tokenizer.string, nlValidRange(range, in: string), let swiftRange = Range(range, in: string) else {
        return []
    }
    var values: [(NSRange, String, UInt64)] = []
    tokenizer.enumerateTokens(in: swiftRange) { tokenRange, attributes in
        let ns = NSRange(tokenRange, in: string)
        values.append((ns, String(string[tokenRange]), UInt64(attributes.rawValue)))
        return true
    }
    return values
}

@_cdecl("nl_tokenizer_create")
public func nl_tokenizer_create(_ unit: Int32) -> UnsafeMutableRawPointer? {
    nlRetain(NLTokenizer(unit: nlTokenUnit(unit)))
}

@_cdecl("nl_tokenizer_unit")
public func nl_tokenizer_unit(_ handle: UnsafeMutableRawPointer?) -> Int32 {
    guard let tokenizer: NLTokenizer = nlBorrow(handle) else { return 0 }
    switch tokenizer.unit {
    case .sentence: return 1
    case .paragraph: return 2
    case .document: return 3
    default: return 0
    }
}

@_cdecl("nl_tokenizer_string")
public func nl_tokenizer_string(
    _ handle: UnsafeMutableRawPointer?,
    _ outString: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let tokenizer: NLTokenizer = nlBorrow(handle) else {
        nlSetError(outError, "invalid tokenizer handle")
        return NL_INVALID_ARGUMENT
    }
    outString.pointee = tokenizer.string.map(nlFfiString) ?? nil
    return NL_OK
}

@_cdecl("nl_tokenizer_set_string")
public func nl_tokenizer_set_string(
    _ handle: UnsafeMutableRawPointer?,
    _ string: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let tokenizer: NLTokenizer = nlBorrow(handle) else {
        nlSetError(outError, "invalid tokenizer handle")
        return NL_INVALID_ARGUMENT
    }
    tokenizer.string = string.map { String(cString: $0) }
    return NL_OK
}

@_cdecl("nl_tokenizer_set_language")
public func nl_tokenizer_set_language(
    _ handle: UnsafeMutableRawPointer?,
    _ language: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let tokenizer: NLTokenizer = nlBorrow(handle), let language else {
        nlSetError(outError, "invalid tokenizer handle or language")
        return NL_INVALID_ARGUMENT
    }
    tokenizer.setLanguage(NLLanguage(rawValue: String(cString: language)))
    return NL_OK
}

@_cdecl("nl_tokenizer_token_range_at_index")
public func nl_tokenizer_token_range_at_index(
    _ handle: UnsafeMutableRawPointer?,
    _ characterIndex: Int,
    _ outRange: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let tokenizer: NLTokenizer = nlBorrow(handle), let outRange else {
        nlSetError(outError, "invalid tokenizer handle or out range")
        return NL_INVALID_ARGUMENT
    }
    guard let string = tokenizer.string, !string.isEmpty else {
        nlSetError(outError, "tokenizer string is empty")
        return NL_INVALID_ARGUMENT
    }
    guard characterIndex >= 0 && characterIndex < string.utf16.count else {
        nlSetError(outError, "character index out of bounds")
        return NL_INVALID_ARGUMENT
    }
    let spans = tokenizerSpans(tokenizer, range: NSRange(location: 0, length: string.utf16.count))
    guard let match = spans.first(where: { span in
        let end = span.0.location + span.0.length
        return characterIndex >= span.0.location && characterIndex < end
    }) else {
        nlSetError(outError, "no token found at index")
        return NL_INVALID_ARGUMENT
    }
    outRange.assumingMemoryBound(to: NLTextRangeRaw.self).pointee = NLTextRangeRaw(start: match.0.location, length: match.0.length)
    return NL_OK
}

@_cdecl("nl_tokenizer_token_range_for_range")
public func nl_tokenizer_token_range_for_range(
    _ handle: UnsafeMutableRawPointer?,
    _ rangeStart: Int,
    _ rangeLength: Int,
    _ outRange: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let tokenizer: NLTokenizer = nlBorrow(handle), let outRange else {
        nlSetError(outError, "invalid tokenizer handle or out range")
        return NL_INVALID_ARGUMENT
    }
    let range = NLTextRangeRaw(start: rangeStart, length: rangeLength)
    guard let string = tokenizer.string, nlValidRange(nsRange(from: range), in: string) else {
        nlSetError(outError, "tokenizer range out of bounds")
        return NL_INVALID_ARGUMENT
    }
    if range.length == 0 {
        return nl_tokenizer_token_range_at_index(handle, range.start, outRange, outError)
    }
    let spans = tokenizerSpans(tokenizer, range: nsRange(from: range))
    guard let first = spans.first, let last = spans.last else {
        nlSetError(outError, "no tokens found in range")
        return NL_INVALID_ARGUMENT
    }
    outRange.assumingMemoryBound(to: NLTextRangeRaw.self).pointee = NLTextRangeRaw(
        start: first.0.location,
        length: (last.0.location + last.0.length) - first.0.location
    )
    return NL_OK
}

@_cdecl("nl_tokenizer_tokens_in_range")
public func nl_tokenizer_tokens_in_range(
    _ handle: UnsafeMutableRawPointer?,
    _ rangeStart: Int,
    _ rangeLength: Int,
    _ outArray: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ outCount: UnsafeMutablePointer<Int>,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let tokenizer: NLTokenizer = nlBorrow(handle) else {
        nlSetError(outError, "invalid tokenizer handle")
        return NL_INVALID_ARGUMENT
    }
    let range = NLTextRangeRaw(start: rangeStart, length: rangeLength)
    let spans = tokenizerSpans(tokenizer, range: nsRange(from: range))
    guard !spans.isEmpty else {
        outArray.pointee = nil
        outCount.pointee = 0
        return NL_OK
    }
    let buffer = UnsafeMutablePointer<NLTokenSpanRaw>.allocate(capacity: spans.count)
    for (index, span) in spans.enumerated() {
        buffer.advanced(by: index).initialize(to: NLTokenSpanRaw(
            start: span.0.location,
            length: span.0.length,
            text: nlFfiString(span.1),
            attributes: span.2
        ))
    }
    outArray.pointee = UnsafeMutableRawPointer(buffer)
    outCount.pointee = spans.count
    return NL_OK
}
