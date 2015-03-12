//! Contains high-level interface for a pull-based XML parser.
//!
//! The most important type in this module is `EventReader`, which provides an iterator
//! view for events in XML document.

use std::io::Read;

use self::parser::PullParser;
use self::events::XmlEvent;

pub use self::config::ParserConfig;

mod lexer;
mod parser;
pub mod config;
pub mod events;

/// Simple wrapper around an `std::io::Read` which provides pull-based XML parsing.
pub struct EventReader<B> {
    source: B,
    parser: PullParser
}

impl<B: Read> EventReader<B> {
    /// Creates a new parser, consuming given `Read`.
    #[inline]
    pub fn new(source: B) -> EventReader<B> {
        EventReader::new_with_config(source, ParserConfig::new())
    }

    /// Creates a new parser with the provded configuration, consuming given `Read`.
    #[inline]
    pub fn new_with_config(source: B, config: ParserConfig) -> EventReader<B> {
        EventReader { source: source, parser: PullParser::new(config) }
    }

    /// Pulls and returns next XML event from the stream.
    ///
    /// If returned event is `xml::event::Error` or `xml::event::EndDocument`, then
    /// further calls to this method will return this event again.
    #[inline]
    pub fn next(&mut self) -> XmlEvent {
        self.parser.next(&mut self.source)
    }

    /// Returns an iterator over XML events.
    ///
    /// When the next event is `xml::event::Error` or `xml::event::EndDocument`, then
    /// it will be returned by the iterator once, and then it will stop producing events.
    #[inline]
    pub fn events<'a>(&'a mut self) -> Events<'a, B> {
        Events { reader: self, finished: false }
    }
}

/// XML events iterator, created by `events()` method on `Parser`.
pub struct Events<'a, B: 'a> {
    reader: &'a mut EventReader<B>,
    finished: bool
}

impl<'a, B: Read> Iterator for Events<'a, B> {
    type Item = XmlEvent;
    
    #[inline]
    fn next(&mut self) -> Option<XmlEvent> {
        if self.finished { None }
        else {
            let ev = self.reader.next();
            match ev {
                XmlEvent::EndDocument | XmlEvent::Error(_) => self.finished = true,
                _ => {}
            }
            Some(ev)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::old_io::File;
    use std::old_io::BufferedReader;

    use super::{EventReader, ParserConfig};

    fn test_sample(path: &str) {
        let file = File::open(&Path::new(path));
        let reader = BufferedReader::new(file);

        let mut eventreader = EventReader::new_with_config(
            reader,
            ParserConfig::new()
                .ignore_comments(true)
                .whitespace_to_characters(true)
                .cdata_to_characters(true)
                .trim_whitespace(true)
                .coalesce_characters(true)
        );

        for e in eventreader.events() {
            println!("{:?}", e);
        }
    }

    #[test]
    #[ignore]
    fn sample_1_test() {
        test_sample("data/sample_1.xml");
    }

    #[test]
    #[ignore]
    fn sample_2_test() {
        test_sample("data/sample_2.xml");
    }

    #[test]
    #[ignore]
    fn sample_3_test() {
        test_sample("data/sample_3.xml");
    }

    #[test]
    #[ignore]
    fn sample_4_test() {
        test_sample("data/sample_4.xml");
    }
}
