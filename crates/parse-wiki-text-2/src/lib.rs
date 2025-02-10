// Copyright 2019 Fredrik Portström <https://portstrom.com>
// This is free software distributed under the terms specified in
// the file LICENSE at the top-level directory of this distribution.

//! Parse wiki text from Mediawiki into a tree of elements.
//!
//! # Introduction
//!
//! Wiki text is a format that follows the PHP maxim “Make everything as inconsistent and confusing as possible”. There are hundreds of millions of interesting documents written in this format, distributed under free licenses on sites that use the Mediawiki software, mainly Wikipedia and Wiktionary. Being able to parse wiki text and process these documents would allow access to a significant part of the world's knowledge.
//!
//! The Mediawiki software itself transforms a wiki text document into an HTML document in an outdated format to be displayed in a browser for a human reader. It does so through a [step by step procedure](https://www.mediawiki.org/wiki/Manual:Parser.php) of string substitutions, with some of the steps depending on the result of previous steps. [The main file for this procedure](https://doc.wikimedia.org/mediawiki-core/master/php/Parser_8php_source.html) has 6200 lines of code and the [second biggest file](https://doc.wikimedia.org/mediawiki-core/master/php/Preprocessor__DOM_8php_source.html) has 2000, and then there is a [1400 line file](https://doc.wikimedia.org/mediawiki-core/master/php/ParserOptions_8php_source.html) just to take options for the parser.
//!
//! What would be more interesting is to parse the wiki text document into a structure that can be used by a computer program to reason about the facts in the document and present them in different ways, making them available for a great variety of applications.
//!
//! Some people have tried to parse wiki text using regular expressions. This is incredibly naive and fails as soon as the wiki text is non-trivial. The capabilities of regular expressions don't come anywhere close to the complexity of the weirdness required to correctly parse wiki text. One project did a brave attempt to use a parser generator to parse wiki text. Wiki text was however never designed for formal parsers, so even parser generators are of no help in correctly parsing wiki text.
//!
//! Wiki text has a long history of poorly designed additions carelessly piled on top of each other. The syntax of wiki text is different in each wiki depending on its configuration. You can't even know what's a start tag until you see the corresponding end tag, and you can't know where the end tag is unless you parse the entire hierarchy of nested tags between the start tag and the end tag. In short: If you think you understand wiki text, you don't understand wiki text.
//!
//! Parse Wiki Text attempts to take all uncertainty out of parsing wiki text by converting it to another format that is easy to work with. The target format is Rust objects that can ergonomically be processed using iterators and match expressions.
//!
//! # Design goals
//!
//! ## Correctness
//!
//! Parse Wiki Text is designed to parse wiki text exactly as parsed by Mediawiki. Even when there is obviously a bug in Mediawiki, Parse Wiki Text replicates that exact bug. If there is something Parse Wiki Text doesn't parse exactly the same as Mediawiki, please report it as an issue.
//!
//! ## Speed
//!
//! Parse Wiki Text is designed to parse a page in as little time as possible. It parses tens of thousands of pages per second on each processor core and can quickly parse an entire wiki with millions of pages. If there is anything that can be changed to make Parse Wiki Text faster, please report it as an issue.
//!
//! ## Safety
//!
//! Parse Wiki Text is designed to work with untrusted inputs. If any input doesn't parse safely with reasonable resources, please report it as an issue. No unsafe code is used.
//!
//! ## Platform support
//!
//! Parse Wiki Text is designed to run in a wide variety of environments, such as:
//!
//! - servers running machine code
//! - browsers running Web Assembly
//! - embedded in other programming languages
//!
//! Parse Wiki Text can be deployed anywhere with no dependencies.
//!
//! # Caution
//!
//! Wiki text is a legacy format used by legacy software. Parse Wiki Text is intended only to recover information that has been written for wikis running legacy software, replicating the exact bugs found in the legacy software. Please don't use wiki text as a format for new applications. Wiki text is a horrible format with an astonishing amount of inconsistencies, bad design choices and bugs. For new applications, please use a format that is designed to be easy to process, such as JSON or even better [CBOR](http://cbor.io). See [Wikidata](https://www.wikidata.org/wiki/Wikidata:Main_Page) for an example of a wiki that uses JSON as its format and provides a rich interface for editing data instead of letting people write code. If you need to take information written in wiki text and reuse it in a new application, you can use Parse Wiki Text to convert it to an intermediate format that you can further process into a modern format.
//!
//! # Site configuration
//!
//! Wiki text has plenty of features that are parsed in a way that depends on the configuration of the wiki. This means the configuration must be known before parsing.
//!
//! - External links are parsed only when the scheme of the URI of the link is in the configured list of valid protocols. When the scheme is not valid, the link is parsed as plain text.
//! - Categories and images superficially look they same way as links, but are parsed differently. These can only be distinguished by knowing the namespace aliases from the configuration of the wiki.
//! - Text matching the configured set of magic words is parsed as magic words.
//! - Extension tags have the same syntax as HTML tags, but are parsed differently. The configuration tells which tag names are to be treated as extension tags.
//!
//! The configuration can be seen by making a request to the [site info](https://www.mediawiki.org/wiki/API:Siteinfo) resource on the wiki. The utility [Fetch site configuration](https://github.com/portstrom/fetch_site_configuration) fetches the parts of the configuration needed for parsing pages in the wiki, and outputs Rust code for instantiating a parser with that configuration. Parse Wiki Text contains a default configuration that can be used for testing.
//!
//! # Limitations
//!
//! Wiki text was never designed to be possible to parse into a structured format. It's designed to be parsed in multiple passes, where each pass depends on the output on the previous pass. Most importantly, templates are expanded in an earlier pass and formatting codes are parsed in a later pass. This means the formatting codes you see in the original text are not necessarily the same as the parser will see after templates have been expanded. Luckily this is as bad for human editors as it is for computers, so people tend to avoid writing templates that cause formatting codes to be parsed in a way that differs from what they would expect from reading the original wiki text before expanding templates. Parse Wiki Text assumes that templates never change the meaning of formatting codes around them.
//!
//! # Sandbox
//!
//! A sandbox ([Github](https://github.com/portstrom/parse_wiki_text_sandbox), [try online](https://portstrom.com/parse_wiki_text_sandbox/)) is available that allows interactively entering wiki text and inspecting the result of parsing it.
//!
//! # Comparison with Mediawiki Parser
//!
//! There is another crate called Mediawiki Parser ([crates.io](https://crates.io/crates/mediawiki_parser), [Github](https://github.com/vroland/mediawiki-parser)) that does basically the same thing, parsing wiki text to a tree of elements. That crate however doesn't take into account any of the astonishing amount of weirdness required to correctly parse wiki text. That crate admittedly only parses a subset of wiki text, with the intention to report errors for any text that is too weird to fit that subset, which is a good intention, but when examining it, that subset is quickly found to be too small to parse pages from actual wikis, and even worse, the error reporting is just an empty promise, and there's no indication when a text is incorrectly parsed.
//!
//! That crate could possibly be improved to always report errors when a text isn't in the supported subset, but pages found in real wikis very often don't conform to the small subset of wiki text that can be parsed without weirdness, so it still wouldn't be useful. Improving that crate to correctly parse a large enough subset of wiki text would be as much effort as starting over from scratch, which is why Parse Wiki Text was made without taking anything from Mediawiki Parser. Parse Wiki Text aims to correctly parse all wiki text, not just a subset, and report warnings when encountering weirdness that should be avoided.
//!
//! # Examples
//!
//! The default configuration is used for testing purposes only.
//! For parsing a real wiki you need a site-specific configuration.
//! Reuse the same configuration when parsing multiple pages for efficiency.
//!
//! ```
//! use parse_wiki_text_2::{Configuration, Node};
//!
//! let wiki_text = "\
//!     ==Our values==\n\
//!     *Correctness\n\
//!     *Speed\n\
//!     *Ergonomics\
//! ";
//!
//! let result = Configuration::default().parse(wiki_text).expect("parsing timed out");
//! assert!(result.warnings.is_empty());
//! # let mut found = false;
//! for node in result.nodes {
//!     if let Node::UnorderedList { items, .. } = node {
//!         println!("Our values are:");
//!
//!         for item in items {
//!             let text = item.nodes.iter().map(|node| match node {
//!                 Node::Text { value, .. } => value,
//!                 _ => ""
//!             }).collect::<String>();
//!
//!             println!("- {text}");
//!     #       found = true;
//!         }
//!     }
//! }
//!
//! # assert!(found);
//! ```

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod bold_italic;
mod case_folding_simple;
mod character_entity;
mod comment;
mod configuration;
mod default;
mod external_link;
mod heading;
mod html_entities;
mod line;
mod link;
mod list;
mod magic_word;
mod parse;
mod positioned;
mod redirect;
mod state;
mod table;
mod tag;
mod template;
mod trie;
mod warning;

