use nom::{
    bytes::complete::{take, take_till, take_until},
    character::{
        complete::{newline, space1},
        is_newline,
    },
    multi::fold_many1,
    sequence::tuple,
    IResult,
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

pub fn parse_log(log: &[u8]) -> anyhow::Result<CacheInfo> {
    let (_, info) = match do_parse(log) {
        Ok(r) => r,
        Err(e) => {
            return Err(anyhow::anyhow!(
                "Failed to parse {:?}",
                e.map_input(|i| String::from_utf8(i.to_vec()))
            ));
        }
    };

    Ok(info)
}

fn do_parse(input: &[u8]) -> IResult<&[u8], CacheInfo> {
    match tuple((
        take_until("these"),
        take(5usize),
        take_until("built:"),
        take(6usize),
        newline,
        fold_many1(
            parse_line,
            || vec![],
            |mut acc, line| {
                acc.push(line);
                acc
            },
        ),
        take_until("these"),
        take(5usize),
        take_until("fetched"),
        take(7usize),
        take_until(":"),
        take(1usize),
        newline,
        fold_many1(
            parse_line,
            || vec![],
            |mut acc, line| {
                acc.push(line);
                acc
            },
        ),
    ))(input)
    {
        Ok((
            rest,
            (
                _,
                _these,
                _,
                _built,
                _newline,
                to_build,
                _,
                _these2,
                _,
                _fetched,
                _,
                _colon,
                _newline2,
                to_fetch,
            ),
        )) => Ok((
            rest,
            CacheInfo {
                derivations_to_build: to_build,
                derivations_to_fetch: to_fetch,
            },
        )),
        Err(e) => Err(e),
    }
}

fn parse_line(input: &[u8]) -> IResult<&[u8], String> {
    match tuple((space1, take_till(is_newline), newline))(input) {
        Ok((rest, (_sp, derivation, _newline))) => {
            Ok((rest, String::from_utf8(derivation.to_vec()).unwrap()))
        }
        Err(e) => Err(e),
    }
}
