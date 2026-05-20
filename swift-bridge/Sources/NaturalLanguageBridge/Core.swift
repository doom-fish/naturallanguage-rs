import CoreML
import Foundation
import NaturalLanguage

public let NL_OK: Int32 = 0
public let NL_INVALID_ARGUMENT: Int32 = -1
public let NL_NO_DOMINANT_LANGUAGE: Int32 = -2
public let NL_UNSUPPORTED: Int32 = -3
public let NL_UNKNOWN: Int32 = -99

@frozen
public struct NLTextRangeRaw {
    public var start: Int
    public var length: Int

    public init(start: Int = 0, length: Int = 0) {
        self.start = start
        self.length = length
    }
}

public struct NLStringRaw {
    public var value: UnsafeMutablePointer<CChar>?
}

public struct NLStringDoubleRaw {
    public var key: UnsafeMutablePointer<CChar>?
    public var value: Double
}

public struct NLTokenSpanRaw {
    public var start: Int
    public var length: Int
    public var text: UnsafeMutablePointer<CChar>?
    public var attributes: UInt64
}

public struct NLTagSpanRaw {
    public var start: Int
    public var length: Int
    public var text: UnsafeMutablePointer<CChar>?
    public var tag: UnsafeMutablePointer<CChar>?
}

public struct NLBytesRaw {
    public var bytes: UnsafeMutableRawPointer?
    public var len: Int
}

public struct NLLanguageHypothesisRefRaw {
    public var language: UnsafePointer<CChar>?
    public var confidence: Double
}

public struct NLEmbeddingVectorEntryRefRaw {
    public var word: UnsafePointer<CChar>?
    public var values: UnsafePointer<Double>?
    public var len: Int
}

public struct NLLabelTermRefRaw {
    public var label: UnsafePointer<CChar>?
    public var term: UnsafePointer<CChar>?
}

public struct NLOrthographyEntryRefRaw {
    public var script: UnsafePointer<CChar>?
    public var language: UnsafePointer<CChar>?
}

public struct NLTokenVectorRaw {
    public var start: Int
    public var length: Int
    public var values: UnsafeMutablePointer<Double>?
    public var len: Int
}

public struct NLHypothesisSetRaw {
    public var entries: UnsafeMutableRawPointer?
    public var count: Int
}

@inline(__always)
func nlFfiString(_ string: String) -> UnsafeMutablePointer<CChar>? {
    strdup(string)
}

@inline(__always)
func nlString(_ ptr: UnsafePointer<CChar>?) -> String? {
    guard let ptr else { return nil }
    return String(cString: ptr)
}

@inline(__always)
func nlCStringArray(_ values: UnsafePointer<UnsafePointer<CChar>?>?, _ count: Int) -> [String] {
    guard let values, count >= 0 else { return [] }
    var result: [String] = []
    result.reserveCapacity(count)
    for index in 0..<count {
        if let value = nlString(values.advanced(by: index).pointee) {
            result.append(value)
        }
    }
    return result
}

@inline(__always)
func nlRetain<T: AnyObject>(_ object: T) -> UnsafeMutableRawPointer {
    Unmanaged.passRetained(object).toOpaque()
}

@inline(__always)
func nlBorrow<T: AnyObject>(_ handle: UnsafeMutableRawPointer?) -> T? {
    guard let handle else { return nil }
    return Unmanaged<T>.fromOpaque(handle).takeUnretainedValue()
}

@inline(__always)
func nlNSErrorCode(_ error: Error) -> Int32 {
    let nsError = error as NSError
    return nsError.code == 0 ? NL_UNKNOWN : Int32(nsError.code)
}

@inline(__always)
func nlSetError(_ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?, _ error: Error) {
    outError?.pointee = nlFfiString((error as NSError).localizedDescription)
}

@inline(__always)
func nlSetError(_ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?, _ message: String) {
    outError?.pointee = nlFfiString(message)
}

@inline(__always)
func nlDistanceType(_ raw: Int32) -> NLDistanceType {
    switch raw {
    case 0: return .cosine
    default: return .cosine
    }
}

@inline(__always)
func nlTokenUnit(_ raw: Int32) -> NLTokenUnit {
    switch raw {
    case 1: return .sentence
    case 2: return .paragraph
    case 3: return .document
    default: return .word
    }
}

