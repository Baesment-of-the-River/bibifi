
use eval::*;

use nom::*;
use nom::IResult::*; /* Error, Incomplete, Done */

use std::str;
use std::str::FromStr;

const IDENT_PREFIX: &'static [u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
const IDENT_CHARS: 	&'static [u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_";
const STRING_CHARS: &'static [u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_,;\\.?!-";

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

#[derive(Debug)]
pub enum Cmd<'a> {
	Exit,
	Return(&'a [u8]),
}

#[derive(Debug)]
pub enum Value<'a> {
	Ident(&'a [u8]),
	Test(&'a [u8]),
}

#[derive(Debug)]
pub enum Right {
	Read, Write, Append, Delegate,
}

macro_rules! utf8 {
    ($a:expr) => (str::from_utf8($a).unwrap())
}

named!(cmd_identifier<&[u8], &[u8]>,
	recognize!(
		chain!(
			is_a!(IDENT_PREFIX) ~
			is_a!(IDENT_CHARS)?,
			|| {}
		)
	)
);

named!(cmd_string<&[u8], Option<&[u8]> >,
	opt!(recognize!(is_a!(STRING_CHARS)))
);

/* <prog> ::= as principal p password s do \n <cmd> */
named!(pub prog_parse<&[u8], Outcome>,
	chain!(
		space?				~
		tag!("as")			~
		space				~
		tag!("principle") 	~
		space				~
		p: cmd_identifier	~
		space				~
		tag!("password") 	~
		space				~
		s: cmd_string		~
		space				~
		tag!("do")			~
		space?				~
		line_ending 		~
		space?				~
		c: cmd_parse,

		|| { eval_cmd(utf8!(p), utf8!(s.unwrap()), c) }
	)
);

/* <cmd> ::= exit \n | return <expr> \n | <prim_cmd> \n <cmd> */
named!(cmd_parse<&[u8], Cmd>,
	alt_complete!(
		chain!(
			tag!("exit")	~
			space? 			~
			line_ending,
			|| { Cmd::Exit }
		)
		|
		chain!(
			tag!("return")	~
			space			~
			e: parse_expr	~
			space?			~
			line_ending,
			|| { Cmd::Return(e) }
		)
		|
		chain!(
			p: parse_prim	~
			space?			~
			line_ending		~
			space?			~
			c: cmd_parse,
			|| { c }
		)
	)
);

named!(parse_expr<&[u8], &[u8]>,
	tag!("something")
);

named!(parse_prim<&[u8], &[u8]>,
	tag!("anything")
);

/* <value> ::= x | x . y | s */
named!(parse_value<&[u8], Value>,
	alt_complete!(
		cmd_identifier
		|
		cmd_string
		|
		chain!(
			a: cmd_identifier	~
			space?				~
			char!(".")			~
			space?				~
			b: cmd_identifier	~
			space?,
			|| { eval_record(a, b) }
		)
	)
);