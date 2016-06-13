use nom::*;
use std::str;
use std::str::FromStr;
use std::ascii::AsciiExt;

use types::*;


named!(i64_digit<i64>,
       map_res!(
           map_res!(digit, str::from_utf8),
           FromStr::from_str
       )
);

fn parse_bulk(input: &[u8]) -> IResult<&[u8], Reply> {
    let (i1, command) = try_parse!(
        input,
        chain!(
            char!(b'$') ~
            d: i64_digit ~
            tag_s!(b"\r\n") ~
            s: map_res!(take!(d), str::from_utf8) ~
            tag_s!(b"\r\n"),
            || { s }
        )
    );

    let command_str = String::from(command.to_ascii_lowercase());
    return IResult::Done(i1, Reply::Bulk(Some(command_str)));
}

fn parse_multi_bulk(input: &[u8]) -> IResult<&[u8], Reply> {
    let (i1, commands) = try_parse!(
        input,
        chain!(
            char!(b'*') ~
            d: i64_digit ~
            tag_s!(b"\r\n") ~
            c: count!(parse_bulk, d as usize),
            || { c }
        )
    );

    return IResult::Done(i1, Reply::MultiBulk(Some(commands)));
}

named!(parse_bytes_nom<Reply>, alt!(
      parse_bulk       => { |res: Reply| res }
    | parse_multi_bulk => { |res: Reply| res }
));

pub struct Parser;

impl Parser {

    pub fn parse(bytes: &[u8]) -> Option<Command> {
        Self::parse_reply(Self::parse_bytes(bytes))
    }

    fn parse_bytes(bytes: &[u8]) -> Reply {
        match parse_bytes_nom(bytes) {
            IResult::Done(_, result) => result,
            _                        => Reply::MultiBulk(None)
        }
    }

    fn parse_reply(reply: Reply) -> Option<Command> {
        match reply {
            Reply::MultiBulk(Some(xs)) => {
                match &xs[..] {
                    [Reply::Bulk(Some(ref get)), Reply::Bulk(Some(ref a))] if get == "get"
                        => Some(Command::Get(a.clone())),
                    [Reply::Bulk(Some(ref set)), Reply::Bulk(Some(ref a)), Reply::Bulk(Some(ref b))] if set == "set"
                        => Some(Command::Set(a.clone(), b.clone())),
                    _   => Some(Command::Unknown),
                }
            },
            _ => None,
        }
    }

}
