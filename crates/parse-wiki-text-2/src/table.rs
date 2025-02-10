// Copyright 2019 Fredrik Portström <https://portstrom.com>
// This is free software distributed under the terms specified in
// the file LICENSE at the top-level directory of this distribution.

use crate::state::TableState;

pub fn parse_heading_cell(state: &mut crate::State) {
    let table = get_table(&mut state.stack);
    let position_before_token = state.scan_position;
    if let crate::state::TableState::HeadingFirstLine = table.state {
        let end = crate::state::skip_whitespace_backwards(&state.wiki_text, position_before_token);
        crate::state::flush(
            &mut state.nodes,
            state.flushed_position,
            end,
            &state.wiki_text,
        );
        if table.rows.is_empty() {
            table.rows.push(crate::TableRow {
                attributes: vec![],
                cells: vec![],
                end,
                start: table.start,
            });
        }
        let row = table.rows.last_mut().unwrap();
        row.cells.push(crate::TableCell {
            attributes: table.child_element_attributes.take(),
            content: std::mem::take(&mut state.nodes),
            end,
            start: table.start,
            type_: crate::TableCellType::Heading,
        });
        row.end = end;
        table.start = position_before_token;
        state.scan_position = position_before_token + 2;
        while let Some(character) = state.wiki_text.as_bytes().get(state.scan_position) {
            match character {
                b'\t' | b' ' => state.scan_position += 1,
                _ => break,
            }
        }
        state.flushed_position = state.scan_position;
    } else {
        state.scan_position += 2;
    }
}

pub fn parse_table_end_of_line(state: &mut crate::State, paragraph_break_possible: bool) {
    let position_before_line_break = state.scan_position;
    let mut position_after_line_break = position_before_line_break + 1;
    let mut scan_position = position_after_line_break;
    loop {
        match state.get_byte(scan_position) {
            Some(b'\n') => {
                scan_position += 1;
                position_after_line_break = scan_position;
            }
            Some(b'\t') | Some(b' ') => scan_position += 1,
            Some(b'!') => {
                change_state(
                    state,
                    TableState::HeadingFirstLine,
                    position_before_line_break,
                    scan_position,
                    scan_position + 1,
                    paragraph_break_possible,
                );
                break;
            }
            Some(b'|') => {
                match state.get_byte(scan_position + 1) {
                    Some(b'+') => change_state(
                        state,
                        TableState::CaptionFirstLine,
                        position_before_line_break,
                        scan_position,
                        scan_position + 2,
                        paragraph_break_possible,
                    ),
                    Some(b'-') => change_state(
                        state,
                        TableState::Row,
                        position_before_line_break,
                        scan_position,
                        scan_position + 2,
                        paragraph_break_possible,
                    ),
                    Some(b'}') => parse_end(
                        state,
                        position_before_line_break,
                        scan_position + 2,
                        paragraph_break_possible,
                    ),
                    _ => change_state(
                        state,
                        TableState::CellFirstLine,
                        position_before_line_break,
                        scan_position,
                        scan_position + 1,
                        paragraph_break_possible,
                    ),
                }
                break;
            }
            _ => {
                parse_line_break(
                    state,
                    position_before_line_break,
                    position_after_line_break,
                    scan_position,
                    paragraph_break_possible,
                );
                break;
            }
        }
    }
}

