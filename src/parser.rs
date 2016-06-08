use types::*;

pub struct Parser;

impl Parser {

    pub fn parse(bytes: &[u8]) -> Option<Command> {
        Self::parse_reply(Self::parse_bytes(bytes))
    }

    fn parse_bytes(bytes: &[u8]) -> Reply {
        // todo
        Reply::Bulk(None)
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
