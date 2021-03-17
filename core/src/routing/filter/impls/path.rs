use std::collections::HashMap;
use std::fmt::{self, Debug};

use regex::Regex;

use crate::http::Request;
use crate::routing::{Filter, PathState};

trait Segment: Send + Sync + Debug {
    fn detect<'a>(&self, segments: Vec<&'a str>) -> (bool, Option<PathMatched<'a>>);
}

struct PathMatched<'a> {
    ending_matched: bool,
    segments: Option<Vec<&'a str>>,
    matched_params: Option<HashMap<String, String>>,
}

#[derive(Debug)]
struct RegexSegment {
    regex: Regex,
    names: Vec<String>,
}
impl RegexSegment {
    fn new(regex: Regex, names: Vec<String>) -> RegexSegment {
        RegexSegment { regex, names }
    }
}
impl Segment for RegexSegment {
    fn detect<'a>(&self, segments: Vec<&'a str>) -> (bool, Option<PathMatched<'a>>) {
        if segments.is_empty() {
            return (false, None);
        }
        let segment = segments[0];
        let caps = self.regex.captures(segment);
        if let Some(caps) = caps {
            let mut kv = HashMap::<String, String>::new();
            for name in &self.names {
                kv.insert(name.clone(), caps[&name[..]].to_owned());
            }
            (
                true,
                Some(PathMatched {
                    ending_matched: false,
                    segments: Some(vec![segment]),
                    matched_params: Some(kv),
                }),
            )
        } else {
            (false, None)
        }
    }
}

// If name starts with *, only match not empty path, if name starts with ** will match empty path.
#[derive(Debug)]
struct RestSegment(String);
impl RestSegment {
    fn new(name: String) -> RestSegment {
        RestSegment(name)
    }
}
impl Segment for RestSegment {
    fn detect<'a>(&self, segments: Vec<&'a str>) -> (bool, Option<PathMatched<'a>>) {
        if !segments.is_empty() || self.0.starts_with("**") {
            let mut kv = HashMap::new();
            kv.insert(self.0.clone(), segments.join("/"));
            (
                true,
                Some(PathMatched {
                    ending_matched: true,
                    segments: Some(segments),
                    matched_params: Some(kv),
                }),
            )
        } else {
            (false, None)
        }
    }
}