pub use configuration::ConfigurationSource;
use configuration::Namespace;
pub use parse::ParseError;
use serde::{Deserialize, Serialize};
use state::{OpenNode, OpenNodeType, State};
use std::{
    borrow::Cow,
    collections::{HashMap, HashSet},
};
use trie::Trie;
pub use warning::{Warning, WarningMessage};

/// Configuration for the parser.
///
/// A configuration to correctly parse a real wiki can be created with `Configuration::new`. A configuration for testing and quick and dirty prototyping can be created with `Default::default`.
pub struct Configuration {
    character_entities: Trie<char>,
    link_trail_character_set: HashSet<char>,
    magic_words: Trie<()>,
    namespaces: Trie<Namespace>,
    protocols: Trie<()>,
    redirect_magic_words: Trie<()>,
    tag_name_map: HashMap<String, TagClass>,
}

/// List item of a definition list.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefinitionListItem {
    /// The byte position in the wiki text where the element ends.
    pub end: usize,

    /// The content of the element.
    pub nodes: Vec<Node>,

    /// The byte position in the wiki text where the element starts.
    pub start: usize,

    /// The type of list item.
    pub type_: DefinitionListItemType,
}

/// Identifier for the type of a definition list item.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum DefinitionListItemType {
    /// Parsed from the code `:`.
    Details,

    /// Parsed from the code `;`.
    Term,
}