fn change_state(
    state: &mut crate::State,
    target_table_state: TableState,
    position_before_line_break: usize,
    position_before_token: usize,
    mut position_after_token: usize,
    paragraph_break_possible: bool,
) {
    while let Some(character) = state.get_byte(position_after_token) {
        match character {
            b'\t' | b' ' => position_after_token += 1,
            _ => break,
        }
    }
    let table = get_table(&mut state.stack);
    let end = crate::state::skip_whitespace_backwards(&state.wiki_text, position_before_line_break);
    if paragraph_break_possible {
        crate::state::flush(
            &mut state.nodes,
            state.flushed_position,
            end,
            &state.wiki_text,
        );
    }
    match table.state {
        TableState::Before => {
            state.warnings.push(crate::Warning {
                end: position_before_line_break,
                message: crate::WarningMessage::StrayTextInTable,
                start: table.start,
            });
            table.before.append(&mut std::mem::take(&mut state.nodes));
        }
        TableState::CaptionFirstLine | TableState::CaptionRemainder => {
            table.captions.push(crate::TableCaption {
                attributes: table.child_element_attributes.take(),
                content: std::mem::take(&mut state.nodes),
                end,
                start: table.start,
            });
        }
        TableState::CellFirstLine | TableState::CellRemainder => {
            if table.rows.is_empty() {
                table.rows.push(crate::TableRow {
                    attributes: vec![],
                    cells: vec![],
                    end,
                    start: table.start,
                });
            }
            let row = table.rows.last_mut().unwrap();
            row.cells.push(crate::TableCell {
                attributes: table.child_element_attributes.take(),
                content: std::mem::take(&mut state.nodes),
                end,
                start: table.start,
                type_: crate::TableCellType::Ordinary,
            });
            row.end = end;
        }
        TableState::HeadingFirstLine | TableState::HeadingRemainder => {
            if table.rows.is_empty() {
                table.rows.push(crate::TableRow {
                    attributes: vec![],
                    cells: vec![],
                    end,
                    start: table.start,
                });
            }
            let row = table.rows.last_mut().unwrap();
            row.cells.push(crate::TableCell {
                attributes: table.child_element_attributes.take(),
                content: std::mem::take(&mut state.nodes),
                end,
                start: table.start,
                type_: crate::TableCellType::Heading,
            });
            row.end = position_before_line_break;
        }
        TableState::Row => {
            table.rows.push(crate::TableRow {
                attributes: std::mem::take(&mut state.nodes),
                cells: vec![],
                end,
                start: table.start,
            });
        }
        TableState::TableAttributes => {
            table.attributes = std::mem::take(&mut state.nodes);
        }
    }
    table.start = position_before_token;
    table.state = target_table_state;
    state.flushed_position = position_after_token;
    state.scan_position = position_after_token;
}

