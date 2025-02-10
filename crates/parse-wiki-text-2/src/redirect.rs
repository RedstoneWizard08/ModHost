// Copyright 2019 Fredrik Portström <https://portstrom.com>
// This is free software distributed under the terms specified in
// the file LICENSE at the top-level directory of this distribution.

pub fn parse_redirect(
    state: &mut crate::State,
    configuration: &crate::Configuration,
    start_position: usize,
) {
    let mut position = match configuration
        .redirect_magic_words
        .find(&state.wiki_text[start_position + 1..])
    {
        Err(_) => return,
        Ok((match_length, _)) => match_length + start_position + 1,
    };
    loop {
        match state.get_byte(position) {
            Some(b'\t') | Some(b'\n') | Some(b' ') => position += 1,
            Some(b':') => {
                position += 1;
                loop {
                    match state.get_byte(position) {
                        Some(b'\t') | Some(b'\n') | Some(b' ') => position += 1,
                        Some(b'[') => break,
                        _ => return,
                    }
                }
                break;
            }
            Some(b'[') => break,
            _ => return,
        }
    }
    if state.get_byte(position + 1) != Some(b'[') {
        return;
    }
    position += 2;
    let target_end_position;
    let target_start_position = position;
    loop {
        match state.get_byte(position) {
            None | Some(b'\n') | Some(b'[') | Some(b'{') | Some(b'}') => return,
            Some(b']') => {
                target_end_position = position;
                break;
            }
            Some(b'|') => {
                state.warnings.push(crate::Warning {
                    end: position + 1,
                    message: crate::WarningMessage::UselessTextInRedirect,
                    start: position,
                });
                target_end_position = position;
                position += 1;
                loop {
                    match state.get_byte(position) {
                        None | Some(b'\n') => return,
                        Some(b']') => break,
                        Some(_) => position += 1,
                    }
                }
                break;
            }
            Some(_) => position += 1,
        }
    }
    if state.get_byte(position + 1) == Some(b']') {
        position += 2;
        state.nodes.push(crate::Node::Redirect {
            end: position,
            start: start_position,
            target: (&state.wiki_text[target_start_position..target_end_position]).into(),
        });
        state.flushed_position = state.skip_whitespace_forwards(position);
        state.scan_position = state.flushed_position;
        if state.wiki_text.len() > position {
            state.warnings.push(crate::Warning {
                end: state.wiki_text.len(),
                message: crate::WarningMessage::TextAfterRedirect,
                start: start_position,
            });
        }
    }
}