/// List item of an ordered list or unordered list.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListItem {
    /// The byte position in the wiki text where the element ends.
    pub end: usize,

    /// The content of the element.
    pub nodes: Vec<Node>,

    /// The byte position in the wiki text where the element starts.
    pub start: usize,
}

/// The same as [`Node::Table`], just for ease of use.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableNode {
    /// The HTML attributes of the element.
    pub attributes: Vec<Node>,

    /// The captions of the table.
    pub captions: Vec<TableCaption>,

    /// The byte position in the wiki text where the element ends.
    pub end: usize,

    /// The rows of the table.
    pub rows: Vec<TableRow>,

    /// The byte position in the wiki text where the element starts.
    pub start: usize,
}

/// Parsed node.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Node {
    /// Toggle bold text. Parsed from the code `'''`.
    Bold {
        /// The byte position in the wiki text where the element ends.
        end: usize,

        /// The byte position in the wiki text where the element starts.
        start: usize,
    },

    /// Toggle bold and italic text. Parsed from the code `'''''`.
    BoldItalic {
        /// The byte position in the wiki text where the element ends.
        end: usize,

        /// The byte position in the wiki text where the element starts.
        start: usize,
    },

    /// Category. Parsed from code starting with `[[`, a category namespace and `:`.
    Category {
        /// The byte position in the wiki text where the element ends.
        end: usize,

        /// Additional information for sorting entries on the category page, if any.
        ordinal: Vec<Node>,

        /// The byte position in the wiki text where the element starts.
        start: usize,

        /// The category referred to.
        target: String,
    },

    /// Character entity. Parsed from code starting with `&` and ending with `;`.
    CharacterEntity {
        /// The character represented.
        character: char,

        /// The byte position in the wiki text where the element ends.
        end: usize,

        /// The byte position in the wiki text where the element starts.
        start: usize,
    },

    /// Comment. Parsed from code starting with `<!--`.
    Comment {
        /// The byte position in the wiki text where the element ends.
        end: usize,

        /// The byte position in the wiki text where the element starts.
        start: usize,
    },

    /// Definition list. Parsed from code starting with `:` or `;`.
    DefinitionList {
        /// The byte position in the wiki text where the element ends.
        end: usize,

        /// The list items of the list.
        items: Vec<DefinitionListItem>,

        /// The byte position in the wiki text where the element starts.
        start: usize,
    },

    /// End tag. Parsed from code starting with `</` and a valid tag name.
    EndTag {
        /// The byte position in the wiki text where the element ends.
        end: usize,

        /// The tag name.
        name: String,

        /// The byte position in the wiki text where the element starts.
        start: usize,
    },

    /// External link. Parsed from code starting with `[` and a valid protocol.
    ExternalLink {
        /// The byte position in the wiki text where the element ends.
        end: usize,

        /// The content of the element.
        nodes: Vec<Node>,

        /// The byte position in the wiki text where the element starts.
        start: usize,
    },

    /// Heading. Parsed from code starting with `=` and ending with `=`.
    Heading {
        /// The byte position in the wiki text where the element ends.
        end: usize,

        /// The level of the heading from 1 to 6.
        level: u8,

        /// The content of the element.
        nodes: Vec<Node>,

        /// The byte position in the wiki text where the element starts.
        start: usize,
    },

    /// Horizontal divider. Parsed from code starting with `----`.
    HorizontalDivider {
        /// The byte position in the wiki text where the element ends.
        end: usize,

        /// The byte position in the wiki text where the element starts.
        start: usize,
    },

    /// Image. Parsed from code starting with `[[`, a file namespace and `:`.
    Image {
        /// The byte position in the wiki text where the element ends.
        end: usize,

        /// The byte position in the wiki text where the element starts.
        start: usize,

        /// The file name of the image.
        target: String,

        /// Additional information for the image.
        text: Vec<Node>,
    },

    /// Toggle italic text. Parsed from the code `''`.
    Italic {
        /// The byte position in the wiki text where the element ends.
        end: usize,

        /// The byte position in the wiki text where the element starts.
        start: usize,
    },

    /// Link. Parsed from code starting with `[[` and ending with `]]`.
    Link {
        /// The byte position in the wiki text where the element ends.
        end: usize,

        /// The byte position in the wiki text where the element starts.
        start: usize,

        /// The target of the link.
        target: String,

        /// The text to display for the link.
        text: Vec<Node>,
    },

    /// Magic word. Parsed from the code `__`, a valid magic word and `__`.
    MagicWord {
        /// The byte position in the wiki text where the element ends.
        end: usize,

        /// The byte position in the wiki text where the element starts.
        start: usize,
    },

    /// Ordered list. Parsed from code starting with `#`.
    OrderedList {
        /// The byte position in the wiki text where the element ends.
        end: usize,

        /// The list items of the list.
        items: Vec<ListItem>,

        /// The byte position in the wiki text where the element starts.
        start: usize,
    },

    /// Paragraph break. Parsed from an empty line between elements that can appear within a paragraph.
    ParagraphBreak {
        /// The byte position in the wiki text where the element ends.
        end: usize,

        /// The byte position in the wiki text where the element starts.
        start: usize,
    },

    /// Parameter. Parsed from code starting with `{{{` and ending with `}}}`.
    Parameter {
        /// The default value of the parameter.
        default: Option<Vec<Node>>,

        /// The byte position in the wiki text where the element ends.
        end: usize,

        /// The name of the parameter.
        name: Vec<Node>,

        /// The byte position in the wiki text where the element starts.
        start: usize,
    },

    /// Block of preformatted text. Parsed from code starting with a space at the beginning of a line.
    Preformatted {
        /// The byte position in the wiki text where the element ends.
        end: usize,

        /// The content of the element.
        nodes: Vec<Node>,

        /// The byte position in the wiki text where the element starts.
        start: usize,
    },

    /// Redirect. Parsed at the start of the wiki text from code starting with `#` followed by a redirect magic word.
    Redirect {
        /// The byte position in the wiki text where the element ends.
        end: usize,

        /// The target of the redirect.
        target: String,

        /// The byte position in the wiki text where the element starts.
        start: usize,
    },

    /// Start tag. Parsed from code starting with `<` and a valid tag name.
    StartTag {
        /// The byte position in the wiki text where the element ends.
        end: usize,

        /// The tag name.
        name: String,

        /// The byte position in the wiki text where the element starts.
        start: usize,
    },

    /// Table. Parsed from code starting with `{|`.
    Table {
        /// The HTML attributes of the element.
        attributes: Vec<Node>,

        /// The captions of the table.
        captions: Vec<TableCaption>,

        /// The byte position in the wiki text where the element ends.
        end: usize,

        /// The rows of the table.
        rows: Vec<TableRow>,

        /// The byte position in the wiki text where the element starts.
        start: usize,
    },

    /// Extension tag. Parsed from code starting with `<` and the tag name of a valid extension tag.
    Tag {
        /// The byte position in the wiki text where the element ends.
        end: usize,

        /// The tag name.
        name: String,

        /// The content of the tag, between the start tag and the end tag, if any.
        nodes: Vec<Node>,

        /// The byte position in the wiki text where the element starts.
        start: usize,
    },

    /// Template. Parsed from code starting with `{{` and ending with `}}`.
    Template {
        /// The byte position in the wiki text where the element ends.
        end: usize,

        /// The name of the template.
        name: Vec<Node>,

        /// The parameters of the template.
        parameters: Vec<Parameter>,

        /// The byte position in the wiki text where the element starts.
        start: usize,
    },

    /// Plain text.
    Text {
        /// The byte position in the wiki text where the element ends.
        end: usize,

        /// The byte position in the wiki text where the element starts.
        start: usize,

        /// The text.
        value: String,
    },

    /// Unordered list. Parsed from code starting with `*`.
    UnorderedList {
        /// The byte position in the wiki text where the element ends.
        end: usize,

        /// The list items of the list.
        items: Vec<ListItem>,

        /// The byte position in the wiki text where the element starts.
        start: usize,
    },
}

