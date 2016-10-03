
use parse::*;

#[derive(Debug)]
pub enum Outcome {
	Success,
	Fail,
	Denied,
}

pub fn eval_prog(input: &[u8]) {

}

pub fn eval_cmd(user: &str, pass: &str, c: Cmd) -> Outcome {
	Outcome::Success
}

pub fn eval_record<'a>(a: &'a[u8], b: &'a[u8]) -> &'a [u8] {
	format!("{:?}.{:?}", a, b).into_bytes()
}