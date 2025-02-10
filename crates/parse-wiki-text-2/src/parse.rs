// Copyright 2019 Fredrik Portström <https://portstrom.com>
// This is free software distributed under the terms specified in
// the file LICENSE at the top-level directory of this distribution.

use thiserror::Error;

/// Get's returned if the parsing received an unrecoverable error.
///
/// At the moment, the only error that can occur is a timeout.
#[derive(Debug, Error)]
pub enum ParseError {
    /// The parsing took too long and was aborted.
    #[error("The parsing took too long and was aborted.")]
    TimedOut {
        /// The time the parsing took before it was aborted.
        execution_time: std::time::Duration,
        /// The output that was generated before the parsing was aborted.
        output: crate::Output,
    },
}

pub fn parse(
    configuration: &crate::Configuration,
    wiki_text: impl AsRef<str>,
    max_duration: std::time::Duration,
) -> Result<crate::Output, ParseError> {
    let wiki_text = wiki_text.as_ref().to_string();

    let mut state = crate::State {
        flushed_position: 0,
        nodes: vec![],
        scan_position: 0,
        stack: vec![],
        warnings: vec![],
        wiki_text: wiki_text.clone(),
    };
    // parse redirects and warn about repeated newlines
    {
        let mut has_line_break = false;
        let mut position = 0;
        loop {
            match state.get_byte(position) {
                Some(b'\n') => {
                    if has_line_break {
                        state.warnings.push(crate::Warning {
                            end: position + 1,
                            message: crate::WarningMessage::RepeatedEmptyLine,
                            start: position,
                        });
                    }
                    has_line_break = true;
                    position += 1;
                    state.flushed_position = position;
                    state.scan_position = position;
                }
                Some(b' ') => position += 1,
                Some(b'#') => {
                    crate::redirect::parse_redirect(&mut state, configuration, position);
                    break;
                }
                _ => break,
            }
        }
    }
    let mut loop_counter = 0;
    let start_time = std::time::Instant::now();

    crate::line::parse_beginning_of_line(&mut state, None);
    loop {
        match state.get_byte(state.scan_position) {
            None => {
                crate::line::parse_end_of_line(&mut state);
                if state.scan_position < state.wiki_text.len() {
                    continue;
                }

                // rewind until we find a text node
                if let Some(crate::OpenNode { nodes, start, .. }) = state.stack.pop() {
                    state.warnings.push(crate::Warning {
                        end: state.scan_position,
                        message: crate::WarningMessage::MissingEndTagRewinding,
                        start,
                    });
                    state.rewind(nodes, start);
                } else {
                    break;
                }
            }
            // invalid characters
            Some(0..=8 | 11..=31 | 127) => {
                state.warnings.push(crate::Warning {
                    end: state.scan_position + 1,
                    message: crate::WarningMessage::InvalidCharacter,
                    start: state.scan_position,
                });
                state.scan_position += 1;
            }
            Some(b'\n') => {
                crate::line::parse_end_of_line(&mut state);
            }
            Some(b'!')
                if state.get_byte(state.scan_position + 1) == Some(b'!')
                    && matches!(
                        state.stack.last(),
                        Some(crate::OpenNode {
                            type_: crate::OpenNodeType::Table(..),
                            ..
                        })
                    ) =>
            {
                crate::table::parse_heading_cell(&mut state);
            }
            Some(b'&') => {
                crate::character_entity::parse_character_entity(&mut state, configuration)
            }
            Some(b'\'') => {
                if state.get_byte(state.scan_position + 1) == Some(b'\'') {
                    crate::bold_italic::parse_bold_italic(&mut state);
                } else {
                    state.scan_position += 1;
                }
            }
            Some(b'<') => match state.get_byte(state.scan_position + 1) {
                Some(b'!')
                    if state.get_byte(state.scan_position + 2) == Some(b'-')
                        && state.get_byte(state.scan_position + 3) == Some(b'-') =>
                {
                    crate::comment::parse_comment(&mut state)
                }
                Some(b'/') => crate::tag::parse_end_tag(&mut state, configuration),
                _ => crate::tag::parse_start_tag(&mut state, configuration),
            },
            Some(b'=') => {
                crate::template::parse_parameter_name_end(&mut state);
            }
            Some(b'[') => {
                if state.get_byte(state.scan_position + 1) == Some(b'[') {
                    crate::link::parse_link_start(&mut state, configuration);
                } else {
                    crate::external_link::parse_external_link_start(&mut state, configuration);
                }
            }
            Some(b']') => match state.stack.pop() {
                None => state.scan_position += 1,
                Some(crate::OpenNode {
                    nodes,
                    start,
                    type_: crate::OpenNodeType::ExternalLink,
                }) => {
                    crate::external_link::parse_external_link_end(&mut state, start, nodes);
                }
                Some(crate::OpenNode {
                    nodes,
                    start,
                    type_: crate::OpenNodeType::Link { namespace, target },
                }) => {
                    if state.get_byte(state.scan_position + 1) == Some(b']') {
                        crate::link::parse_link_end(
                            &mut state,
                            configuration,
                            start,
                            nodes,
                            namespace,
                            target,
                        );
                    } else {
                        state.scan_position += 1;
                        state.stack.push(crate::OpenNode {
                            nodes,
                            start,
                            type_: crate::OpenNodeType::Link { namespace, target },
                        });
                    }
                }
                Some(open_node) => {
                    state.scan_position += 1;
                    state.stack.push(open_node);
                }
            },
            Some(b'_') => {
                if state.get_byte(state.scan_position + 1) == Some(b'_') {
                    crate::magic_word::parse_magic_word(&mut state, configuration);
                } else {
                    state.scan_position += 1;
                }
            }
            Some(b'{') => {
                if state.get_byte(state.scan_position + 1) == Some(b'{') {
                    crate::template::parse_template_start(&mut state);
                } else {
                    state.scan_position += 1;
                }
            }
            Some(b'|') => match state.stack.last_mut() {
                Some(crate::OpenNode {
                    type_: crate::OpenNodeType::Parameter { default: None, .. },
                    ..
                }) => {
                    crate::template::parse_parameter_separator(&mut state);
                }
                Some(crate::OpenNode {
                    type_: crate::OpenNodeType::Table(..),
                    ..
                }) => {
                    crate::table::parse_inline_token(&mut state);
                }
                Some(crate::OpenNode {
                    type_: crate::OpenNodeType::Template { .. },
                    ..
                }) => {
                    crate::template::parse_template_separator(&mut state);
                }
                _ => state.scan_position += 1,
            },
            Some(b'}') => {
                if state.get_byte(state.scan_position + 1) == Some(b'}') {
                    crate::template::parse_template_end(&mut state);
                } else {
                    state.scan_position += 1;
                }
            }
            _ => {
                state.scan_position += 1;
            }
        }

        if !max_duration.is_zero() && loop_counter == 10_000 {
            loop_counter = 0;
            if start_time.elapsed() > max_duration {
                state.flush(state.scan_position);

                return Err(ParseError::TimedOut {
                    execution_time: start_time.elapsed(),
                    output: crate::Output {
                        nodes: state.nodes,
                        warnings: state.warnings,
                    },
                });
            }
        }

        loop_counter += 1;
    }

    let end_position = state.skip_whitespace_backwards(wiki_text.len());
    state.flush(end_position);
    Ok(crate::Output {
        nodes: state.nodes,
        warnings: state.warnings,
    })
}