fn parse_end(
    state: &mut crate::State,
    position_before_line_break: usize,
    position_after_token: usize,
    paragraph_break_possible: bool,
) {
    let open_node = state.stack.pop().unwrap();
    match open_node.type_ {
        crate::OpenNodeType::Table(crate::state::Table {
            mut attributes,
            mut before,
            mut captions,
            mut child_element_attributes,
            mut rows,
            start,
            state: table_state,
        }) => {
            if paragraph_break_possible {
                state.flush(crate::state::skip_whitespace_backwards(
                    &state.wiki_text,
                    position_before_line_break,
                ));
            }
            match table_state {
                TableState::Before => {
                    state.warnings.push(crate::Warning {
                        end: position_before_line_break,
                        message: crate::WarningMessage::StrayTextInTable,
                        start,
                    });
                    before.append(&mut std::mem::replace(&mut state.nodes, open_node.nodes));
                }
                TableState::CaptionFirstLine | TableState::CaptionRemainder => {
                    captions.push(crate::TableCaption {
                        attributes: child_element_attributes.take(),
                        content: std::mem::replace(&mut state.nodes, open_node.nodes),
                        end: position_before_line_break,
                        start,
                    });
                }
                TableState::CellFirstLine | TableState::CellRemainder => {
                    if rows.is_empty() {
                        rows.push(crate::TableRow {
                            attributes: vec![],
                            cells: vec![],
                            end: 0,
                            start,
                        });
                    }
                    let row = rows.last_mut().unwrap();
                    row.cells.push(crate::TableCell {
                        attributes: child_element_attributes.take(),
                        content: std::mem::replace(&mut state.nodes, open_node.nodes),
                        end: position_before_line_break,
                        start,
                        type_: crate::TableCellType::Ordinary,
                    });
                    row.end = position_before_line_break;
                }
                TableState::HeadingFirstLine | TableState::HeadingRemainder => {
                    if rows.is_empty() {
                        rows.push(crate::TableRow {
                            attributes: vec![],
                            cells: vec![],
                            end: 0,
                            start,
                        });
                    }
                    let row = rows.last_mut().unwrap();
                    row.cells.push(crate::TableCell {
                        attributes: child_element_attributes.take(),
                        content: std::mem::replace(&mut state.nodes, open_node.nodes),
                        end: position_before_line_break,
                        start,
                        type_: crate::TableCellType::Heading,
                    });
                    row.end = position_before_line_break;
                }
                TableState::Row => {
                    rows.push(crate::TableRow {
                        attributes: std::mem::replace(&mut state.nodes, open_node.nodes),
                        cells: vec![],
                        end: position_before_line_break,
                        start,
                    });
                }
                TableState::TableAttributes => {
                    attributes = std::mem::replace(&mut state.nodes, open_node.nodes);
                }
            }
            state.scan_position = position_after_token;
            state.nodes.append(&mut before);
            state.nodes.push(crate::Node::Table {
                attributes,
                captions,
                end: state.scan_position,
                rows,
                start: open_node.start,
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
        }
        _ => unreachable!(),
    }
}

fn parse_line_break(
    state: &mut crate::State,
    position_before_line_break: usize,
    position_after_line_break: usize,
    position_after_token: usize,
    paragraph_break_possible: bool,
) {
    {
        let table = get_table(&mut state.stack);
        match table.state {
            TableState::Before | TableState::CaptionRemainder => {
                state.scan_position = position_after_token
            }
            TableState::CaptionFirstLine => {
                table.state = TableState::CaptionRemainder;
                if state.nodes.is_empty() && state.flushed_position == position_before_line_break {
                    state.flushed_position = position_after_token;
                }
                state.scan_position = position_after_token;
                if position_after_token != position_after_line_break {
                    return;
                }
            }
            TableState::CellFirstLine => {
                crate::state::flush(
                    &mut state.nodes,
                    state.flushed_position,
                    crate::state::skip_whitespace_backwards(
                        &state.wiki_text,
                        position_before_line_break,
                    ),
                    &state.wiki_text,
                );
                state.nodes.push(crate::Node::ParagraphBreak {
                    end: position_after_line_break,
                    start: position_before_line_break,
                });
                table.start = position_after_line_break;
                table.state = TableState::CellRemainder;
                state.flushed_position = position_after_line_break;
                state.scan_position = position_after_line_break;
            }
            TableState::HeadingFirstLine => {
                crate::state::flush(
                    &mut state.nodes,
                    state.flushed_position,
                    crate::state::skip_whitespace_backwards(
                        &state.wiki_text,
                        position_before_line_break,
                    ),
                    &state.wiki_text,
                );
                state.nodes.push(crate::Node::ParagraphBreak {
                    end: position_after_line_break,
                    start: position_before_line_break,
                });
                table.start = position_after_line_break;
                table.state = TableState::HeadingRemainder;
                state.flushed_position = position_after_line_break;
                state.scan_position = position_after_line_break;
            }
            TableState::CellRemainder | TableState::HeadingRemainder => {
                state.scan_position = position_before_line_break + 1
            }
            TableState::TableAttributes => {
                crate::state::flush(
                    &mut state.nodes,
                    state.flushed_position,
                    crate::state::skip_whitespace_backwards(
                        &state.wiki_text,
                        position_before_line_break,
                    ),
                    &state.wiki_text,
                );
                table.attributes = std::mem::take(&mut state.nodes);
                table.start = position_after_token;
                table.state = TableState::Before;
                state.flushed_position = position_after_token;
                state.scan_position = position_after_token;
                if position_after_token != position_after_line_break {
                    return;
                }
            }
            TableState::Row => {
                crate::state::flush(
                    &mut state.nodes,
                    state.flushed_position,
                    crate::state::skip_whitespace_backwards(
                        &state.wiki_text,
                        position_before_line_break,
                    ),
                    &state.wiki_text,
                );
                table.rows.push(crate::TableRow {
                    attributes: std::mem::take(&mut state.nodes),
                    cells: vec![],
                    end: position_before_line_break,
                    start: table.start,
                });
                table.start = position_after_token;
                table.state = TableState::Before;
                state.flushed_position = position_after_token;
                state.scan_position = position_after_token;
                if position_after_token != position_after_line_break {
                    return;
                }
            }
        }
    }
    crate::line::parse_beginning_of_line(
        state,
        if paragraph_break_possible {
            Some(position_before_line_break)
        } else {
            None
        },
    );
}

pub fn parse_inline_token(state: &mut crate::State) {
    let table = get_table(&mut state.stack);
    let position_before_token = state.scan_position;
    if state
        .wiki_text
        .as_bytes()
        .get(position_before_token + 1)
        .cloned()
        == Some(b'|')
    {
        match table.state {
            crate::state::TableState::CaptionFirstLine => {
                let end = crate::state::skip_whitespace_backwards(
                    &state.wiki_text,
                    position_before_token,
                );
                crate::state::flush(
                    &mut state.nodes,
                    state.flushed_position,
                    end,
                    &state.wiki_text,
                );
                table.captions.push(crate::TableCaption {
                    attributes: table.child_element_attributes.take(),
                    content: std::mem::take(&mut state.nodes),
                    end,
                    start: table.start,
                });
                table.start = position_before_token;
                state.scan_position = position_before_token + 2;
                while let Some(character) = state.wiki_text.as_bytes().get(state.scan_position) {
                    match character {
                        b'\t' | b' ' => state.scan_position += 1,
                        _ => break,
                    }
                }
                state.flushed_position = state.scan_position;
            }
            crate::state::TableState::CellFirstLine => {
                let end = crate::state::skip_whitespace_backwards(
                    &state.wiki_text,
                    position_before_token,
                );
                crate::state::flush(
                    &mut state.nodes,
                    state.flushed_position,
                    end,
                    &state.wiki_text,
                );
                if table.rows.is_empty() {
                    table.rows.push(crate::TableRow {
                        attributes: vec![],
                        cells: vec![],
                        end,
                        start: table.start,
                    });
                }
                let row = table.rows.last_mut().unwrap();
                row.cells.push(crate::TableCell {
                    attributes: table.child_element_attributes.take(),
                    content: std::mem::take(&mut state.nodes),
                    end,
                    start: table.start,
                    type_: crate::TableCellType::Ordinary,
                });
                row.end = end;
                table.start = position_before_token;
                state.scan_position = position_before_token + 2;
                while let Some(character) = state.wiki_text.as_bytes().get(state.scan_position) {
                    match character {
                        b'\t' | b' ' => state.scan_position += 1,
                        _ => break,
                    }
                }
                state.flushed_position = state.scan_position;
            }
            crate::state::TableState::HeadingFirstLine => {
                let end = crate::state::skip_whitespace_backwards(
                    &state.wiki_text,
                    position_before_token,
                );
                crate::state::flush(
                    &mut state.nodes,
                    state.flushed_position,
                    end,
                    &state.wiki_text,
                );
                if table.rows.is_empty() {
                    table.rows.push(crate::TableRow {
                        attributes: vec![],
                        cells: vec![],
                        end,
                        start: table.start,
                    });
                }
                let row = table.rows.last_mut().unwrap();
                row.cells.push(crate::TableCell {
                    attributes: table.child_element_attributes.take(),
                    content: std::mem::take(&mut state.nodes),
                    end,
                    start: table.start,
                    type_: crate::TableCellType::Heading,
                });
                row.end = end;
                table.start = position_before_token;
                state.scan_position = position_before_token + 2;
                while let Some(character) = state.wiki_text.as_bytes().get(state.scan_position) {
                    match character {
                        b'\t' | b' ' => state.scan_position += 1,
                        _ => break,
                    }
                }
                state.flushed_position = state.scan_position;
            }
            _ => state.scan_position += 2,
        }
    } else {
        match table.state {
            crate::state::TableState::CaptionFirstLine
            | crate::state::TableState::CellFirstLine
            | crate::state::TableState::HeadingFirstLine
                if table.child_element_attributes.is_none() =>
            {
                crate::state::flush(
                    &mut state.nodes,
                    state.flushed_position,
                    crate::state::skip_whitespace_backwards(
                        &state.wiki_text,
                        position_before_token,
                    ),
                    &state.wiki_text,
                );
                table.child_element_attributes = Some(std::mem::take(&mut state.nodes));
                state.scan_position = position_before_token + 1;
                while let Some(character) = state.wiki_text.as_bytes().get(state.scan_position) {
                    match character {
                        b'\t' | b' ' => state.scan_position += 1,
                        _ => break,
                    }
                }
                state.flushed_position = state.scan_position;
            }
            _ => state.scan_position += 1,
        }
    }
}

pub fn start_table(state: &mut crate::State, position_before_line_break: Option<usize>) {
    if let Some(position) = position_before_line_break {
        crate::state::flush(
            &mut state.nodes,
            state.flushed_position,
            crate::state::skip_whitespace_backwards(&state.wiki_text, position),
            &state.wiki_text,
        );
    }
    state.flushed_position = state.scan_position;
    let mut position = state.scan_position + 2;
    while let Some(b'\t') | Some(b' ') = state.get_byte(position) {
        position += 1;
    }
    state.push_open_node(
        crate::OpenNodeType::Table(crate::state::Table {
            attributes: vec![],
            before: vec![],
            captions: vec![],
            child_element_attributes: None,
            rows: vec![],
            start: 0,
            state: crate::state::TableState::TableAttributes,
        }),
        position,
    );
}

fn get_table(stack: &mut [crate::OpenNode]) -> &mut crate::state::Table {
    match stack.last_mut() {
        Some(crate::OpenNode {
            type_: crate::OpenNodeType::Table(table),
            ..
        }) => table,
        _ => unreachable!(),
    }
}