#[derive(Debug)]
struct ConstSegment(String);
impl ConstSegment {
    fn new(segment: String) -> ConstSegment {
        ConstSegment(segment)
    }
}
impl Segment for ConstSegment {
    fn detect<'a>(&self, segments: Vec<&'a str>) -> (bool, Option<PathMatched<'a>>) {
        if segments.is_empty() {
            return (false, None);
        }
        if self.0 == segments[0] {
            (true, Some(PathMatched {
                ending_matched: false,
                segments: Some(vec![segments[0]]), 
                matched_params:None,
            }))
        } else {
            (false, None)
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
        if !self.path.is_empty() && self.offset < self.path.len() - 1 {
            self.offset += 1;
            if skip_blank {
                self.skip_blank();
            }
            return Some(self.path[self.offset]);
        }
        None
    }
    fn peek(&self, skip_blank: bool) -> Option<char> {
        if !self.path.is_empty() && self.offset < self.path.len() - 1 {
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
    fn curr(&self) -> char {
        self.path[self.offset]
    }
    fn scan_ident(&mut self) -> Result<String, String> {
        let mut ident = "".to_owned();
        let mut ch = self.curr();
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
        let mut ch = self.curr();
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
        let mut ch = self.curr();
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
    fn scan_segment(&mut self) -> Result<Box<dyn Segment>, String> {
        let mut const_seg = "".to_owned();
        let mut regex_seg = "".to_owned();
        let mut regex_names = vec![];
        let mut ch = self.curr();
        if ch == '<' {
            ch = self.next(true).expect("char is needed");
            if ch == '*' {
                self.next(true);
                let rest_seg = self.scan_ident()?;
                if self.offset < self.path.len() - 1 {
                    panic!("no chars allowed after rest egment");
                }
                return Ok(Box::new(RestSegment::new(format!("*{}", rest_seg))));
            } else {
                let rname = self.scan_ident()?;
                if rname.is_empty() {
                    return Err("name must not equal empty string".to_owned());
                } else {
                    regex_names.push(rname.clone());
                }
                let mut rrgex = "[^/]+".to_owned();
                ch = self.curr();
                if ch == ':' {
                    let is_slash = match self.next(true) {
                        Some(c) => c == '/',
                        None => false,
                    };
                    if !is_slash {
                        return Err(format!("except '/' to start regex current char is '{}'", self.curr()));
                    }
                    self.next(false);
                    rrgex = self.scan_regex()?;
                }
                if self.curr() != '>' {
                    return Err(format!("except '>' to end regex segment, current char is '{}'", self.curr()));
                } else {
                    self.next(false);
                }
                if !const_seg.is_empty() {
                    regex_seg.push_str(&const_seg);
                    const_seg.clear();
                }
                regex_seg.push_str(&("(?P<".to_owned() + &rname + ">" + &rrgex + ")"));
            }
        } else {
            const_seg = self.scan_const()?;
        }
        if self.offset < self.path.len() - 1 && self.curr() != '/' {
            return Err(format!("expect '/' here, but found {:?}   {:?}", self.curr(), self.offset));
        }
        if !regex_seg.is_empty() {
            if !const_seg.is_empty() {
                regex_seg.push_str(&const_seg);
            }
            let regex = Regex::new(&regex_seg);
            match regex {
                Ok(r) => Ok(Box::new(RegexSegment::new(r, regex_names))),
                Err(_) => Err("regex error".to_owned()),
            }
        } else if !const_seg.is_empty() {
            Ok(Box::new(ConstSegment::new(const_seg)))
        } else {
            Err("parse path error 1".to_owned())
        }
    }
    fn skip_blank(&mut self) {
        let mut ch = self.curr();
        while ch == ' ' || ch == '\t' {
            if !self.path.is_empty() && self.offset < self.path.len() - 1 {
                self.offset += 1;
                ch = self.path[self.offset];
            } else {
                break;
            }
        }
    }
    fn skip_slash(&mut self) {
        let mut ch = self.path[self.offset];
        while ch == '/' {
            if let Some(c) = self.next(false) {
                ch = c;
            } else {
                break;
            }
        }
    }
    fn parse(&mut self) -> Result<Vec<Box<dyn Segment>>, String> {
        let mut segments: Vec<Box<dyn Segment>> = vec![];
        let ch = '/';
        loop {
            if ch == '/' {
                self.skip_slash();
                if self.offset >= self.path.len() - 1 {
                    break;
                }
                segments.push(self.scan_segment()?);
            } else {
                return Err("parse path error 2".to_owned());
            }
            if self.offset >= self.path.len() - 1 {
                break;
            }
        }
        Ok(segments)
    }
}

pub struct PathFilter {
    raw_value: String,
    segments: Vec<Box<dyn Segment>>,
}

impl Debug for PathFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ raw_value: '{}'}}", &self.raw_value)
    }
}
impl Filter for PathFilter {
    fn filter(&self, _req: &mut Request, path: &mut PathState) -> bool {
        if path.ending_matched {
            return false;
        }
        if !self.segments.is_empty() {
            let mut params = HashMap::<String, String>::new();
            let mut match_cursor = path.match_cursor;
            for ps in &self.segments {
                let (matched, detail) = ps.detect(path.segments[match_cursor..].iter().map(AsRef::as_ref).collect());
                if !matched {
                    return false;
                } else if let Some(detail) = detail{
                    if let Some(kv) = detail.matched_params {
                        params.extend(kv);
                    }
                    if let Some(segs) = detail.segments {
                        match_cursor += segs.len();
                    }
                    if detail.ending_matched {
                        path.ending_matched = true;
                        break;
                    }
                } else {
                    return false;
                }
            }
            if !params.is_empty() {
                path.params.extend(params);
            }
            path.match_cursor = match_cursor;
            true
        } else {
            false
        }
    }
}
impl PathFilter {
    pub fn new(value: impl Into<String>) -> Self {
        let raw_value = value.into();
        let mut parser = PathParser::new(&raw_value);
        let segments = match parser.parse() {
            Ok(segments) => segments,
            Err(e) => {
                panic!(e);
            }
        };
        PathFilter { raw_value, segments }
    }
}
