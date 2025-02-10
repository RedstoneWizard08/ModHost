// Copyright 2019 Fredrik Portström <https://portstrom.com>
// This is free software distributed under the terms specified in
// the file LICENSE at the top-level directory of this distribution.

pub fn parse_external_link_end(
    state: &mut crate::State,
    start_position: usize,
    nodes: Vec<crate::Node>,
) {
    let scan_position = state.scan_position;
    state.flush(scan_position);
    state.scan_position += 1;
    state.flushed_position = state.scan_position;
    let nodes = std::mem::replace(&mut state.nodes, nodes);
    state.nodes.push(crate::Node::ExternalLink {
        end: state.scan_position,
        nodes,
        start: start_position,
    });
}

pub fn parse_external_link_end_of_line(state: &mut crate::State) {
    let end = state.scan_position;
    let open_node = state.stack.pop().unwrap();
    state.warnings.push(crate::Warning {
        end,
        message: crate::WarningMessage::InvalidLinkSyntax,
        start: open_node.start,
    });
    state.rewind(open_node.nodes, open_node.start);
}

pub fn parse_external_link_start(state: &mut crate::State, configuration: &crate::Configuration) {
    let scheme_start_position = state.scan_position + 1;
    match configuration
        .protocols
        .find(&state.wiki_text[scheme_start_position..])
    {
        Err(_) => {
            state.scan_position = scheme_start_position;
        }
        Ok(_) => {
            state.push_open_node(crate::OpenNodeType::ExternalLink, scheme_start_position);
        }
    }
}
