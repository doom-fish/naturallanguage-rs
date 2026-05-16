import CoreML
import Foundation
import NaturalLanguage

private func nlModel(_ handle: UnsafeMutableRawPointer?) -> NLModel? {
    nlBorrow(handle)
}

private func nlModelConfigurationObject(_ handle: UnsafeMutableRawPointer?) -> NLModelConfiguration? {
    nlBorrow(handle)
}

private func nlCoreMlModel(_ handle: UnsafeMutableRawPointer?) -> MLModel? {
    nlBorrow(handle)
}

private func nlModelType(_ raw: Int32) -> NLModel.ModelType {
    raw == 1 ? .sequence : .classifier
}

@_cdecl("nl_coreml_model_create_from_source_path")
public func nl_coreml_model_create_from_source_path(
    _ path: UnsafePointer<CChar>?,
    _ outHandle: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let path else {
        nlSetError(outError, "path is required")
        return NL_INVALID_ARGUMENT
    }
    do {
        let compiled = try MLModel.compileModel(at: URL(fileURLWithPath: String(cString: path)))
        let model = try MLModel(contentsOf: compiled)
        outHandle.pointee = nlRetain(model)
        return NL_OK
    } catch {
        nlSetError(outError, error)
        outHandle.pointee = nil
        return nlNSErrorCode(error)
    }
}

@_cdecl("nl_coreml_model_create_from_compiled_path")
public func nl_coreml_model_create_from_compiled_path(
    _ path: UnsafePointer<CChar>?,
    _ outHandle: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let path else {
        nlSetError(outError, "path is required")
        return NL_INVALID_ARGUMENT
    }
    do {
        let model = try MLModel(contentsOf: URL(fileURLWithPath: String(cString: path)))
        outHandle.pointee = nlRetain(model)
        return NL_OK
    } catch {
        nlSetError(outError, error)
        outHandle.pointee = nil
        return nlNSErrorCode(error)
    }
}

@_cdecl("nl_model_with_contents_of_url")
public func nl_model_with_contents_of_url(
    _ path: UnsafePointer<CChar>?,
    _ outHandle: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let path else {
        nlSetError(outError, "path is required")
        return NL_INVALID_ARGUMENT
    }
    do {
        let model = try NLModel(contentsOf: URL(fileURLWithPath: String(cString: path)))
        outHandle.pointee = nlRetain(model)
        return NL_OK
    } catch {
        nlSetError(outError, error)
        outHandle.pointee = nil
        return nlNSErrorCode(error)
    }
}

@_cdecl("nl_model_with_mlmodel")
public func nl_model_with_mlmodel(
    _ coremlHandle: UnsafeMutableRawPointer?,
    _ outHandle: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let model = nlCoreMlModel(coremlHandle) else {
        nlSetError(outError, "invalid MLModel handle")
        return NL_INVALID_ARGUMENT
    }
    do {
        let nlModel = try NLModel(mlModel: model)
        outHandle.pointee = nlRetain(nlModel)
        return NL_OK
    } catch {
        nlSetError(outError, error)
        outHandle.pointee = nil
        return nlNSErrorCode(error)
    }
}

@_cdecl("nl_model_configuration")
public func nl_model_configuration(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let model = nlModel(handle) else { return nil }
    return nlRetain(model.configuration)
}

@_cdecl("nl_model_predicted_label_for_string")
public func nl_model_predicted_label_for_string(
    _ handle: UnsafeMutableRawPointer?,
    _ text: UnsafePointer<CChar>?,
    _ outLabel: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let model = nlModel(handle), let text else {
        nlSetError(outError, "invalid NLModel handle or text")
        return NL_INVALID_ARGUMENT
    }
    outLabel.pointee = model.predictedLabel(for: String(cString: text)).map(nlFfiString) ?? nil
    return NL_OK
}

@_cdecl("nl_model_predicted_labels_for_tokens")
public func nl_model_predicted_labels_for_tokens(
    _ handle: UnsafeMutableRawPointer?,
    _ tokens: UnsafePointer<UnsafePointer<CChar>?>?,
    _ count: Int,
    _ outArray: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ outCount: UnsafeMutablePointer<Int>,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let model = nlModel(handle) else {
        nlSetError(outError, "invalid NLModel handle")
        return NL_INVALID_ARGUMENT
    }
    let labels = model.predictedLabels(forTokens: nlCStringArray(tokens, count))
    let (array, count) = nlStringArray(labels)
    outArray.pointee = array
    outCount.pointee = count
    return NL_OK
}

