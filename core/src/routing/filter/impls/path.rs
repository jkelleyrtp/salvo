use std::fmt::{self, Debug};

use regex::Regex;

use crate::http::Request;
use crate::routing::{Filter, PathState};

trait PathPart: Send + Sync + Debug {
    fn detect<'a>(&self, state: &mut PathState) -> bool;
}

#[derive(Debug)]
struct CombPart(Vec<Box<dyn PathPart>>);
impl PathPart for CombPart {
    fn detect<'a>(&self, state: &mut PathState) -> bool {
        for child in &self.0 {
            if !child.detect(state) {
                return false;
            }
        }
        true
    }
}
#[derive(Debug, Eq, PartialEq)]
struct NamedPart(String);
impl PathPart for NamedPart {
    fn detect<'a>(&self, state: &mut PathState) -> bool {
        let url_path = &state.url_path[state.cursor..];
        if url_path.is_empty() {
            return false;
        }
        let segment = url_path.splitn(2, '/').collect::<Vec<_>>()[0];
        state.params.insert(self.0.clone(), segment.to_owned());
        state.cursor += segment.len();
        true
    }
}

#[derive(Debug)]
struct RegexPart {
    name: String,
    regex: Regex,
}
impl RegexPart {
    fn new(name: String, regex: Regex) -> RegexPart {
        RegexPart { name, regex }
    }
}
impl PartialEq for RegexPart {
    fn eq(&self, other: &Self) -> bool {
        self.regex.as_str() == other.regex.as_str()
    }
}
impl PathPart for RegexPart {
    fn detect<'a>(&self, state: &mut PathState) -> bool {
        let url_path = &state.url_path[state.cursor..];
        if url_path.is_empty() {
            return false;
        }
        let segment = url_path.splitn(2, '/').collect::<Vec<_>>()[0];
        let caps = self.regex.captures(segment);
        if let Some(caps) = caps {
            state
                .params
                .insert(self.name.clone(), caps[&self.name[..]].to_owned());
            state.cursor += segment.len();
            true
        } else {
            false
        }
    }
}

// If name starts with *, only match not empty path, if name starts with ** will match empty path.
#[derive(Eq, PartialEq, Debug)]
struct RestPart(String);
impl RestPart {
    fn new(name: String) -> RestPart {
        RestPart(name)
    }
}
impl PathPart for RestPart {
    fn detect<'a>(&self, state: &mut PathState) -> bool {
        let url_path = &state.url_path[state.cursor..];
        if !url_path.is_empty() || self.0.starts_with("**") {
            state.params.insert(self.0.clone(), url_path.to_owned());
            true
        } else {
            false
        }
    }
}

#[derive(Eq, PartialEq, Debug)]
struct ConstPart(String);
impl ConstPart {
    fn new(segment: String) -> ConstPart {
        ConstPart(segment)
    }
}
impl PathPart for ConstPart {
    fn detect<'a>(&self, state: &mut PathState) -> bool {
        let url_path = &state.url_path[state.cursor..];
        if url_path.is_empty() {
            return false;
        }
        let segment = url_path.splitn(2, '/').collect::<Vec<_>>()[0];
        if self.0 == segment {
            state.cursor += segment.len();
            true
        } else {
            false
        }
    }
}

