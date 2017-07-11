use ::value;
use nom::{self, IResult};
use parser::errors::{ErrorKind, is_custom};
use chrono::{self, FixedOffset};
use parser::Span;

// ;; Date and Time (as defined in RFC 3339)

// date-time      = offset-date-time / local-date-time / local-date / local-time
named!(pub date_time(Span) -> value::DateTime,
       alt_custom!(e_kind!(nom::ErrorKind::Alt),
           offset_date_time
         | local_date_time
         | local_date
         | local_time
       )
);

fn digits(input: Span, n: u8) -> IResult<Span, Span> {
    verify!(input, complete!(take!(n)),
            |s: Span| s.fragment.bytes().all(nom::is_digit)
    )
}

// date-fullyear  = 4DIGIT
named!(date_fullyear(Span) -> Span,
       call!(digits, 4)
);
// date-month     = 2DIGIT  ; 01-12
named!(date_month(Span) -> Span,
       err!(ErrorKind::InvalidDateTime,
            call!(digits, 2))
);
// date-mday      = 2DIGIT  ; 01-28, 01-29, 01-30, 01-31 based on month/year
named!(date_mday(Span) -> Span,
       err!(ErrorKind::InvalidDateTime,
            call!(digits, 2))
);
// time-hour      = 2DIGIT  ; 00-23
named!(time_hour(Span) -> Span,
       call!(digits, 2)
);
// time-minute    = 2DIGIT  ; 00-59
named!(time_minute(Span) -> Span,
       err!(ErrorKind::InvalidDateTime,
            call!(digits, 2))
);
// time-second    = 2DIGIT  ; 00-58, 00-59, 00-60 based on leap second rules
named!(time_second(Span) -> Span,
       err!(ErrorKind::InvalidDateTime,
            call!(digits, 2))
);

// time-numoffset = ( "+" / "-" ) time-hour ":" time-minute
named!(#[inline], time_numoffset(Span) -> (),
       do_parse!(
           complete!(one_of!("+-")) >>
           err!(ErrorKind::InvalidDateTime,
                call!(time_hour)) >>
               err!(ErrorKind::InvalidDateTime,
                    complete!(tag!(":"))) >>
               time_minute >>
               ()
       )
);
// time-offset    = "Z" / time-numoffset
named!(#[inline], time_offset(Span) -> (),
       alt_complete!(
           tag!("Z") => { |_| () }
         | time_numoffset
       )
);

// time-secfrac   = "." 1*DIGIT
// partial-time   = time-hour ":" time-minute ":" time-second [time-secfrac]
named!(#[inline], partial_time(Span) -> (),
       do_parse!(
           time_hour >>
           complete!(tag!(":")) >>
           err!(ErrorKind::InvalidDateTime,
                tuple!(
                    time_minute,
                    tag!(":"),
                    time_second,
                    opt!(complete!(tuple!(tag!("."), call!(nom::digit))))
                )
           ) >>
           ()
       )
);

// full-date      = date-fullyear "-" date-month "-" date-mday
named!(#[inline], full_date(Span) -> (),
       do_parse!(
           date_fullyear >>
           complete!(tag!("-")) >>
           date_month >>
           err!(ErrorKind::InvalidDateTime,
                complete!(tag!("-"))) >>
           date_mday >>
           ()
       )
);

// full-time      = partial-time time-offset
named!(#[inline], full_time(Span) -> (),
       do_parse!(
           partial_time >>
           time_offset >>
           ()
       )
);

macro_rules! date_time (
    ($input:expr, $parse_dt:ty, $value_dt:path, $submac:ident!( $($args:tt)*)) => (
        do_parse!(
            $input,
         s: recognize!($submac!($($args)*)) >>
         d: err!(ErrorKind::InvalidDateTime,
                 map!(expr_res!(s.fragment.parse::<$parse_dt>()),
                      $value_dt)
            ) >>
            (d)
        )
    );
    ($input:expr, $parse_dt:ty, $value_dt:path, $f:expr) => (
        date_time!($input, $parse_dt, $value_dt, call!($f));
    );
);

// ;; Offset Date-Time

// offset-date-time = full-date "T" full-time
named!(pub offset_date_time(Span) -> value::DateTime,
       date_time!(
           chrono::DateTime<FixedOffset>,
           value::DateTime::OffsetDateTime,
           tuple!(
               full_date,
               complete!(tag!("T")),
               full_time
           )
       )
);

// ;; Local Date-Time

// local-date-time = full-date "T" partial-time
named!(pub local_date_time(Span) -> value::DateTime,
       date_time!(
           chrono::NaiveDateTime,
           value::DateTime::LocalDateTime,
           tuple!(
               full_date,
               complete!(tag!("T")),
               err!(ErrorKind::InvalidDateTime,
                    call!(partial_time))
           )
       )
);


// ;; Local Date

// local-date = full-date
named!(pub local_date(Span) -> value::DateTime,
       date_time!(
           chrono::NaiveDate,
           value::DateTime::LocalDate,
           full_date
       )
);

// ;; Local Time

// local-time = partial-time
named!(pub local_time(Span) -> value::DateTime,
       date_time!(
           chrono::NaiveTime,
           value::DateTime::LocalTime,
           partial_time
       )
);
