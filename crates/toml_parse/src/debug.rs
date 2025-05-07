use crate::decode::Encoding;
use crate::ErrorSink;
use crate::Span;

pub(crate) struct DebugEventReceiver<'r> {
    receiver: &'r mut dyn crate::parser::EventReceiver,
    indent: usize,
}

impl<'r> DebugEventReceiver<'r> {
    pub(crate) fn new(receiver: &'r mut dyn crate::parser::EventReceiver) -> Self {
        Self {
            receiver,
            indent: 0,
        }
    }

    fn render_event(&mut self, span: Span, text: &str, style: anstyle::Style) {
        #![allow(unexpected_cfgs)] // HACK: fixed in newer versions
        let depth = self.indent;
        anstream::eprintln!("{:depth$}{style}{text}: {span:?}{style:#}", "");
    }
}

impl crate::parser::EventReceiver for DebugEventReceiver<'_> {
    fn std_table_open(&mut self, span: Span, error: &mut dyn ErrorSink) {
        self.receiver.std_table_open(span, error);
        self.render_event(span, "[", anstyle::Style::new() | anstyle::Effects::DIMMED);
        self.indent += 1;
    }
    fn std_table_close(&mut self, span: Span, error: &mut dyn ErrorSink) {
        self.receiver.std_table_close(span, error);
        self.indent -= 1;
        self.render_event(span, "]", anstyle::Style::new() | anstyle::Effects::DIMMED);
    }
    fn array_table_open(&mut self, span: Span, error: &mut dyn ErrorSink) {
        self.receiver.array_table_open(span, error);
        self.render_event(span, "[[", anstyle::Style::new() | anstyle::Effects::DIMMED);
        self.indent += 1;
    }
    fn array_table_close(&mut self, span: Span, error: &mut dyn ErrorSink) {
        self.receiver.array_table_close(span, error);
        self.indent -= 1;
        self.render_event(span, "]]", anstyle::Style::new() | anstyle::Effects::DIMMED);
    }
    fn inline_table_open(&mut self, span: Span, error: &mut dyn ErrorSink) -> bool {
        let allowed = self.receiver.inline_table_open(span, error);
        self.render_event(span, "{", anstyle::Style::new() | anstyle::Effects::DIMMED);
        self.indent += 1;
        allowed
    }
    fn inline_table_close(&mut self, span: Span, error: &mut dyn ErrorSink) {
        self.receiver.inline_table_close(span, error);
        self.indent -= 1;
        self.render_event(span, "}", anstyle::Style::new() | anstyle::Effects::DIMMED);
    }
    fn array_open(&mut self, span: Span, error: &mut dyn ErrorSink) -> bool {
        let allowed = self.receiver.array_open(span, error);
        self.render_event(span, "[", anstyle::Style::new() | anstyle::Effects::DIMMED);
        self.indent += 1;
        allowed
    }
    fn array_close(&mut self, span: Span, error: &mut dyn ErrorSink) {
        self.receiver.array_close(span, error);
        self.indent -= 1;
        self.render_event(span, "]", anstyle::Style::new() | anstyle::Effects::DIMMED);
    }
    fn simple_key(&mut self, span: Span, encoding: Option<Encoding>, error: &mut dyn ErrorSink) {
        self.receiver.simple_key(span, encoding, error);
        self.render_event(span, "<key>", anstyle::AnsiColor::Magenta.on_default());
    }
    fn key_sep(&mut self, span: Span, error: &mut dyn ErrorSink) {
        self.receiver.key_sep(span, error);
        self.render_event(span, ".", anstyle::Style::new() | anstyle::Effects::DIMMED);
    }
    fn key_val_sep(&mut self, span: Span, error: &mut dyn ErrorSink) {
        self.receiver.key_val_sep(span, error);
        self.render_event(span, "=", anstyle::Style::new() | anstyle::Effects::DIMMED);
    }
    fn scalar(&mut self, span: Span, encoding: Option<Encoding>, error: &mut dyn ErrorSink) {
        self.receiver.scalar(span, encoding, error);
        self.render_event(span, "<scalar>", anstyle::AnsiColor::Green.on_default());
    }
    fn value_sep(&mut self, span: Span, error: &mut dyn ErrorSink) {
        self.receiver.value_sep(span, error);
        self.render_event(span, ",", anstyle::Style::new() | anstyle::Effects::DIMMED);
    }
    fn whitespace(&mut self, span: Span, error: &mut dyn ErrorSink) {
        self.receiver.whitespace(span, error);
        self.render_event(span, "<whitespace>", anstyle::AnsiColor::Cyan.on_default());
    }
    fn comment(&mut self, span: Span, error: &mut dyn ErrorSink) {
        self.receiver.comment(span, error);
        self.render_event(span, "<comment>", anstyle::AnsiColor::Cyan.on_default());
    }
    fn newline(&mut self, span: Span, error: &mut dyn ErrorSink) {
        self.receiver.newline(span, error);
        self.render_event(span, "<newline>", anstyle::AnsiColor::Cyan.on_default());
    }
    fn error(&mut self, span: Span, error: &mut dyn ErrorSink) {
        self.receiver.error(span, error);
        self.render_event(span, "<error>", anstyle::AnsiColor::Red.on_default());
    }
}
