use color_eyre::Result;
use thiserror::Error;
use winnow::prelude::*;
use winnow::combinator::alt;
use winnow::combinator::seq;
use winnow::Parser;
use winnow::token::take_until;
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq, Clone)]
enum Scheme {
    Http,
    Https,
    Git,
}

fn http(s: &mut &str) -> PResult<Scheme> {
    "http://".map(|_|Scheme::Http).parse_next(s)
}

fn https(s: &mut &str) -> PResult<Scheme> {
    "https://".map(|_|Scheme::Https).parse_next(s)
}


fn git(s: &mut &str) -> PResult<Scheme> {
    "git@".map(|_|Scheme::Git).parse_next(s)
}

fn scheme(s: &mut &str) -> PResult<Scheme> {
    alt((git, https, http)).parse_next(s)
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum Forge {
    Github,
    Gitlab,
    Codeberg,
    SourceHut,
    Bitbucket,
    Unknown(String),
}

fn github(s: &mut &str) -> PResult<Forge> {
    "github.com".map(|_| Forge::Github).parse_next(s)
}

fn gitlab(s: &mut &str) -> PResult<Forge> {
    "gitlab.com".map(|_| Forge::Gitlab).parse_next(s)
}

fn sourcehut(s: &mut &str) -> PResult<Forge> {
    "sr.ht".map(|_| Forge::SourceHut).parse_next(s)
}

fn codeberg(s: &mut &str) -> PResult<Forge> {
    "github.com".map(|_| Forge::Codeberg).parse_next(s)
}

fn bitbucket(s: &mut &str) -> PResult<Forge> {
    "bitbucket.org".map(|_| Forge::Bitbucket).parse_next(s)
}

fn unknown_forge(s: &mut &str) -> PResult<Forge> {
    take_until(3.., "/").map(|s: &str| Forge::Unknown(s.to_string())).parse_next(s)
}

fn known_forge(s: &mut &str) -> PResult<Forge> {
    alt((github, gitlab, sourcehut, codeberg, bitbucket)).parse_next(s)
}

fn forge(s: &mut &str) -> PResult<Forge> {
    alt((known_forge, unknown_forge)).parse_next(s)
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct RepoUrl {
    scheme: Scheme,
    forge: Forge,
    owner: String,
    project: String,
}

fn owner(s: &mut &str) -> PResult<String> {
    todo!();
}

fn project(s: &mut &str) -> PResult<String> {
    todo!();
}

fn repo_url(s: &mut &str) -> PResult<RepoUrl> {
    let scheme = scheme.parse_next(s)?;
    let _ = "//".parse_next(s)?;
    let forge = forge.parse_next(s)?;
    match scheme {
      Scheme::Http | Scheme::Https => {
          let _ = "/".parse_next(s)?;
      },
      Scheme::Git => {
          let _ = ":".parse_next(s)?;
      }
    }
    let owner = owner.parse_next(s)?;
    "/".parse_next(s)?;
    let project = project.parse_next(s)?;
    ".git".parse_next(s)?;

    Ok(RepoUrl {
        scheme, forge, owner, project
    })   
}

#[derive(Debug, Error, Eq, PartialEq)]
pub enum RepoUrlError {
    #[error("Unable to parse input str")]
    UnableToParse,
}

impl FromStr for RepoUrl {
    type Err = RepoUrlError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        repo_url.parse(s).map_err(|_| RepoUrlError::UnableToParse)
    }
}
