// Copyright 2019 Fredrik Portström <https://portstrom.com>
// This is free software distributed under the terms specified in
// the file LICENSE at the top-level directory of this distribution.

pub fn parse_parameter_name_end(state: &mut crate::State) {
    let stack_length = state.stack.len();
    if stack_length > 0 {
        if let crate::OpenNode {
            type_:
                crate::OpenNodeType::Template {
                    name: Some(_),
                    parameters,
                },
            ..
        } = &mut state.stack[stack_length - 1]
        {
            let parameters_length = parameters.len();
            let name = &mut parameters[parameters_length - 1].name;
            if name.is_none() {
                crate::state::flush(
                    &mut state.nodes,
                    state.flushed_position,
                    crate::state::skip_whitespace_backwards(&state.wiki_text, state.scan_position),
                    &state.wiki_text,
                );
                state.flushed_position = crate::state::skip_whitespace_forwards(
                    &state.wiki_text,
                    state.scan_position + 1,
                );
                state.scan_position = state.flushed_position;
                *name = Some(std::mem::take(&mut state.nodes));
                return;
            }
        }
    }
    state.scan_position += 1;
}

pub fn parse_parameter_separator(state: &mut crate::State) {
    match state.stack.last_mut() {
        Some(crate::OpenNode {
            type_: crate::OpenNodeType::Parameter { default, name },
            ..
        }) => {
            if name.is_none() {
                let position =
                    crate::state::skip_whitespace_backwards(&state.wiki_text, state.scan_position);
                crate::state::flush(
                    &mut state.nodes,
                    state.flushed_position,
                    position,
                    &state.wiki_text,
                );
                *name = Some(std::mem::take(&mut state.nodes));
            } else {
                crate::state::flush(
                    &mut state.nodes,
                    state.flushed_position,
                    state.scan_position,
                    &state.wiki_text,
                );
                *default = Some(std::mem::take(&mut state.nodes));
                state.warnings.push(crate::Warning {
                    end: state.scan_position + 1,
                    message: crate::WarningMessage::UselessTextInParameter,
                    start: state.scan_position,
                });
            }
            state.scan_position += 1;
            state.flushed_position = state.scan_position;
        }
        _ => unreachable!(),
    }
}

pub fn parse_template_end(state: &mut crate::State) {
    match state.stack.last() {
        Some(crate::OpenNode {
            type_: crate::OpenNodeType::Parameter { .. },
            ..
        }) => match state.stack.pop() {
            Some(crate::OpenNode {
                nodes,
                start,
                type_: crate::OpenNodeType::Parameter { default, name },
            }) => {
                if state.get_byte(state.scan_position + 2) == Some(b'}') {
                    if let Some(name) = name {
                        let start_position = state.scan_position;
                        state.flush(start_position);
                        let nodes = std::mem::replace(&mut state.nodes, nodes);
                        state.nodes.push(crate::Node::Parameter {
                            default: Some(default.unwrap_or(nodes)),
                            end: state.scan_position,
                            name,
                            start,
                        });
                    } else {
                        let start_position = state.skip_whitespace_backwards(state.scan_position);
                        state.flush(start_position);
                        let nodes = std::mem::replace(&mut state.nodes, nodes);
                        state.nodes.push(crate::Node::Parameter {
                            default: None,
                            end: state.scan_position,
                            name: nodes,
                            start,
                        });
                    }
                    state.scan_position += 3;
                    state.flushed_position = state.scan_position;
                } else {
                    state.warnings.push(crate::Warning {
                        end: state.scan_position + 2,
                        message: crate::WarningMessage::UnexpectedEndTagRewinding,
                        start: state.scan_position,
                    });
                    state.rewind(nodes, start);
                }
            }
            _ => unreachable!(),
        },
        Some(crate::OpenNode {
            type_: crate::OpenNodeType::Template { .. },
            ..
        }) => match state.stack.pop() {
            Some(crate::OpenNode {
                nodes,
                start,
                type_:
                    crate::OpenNodeType::Template {
                        name,
                        mut parameters,
                    },
            }) => {
                let position = state.skip_whitespace_backwards(state.scan_position);
                state.flush(position);
                state.scan_position += 2;
                state.flushed_position = state.scan_position;
                let name = match name {
                    None => std::mem::replace(&mut state.nodes, nodes),
                    Some(name) => {
                        let parameters_length = parameters.len();
                        let parameter = &mut parameters[parameters_length - 1];
                        parameter.end = position;
                        parameter.value = std::mem::replace(&mut state.nodes, nodes);
                        name
                    }
                };
                state.nodes.push(crate::Node::Template {
                    end: state.scan_position,
                    name,
                    parameters,
                    start,
                });
            }
            _ => unreachable!(),
        },
        _ => {
            if state
                .stack
                .iter()
                .rev()
                .skip(1)
                .any(|item| match item.type_ {
                    crate::OpenNodeType::Parameter { .. } => {
                        state.get_byte(state.scan_position + 2) == Some(b'}')
                    }
                    crate::OpenNodeType::Template { .. } => true,
                    _ => false,
                })
            {
                state.warnings.push(crate::Warning {
                    end: state.scan_position + 2,
                    message: crate::WarningMessage::UnexpectedEndTagRewinding,
                    start: state.scan_position,
                });
                let open_node = state.stack.pop().unwrap();
                state.rewind(open_node.nodes, open_node.start);
            } else {
                state.warnings.push(crate::Warning {
                    end: state.scan_position + 2,
                    message: crate::WarningMessage::UnexpectedEndTag,
                    start: state.scan_position,
                });
                state.scan_position += 2;
            }
        }
    }
}

pub fn parse_template_separator(state: &mut crate::State) {
    match state.stack.last_mut() {
        Some(crate::OpenNode {
            type_: crate::OpenNodeType::Template { name, parameters },
            ..
        }) => {
            let position =
                crate::state::skip_whitespace_backwards(&state.wiki_text, state.scan_position);
            crate::state::flush(
                &mut state.nodes,
                state.flushed_position,
                position,
                &state.wiki_text,
            );
            state.flushed_position =
                crate::state::skip_whitespace_forwards(&state.wiki_text, state.scan_position + 1);
            state.scan_position = state.flushed_position;
            if name.is_none() {
                *name = Some(std::mem::take(&mut state.nodes));
            } else {
                let parameters_length = parameters.len();
                let parameter = &mut parameters[parameters_length - 1];
                parameter.end = position;
                parameter.value = std::mem::take(&mut state.nodes);
            }
            parameters.push(crate::Parameter {
                end: 0,
                name: None,
                start: state.scan_position,
                value: vec![],
            });
        }
        _ => unreachable!(),
    }
}

/// expects scan_position and scan_position + 1 to be a curly brace
pub fn parse_template_start(state: &mut crate::State) {
    let scan_position = state.scan_position;
    debug_assert_eq!(state.get_byte(scan_position), Some(b'{'));
    debug_assert_eq!(state.get_byte(scan_position + 1), Some(b'{'));

    // if the template has three braces it is a parameter
    if state.get_byte(state.scan_position + 2) == Some(b'{') {
        let position = state.skip_whitespace_forwards(scan_position + 3);
        state.push_open_node(
            crate::OpenNodeType::Parameter {
                default: None,
                name: None,
            },
            position,
        );
    } else {
        let position = state.skip_whitespace_forwards(scan_position + 2);
        state.push_open_node(
            crate::OpenNodeType::Template {
                name: None,
                parameters: vec![],
            },
            position,
        );
    }
}
