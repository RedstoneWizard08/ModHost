// Copyright 2019 Fredrik Portström <https://portstrom.com>
// This is free software distributed under the terms specified in
// the file LICENSE at the top-level directory of this distribution.

pub fn parse_beginning_of_line(state: &mut crate::State, line_start_position: Option<usize>) {
    let mut has_line_break = false;
    'a: loop {
        match state.get_byte(state.scan_position) {
            None => {
                if line_start_position.is_none() {
                    state.flushed_position = state.scan_position;
                }
                return;
            }
            Some(b'\t') => {
                state.scan_position += 1;
                loop {
                    match state.get_byte(state.scan_position) {
                        None | Some(b'\n') => continue 'a,
                        Some(b'\t') | Some(b' ') => state.scan_position += 1,
                        Some(_) => break 'a,
                    }
                }
            }
            Some(b'\n') => {
                if has_line_break {
                    state.warnings.push(crate::Warning {
                        end: state.scan_position + 1,
                        message: crate::WarningMessage::RepeatedEmptyLine,
                        start: state.scan_position,
                    });
                }
                has_line_break = true;
                state.scan_position += 1;
            }
            Some(b' ') => {
                state.scan_position += 1;
                let start_position = state.scan_position;
                loop {
                    match state.get_byte(state.scan_position) {
                        None => return,
                        Some(b'\n') => break,
                        Some(b'\t') | Some(b' ') => state.scan_position += 1,
                        Some(b'{') if state.get_byte(state.scan_position + 1) == Some(b'|') => {
                            crate::table::start_table(state, line_start_position);
                            return;
                        }
                        Some(_) => {
                            if let Some(position) = line_start_position {
                                let position = state.skip_whitespace_backwards(position);
                                state.flush(position);
                            }
                            state.flushed_position = state.scan_position;
                            state.push_open_node(crate::OpenNodeType::Preformatted, start_position);
                            return;
                        }
                    }
                }
            }
            Some(b'#') | Some(b'*') | Some(b':') | Some(b';') => {
                if let Some(position) = line_start_position {
                    let position = state.skip_whitespace_backwards(position);
                    state.flush(position);
                }
                state.flushed_position = state.scan_position;
                while crate::list::parse_list_item_start(state) {}
                crate::list::skip_spaces(state);
                return;
            }
            Some(b'-') => {
                if state.get_byte(state.scan_position + 1) == Some(b'-')
                    && state.get_byte(state.scan_position + 2) == Some(b'-')
                    && state.get_byte(state.scan_position + 3) == Some(b'-')
                {
                    if let Some(position) = line_start_position {
                        let position = state.skip_whitespace_backwards(position);
                        state.flush(position);
                    }
                    let start = state.scan_position;
                    state.scan_position += 4;
                    while state.get_byte(state.scan_position) == Some(b'-') {
                        state.scan_position += 1;
                    }
                    state.nodes.push(crate::Node::HorizontalDivider {
                        end: state.scan_position,
                        start,
                    });
                    while let Some(character) = state.get_byte(state.scan_position) {
                        match character {
                            b'\t' | b' ' => state.scan_position += 1,
                            b'\n' => {
                                state.scan_position += 1;
                                state.skip_empty_lines();
                                break;
                            }
                            _ => break,
                        }
                    }
                    state.flushed_position = state.scan_position;
                    return;
                }
                break;
            }
            Some(b'=') => {
                if let Some(position) = line_start_position {
                    let position = state.skip_whitespace_backwards(position);
                    state.flush(position);
                }
                crate::heading::parse_heading_start(state);
                return;
            }
            Some(b'{') => {
                if state.get_byte(state.scan_position + 1) == Some(b'|') {
                    crate::table::start_table(state, line_start_position);
                    return;
                }
                break;
            }
            Some(_) => break,
        }
    }
    match line_start_position {
        None => state.flushed_position = state.scan_position,
        Some(position) => {
            if has_line_break {
                let flush_position = state.skip_whitespace_backwards(position);
                state.flush(flush_position);
                state.nodes.push(crate::Node::ParagraphBreak {
                    end: state.scan_position,
                    start: position,
                });
                state.flushed_position = state.scan_position;
            }
        }
    }
}

pub fn parse_end_of_line(state: &mut crate::State) {
    match state.stack.last() {
        None => {
            let position = state.scan_position;
            state.scan_position += 1;
            parse_beginning_of_line(state, Some(position));
        }
        Some(crate::OpenNode {
            type_: crate::OpenNodeType::DefinitionList { .. },
            ..
        })
        | Some(crate::OpenNode {
            type_: crate::OpenNodeType::OrderedList { .. },
            ..
        })
        | Some(crate::OpenNode {
            type_: crate::OpenNodeType::UnorderedList { .. },
            ..
        }) => {
            crate::list::parse_list_end_of_line(state);
        }
        Some(crate::OpenNode {
            type_: crate::OpenNodeType::ExternalLink,
            ..
        }) => {
            crate::external_link::parse_external_link_end_of_line(state);
        }
        Some(crate::OpenNode {
            type_: crate::OpenNodeType::Heading { .. },
            ..
        }) => {
            crate::heading::parse_heading_end(state);
        }
        Some(crate::OpenNode {
            type_: crate::OpenNodeType::Link { .. },
            ..
        })
        | Some(crate::OpenNode {
            type_: crate::OpenNodeType::Parameter { .. },
            ..
        })
        | Some(crate::OpenNode {
            type_: crate::OpenNodeType::Tag { .. },
            ..
        })
        | Some(crate::OpenNode {
            type_: crate::OpenNodeType::Template { .. },
            ..
        }) => {
            state.scan_position += 1;
        }
        Some(crate::OpenNode {
            type_: crate::OpenNodeType::Preformatted,
            ..
        }) => {
            parse_preformatted_end_of_line(state);
        }
        Some(crate::OpenNode {
            type_: crate::OpenNodeType::Table { .. },
            ..
        }) => {
            crate::table::parse_table_end_of_line(state, true);
        }
    }
}

fn parse_preformatted_end_of_line(state: &mut crate::State) {
    if state.get_byte(state.scan_position + 1) == Some(b' ') {
        let mut position = state.scan_position + 2;
        loop {
            match state.get_byte(position) {
                None => break,
                Some(b'\t') | Some(b' ') => position += 1,
                Some(b'{') if state.get_byte(position + 1) == Some(b'|') => {
                    break;
                }
                Some(b'|')
                    if state.get_byte(position + 1) == Some(b'}')
                        && state.stack.len() > 1
                        && matches!(
                            state.stack.get(state.stack.len() - 2),
                            Some(crate::OpenNode {
                                type_: crate::OpenNodeType::Table { .. },
                                ..
                            })
                        ) =>
                {
                    break;
                }
                Some(_) => {
                    let position = state.scan_position + 1;
                    state.flush(position);
                    state.scan_position += 2;
                    state.flushed_position = state.scan_position;
                    return;
                }
            }
        }
    }
    let open_node = state.stack.pop().unwrap();
    let position = state.skip_whitespace_backwards(state.scan_position);
    state.flush(position);
    state.scan_position += 1;
    let nodes = std::mem::replace(&mut state.nodes, open_node.nodes);
    state.nodes.push(crate::Node::Preformatted {
        end: state.scan_position,
        nodes,
        start: open_node.start,
    });
    state.skip_empty_lines();
}
