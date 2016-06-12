use nom::*;
use std::str;
use std::str::FromStr;
use std::ascii::AsciiExt;

use types::*;


named!(i64_digit<i64>,
       map_res!(
           map_res!(
               digit,
               str::from_utf8
           ),
           FromStr::from_str
       )
);

fn parse_bulk(input: &[u8]) -> IResult<&[u8], Reply> {
    let (i1, digit) = try_parse!(
        input,
        chain!(
            char!(b'$') ~
            d: i64_digit ~
            tag_s!(b"\r\n"),
            || { d }
        )
    );

    if digit < 0 {
        return IResult::Done(input, Reply::Bulk(None));
    } else {
        let (i2, command) =
            try_parse!(i1,
                chain!(
                    s: map_res!(take!(digit), str::from_utf8) ~
                    tag_s!(b"\r\n"),
                    || { s }
                )
            );
        let command_str = String::from(command.to_ascii_lowercase());
        return IResult::Done(i2, Reply::Bulk(Some(command_str)));
    }
}

fn parse_multi_bulk(input: &[u8]) -> IResult<&[u8], Reply> {
    let (i1, digit) = try_parse!(
        input,
        chain!(
            char!(b'*') ~
            d: i64_digit ~
            tag_s!(b"\r\n"),
            || { d }
        )
    );

    if digit < 0 {
        return IResult::Done(input, Reply::MultiBulk(None));
    } else {
        let mut count = 1;
        let mut im = i1;
        let mut commands = vec![];
        while count <= digit {
            let (i2, command) = try_parse!(im, parse_bulk);
            im = i2;
            count = count + 1;
            commands.push(command);
        }
        return IResult::Done(im, Reply::MultiBulk(Some(commands)));
    }
}

pub struct Parser;

impl Parser {

    pub fn parse(bytes: &[u8]) -> Option<Command> {
        Self::parse_reply(Self::parse_bytes(bytes))
    }

    fn parse_bytes(bytes: &[u8]) -> Reply {
        match parse_bulk(bytes) {
            IResult::Done(_, result) => result,
            _ => match parse_multi_bulk(bytes) {
                IResult::Done(_, result) => result,
                _ => Reply::MultiBulk(None)
            }
        }
    }

    fn parse_reply_get(xs: Vec<Reply>) -> Option<Command> {
        if xs[0] == Reply::Bulk(Some("get".to_string())) {
            match xs[1] {
                Reply::Bulk(Some(ref a)) => Some(Command::Get(a.clone())),
                _ => Some(Command::Unknown)
            }
        } else {
            return Some(Command::Unknown);
        }
    }

    fn parse_reply_set(xs: Vec<Reply>) -> Option<Command> {
        if (xs[0]) == Reply::Bulk(Some("set".to_string())) {
            match (xs[1].clone(), xs[2].clone()) {
                (Reply::Bulk(Some(ref a)), Reply::Bulk(Some(ref b))) =>
                    Some(Command::Set(a.clone(), b.clone())),
                (_,_) => Some(Command::Unknown)
            }
        } else {
            return Some(Command::Unknown);
        }
    }

    fn parse_reply(reply: Reply) -> Option<Command> {
        match reply {
            Reply::MultiBulk(Some(xs)) => {
                match xs.len() {
                    2 => {
                        let get = Self::parse_reply_get(xs);
                        return get;
                    },
                    3 => {
                        let set = Self::parse_reply_set(xs);
                        return set;
                    },
                    _ => Some(Command::Unknown),
                }
            },
            _ => None,
        }
    }

}
