use std::mem;
use nom::{self, IResult, InputLength, Slice};
use parser::{Span, Parser};
use parser::errors::{ErrorKind, is_custom};
use parser::trivia::{line_trailing, ws};
use parser::key::key;
use parser::LenWorkaround;
use ::decor::{InternalString, Repr};
use ::document::Document;
use ::table::{Table, TableEntry, Header, HeaderKind};


// table-key-sep   = ws %x2E ws  ; . Period
const TABLE_KEY_SEP: &'static str = ".";

// note: this rule is not present in the original grammar
// key-path = key *( table-key-sep key)
named!(key_path(Span) -> (Vec<InternalString>, Span),
       with_input!(
           err!(ErrorKind::InvalidHeader,
                separated_nonempty_list_complete!(
                    tag!(TABLE_KEY_SEP),
                    map!(ws!(key), |(key, _)| key)
                )
           )
       )
);

impl Default for Parser {
    fn default() -> Self {
        let mut doc = Document::new();
        let root = doc.root_mut() as *mut Table;
        Self {
            document: doc,
            current_table: root,
        }
    }
}

impl Parser {
    // ;; Table

    // table = std-table / array-table
    pub fn table(self, input: Span) -> (Self, IResult<Span, ()>) {
        let parser = self;
        let (parser, result) = parser.array_table(input);
        match result {
            IResult::Done(..) => return (parser, result),
            IResult::Error(e) => {
                if is_custom(&e) {
                    return (parser, IResult::Error(e));
                }
            }
            _ => unreachable!("array_table is complete"),
        }
        parser.std_table(input)
    }

    // ;; Standard Table

    // std-table-open  = %x5B ws     ; [ Left square bracket
    const STD_TABLE_OPEN: &'static str = "[";
    // std-table-close = ws %x5D     ; ] Right square bracket
    const STD_TABLE_CLOSE: &'static str = "]";

    // std-table = std-table-open key *( table-key-sep key) std-table-close
    method!(std_table<Parser>(Span) -> (), mut self,
        do_parse!(
            w: ws >>
            h: delimited!(
                   complete!(tag!(Parser::STD_TABLE_OPEN)),
                   key_path,
                   err_m!(self, ErrorKind::InvalidHeader,
                          complete!(tag!(Parser::STD_TABLE_CLOSE)))) >>
            t: line_trailing >>
               err_parser!(self, Self::on_std_header,
                           call!(w.fragment, h, t.fragment)) >>
               ()
        )
    );

    // ;; Array Table

    // array-table-open  = %x5B.5B ws  ; [[ Double left square bracket
    const ARRAY_TABLE_OPEN: &'static str = "[[";
    // array-table-close = ws %x5D.5D  ; ]] Double right quare bracket
    const ARRAY_TABLE_CLOSE: &'static str = "]]";

    // array-table = array-table-open key *( table-key-sep key) array-table-close
    method!(array_table<Parser>(Span) -> (), mut self,
            do_parse!(
             w: ws >>
             h: delimited!(
                    complete!(tag!(Parser::ARRAY_TABLE_OPEN)),
                    key_path,
                    err_m!(self, ErrorKind::InvalidHeader,
                           complete!(tag!(Parser::ARRAY_TABLE_CLOSE)))
                ) >>
             t: line_trailing >>
                err_parser!(self, Self::on_array_header,
                            call!(w.fragment, h, t.fragment)) >>
                ()
            )
    );

    fn descend_path<'a>(table: &'a mut Table, path: &[InternalString]) -> Option<&'a mut Table> {
        if let Some(key) = path.get(0) {
            let header = table.child_header(key, HeaderKind::Implicit);
            match table.append_table_with_header(key, header) {
                TableEntry::Value(..) => None,
                TableEntry::Array(array) => {
                    debug_assert!(!array.is_empty());

                    let i = array.len() - 1;
                    let last_child = array.get_mut(i).unwrap();

                    Self::descend_path(last_child, &path[1..])
                }
                TableEntry::Table(sweet_child_of_mine) => {
                    Parser::descend_path(sweet_child_of_mine, &path[1..])
                }
                _ => unreachable!("`insert_table` can't return a vacant entry"),
            }
        } else {
            Some(table)
        }
    }

    fn on_std_header<'a>(
        rest: Span<'a>,
        me: &mut Parser,
        whitespaces: &str,
        header: (Vec<InternalString>, Span<'a>),
        trailing: &str
    ) -> IResult<Span<'a>, ()>
    {
        let (header, span) = header;
        debug_assert!(!header.is_empty());

        let mut leading = mem::replace(&mut me.document.trailing, InternalString::new());
        leading.push_str(whitespaces);
        let mut table = me.document.root_mut();

        let table = Self::descend_path(table, &header[..header.len() - 1]);
        let key = &header[header.len() - 1];

        if let Some(table) = table {
            let header = Header {
                repr: Repr::new(leading, span.fragment.into(), trailing.into()),
                kind: HeaderKind::Standard,
            };

            match table.entry(key) {
                // if [a.b.c] header preceded [a.b]
                TableEntry::Table(ref mut t) if t.header.kind == HeaderKind::Implicit => {
                    t.move_to_end();
                    t.header = header;
                    me.current_table = *t;

                    return IResult::Done(rest, ());
                }
                TableEntry::Vacant(ref mut parent) => {
                    let mut table = parent.append_table_with_header(key, header);
                    me.current_table = table.as_table_mut().unwrap();

                    return IResult::Done(rest, ());
                }
                _ => {}
            }
        }
        e!(ErrorKind::DuplicateKey, span)
    }

    fn on_array_header<'a>(
        rest: Span<'a>,
        me: &mut Parser,
        whitespaces: &str,
        header: (Vec<InternalString>, Span<'a>),
        trailing: &str
    ) -> IResult<Span<'a>, ()>
    {
        let (header, span) = header;
        debug_assert!(!header.is_empty());

        let mut leading = mem::replace(&mut me.document.trailing, InternalString::new());
        leading.push_str(whitespaces);

        let mut table = me.document.root_mut();
        let key = &header[header.len() - 1];
        let table = Self::descend_path(table, &header[..header.len() - 1]);

        if let Some(table) = table {
            if !table.contains_table(key) && !table.contains_value(key) {
                let header = Header {
                    repr: Repr::new(leading, span.fragment.into(), trailing.into()),
                    kind: HeaderKind::Array,
                };

                let array = table.insert_array_assume_vacant(key);
                me.current_table = array.append_with_header(header, me.current_table);

                return IResult::Done(rest, ());
            }
        }
        e!(ErrorKind::DuplicateKey, span)
    }
}
