#![cfg(all(feature = "tag", feature = "language_detection"))]

use naturallanguage::recognizer::LanguageRecognizer;
use naturallanguage::tagger::Tagger;
use naturallanguage::tokenizer::Tokenizer;

trait AmbiguousIfSync<A> {
    fn assert_not_sync() {}
}

impl<T: ?Sized> AmbiguousIfSync<()> for T {}

impl<T: ?Sized + Sync> AmbiguousIfSync<u8> for T {}

trait AmbiguousIfClone<A> {
    fn assert_not_clone() {}
}

impl<T: ?Sized> AmbiguousIfClone<()> for T {}

impl<T: Clone> AmbiguousIfClone<u8> for T {}

const fn assert_send<T: Send>() {}

#[test]
fn single_thread_objects_are_send_but_not_sync() {
    assert_send::<Tagger>();
    assert_send::<Tokenizer>();
    assert_send::<LanguageRecognizer>();
    <Tagger as AmbiguousIfSync<_>>::assert_not_sync();
    <Tokenizer as AmbiguousIfSync<_>>::assert_not_sync();
    <LanguageRecognizer as AmbiguousIfSync<_>>::assert_not_sync();
}

#[test]
fn a_tokenizer_can_move_to_another_thread() {
    let text = "move me across";
    let mut tokenizer =
        Tokenizer::new(naturallanguage::tokenizer::TokenUnit::Word).expect("tokenizer");
    tokenizer.set_string(Some(text)).expect("set string");
    let full = naturallanguage::TextRange::new(0, text.encode_utf16().count());
    let tokens = std::thread::spawn(move || tokenizer.tokens_in_range(full).expect("tokens"))
        .join()
        .expect("tokenizer thread");
    assert_eq!(tokens.len(), 3);
}

#[cfg(feature = "contextual_embedding")]
#[test]
fn contextual_embeddings_are_not_shared_through_clone() {
    <naturallanguage::ContextualEmbedding as AmbiguousIfClone<_>>::assert_not_clone();
    assert_send::<naturallanguage::ContextualEmbedding>();
}