@inline(__always)
func nsRange(from raw: NLTextRangeRaw) -> NSRange {
    NSRange(location: raw.start, length: raw.length)
}

@inline(__always)
func nlRange(from range: Range<String.Index>, in string: String) -> NLTextRangeRaw {
    let nsRange = NSRange(range, in: string)
    return NLTextRangeRaw(start: nsRange.location, length: nsRange.length)
}

@inline(__always)
func nlIndex(_ utf16Offset: Int, in string: String) -> String.Index {
    String.Index(utf16Offset: utf16Offset, in: string)
}

@inline(__always)
func nlValidIndex(_ index: Int, in string: String?) -> Bool {
    guard let string else { return false }
    return index >= 0 && index <= string.utf16.count
}

@inline(__always)
func nlValidRange(_ range: NSRange, in string: String?) -> Bool {
    guard let string else { return false }
    let count = string.utf16.count
    return range.location >= 0 && range.location <= count && range.length >= 0 && range.location + range.length <= count
}

@inline(__always)
func nlStringArray(_ values: [String]) -> (UnsafeMutableRawPointer?, Int) {
    guard !values.isEmpty else { return (nil, 0) }
    let buffer = UnsafeMutablePointer<NLStringRaw>.allocate(capacity: values.count)
    for (index, value) in values.enumerated() {
        buffer.advanced(by: index).initialize(to: NLStringRaw(value: nlFfiString(value)))
    }
    return (UnsafeMutableRawPointer(buffer), values.count)
}

@inline(__always)
func nlStringDoubleArray(_ values: [(String, Double)]) -> (UnsafeMutableRawPointer?, Int) {
    guard !values.isEmpty else { return (nil, 0) }
    let buffer = UnsafeMutablePointer<NLStringDoubleRaw>.allocate(capacity: values.count)
    for (index, value) in values.enumerated() {
        buffer.advanced(by: index).initialize(to: NLStringDoubleRaw(key: nlFfiString(value.0), value: value.1))
    }
    return (UnsafeMutableRawPointer(buffer), values.count)
}

@inline(__always)
func nlHandleArray(_ values: [UnsafeMutableRawPointer]) -> (UnsafeMutableRawPointer?, Int) {
    guard !values.isEmpty else { return (nil, 0) }
    let buffer = UnsafeMutablePointer<UnsafeMutableRawPointer?>.allocate(capacity: values.count)
    for (index, value) in values.enumerated() {
        buffer.advanced(by: index).initialize(to: value)
    }
    return (UnsafeMutableRawPointer(buffer), values.count)
}

@inline(__always)
func nlUsizeArray(_ values: [Int]) -> (UnsafeMutableRawPointer?, Int) {
    guard !values.isEmpty else { return (nil, 0) }
    let buffer = UnsafeMutablePointer<Int>.allocate(capacity: values.count)
    for (index, value) in values.enumerated() {
        buffer.advanced(by: index).initialize(to: value)
    }
    return (UnsafeMutableRawPointer(buffer), values.count)
}

@inline(__always)
func nlCopyBytes(_ data: Data) -> NLBytesRaw {
    guard !data.isEmpty else { return NLBytesRaw(bytes: nil, len: 0) }
    let ptr = malloc(data.count)
    guard let ptr else { return NLBytesRaw(bytes: nil, len: 0) }
    data.copyBytes(to: ptr.assumingMemoryBound(to: UInt8.self), count: data.count)
    return NLBytesRaw(bytes: ptr, len: data.count)
}

@inline(__always)
func nlCopyDoubles(_ values: [Double]) -> UnsafeMutablePointer<Double>? {
    guard !values.isEmpty else { return nil }
    let buffer = UnsafeMutablePointer<Double>.allocate(capacity: values.count)
    for (index, value) in values.enumerated() {
        buffer.advanced(by: index).initialize(to: value)
    }
    return buffer
}

@_cdecl("nl_object_retain")
public func nl_object_retain(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let handle else { return nil }
    let object = Unmanaged<AnyObject>.fromOpaque(handle).takeUnretainedValue()
    return nlRetain(object)
}