/// Output of parsing wiki text.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Output {
    /// The top level of parsed nodes.
    pub nodes: Vec<Node>,

    /// Warnings from the parser telling that something is not well-formed.
    pub warnings: Vec<Warning>,
}

/// Template parameter.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parameter {
    /// The byte position in the wiki text where the element ends.
    pub end: usize,

    /// The name of the parameter, if any.
    pub name: Option<Vec<Node>>,

    /// The byte position in the wiki text where the element starts.
    pub start: usize,

    /// The value of the parameter.
    pub value: Vec<Node>,
}

/// Element that has a start position and end position.
pub trait Positioned {
    /// The byte position in the wiki text where the element ends.
    fn end(&self) -> usize;

    /// The byte position in the wiki text where the element starts.
    fn start(&self) -> usize;
}

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq, Serialize, Deserialize)]
enum TagClass {
    ExtensionTag,
    Tag,
}

/// Table caption.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableCaption {
    /// The HTML attributes of the element.
    pub attributes: Option<Vec<Node>>,

    /// The content of the element.
    pub content: Vec<Node>,

    /// The byte position in the wiki text where the element ends.
    pub end: usize,

    /// The byte position in the wiki text where the element starts.
    pub start: usize,
}

/// Table cell.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableCell {
    /// The HTML attributes of the element.
    pub attributes: Option<Vec<Node>>,

    /// The content of the element.
    pub content: Vec<Node>,

    /// The byte position in the wiki text where the element ends.
    pub end: usize,

    /// The byte position in the wiki text where the element starts.
    pub start: usize,

    /// The type of cell.
    pub type_: TableCellType,
}

/// Type of table cell.
#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum TableCellType {
    /// Heading cell.
    Heading,

    /// Ordinary cell.
    Ordinary,
}

/// Table row.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableRow {
    /// The HTML attributes of the element.
    pub attributes: Vec<Node>,

    /// The cells in the row.
    pub cells: Vec<TableCell>,

    /// The byte position in the wiki text where the element ends.
    pub end: usize,

    /// The byte position in the wiki text where the element starts.
    pub start: usize,
}
