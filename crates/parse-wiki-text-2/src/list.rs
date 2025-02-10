// Copyright 2019 Fredrik Portström <https://portstrom.com>
// This is free software distributed under the terms specified in
// the file LICENSE at the top-level directory of this distribution.

pub fn parse_list_end_of_line(state: &mut crate::State) {
    let item_end_position = state.skip_whitespace_backwards(state.scan_position);
    state.flush(item_end_position);
    state.scan_position += 1;
    let mut level = 0;
    for open_node in &state.stack {
        match open_node.type_ {
            crate::OpenNodeType::Table { .. } | crate::OpenNodeType::Tag { .. } => level += 1,
            _ => break,
        }
    }
    let start_level = level;
    let mut term_level = None;
    while level < state.stack.len() {
        match (
            &state.stack[level].type_,
            state.get_byte(state.scan_position),
        ) {
            (crate::OpenNodeType::DefinitionList { .. }, Some(b':'))
            | (crate::OpenNodeType::OrderedList { .. }, Some(b'#'))
            | (crate::OpenNodeType::UnorderedList { .. }, Some(b'*')) => {
                level += 1;
                state.scan_position += 1;
            }
            (crate::OpenNodeType::DefinitionList { .. }, Some(b';')) => {
                if term_level.is_none() {
                    term_level = Some(level);
                }
                level += 1;
                state.scan_position += 1;
            }
            _ => break,
        }
    }
    if let Some(term_level) = term_level {
        if level < state.stack.len()
            || matches!(
                state.get_byte(state.scan_position),
                Some(b'#') | Some(b'*') | Some(b':') | Some(b';')
            )
        {
            state.scan_position -= level - term_level;
            level = term_level;
            state.warnings.push(crate::Warning {
                end: state.scan_position,
                message: crate::WarningMessage::DefinitionTermContinuation,
                start: state.scan_position - 1,
            });
        }
    }
    while level < state.stack.len() {
        let open_node = state.stack.pop().unwrap();
        let node = match open_node.type_ {
            crate::OpenNodeType::DefinitionList { mut items } => {
                {
                    let item_index = items.len() - 1;
                    let last_item = &mut items[item_index];
                    last_item.end = item_end_position;
                    last_item.nodes = std::mem::replace(&mut state.nodes, open_node.nodes);
                }
                crate::Node::DefinitionList {
                    end: item_end_position,
                    items,
                    start: open_node.start,
                }
            }
            crate::OpenNodeType::OrderedList { mut items } => {
                {
                    let item_index = items.len() - 1;
                    let last_item = &mut items[item_index];
                    last_item.end = item_end_position;
                    last_item.nodes = std::mem::replace(&mut state.nodes, open_node.nodes);
                }
                crate::Node::OrderedList {
                    end: item_end_position,
                    items,
                    start: open_node.start,
                }
            }
            crate::OpenNodeType::UnorderedList { mut items } => {
                {
                    let item_index = items.len() - 1;
                    let last_item = &mut items[item_index];
                    last_item.end = item_end_position;
                    last_item.nodes = std::mem::replace(&mut state.nodes, open_node.nodes);
                }
                crate::Node::UnorderedList {
                    end: item_end_position,
                    items,
                    start: open_node.start,
                }
            }
            _ => unreachable!(),
        };
        state.nodes.push(node);
    }
    state.flushed_position = state.scan_position;
    if parse_list_item_start(state) {
        while parse_list_item_start(state) {}
        skip_spaces(state);
    } else if level > start_level {
        match state.stack.get_mut(level - 1) {
            Some(crate::OpenNode {
                type_: crate::OpenNodeType::DefinitionList { items },
                ..
            }) => {
                {
                    let item_index = items.len() - 1;
                    let last_item = &mut items[item_index];
                    last_item.end = item_end_position;
                    last_item.nodes = std::mem::take(&mut state.nodes);
                }
                items.push(crate::DefinitionListItem {
                    end: 0,
                    nodes: vec![],
                    start: state.scan_position - 1,
                    type_: if state
                        .wiki_text
                        .as_bytes()
                        .get(state.scan_position - 1)
                        .cloned()
                        == Some(b';')
                    {
                        crate::DefinitionListItemType::Term
                    } else {
                        crate::DefinitionListItemType::Details
                    },
                });
            }
            Some(crate::OpenNode {
                type_: crate::OpenNodeType::OrderedList { items },
                ..
            }) => {
                {
                    let item_index = items.len() - 1;
                    let last_item = &mut items[item_index];
                    last_item.end = item_end_position;
                    last_item.nodes = std::mem::take(&mut state.nodes);
                };
                items.push(crate::ListItem {
                    end: 0,
                    nodes: vec![],
                    start: state.scan_position - 1,
                });
            }
            Some(crate::OpenNode {
                type_: crate::OpenNodeType::UnorderedList { items },
                ..
            }) => {
                {
                    let item_index = items.len() - 1;
                    let last_item = &mut items[item_index];
                    last_item.end = item_end_position;
                    last_item.nodes = std::mem::take(&mut state.nodes);
                };
                items.push(crate::ListItem {
                    end: 0,
                    nodes: vec![],
                    start: state.scan_position - 1,
                });
            }
            _ => unreachable!(),
        }
        skip_spaces(state);
    } else {
        state.skip_empty_lines();
    }
}

pub fn parse_list_item_start(state: &mut crate::State) -> bool {
    let open_node_type = match state.get_byte(state.scan_position) {
        Some(b'#') => crate::OpenNodeType::OrderedList {
            items: vec![crate::ListItem {
                end: 0,
                nodes: vec![],
                start: state.scan_position + 1,
            }],
        },
        Some(b'*') => crate::OpenNodeType::UnorderedList {
            items: vec![crate::ListItem {
                end: 0,
                nodes: vec![],
                start: state.scan_position + 1,
            }],
        },
        Some(b':') => crate::OpenNodeType::DefinitionList {
            items: vec![crate::DefinitionListItem {
                end: 0,
                nodes: vec![],
                start: state.scan_position + 1,
                type_: crate::DefinitionListItemType::Details,
            }],
        },
        Some(b';') => crate::OpenNodeType::DefinitionList {
            items: vec![crate::DefinitionListItem {
                end: 0,
                nodes: vec![],
                start: state.scan_position + 1,
                type_: crate::DefinitionListItemType::Term,
            }],
        },
        _ => return false,
    };
    let position = state.scan_position + 1;
    state.push_open_node(open_node_type, position);
    true
}

pub fn skip_spaces(state: &mut crate::State) {
    while matches!(
        state.get_byte(state.scan_position),
        Some(b'\t') | Some(b' ')
    ) {
        state.scan_position += 1;
    }
    state.flushed_position = state.scan_position;
}
