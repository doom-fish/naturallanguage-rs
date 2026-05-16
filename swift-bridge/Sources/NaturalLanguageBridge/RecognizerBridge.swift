import Foundation
import NaturalLanguage

@_cdecl("nl_language_recognizer_create")
public func nl_language_recognizer_create() -> UnsafeMutableRawPointer? {
    nlRetain(NLLanguageRecognizer())
}

@_cdecl("nl_language_recognizer_process_string")
public func nl_language_recognizer_process_string(
    _ handle: UnsafeMutableRawPointer?,
    _ text: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let recognizer: NLLanguageRecognizer = nlBorrow(handle), let text else {
        nlSetError(outError, "invalid language recognizer handle or text")
        return NL_INVALID_ARGUMENT
    }
    recognizer.processString(String(cString: text))
    return NL_OK
}

@_cdecl("nl_language_recognizer_reset")
public func nl_language_recognizer_reset(_ handle: UnsafeMutableRawPointer?) {
    guard let recognizer: NLLanguageRecognizer = nlBorrow(handle) else { return }
    recognizer.reset()
}

@_cdecl("nl_language_recognizer_dominant_language")
public func nl_language_recognizer_dominant_language(
    _ handle: UnsafeMutableRawPointer?,
    _ outLanguage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let recognizer: NLLanguageRecognizer = nlBorrow(handle) else {
        nlSetError(outError, "invalid language recognizer handle")
        return NL_INVALID_ARGUMENT
    }
    guard let language = recognizer.dominantLanguage else {
        outLanguage.pointee = nil
        return NL_NO_DOMINANT_LANGUAGE
    }
    outLanguage.pointee = nlFfiString(language.rawValue)
    return NL_OK
}

@_cdecl("nl_language_recognizer_language_hypotheses")
public func nl_language_recognizer_language_hypotheses(
    _ handle: UnsafeMutableRawPointer?,
    _ maxHypotheses: Int,
    _ outArray: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ outCount: UnsafeMutablePointer<Int>,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let recognizer: NLLanguageRecognizer = nlBorrow(handle) else {
        nlSetError(outError, "invalid language recognizer handle")
        return NL_INVALID_ARGUMENT
    }
    let sorted = recognizer.languageHypotheses(withMaximum: maxHypotheses)
        .sorted { $0.value > $1.value }
    guard !sorted.isEmpty else {
        outArray.pointee = nil
        outCount.pointee = 0
        return NL_OK
    }
    let buffer = UnsafeMutablePointer<NLLanguageHypothesisRaw>.allocate(capacity: sorted.count)
    for (index, value) in sorted.enumerated() {
        buffer.advanced(by: index).initialize(to: NLLanguageHypothesisRaw(
            language: nlFfiString(value.key.rawValue),
            confidence: value.value
        ))
    }
    outArray.pointee = UnsafeMutableRawPointer(buffer)
    outCount.pointee = sorted.count
    return NL_OK
}

@_cdecl("nl_language_recognizer_language_hints")
public func nl_language_recognizer_language_hints(
    _ handle: UnsafeMutableRawPointer?,
    _ outArray: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ outCount: UnsafeMutablePointer<Int>,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let recognizer: NLLanguageRecognizer = nlBorrow(handle) else {
        nlSetError(outError, "invalid language recognizer handle")
        return NL_INVALID_ARGUMENT
    }
    let sorted = recognizer.languageHints.sorted { $0.value > $1.value }
    guard !sorted.isEmpty else {
        outArray.pointee = nil
        outCount.pointee = 0
        return NL_OK
    }
    let buffer = UnsafeMutablePointer<NLLanguageHypothesisRaw>.allocate(capacity: sorted.count)
    for (index, value) in sorted.enumerated() {
        buffer.advanced(by: index).initialize(to: NLLanguageHypothesisRaw(
            language: nlFfiString(value.key.rawValue),
            confidence: value.value
        ))
    }
    outArray.pointee = UnsafeMutableRawPointer(buffer)
    outCount.pointee = sorted.count
    return NL_OK
}

@_cdecl("nl_language_recognizer_set_language_hints")
public func nl_language_recognizer_set_language_hints(
    _ handle: UnsafeMutableRawPointer?,
    _ hints: UnsafeMutableRawPointer?,
    _ count: Int,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let recognizer: NLLanguageRecognizer = nlBorrow(handle) else {
        nlSetError(outError, "invalid language recognizer handle")
        return NL_INVALID_ARGUMENT
    }
    var value: [NLLanguage: Double] = [:]
    if let hints, count > 0 {
        let typed = hints.assumingMemoryBound(to: NLLanguageHypothesisRefRaw.self)
        for index in 0..<count {
            let row = typed.advanced(by: index).pointee
            guard let languageString = nlString(row.language) else { continue }
            value[NLLanguage(rawValue: languageString)] = row.confidence
        }
    }
    recognizer.languageHints = value
    return NL_OK
}

@_cdecl("nl_language_recognizer_language_constraints")
public func nl_language_recognizer_language_constraints(
    _ handle: UnsafeMutableRawPointer?,
    _ outArray: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ outCount: UnsafeMutablePointer<Int>,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let recognizer: NLLanguageRecognizer = nlBorrow(handle) else {
        nlSetError(outError, "invalid language recognizer handle")
        return NL_INVALID_ARGUMENT
    }
    let values = recognizer.languageConstraints.map(\.rawValue)
    let (array, count) = nlStringArray(values)
    outArray.pointee = array
    outCount.pointee = count
    return NL_OK
}

@_cdecl("nl_language_recognizer_set_language_constraints")
public func nl_language_recognizer_set_language_constraints(
    _ handle: UnsafeMutableRawPointer?,
    _ constraints: UnsafePointer<UnsafePointer<CChar>?>?,
    _ count: Int,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let recognizer: NLLanguageRecognizer = nlBorrow(handle) else {
        nlSetError(outError, "invalid language recognizer handle")
        return NL_INVALID_ARGUMENT
    }
    guard let constraints, count >= 0 else {
        recognizer.languageConstraints = []
        return NL_OK
    }
    var values: [NLLanguage] = []
    values.reserveCapacity(count)
    for index in 0..<count {
        guard let language = nlString(constraints.advanced(by: index).pointee) else { continue }
        values.append(NLLanguage(rawValue: language))
    }
    recognizer.languageConstraints = values
    return NL_OK
}
