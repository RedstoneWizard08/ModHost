// Copyright 2019 Fredrik Portström <https://portstrom.com>
// This is free software distributed under the terms specified in
// the file LICENSE at the top-level directory of this distribution.

pub fn parse_heading_end(state: &mut crate::State) {
    let mut end_position = state.scan_position;
    while let Some(byte) = state.get_byte(end_position - 1) {
        if byte == b'\t' || byte == b' ' {
            end_position -= 1;
        } else {
            break;
        }
    }
    let open_node = state.stack.pop().unwrap();
    if state.get_byte(end_position - 1) != Some(b'=') || end_position < open_node.start + 3 {
        state.warnings.push(crate::Warning {
            end: end_position,
            message: crate::WarningMessage::InvalidHeadingSyntaxRewinding,
            start: open_node.start,
        });
        state.rewind(open_node.nodes, open_node.start);
        return;
    }
    let start_level = match open_node.type_ {
        crate::OpenNodeType::Heading { level } => level,
        _ => unreachable!(),
    };
    let mut end_level: u8 = 1;
    while end_level < start_level
        && end_position - end_level as usize > open_node.start + end_level as usize + 2
        && state.get_byte(end_position - end_level as usize - 1) == Some(b'=')
    {
        end_level += 1;
    }
    let position = state.skip_whitespace_backwards(end_position - end_level as usize);
    if end_level < start_level {
        state.warnings.push(crate::Warning {
            end: end_position,
            message: crate::WarningMessage::UnexpectedHeadingLevelCorrecting,
            start: open_node.start,
        });
        let inner_start_position = open_node.start + end_level as usize;
        if match state.nodes.get_mut(0) {
            None => {
                state.flushed_position = inner_start_position;
                false
            }
            Some(crate::Node::Text { end, start, value }) => {
                *start = inner_start_position;
                *value = (&state.wiki_text[inner_start_position..*end]).into();
                false
            }
            Some(_) => true,
        } {
            let end = state.skip_whitespace_forwards(open_node.start + start_level as usize);
            state.nodes.insert(
                0,
                crate::Node::Text {
                    end,
                    start: inner_start_position,
                    value: (&state.wiki_text[inner_start_position..end]).into(),
                },
            );
        }
    }
    state.flush(position);
    let nodes = std::mem::replace(&mut state.nodes, open_node.nodes);
    state.nodes.push(crate::Node::Heading {
        end: end_position,
        level: end_level,
        nodes,
        start: open_node.start,
    });
    state.scan_position += 1;
    state.skip_empty_lines();
}

pub fn parse_heading_start(state: &mut crate::State) {
    let mut level = 1;
    while state.get_byte(state.scan_position + level) == Some(b'=') && level < 6 {
        level += 1;
    }
    let position = state.skip_whitespace_forwards(state.scan_position + level);
    state.flushed_position = position;
    state.push_open_node(
        crate::OpenNodeType::Heading { level: level as u8 },
        position,
    );
}
