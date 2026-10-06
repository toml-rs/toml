use serde_spanned::Spanned;

use crate::alloc_prelude::*;
use crate::de::DeString;
use crate::de::parser::prelude::*;

/// ```abnf
/// key = simple-key / dotted-key
/// dotted-key = simple-key 1*( dot-sep simple-key )
/// ```
///
/// Append the path and return its span and the final key.
pub(crate) fn on_key<'i>(
    key_event: &toml_parser::parser::Event,
    input: &mut Input<'_>,
    source: toml_parser::Source<'i>,
    errors: &mut dyn ErrorSink,
    arena: &mut Vec<Spanned<DeString<'i>>>,
) -> (toml_parser::Span, Option<Spanned<DeString<'i>>>) {
    #[cfg(feature = "debug")]
    let _scope = TraceScope::new("key::on_key");
    let result_path_start = arena.len();
    let mut result_key = None;

    let mut state = State::new(key_event);
    if more_key(input) {
        while let Some(event) = input.next_token() {
            match event.kind() {
                EventKind::StdTableOpen
                | EventKind::ArrayTableOpen
                | EventKind::InlineTableOpen
                | EventKind::InlineTableClose
                | EventKind::ArrayOpen
                | EventKind::ArrayClose
                | EventKind::Scalar
                | EventKind::ValueSep
                | EventKind::Comment
                | EventKind::Newline
                | EventKind::KeyValSep
                | EventKind::StdTableClose
                | EventKind::ArrayTableClose
                | EventKind::Error => {
                    #[cfg(feature = "debug")]
                    trace(
                        &format!("unexpected {event:?}"),
                        anstyle::AnsiColor::Red.on_default(),
                    );
                    continue;
                }
                EventKind::SimpleKey => {
                    state.current_key = Some(*event);

                    if !more_key(input) {
                        break;
                    }
                }
                EventKind::Whitespace => {
                    state.whitespace(event);
                }
                EventKind::KeySep => {
                    state.close_key(arena, &mut result_key, source, errors);
                }
            }
        }
    }

    state.close_key(arena, &mut result_key, source, errors);
    let result_path_span = toml_parser::Span::new_unchecked(result_path_start, arena.len());

    #[cfg(not(feature = "unbounded"))]
    if super::LIMIT <= result_path_span.len() as u32 {
        errors.report_error(ParseError::new("recursion limit"));
        arena.truncate(result_path_start);
        return (result_path_span.before(), None);
    }

    (result_path_span, result_key)
}

fn more_key(input: &Input<'_>) -> bool {
    let first = input.get(0).map(|e| e.kind());
    let second = input.get(1).map(|e| e.kind());
    if first == Some(EventKind::KeySep) {
        true
    } else if first == Some(EventKind::Whitespace) && second == Some(EventKind::KeySep) {
        true
    } else {
        false
    }
}

struct State {
    current_key: Option<toml_parser::parser::Event>,
}

impl State {
    fn new(key_event: &toml_parser::parser::Event) -> Self {
        Self {
            current_key: Some(*key_event),
        }
    }

    fn whitespace(&mut self, _event: &toml_parser::parser::Event) {}

    fn close_key<'i>(
        &mut self,
        arena: &mut Vec<Spanned<DeString<'i>>>,
        result_key: &mut Option<Spanned<DeString<'i>>>,
        source: toml_parser::Source<'i>,
        errors: &mut dyn ErrorSink,
    ) {
        let Some(key) = self.current_key.take() else {
            return;
        };

        let key_span = key.span();
        let key_span = key_span.start()..key_span.end();

        let raw = source.get(key).unwrap();
        let mut decoded = alloc::borrow::Cow::Borrowed("");
        raw.decode_key(&mut decoded, errors);

        let key = Spanned::new(key_span, decoded);
        if let Some(last_key) = result_key.replace(key) {
            arena.push(last_key);
        }
    }
}
