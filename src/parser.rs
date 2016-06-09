use nom::*;

use types::*;

// macros + parser combinators definitions
named!(parens, delimited!(char!('('), is_not!(")"), char!(')')));
named!(bulk, delimited!(char!('$'), digit, line_ending));
named!(multi_bulk, delimited!(char!('*'), digit, line_ending));
named!(parse_reply_bytes, alt!(bulk | multi_bulk));

// "*2\r\n$3\r\nget\r\n$4\r\nname\r\n"

pub struct Parser;

impl Parser {

    pub fn parse(bytes: &[u8]) -> Option<Command> {
        Self::parse_reply(Self::parse_bytes(bytes))
    }

    fn parse_bytes(bytes: &[u8]) -> Reply {
        // todo
        parens(bytes);
        parse_reply_bytes(bytes);
        //
        Reply::Bulk(None);
        return Reply::MultiBulk(None);
    }

    fn parse_reply(reply: Reply) -> Option<Command> {
        match reply {
            Reply::MultiBulk(Some(xs)) => {
                match &xs[..] {
                    [Reply::Bulk(Some("get")), Reply::Bulk(Some(a))]
                        => Some(Command::Get(a)),
                    [Reply::Bulk(Some("set")), Reply::Bulk(Some(a)), Reply::Bulk(Some(b))]
                        => Some(Command::Set(a, b)),
                    _
                        => Some(Command::Unknown),
                }
            },
            _ => None,
        }
    }

}