@_cdecl("nl_model_predicted_label_hypotheses_for_string")
public func nl_model_predicted_label_hypotheses_for_string(
    _ handle: UnsafeMutableRawPointer?,
    _ text: UnsafePointer<CChar>?,
    _ maximumCount: Int,
    _ outArray: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ outCount: UnsafeMutablePointer<Int>,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let model = nlModel(handle), let text else {
        nlSetError(outError, "invalid NLModel handle or text")
        return NL_INVALID_ARGUMENT
    }
    let values = model.predictedLabelHypotheses(for: String(cString: text), maximumCount: maximumCount)
        .map { ($0.key, $0.value) }
        .sorted { $0.1 > $1.1 }
    let (array, count) = nlStringDoubleArray(values)
    outArray.pointee = array
    outCount.pointee = count
    return NL_OK
}

@_cdecl("nl_model_predicted_label_hypotheses_for_tokens")
public func nl_model_predicted_label_hypotheses_for_tokens(
    _ handle: UnsafeMutableRawPointer?,
    _ tokens: UnsafePointer<UnsafePointer<CChar>?>?,
    _ count: Int,
    _ maximumCount: Int,
    _ outArray: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ outCount: UnsafeMutablePointer<Int>,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let model = nlModel(handle) else {
        nlSetError(outError, "invalid NLModel handle")
        return NL_INVALID_ARGUMENT
    }
    let hypotheses = model.predictedLabelHypotheses(forTokens: nlCStringArray(tokens, count), maximumCount: maximumCount)
    guard !hypotheses.isEmpty else {
        outArray.pointee = nil
        outCount.pointee = 0
        return NL_OK
    }
    let buffer = UnsafeMutablePointer<NLHypothesisSetRaw>.allocate(capacity: hypotheses.count)
    for (index, hypothesis) in hypotheses.enumerated() {
        let pairs = hypothesis.map { ($0.key, $0.value) }.sorted { $0.1 > $1.1 }
        let (entries, entryCount) = nlStringDoubleArray(pairs)
        buffer.advanced(by: index).initialize(to: NLHypothesisSetRaw(entries: entries, count: entryCount))
    }
    outArray.pointee = UnsafeMutableRawPointer(buffer)
    outCount.pointee = hypotheses.count
    return NL_OK
}

@_cdecl("nl_model_configuration_type")
public func nl_model_configuration_type(_ handle: UnsafeMutableRawPointer?) -> Int32 {
    guard let configuration = nlModelConfigurationObject(handle) else { return 0 }
    return configuration.type == .sequence ? 1 : 0
}

@_cdecl("nl_model_configuration_language")
public func nl_model_configuration_language(
    _ handle: UnsafeMutableRawPointer?,
    _ outLanguage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let configuration = nlModelConfigurationObject(handle) else {
        nlSetError(outError, "invalid NLModelConfiguration handle")
        return NL_INVALID_ARGUMENT
    }
    outLanguage.pointee = configuration.language.map { nlFfiString($0.rawValue) } ?? nil
    return NL_OK
}

@_cdecl("nl_model_configuration_revision")
public func nl_model_configuration_revision(_ handle: UnsafeMutableRawPointer?) -> Int {
    guard let configuration = nlModelConfigurationObject(handle) else { return 0 }
    return configuration.revision
}

@_cdecl("nl_model_supported_revisions_for_type")
public func nl_model_supported_revisions_for_type(
    _ modelType: Int32,
    _ outArray: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ outCount: UnsafeMutablePointer<Int>
) -> Int32 {
    let revisions = Array(NLModelConfiguration.supportedRevisions(for: nlModelType(modelType)))
    let (array, count) = nlUsizeArray(revisions)
    outArray.pointee = array
    outCount.pointee = count
    return NL_OK
}

@_cdecl("nl_model_current_revision_for_type")
public func nl_model_current_revision_for_type(_ modelType: Int32) -> Int {
    NLModelConfiguration.currentRevision(for: nlModelType(modelType))
}
