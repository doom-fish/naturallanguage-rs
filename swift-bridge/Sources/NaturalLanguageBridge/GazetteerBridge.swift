import Foundation
import NaturalLanguage

private func nlGazetteer(_ handle: UnsafeMutableRawPointer?) -> NLGazetteer? {
    nlBorrow(handle)
}

private func nlGazetteerDictionary(
    _ entries: UnsafeMutableRawPointer?,
    _ count: Int
) -> [String: [String]] {
    guard let entries, count > 0 else { return [:] }
    let typed = entries.assumingMemoryBound(to: NLLabelTermRefRaw.self)
    var dictionary: [String: [String]] = [:]
    for index in 0..<count {
        let row = typed.advanced(by: index).pointee
        guard let label = nlString(row.label), let term = nlString(row.term) else { continue }
        dictionary[label, default: []].append(term)
    }
    return dictionary
}

@_cdecl("nl_gazetteer_with_contents_of_url")
public func nl_gazetteer_with_contents_of_url(
    _ path: UnsafePointer<CChar>?,
    _ outHandle: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let path else {
        nlSetError(outError, "path is required")
        return NL_INVALID_ARGUMENT
    }
    if #available(macOS 10.15, *) {
        do {
            let gazetteer = try NLGazetteer(contentsOf: URL(fileURLWithPath: String(cString: path)))
            outHandle.pointee = nlRetain(gazetteer)
            return NL_OK
        } catch {
            nlSetError(outError, error)
            outHandle.pointee = nil
            return nlNSErrorCode(error)
        }
    }
    nlSetError(outError, "NLGazetteer requires macOS 10.15+")
    return NL_UNSUPPORTED
}

@_cdecl("nl_gazetteer_with_data")
public func nl_gazetteer_with_data(
    _ bytes: UnsafePointer<UInt8>?,
    _ len: Int,
    _ outHandle: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let bytes, len >= 0 else {
        nlSetError(outError, "invalid gazetteer data")
        return NL_INVALID_ARGUMENT
    }
    if #available(macOS 10.15, *) {
        do {
            let data = Data(bytes: bytes, count: len)
            let gazetteer = try NLGazetteer(data: data)
            outHandle.pointee = nlRetain(gazetteer)
            return NL_OK
        } catch {
            nlSetError(outError, error)
            outHandle.pointee = nil
            return nlNSErrorCode(error)
        }
    }
    nlSetError(outError, "NLGazetteer requires macOS 10.15+")
    return NL_UNSUPPORTED
}

@_cdecl("nl_gazetteer_with_dictionary")
public func nl_gazetteer_with_dictionary(
    _ entries: UnsafeMutableRawPointer?,
    _ count: Int,
    _ language: UnsafePointer<CChar>?,
    _ outHandle: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    if #available(macOS 10.15, *) {
        do {
            let gazetteer = try NLGazetteer(
                dictionary: nlGazetteerDictionary(entries, count),
                language: language.map { NLLanguage(rawValue: String(cString: $0)) }
            )
            outHandle.pointee = nlRetain(gazetteer)
            return NL_OK
        } catch {
            nlSetError(outError, error)
            outHandle.pointee = nil
            return nlNSErrorCode(error)
        }
    }
    nlSetError(outError, "NLGazetteer requires macOS 10.15+")
    return NL_UNSUPPORTED
}

@_cdecl("nl_gazetteer_label_for_string")
public func nl_gazetteer_label_for_string(
    _ handle: UnsafeMutableRawPointer?,
    _ text: UnsafePointer<CChar>?,
    _ outLabel: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let gazetteer = nlGazetteer(handle), let text else {
        nlSetError(outError, "invalid gazetteer handle or text")
        return NL_INVALID_ARGUMENT
    }
    outLabel.pointee = gazetteer.label(for: String(cString: text)).map(nlFfiString) ?? nil
    return NL_OK
}

@_cdecl("nl_gazetteer_language")
public func nl_gazetteer_language(
    _ handle: UnsafeMutableRawPointer?,
    _ outLanguage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let gazetteer = nlGazetteer(handle) else {
        nlSetError(outError, "invalid gazetteer handle")
        return NL_INVALID_ARGUMENT
    }
    if #available(macOS 10.15, *) {
        outLanguage.pointee = gazetteer.language.map { nlFfiString($0.rawValue) } ?? nil
        return NL_OK
    }
    nlSetError(outError, "NLGazetteer requires macOS 10.15+")
    return NL_UNSUPPORTED
}

@_cdecl("nl_gazetteer_data")
public func nl_gazetteer_data(
    _ handle: UnsafeMutableRawPointer?,
    _ outBytes: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let gazetteer = nlGazetteer(handle) else {
        nlSetError(outError, "invalid gazetteer handle")
        return NL_INVALID_ARGUMENT
    }
    if #available(macOS 10.15, *) {
        guard let outBytes else {
            nlSetError(outError, "out bytes pointer is required")
            return NL_INVALID_ARGUMENT
        }
        outBytes.assumingMemoryBound(to: NLBytesRaw.self).pointee = nlCopyBytes(gazetteer.data)
        return NL_OK
    }
    nlSetError(outError, "NLGazetteer requires macOS 10.15+")
    return NL_UNSUPPORTED
}

@_cdecl("nl_gazetteer_write_dictionary")
public func nl_gazetteer_write_dictionary(
    _ entries: UnsafeMutableRawPointer?,
    _ count: Int,
    _ language: UnsafePointer<CChar>?,
    _ path: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let path else {
        nlSetError(outError, "path is required")
        return NL_INVALID_ARGUMENT
    }
    if #available(macOS 10.15, *) {
        do {
            try NLGazetteer.write(
                nlGazetteerDictionary(entries, count),
                language: language.map { NLLanguage(rawValue: String(cString: $0)) },
                to: URL(fileURLWithPath: String(cString: path))
            )
            return NL_OK
        } catch {
            nlSetError(outError, error)
            return nlNSErrorCode(error)
        }
    }
    nlSetError(outError, "NLGazetteer requires macOS 10.15+")
    return NL_UNSUPPORTED
}