@_cdecl("nl_object_release")
public func nl_object_release(_ handle: UnsafeMutableRawPointer?) {
    guard let handle else { return }
    Unmanaged<AnyObject>.fromOpaque(handle).release()
}

@_cdecl("nl_strings_free")
public func nl_strings_free(_ array: UnsafeMutableRawPointer?, _ count: Int) {
    guard let array else { return }
    let typed = array.assumingMemoryBound(to: NLStringRaw.self)
    for index in 0..<count {
        if let value = typed.advanced(by: index).pointee.value { free(value) }
    }
    typed.deallocate()
}

@_cdecl("nl_string_doubles_free")
public func nl_string_doubles_free(_ array: UnsafeMutableRawPointer?, _ count: Int) {
    guard let array else { return }
    let typed = array.assumingMemoryBound(to: NLStringDoubleRaw.self)
    for index in 0..<count {
        if let key = typed.advanced(by: index).pointee.key { free(key) }
    }
    typed.deallocate()
}

@_cdecl("nl_handle_array_free")
public func nl_handle_array_free(_ array: UnsafeMutableRawPointer?, _ count: Int) {
    guard let array else { return }
    let typed = array.assumingMemoryBound(to: UnsafeMutableRawPointer?.self)
    typed.deallocate()
}

@_cdecl("nl_usizes_free")
public func nl_usizes_free(_ array: UnsafeMutableRawPointer?, _ count: Int) {
    guard let array else { return }
    let typed = array.assumingMemoryBound(to: Int.self)
    typed.deallocate()
}

@_cdecl("nl_bytes_free")
public func nl_bytes_free(_ bytes: UnsafeMutableRawPointer?) {
    guard let bytes else { return }
    free(bytes)
}

@_cdecl("nl_doubles_free")
public func nl_doubles_free(_ values: UnsafeMutablePointer<Double>?, _ count: Int) {
    guard let values else { return }
    values.deallocate()
}

@_cdecl("nl_token_spans_free")
public func nl_token_spans_free(_ array: UnsafeMutableRawPointer?, _ count: Int) {
    guard let array else { return }
    let typed = array.assumingMemoryBound(to: NLTokenSpanRaw.self)
    for index in 0..<count {
        if let text = typed.advanced(by: index).pointee.text { free(text) }
    }
    typed.deallocate()
}

@_cdecl("nl_tag_span_clear")
public func nl_tag_span_clear(_ span: UnsafeMutableRawPointer?) {
    guard let span else { return }
    let typed = span.assumingMemoryBound(to: NLTagSpanRaw.self)
    if let text = typed.pointee.text { free(text) }
    if let tag = typed.pointee.tag { free(tag) }
    typed.pointee.text = nil
    typed.pointee.tag = nil
}

@_cdecl("nl_tag_spans_free")
public func nl_tag_spans_free(_ array: UnsafeMutableRawPointer?, _ count: Int) {
    guard let array else { return }
    let typed = array.assumingMemoryBound(to: NLTagSpanRaw.self)
    for index in 0..<count {
        let entry = typed.advanced(by: index)
        if let text = entry.pointee.text { free(text) }
        if let tag = entry.pointee.tag { free(tag) }
    }
    typed.deallocate()
}

@_cdecl("nl_token_vector_clear")
public func nl_token_vector_clear(_ vector: UnsafeMutableRawPointer?) {
    guard let vector else { return }
    let typed = vector.assumingMemoryBound(to: NLTokenVectorRaw.self)
    if let values = typed.pointee.values { values.deallocate() }
    typed.pointee.values = nil
    typed.pointee.len = 0
}

@_cdecl("nl_token_vectors_free")
public func nl_token_vectors_free(_ array: UnsafeMutableRawPointer?, _ count: Int) {
    guard let array else { return }
    let typed = array.assumingMemoryBound(to: NLTokenVectorRaw.self)
    for index in 0..<count {
        if let values = typed.advanced(by: index).pointee.values { values.deallocate() }
    }
    typed.deallocate()
}

@_cdecl("nl_hypothesis_sets_free")
public func nl_hypothesis_sets_free(_ array: UnsafeMutableRawPointer?, _ count: Int) {
    guard let array else { return }
    let typed = array.assumingMemoryBound(to: NLHypothesisSetRaw.self)
    typed.deallocate()
}
