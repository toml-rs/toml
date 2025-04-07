use winnow::stream::Stream as _;

use super::EventReceiver;
use crate::lexer::Token;
use crate::ErrorSink;

/// Parse lexed tokens into [`Event`][super::Event]s
pub fn parse_document(
    mut tokens: &[Token],
    receiver: &mut dyn EventReceiver,
    error: &mut dyn ErrorSink,
) {
    #[cfg(feature = "unstable-debug")]
    let mut receiver = super::DebugEventReceiver::new(receiver);
    #[cfg(feature = "unstable-debug")]
    let receiver = &mut receiver;
    document(&mut tokens, receiver, error);
}

/// Parse lexed tokens into [`Event`][super::Event]s
pub fn parse_key(
    mut tokens: &[Token],
    receiver: &mut dyn EventReceiver,
    error: &mut dyn ErrorSink,
) {
    #[cfg(feature = "unstable-debug")]
    let mut receiver = super::DebugEventReceiver::new(receiver);
    #[cfg(feature = "unstable-debug")]
    let receiver = &mut receiver;
    key(&mut tokens, receiver, error);
}

/// Parse lexed tokens into [`Event`][super::Event]s
pub fn parse_simple_key(
    mut tokens: &[Token],
    receiver: &mut dyn EventReceiver,
    error: &mut dyn ErrorSink,
) {
    #[cfg(feature = "unstable-debug")]
    let mut receiver = super::DebugEventReceiver::new(receiver);
    #[cfg(feature = "unstable-debug")]
    let receiver = &mut receiver;
    simple_key(&mut tokens, receiver, error);
}

/// Parse lexed tokens into [`Event`][super::Event]s
pub fn parse_value(
    mut tokens: &[Token],
    receiver: &mut dyn EventReceiver,
    error: &mut dyn ErrorSink,
) {
    #[cfg(feature = "unstable-debug")]
    let mut receiver = super::DebugEventReceiver::new(receiver);
    #[cfg(feature = "unstable-debug")]
    let receiver = &mut receiver;
    value(&mut tokens, receiver, error);
}

/// Parse a TOML Document
///
/// ```bnf
/// toml = expression *( newline expression )
///
/// expression =  ws [ comment ]
/// expression =/ ws keyval ws [ comment ]
/// expression =/ ws table ws [ comment ]
///
/// ;; Key-Value pairs
///
/// keyval = key keyval-sep val
///
/// key = simple-key / dotted-key
/// simple-key = quoted-key / unquoted-key
///
/// quoted-key = basic-string / literal-string
/// dotted-key = simple-key 1*( dot-sep simple-key )
///
/// dot-sep   = ws %x2E ws  ; . Period
/// keyval-sep = ws %x3D ws ; =
///
/// val = string / boolean / array / inline-table / date-time / float / integer
///
/// ;; Array
///
/// array = array-open [ array-values ] ws-comment-newline array-close
///
/// array-open =  %x5B ; [
/// array-close = %x5D ; ]
///
/// array-values =  ws-comment-newline val ws-comment-newline array-sep array-values
/// array-values =/ ws-comment-newline val ws-comment-newline [ array-sep ]
///
/// array-sep = %x2C  ; , Comma
///
/// ;; Table
///
/// table = std-table / array-table
///
/// ;; Standard Table
///
/// std-table = std-table-open key std-table-close
///
/// ;; Inline Table
///
/// inline-table = inline-table-open [ inline-table-keyvals ] inline-table-close
///
/// inline-table-keyvals = keyval [ inline-table-sep inline-table-keyvals ]
///
/// ;; Array Table
///
/// array-table = array-table-open key array-table-close
/// ```
fn document(tokens: &mut &[Token], receiver: &mut dyn EventReceiver, error: &mut dyn ErrorSink) {
    while let Some(token) = tokens.next_token() {
        receiver.error(token.span(), error);
        error.report_error(token.to_error(&[]));
    }
}

/// Parse a TOML key
///
/// ```bnf
/// ;; Key-Value pairs
///
/// key = simple-key / dotted-key
/// simple-key = quoted-key / unquoted-key
///
/// quoted-key = basic-string / literal-string
/// dotted-key = simple-key 1*( dot-sep simple-key )
///
/// dot-sep   = ws %x2E ws  ; . Period
/// ```
fn key(tokens: &mut &[Token], receiver: &mut dyn EventReceiver, error: &mut dyn ErrorSink) {
    while let Some(token) = tokens.next_token() {
        receiver.error(token.span(), error);
        error.report_error(token.to_error(&[]));
    }
}

/// Parse a TOML simple key
///
/// ```bnf
/// ;; Key-Value pairs
///
/// simple-key = quoted-key / unquoted-key
///
/// quoted-key = basic-string / literal-string
/// ```
fn simple_key(tokens: &mut &[Token], receiver: &mut dyn EventReceiver, error: &mut dyn ErrorSink) {
    while let Some(token) = tokens.next_token() {
        receiver.error(token.span(), error);
        error.report_error(token.to_error(&[]));
    }
}

/// Parse a TOML Document
///
/// ```bnf
/// ;; Key-Value pairs
///
/// val = string / boolean / array / inline-table / date-time / float / integer
///
/// ;; Array
///
/// array = array-open [ array-values ] ws-comment-newline array-close
///
/// array-open =  %x5B ; [
/// array-close = %x5D ; ]
///
/// array-values =  ws-comment-newline val ws-comment-newline array-sep array-values
/// array-values =/ ws-comment-newline val ws-comment-newline [ array-sep ]
///
/// array-sep = %x2C  ; , Comma
///
/// ;; Inline Table
///
/// inline-table = inline-table-open [ inline-table-keyvals ] inline-table-close
///
/// inline-table-keyvals = keyval [ inline-table-sep inline-table-keyvals ]
/// ```
fn value(tokens: &mut &[Token], receiver: &mut dyn EventReceiver, error: &mut dyn ErrorSink) {
    while let Some(token) = tokens.next_token() {
        receiver.error(token.span(), error);
        error.report_error(token.to_error(&[]));
    }
}
