use std::{fmt, path::Path};

use super::{Thing, util::{SrcRegion, SrcLoc}};

#[derive(Clone, Debug, PartialEq)]
pub enum ErrorKind {
    Spurious, // Never revealed to user
    UnexpectedChar(char),
    UnknownOperator(String),
    UnexpectedEof,
    Expected(Thing),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Error {
    kind: ErrorKind,
    region: Option<SrcRegion>,
    while_parsing: Vec<Thing>,
    hint: Option<&'static str>,
}

impl Error {
    pub fn spurious() -> Self {
        Self::from(ErrorKind::Spurious)
    }

    pub fn unexpected_char(c: char) -> Self {
        Self::from(ErrorKind::UnexpectedChar(c))
    }

    pub fn unknown_operator(op: String) -> Self {
        Self::from(ErrorKind::UnknownOperator(op))
    }

    pub fn unexpected_eof() -> Self {
        Self::from(ErrorKind::UnexpectedEof)
    }

    pub fn expected(thing: impl Into<Thing>) -> Self {
        Self::from(ErrorKind::Expected(thing.into()))
    }

    pub fn at(mut self, region: impl Into<Option<SrcRegion>>) -> Self {
        self.region = region.into();
        self
    }

    pub fn while_parsing(mut self, thing: impl Into<Thing>) -> Self {
        self.while_parsing.push(thing.into());
        self
    }

    pub fn hint(mut self, hint: &'static str) -> Self {
        self.hint = Some(hint);
        self
    }

    pub fn max(self, other: Self) -> Self {
        match (self.region, other.region) {
            (Some(region_a), Some(region_b)) if region_a.later_than(region_b) => self,
            _ => other,
        }
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Self {
        Self {
            kind,
            region: None,
            while_parsing: Vec::new(),
            hint: None,
        }
    }
}

pub struct ErrorCtx<'a> {
    file: &'a Path,
    src: &'a str,
}

impl<'a> ErrorCtx<'a> {
    pub fn fmt_error(&self, error: &Error, output: &mut dyn fmt::Write) -> fmt::Result {
        match error.region {
            Some(SrcRegion::Range(start, stop)) => {
                let start_line = start.in_context(&self.src).0;
                let stop_line = start.in_context(&self.src).1;
                write!(output, "{}", &self.src[start_line..stop_line])?;
            }
            _ => {},
        }
        write!(output, "{:?}", error.kind)
    }
}

#[test]
fn unexpected_eof_error_formatting() {
    let mut formatted_error = String::new();

    let ctx = ErrorCtx {
        file: Path::new("unexpected_eof.ln"),
        src: "let x ~ y",
    };

    let error = crate::lex::lex(&ctx.src)
        .and_then(|tkns| {
            crate::parse::parse(&tkns.0)
        })
        .err()
        .expect("parsing succeeded and it shouldn't have")
        .pop()
        .expect("no errors were provided");

    println!("{:?}", error);
    
    ctx.fmt_error(&error, &mut formatted_error);
    assert_eq!(
        formatted_error,
        "uh".to_string(),
    );
}
