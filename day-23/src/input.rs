use std::{io::BufRead, str::FromStr};

use anyhow::{Context, Result};
use itertools::Itertools;

use crate::{Nanobot, Point};

pub fn parse<R: BufRead>(input: R) -> Result<Vec<Nanobot>> {
    input.lines().map(|l| l?.parse()).try_collect()
}

impl FromStr for Nanobot {
    type Err = anyhow::Error;

    /// pos=<26057576,-10751309,46491633>, r=91461401
    fn from_str(s: &str) -> Result<Self> {
        let (pos, r) = s.split_once(", ").context("expected ', '")?;

        let (x, y, z) = pos
            .strip_prefix("pos=<")
            .context("expected pos=<")?
            .strip_suffix(">")
            .context("expected >")?
            .split(',')
            .map(str::parse)
            .collect_tuple()
            .context("expected 3-tuple")?;
        let (x, y, z) = (x?, y?, z?);
        let pos = Point { x, y, z };

        let range = r.strip_prefix("r=").context("expected r=")?.parse()?;

        Ok(Self { pos, range })
    }
}
