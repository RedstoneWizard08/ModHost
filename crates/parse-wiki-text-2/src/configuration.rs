// Copyright 2019 Fredrik Portström <https://portstrom.com>
// This is free software distributed under the terms specified in
// the file LICENSE at the top-level directory of this distribution.

/// Site specific configuration of a wiki.
///
/// This is generated using the program [`fetch_mediawiki_configuration`](https://github.com/portstrom/fetch_mediawiki_configuration).
pub struct ConfigurationSource {
    /// Aliases of the category namespace.
    pub category_namespaces: Vec<String>,

    /// Tag names of extension tags.
    pub extension_tags: Vec<String>,

    /// Aliases of the file namespace.
    pub file_namespaces: Vec<String>,

    /// Characters that can appear in link trails.
    pub link_trail: String,

    /// Magic words that can appear between `__` and `__`.
    pub magic_words: Vec<String>,

    /// Protocols that can be used for external links.
    pub protocols: Vec<String>,

    /// Magic words that can be used for redirects.
    pub redirect_magic_words: Vec<String>,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Namespace {
    Category,
    File,
}

impl crate::Configuration {
    /// Allocates and returns a new configuration based on the given site specific configuration.
    #[must_use]
    pub fn new(source: &ConfigurationSource) -> Self {
        let mut configuration = crate::Configuration {
            character_entities: crate::Trie::new(),
            link_trail_character_set: crate::HashSet::new(),
            magic_words: crate::Trie::new(),
            namespaces: crate::Trie::new(),
            protocols: crate::Trie::new(),
            redirect_magic_words: crate::Trie::new(),
            tag_name_map: crate::HashMap::new(),
        };
        for (name, character) in crate::html_entities::HTML_ENTITIES {
            configuration
                .character_entities
                .add_case_sensitive_term(&format!("{};", name), *character);
        }
        for character in source.link_trail.chars() {
            configuration.link_trail_character_set.insert(character);
        }
        for protocol in &source.protocols {
            configuration.protocols.add_term(protocol, ());
        }
        for magic_word in &source.magic_words {
            configuration.magic_words.add_term(magic_word, ());
        }
        for namespace in &source.category_namespaces {
            configuration
                .namespaces
                .add_term(format!("{}:", namespace), Namespace::Category);
        }
        for namespace in &source.file_namespaces {
            configuration
                .namespaces
                .add_term(format!("{}:", namespace), Namespace::File);
        }
        for redirect_magic_word in &source.redirect_magic_words {
            configuration
                .redirect_magic_words
                .add_term(redirect_magic_word, ());
        }
        for tag_name in &source.extension_tags {
            configuration
                .tag_name_map
                .insert(tag_name.to_string(), crate::TagClass::ExtensionTag);
        }
        for tag_name in [
            "abbr",
            "b",
            "bdi",
            "bdo",
            "blockquote",
            "br",
            "caption",
            "center",
            "cite",
            "code",
            "data",
            "dd",
            "del",
            "dfn",
            "div",
            "dl",
            "dt",
            "em",
            "font",
            "h1",
            "h2",
            "h3",
            "h4",
            "h5",
            "h6",
            "hr",
            "i",
            "ins",
            "kbd",
            "li",
            "mark",
            "ol",
            "p",
            "pre",
            "q",
            "rb",
            "rp",
            "rt",
            "ruby",
            "s",
            "samp",
            "small",
            "span",
            "strike",
            "strong",
            "sub",
            "sup",
            "table",
            "td",
            "th",
            "time",
            "tr",
            "tt",
            "u",
            "ul",
            "var",
            "wbr",
        ]
        .iter()
        {
            configuration
                .tag_name_map
                .insert(tag_name.to_string(), crate::TagClass::Tag);
        }
        configuration
    }

    /// Parses wiki text into structured data with a user defined timeout.
    pub fn parse_with_timeout(
        &self,
        wiki_text: impl AsRef<str>,
        max_duration: std::time::Duration,
    ) -> Result<crate::Output, crate::parse::ParseError> {
        crate::parse::parse(self, wiki_text, max_duration)
    }
    /// Parses wiki text into structured data with a default timeout of 5 seconds.
    pub fn parse(
        &self,
        wiki_text: impl AsRef<str>,
    ) -> Result<crate::Output, crate::parse::ParseError> {
        crate::parse::parse(self, wiki_text, std::time::Duration::from_secs(5))
    }
    /// Parses wiki text into structured data with no time out.
    /// This function may run for extremely long lengths of time on certain articles
    pub fn parse_without_timeout(
        &self,
        wiki_text: impl AsRef<str>,
    ) -> Result<crate::Output, crate::parse::ParseError> {
        crate::parse::parse(self, wiki_text, std::time::Duration::ZERO)
    }
}

impl Default for crate::Configuration {
    /// Allocates and returns a configuration suitable for testing and quick and dirty prototyping. For correctly parsing an actual wiki, please get the correct site configuration for that particular wiki.
    fn default() -> Self {
        crate::default::create_configuration()
    }
}