struct PathParser {
    offset: usize,
    path: Vec<char>,
}
impl PathParser {
    fn new(raw_value: &str) -> PathParser {
        PathParser {
            offset: 0,
            path: raw_value.chars().collect(),
        }
    }
    fn next(&mut self, skip_blank: bool) -> Option<char> {
        if self.offset < self.path.len() - 1 {
            self.offset += 1;
            if skip_blank {
                self.skip_blank();
            }
            Some(self.path[self.offset])
        } else {
            self.offset = self.path.len();
            None
        }
    }
    fn peek(&self, skip_blank: bool) -> Option<char> {
        if self.offset < self.path.len() - 1 {
            if skip_blank {
                let mut offset = self.offset + 1;
                let mut ch = self.path[offset];
                while ch == ' ' || ch == '\t' {
                    offset += 1;
                    if offset >= self.path.len() {
                        return None;
                    }
                    ch = self.path[offset]
                }
                Some(ch)
            } else {
                Some(self.path[self.offset + 1])
            }
        } else {
            None
        }
    }
    fn curr(&self) -> Option<char> {
        self.path.get(self.offset).copied()
    }
    fn scan_ident(&mut self) -> Result<String, String> {
        let mut ident = "".to_owned();
        let mut ch = self
            .curr()
            .ok_or_else(|| "current postion is out of index when scan ident".to_owned())?;
        while ch != '/' && ch != ':' && ch != '<' && ch != '>' {
            ident.push(ch);
            if let Some(c) = self.next(false) {
                ch = c;
            } else {
                break;
            }
        }
        if ident.is_empty() {
            Err("ident segment is empty".to_owned())
        } else {
            Ok(ident)
        }
    }
    fn scan_regex(&mut self) -> Result<String, String> {
        let mut regex = "".to_owned();
        let mut ch = self
            .curr()
            .ok_or_else(|| "current postion is out of index when scan regex".to_owned())?;
        loop {
            regex.push(ch);
            if let Some(c) = self.next(false) {
                ch = c;
                if ch == '/' {
                    let pch = self.peek(true);
                    if pch.is_none() {
                        return Err("path end but regex is not ended".to_owned());
                    } else if let Some('>') = pch {
                        self.next(true);
                        break;
                    }
                }
            } else {
                break;
            }
        }
        if regex.is_empty() {
            Err("regex segment is empty".to_owned())
        } else {
            Ok(regex)
        }
    }
    fn scan_const(&mut self) -> Result<String, String> {
        let mut cnst = "".to_owned();
        let mut ch = self
            .curr()
            .ok_or_else(|| "current postion is out of index when scan const".to_owned())?;
        while ch != '/' && ch != ':' && ch != '<' && ch != '>' {
            cnst.push(ch);
            if let Some(c) = self.next(false) {
                ch = c;
            } else {
                break;
            }
        }
        if cnst.is_empty() {
            Err("const segment is empty".to_owned())
        } else {
            Ok(cnst)
        }
    }
    fn skip_blank(&mut self) {
        if let Some(mut ch) = self.curr() {
            while ch == ' ' || ch == '\t' {
                if self.offset < self.path.len() - 1 {
                    self.offset += 1;
                    ch = self.path[self.offset];
                } else {
                    break;
                }
            }
        }
    }
    fn skip_slash(&mut self) {
        if let Some(mut ch) = self.curr() {
            while ch == '/' {
                if let Some(c) = self.next(false) {
                    ch = c;
                } else {
                    break;
                }
            }
        }
    }
    fn scan_parts(&mut self) -> Result<Vec<Box<dyn PathPart>>, String> {
        let mut ch = self
            .curr()
            .ok_or_else(|| "current postion is out of index when scan part".to_owned())?;
        let mut parts: Vec<Box<dyn PathPart>> = vec![];
        while ch != '/' {
            if ch == '<' {
                ch = self
                    .next(true)
                    .ok_or_else(|| "char is needed after <".to_owned())?;
                if ch == '*' {
                    self.next(true);
                    let name = format!("*{}", self.scan_ident().unwrap_or_default());
                    if self.offset < self.path.len() - 1 {
                        return Err("no chars allowed after rest segment".to_owned());
                    }
                    parts.push(Box::new(RestPart::new(name)));
                    self.next(false);
                    break;
                } else {
                    let name = self.scan_ident()?;
                    if name.is_empty() {
                        return Err("name is empty string".to_owned());
                    }
                    self.skip_blank();
                    ch = self
                        .curr()
                        .ok_or_else(|| "current position is out of index".to_owned())?;
                    if ch == ':' {
                        let is_slash = match self.next(true) {
                            Some(c) => c == '/',
                            None => false,
                        };
                        if !is_slash {
                            return Err(format!(
                                "except '/' to start regex, but found {:?} at offset: {}",
                                self.curr(),
                                self.offset
                            ));
                        }
                        self.next(false);
                        let regex = Regex::new(&self.scan_regex()?).map_err(|e| e.to_string())?;
                        parts.push(Box::new(RegexPart::new(name, regex)));
                    } else if ch == '>' {
                        parts.push(Box::new(NamedPart(name)));
                        if !self.peek(false).map(|c| c == '/').unwrap_or(true) {
                            return Err(format!(
                                "named part must be the last one in current segement, expect '/' or end, but found {:?} at offset: {}",
                                self.curr(),
                                self.offset
                            ));
                        }
                    }
                    if let Some(c) = self.curr() {
                        if c != '>' {
                            return Err(format!(
                                "except '>' to end regex segment, but found {:?} at offset: {}",
                                c, self.offset
                            ));
                        } else {
                            self.next(false);
                        }
                    } else {
                        break;
                    }
                }
            } else {
                let part = self.scan_const().unwrap_or_default();
                if part.is_empty() {
                    return Err("const part is empty string".to_owned());
                }
                parts.push(Box::new(ConstPart::new(part)));
            }
            if let Some(c) = self.curr() {
                if c == '/' {
                    break;
                }
                ch = c;
            } else {
                break;
            }
        }
        Ok(parts)
    }
    fn parse(&mut self) -> Result<Vec<Box<dyn PathPart>>, String> {
        let mut path_parts: Vec<Box<dyn PathPart>> = vec![];
        if self.path.is_empty() {
            return Ok(path_parts);
        }
        loop {
            self.skip_slash();
            if self.offset >= self.path.len() - 1 {
                break;
            }
            if self.curr().map(|c| c == '/').unwrap_or(false) {
                return Err(format!(
                    "'/' is not allowed after '/' at offset {:?}",
                    self.offset
                ));
            }
            let mut parts = self.scan_parts()?;
            if parts.len() > 1 {
                path_parts.push(Box::new(CombPart(parts)));
            } else if !parts.is_empty() {
                path_parts.push(parts.pop().unwrap());
            } else {
                return Err("scan parts is empty".to_owned());
            }
            if self.curr().map(|c| c != '/').unwrap_or(false) {
                return Err(format!(
                    "expect '/', but found {:?} at offset {:?}",
                    self.curr(),
                    self.offset
                ));
            }
            self.next(true);
            if self.offset >= self.path.len() - 1 {
                break;
            }
        }
        Ok(path_parts)
    }
}

