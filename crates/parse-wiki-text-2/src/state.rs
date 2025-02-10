// Copyright 2019 Fredrik Portström <https://portstrom.com>
// This is free software distributed under the terms specified in
// the file LICENSE at the top-level directory of this distribution.

pub struct OpenNode {
    pub nodes: Vec<crate::Node>,
    pub start: usize,
    pub type_: OpenNodeType,
}

pub enum OpenNodeType {
    DefinitionList {
        items: Vec<crate::DefinitionListItem>,
    },
    ExternalLink,
    Heading {
        level: u8,
    },
    Link {
        namespace: Option<crate::Namespace>,
        target: String,
    },
    OrderedList {
        items: Vec<crate::ListItem>,
    },
    Parameter {
        default: Option<Vec<crate::Node>>,
        name: Option<Vec<crate::Node>>,
    },
    Preformatted,
    Table(Table),
    Tag {
        name: String,
    },
    Template {
        name: Option<Vec<crate::Node>>,
        parameters: Vec<crate::Parameter>,
    },
    UnorderedList {
        items: Vec<crate::ListItem>,
    },
}

pub struct State {
    pub flushed_position: usize,
    pub nodes: Vec<crate::Node>,
    pub scan_position: usize,
    pub stack: Vec<OpenNode>,
    pub warnings: Vec<crate::Warning>,
    pub wiki_text: String,
}

pub struct Table {
    pub attributes: Vec<crate::Node>,
    pub before: Vec<crate::Node>,
    pub captions: Vec<crate::TableCaption>,
    pub child_element_attributes: Option<Vec<crate::Node>>,
    pub rows: Vec<crate::TableRow>,
    pub start: usize,
    pub state: TableState,
}

pub enum TableState {
    Before,
    CaptionFirstLine,
    CaptionRemainder,
    CellFirstLine,
    CellRemainder,
    HeadingFirstLine,
    HeadingRemainder,
    Row,
    TableAttributes,
}

impl State {
    pub fn flush(&mut self, end_position: usize) {
        flush(
            &mut self.nodes,
            self.flushed_position,
            end_position,
            &self.wiki_text,
        );
    }

    pub fn get_byte(&self, position: usize) -> Option<u8> {
        self.wiki_text.as_bytes().get(position).copied()
    }

    pub fn push_open_node(&mut self, type_: OpenNodeType, inner_start_position: usize) {
        let scan_position = self.scan_position;
        self.flush(scan_position);
        self.stack.push(OpenNode {
            nodes: std::mem::take(&mut self.nodes),
            start: scan_position,
            type_,
        });
        self.scan_position = inner_start_position;
        self.flushed_position = inner_start_position;
    }

    pub fn rewind(&mut self, nodes: Vec<crate::Node>, position: usize) {
        self.scan_position = position + 1;
        self.nodes = nodes;

        let last_text_node_start = match self.nodes.last() {
            Some(crate::Node::Text { start, .. }) => Some(*start),
            _ => None,
        };
        if let Some(position_before_text) = last_text_node_start {
            self.nodes.pop();
            self.flushed_position = position_before_text;
        } else {
            self.flushed_position = position;
        }
    }

    pub fn skip_empty_lines(&mut self) {
        match self.stack.last() {
            Some(OpenNode {
                type_: OpenNodeType::Table { .. },
                ..
            }) => {
                self.scan_position -= 1;
                crate::table::parse_table_end_of_line(self, false);
            }
            _ => {
                crate::line::parse_beginning_of_line(self, None);
            }
        }
    }

    pub fn skip_whitespace_backwards(&self, position: usize) -> usize {
        skip_whitespace_backwards(&self.wiki_text, position)
    }

    pub fn skip_whitespace_forwards(&self, position: usize) -> usize {
        skip_whitespace_forwards(&self.wiki_text, position)
    }
}

pub fn flush(
    nodes: &mut Vec<crate::Node>,
    flushed_position: usize,
    end_position: usize,
    wiki_text: impl AsRef<str>,
) {
    if end_position > flushed_position {
        nodes.push(crate::Node::Text {
            end: end_position,
            start: flushed_position,
            value: (&wiki_text.as_ref()[flushed_position..end_position]).into(),
        });
    }
}

pub fn skip_whitespace_backwards(wiki_text: impl AsRef<str>, position: usize) -> usize {
    let slice = wiki_text.as_ref().as_bytes().get(..position).unwrap_or(&[]);

    let non_whitespace_position = slice
        .iter()
        .rev()
        .position(|b| !matches!(b, b'\t' | b'\n' | b' '))
        .unwrap_or(slice.len());

    position - non_whitespace_position
}

pub fn skip_whitespace_forwards(wiki_text: impl AsRef<str>, position: usize) -> usize {
    let slice = wiki_text.as_ref().as_bytes().get(position..).unwrap_or(&[]);

    let non_whitespace_position = slice
        .iter()
        .position(|b| !matches!(b, b'\t' | b'\n' | b' '))
        .unwrap_or(slice.len());

    position + non_whitespace_position
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skip_whitespace_backwards() {
        assert_eq!(skip_whitespace_backwards("0\n2", 2), 1);
        assert_eq!(skip_whitespace_backwards("\n\n", 2), 0);
        assert_eq!(skip_whitespace_backwards("\n\n", 1), 0);
        assert_eq!(skip_whitespace_backwards("1\n\n", 2), 1);
        assert_eq!(skip_whitespace_backwards("\n1\n", 2), 2);
    }

    #[test]
    fn test_skip_whitespace_forwards() {
        assert_eq!(skip_whitespace_forwards("\n\n", 0), 2);
        assert_eq!(skip_whitespace_forwards("1\n\n", 0), 0);
        assert_eq!(skip_whitespace_forwards("1 \n1", 1), 3);
        assert_eq!(skip_whitespace_forwards("1 \n1", 6), 6);
        assert_eq!(skip_whitespace_forwards("1 \n1", 0), 0);
    }
}
