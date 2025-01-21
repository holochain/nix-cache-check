use nom::{
    bytes::complete::{take, take_till, take_until},
    character::{
        complete::{newline, space1},
        is_newline,
    },
    combinator::opt,
    error::{context, convert_error, ContextError},
    multi::fold_many1,
    sequence::{preceded, tuple},
    Finish, IResult,
};

#[derive(Debug)]
pub struct CacheInfo {
    derivations_to_build: Vec<String>,
    derivations_to_fetch: Vec<String>,
}

impl CacheInfo {
    pub fn get_derivations_to_build(&self) -> &[String] {
        &self.derivations_to_build
    }

    pub fn get_derivations_to_fetch(&self) -> &[String] {
        &self.derivations_to_fetch
    }
}

pub fn parse_log(log: &str) -> anyhow::Result<CacheInfo> {
    let (_, info) = match do_parse(log).finish() {
        Ok(r) => r,
        Err(e) => {
            return Err(anyhow::anyhow!(
                "Failed to parse: {}",
                convert_error(log, e)
            ));
        }
    };

    Ok(info)
}

fn do_parse<'a, E>(input: &'a str) -> IResult<&'a str, CacheInfo, E>
where
    E: nom::error::ParseError<&'a str> + ContextError<&'a str>,
{
    match tuple((
        opt(preceded(
            context(
                "start-of-built-derivations",
                tuple((
                    take_until("these"),
                    take(5usize),
                    take_until("built:"),
                    take(6usize),
                    newline,
                )),
            ),
            fold_many1(
                context("build-derivation-line", parse_line::<E>),
                Vec::new,
                |mut acc, line| {
                    acc.push(line);
                    acc
                },
            ),
        )),
        opt(preceded(
            context(
                "start-of-fetched-derivations",
                tuple((
                    take_until("these "),
                    take(6usize),
                    take_until("fetched"),
                    take(7usize),
                    take_until(":"),
                    take(1usize),
                    newline,
                )),
            ),
            fold_many1(
                context("fetch-derivation-line", parse_line::<E>),
                Vec::new,
                |mut acc, line| {
                    acc.push(line);
                    acc
                },
            ),
        )),
    ))(input)
    {
        Ok((rest, (to_build, to_fetch))) => Ok((
            rest,
            CacheInfo {
                derivations_to_build: to_build.unwrap_or_else(Vec::new),
                derivations_to_fetch: to_fetch.unwrap_or_else(Vec::new),
            },
        )),
        Err(e) => Err(e),
    }
}

fn parse_line<'a, E>(input: &'a str) -> IResult<&'a str, String, E>
where
    E: nom::error::ParseError<&'a str>,
{
    match tuple((space1, take_till(|c| is_newline(c as u8)), newline))(input) {
        Ok((rest, (_sp, derivation, _newline))) => Ok((rest, derivation.to_string())),
        Err(e) => Err(e),
    }
}