pub struct PathFilter {
    raw_value: String,
    path_parts: Vec<Box<dyn PathPart>>,
}

impl Debug for PathFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ raw_value: '{}'}}", &self.raw_value)
    }
}
impl Filter for PathFilter {
    fn filter(&self, _req: &mut Request, state: &mut PathState) -> bool {
        self.detect(state)
    }
}
impl PathFilter {
    pub fn new(value: impl Into<String>) -> Self {
        let raw_value = value.into();
        let mut parser = PathParser::new(&raw_value);
        let path_parts = match parser.parse() {
            Ok(path_parts) => path_parts,
            Err(e) => {
                panic!("{}", e);
            }
        };
        PathFilter {
            raw_value,
            path_parts,
        }
    }
    pub(crate) fn detect(&self, state: &mut PathState) -> bool {
        if state.ended() {
            return false;
        }
        if !self.path_parts.is_empty() {
            for ps in &self.path_parts {
                if ps.detect(state) {
                    if state.ended() {
                        return false;
                    }
                    let rest = &state.url_path[state.cursor..];
                    if rest.starts_with('/') {
                        state.cursor += 1;
                    }
                }
            }
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PathParser;
    use crate::routing::{PathFilter, PathState};
    use std::collections::HashMap;

    #[test]
    fn test_parse_empty() {
        let segments = PathParser::new("").parse().unwrap();
        assert!(segments.is_empty());
    }
    #[test]
    fn test_parse_root() {
        let segments = PathParser::new("/").parse().unwrap();
        assert!(segments.is_empty());
    }
    #[test]
    fn test_parse_rest_without_name() {
        let segments = PathParser::new("/hello/<*>").parse().unwrap();
        assert_eq!(
            format!("{:?}", segments),
            r#"[ConstPart("hello"), RestPart("*")]"#
        );
    }

    #[test]
    fn test_parse_single_const() {
        let segments = PathParser::new("/hello").parse().unwrap();
        assert_eq!(format!("{:?}", segments), r#"[ConstPart("hello")]"#);
    }
    #[test]
    fn test_parse_multi_const() {
        let segments = PathParser::new("/hello/world").parse().unwrap();
        assert_eq!(
            format!("{:?}", segments),
            r#"[ConstPart("hello"), ConstPart("world")]"#
        );
    }
    #[test]
    fn test_parse_single_regex() {
        let segments = PathParser::new(r"/<abc:/\d+/>").parse().unwrap();
        assert_eq!(
            format!("{:?}", segments),
            r#"[RegexPart { name: "abc", regex: \d+ }]"#
        );
    }
    #[test]
    fn test_parse_single_regex_with_prefix() {
        let segments = PathParser::new(r"/prefix_<abc:/\d+/>").parse().unwrap();
        assert_eq!(
            format!("{:?}", segments),
            r#"[CombPart([ConstPart("prefix_"), RegexPart { name: "abc", regex: \d+ }])]"#
        );
    }
    #[test]
    fn test_parse_single_regex_with_suffix() {
        let segments = PathParser::new(r"/<abc:/\d+/>_suffix.png").parse().unwrap();
        assert_eq!(
            format!("{:?}", segments),
            r#"[CombPart([RegexPart { name: "abc", regex: \d+ }, ConstPart("_suffix.png")])]"#
        );
    }
    #[test]
    fn test_parse_single_regex_with_prefix_and_suffix() {
        let segments = PathParser::new(r"/prefix<abc:/\d+/>suffix.png")
            .parse()
            .unwrap();
        assert_eq!(
            format!("{:?}", segments),
            r#"[CombPart([ConstPart("prefix"), RegexPart { name: "abc", regex: \d+ }, ConstPart("suffix.png")])]"#
        );
    }
    #[test]
    fn test_parse_multi_regex() {
        let segments = PathParser::new(r"/first<id>/prefix<abc:/\d+/>")
            .parse()
            .unwrap();
        assert_eq!(
            format!("{:?}", segments),
            r#"[CombPart([ConstPart("first"), NamedPart("id")]), CombPart([ConstPart("prefix"), RegexPart { name: "abc", regex: \d+ }])]"#
        );
    }
    #[test]
    fn test_parse_multi_regex_with_prefix() {
        let segments = PathParser::new(r"/first<id>/prefix<abc:/\d+/>")
            .parse()
            .unwrap();
        assert_eq!(
            format!("{:?}", segments),
            r#"[CombPart([ConstPart("first"), NamedPart("id")]), CombPart([ConstPart("prefix"), RegexPart { name: "abc", regex: \d+ }])]"#
        );
    }
    #[test]
    fn test_parse_multi_regex_with_suffix() {
        let segments = PathParser::new(r"/first<id:/\d+/>/prefix<abc:/\d+/>")
            .parse()
            .unwrap();
        assert_eq!(
            format!("{:?}", segments),
            r#"[CombPart([ConstPart("first"), RegexPart { name: "id", regex: \d+ }]), CombPart([ConstPart("prefix"), RegexPart { name: "abc", regex: \d+ }])]"#
        );
    }
    #[test]
    fn test_parse_multi_regex_with_prefix_and_suffix() {
        let segments = PathParser::new(r"/first<id>/prefix<abc:/\d+/>ext")
            .parse()
            .unwrap();
        assert_eq!(
            format!("{:?}", segments),
            r#"[CombPart([ConstPart("first"), NamedPart("id")]), CombPart([ConstPart("prefix"), RegexPart { name: "abc", regex: \d+ }, ConstPart("ext")])]"#
        );
    }
    #[test]
    fn test_parse_rest() {
        let segments = PathParser::new(r"/first<id>/<*rest>").parse().unwrap();
        assert_eq!(
            format!("{:?}", segments),
            r#"[CombPart([ConstPart("first"), NamedPart("id")]), RestPart("*rest")]"#
        );
    }
    #[test]
    fn test_parse_named_failed1() {
        assert!(PathParser::new(r"/first<id>ext2").parse().is_err());
    }

    #[test]
    fn test_parse_rest_failed1() {
        assert!(PathParser::new(r"/first<id>ext2<*rest>").parse().is_err());
    }
    #[test]
    fn test_parse_rest_failed2() {
        assert!(PathParser::new(r"/first<id>ext2/<*rest>wefwe")
            .parse()
            .is_err());
    }

    #[test]
    fn test_detect_consts() {
        let filter = PathFilter::new("/hello/world");
        let mut state = PathState::new("hello/world");
        filter.detect(&mut state);
        assert_eq!(
            state,
            PathState {
                url_path: "hello/world".into(),
                cursor: 2,
                params: HashMap::new(),
            }
        );
    }

    #[test]
    fn test_detect_const_and_named() {
        let filter = PathFilter::new("/hello/world<id>");
        let mut state = PathState::new("hello/worldabc");
        filter.detect(&mut state);
        assert_eq!(
            state,
            PathState {
                url_path: "hello/world".into(),
                cursor: 2,
                params: HashMap::new(),
            }
        );
    }
}
