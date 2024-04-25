use std::collections::HashMap;

use nvim_oxi::api::{self,  types::*, Window, Buffer};
use nvim_oxi::{self as oxi};
use once_cell::sync::Lazy;
use regex::Regex;

use crate::types::*;

pub(crate) fn get_markdown_table () 
    -> oxi::Result<(usize, usize, Vec<String>)> 
{
    let mut win = Window::current();

    let init_pos = win.get_cursor()?;
    
    // Go to end of the table and get line
    api::feedkeys("vip", Mode::Visual, false);
    let (last_row, _) = win.get_cursor()?;

    api::feedkeys("o", Mode::Visual, false);
    let (first_row, _) = win.get_cursor()?;
    api::feedkeys("<esc>", Mode::Visual, false);

    let buf = Buffer::current();
    // Get whole table
    let oxi_lines = 
        buf.get_lines(first_row-1..last_row, false)?;

    let lines = oxi_lines.into_iter()
        .map(|l| l.to_string_lossy()
        .into_owned()).collect::<Vec<String>>();
    
    // Replace cursor where it was
    win.set_cursor(init_pos.0, init_pos.1)?;

    Ok((first_row, last_row, lines))
}

pub(crate) fn get_end_bars(line: &str) -> Vec<Bar> {
    line[1..].bytes().enumerate()
        .filter(|(_, b)| *b == b'|')
        .enumerate()
        .map(|(col, (pos, _))| Bar{col, pos: pos + 1})
        .collect::<Vec<Bar>>()
}

pub(crate) fn is_lign_row (line: &str) -> bool {
    static LIGN_LINE: Lazy<Regex> = Lazy::new(|| {
        Regex::new("[^ :|-]").unwrap()
    });

    !LIGN_LINE.is_match(line)
}

fn compute_aligns (line: &str) -> HashMap<usize, Align> { 
    let mut prev_pos = 0;
    let mut aligns = HashMap::new();
    for Bar{col, pos} in get_end_bars(line).iter() {
        let cell = line[prev_pos + 1..*pos].trim();
        let mut align: Align = Default::default();
        if cell.starts_with(':') && cell.ends_with(':') {
            align = Align::Center;
        } else if cell.starts_with(':') {
            align = Align::Left;
        } else if cell.ends_with(':') {
            align = Align::Right;
        } 
        aligns.insert(*col, align);

        prev_pos = *pos;
    }
    aligns
}

pub(crate) fn str_len(str: &str) -> usize {
    let bytes_num = str.bytes().len();
    let non_ascii_num = str.chars().filter(|c| !c.is_ascii()).count();
    bytes_num - non_ascii_num
}

pub(crate) fn compute_widths_and_aligns (lines: &mut [String]) 
    -> (HashMap<usize, usize>, HashMap<usize, Align>)
{
    let mut max_widths: HashMap<usize, usize> = HashMap::new();
    let mut aligns = HashMap::new();

    for line in lines.iter_mut() {
        if !line.starts_with('|') {
            *line = format!("|{}", line);
        }

        if !line.ends_with('|') {
            line.push('|')
        }

        if is_lign_row(line) {
            aligns = compute_aligns(line);
            continue;
        }

        let mut prev_pos = 0;
        for Bar{col, pos} in get_end_bars(line).iter() {

            let max_w = if let Some(w) = max_widths.get(col) {
                *w
            } else {
                0
            };
            aligns.entry(*col).or_insert(Align::Left);
            let cell = line[prev_pos + 1..*pos].trim();

            let mut len = str_len(cell);
            
            if len > max_w {
                if len < 3 {
                    len = 3;
                }
                max_widths.insert(*col, len);
            }

            prev_pos = *pos;
        }
    }

    (max_widths, aligns)
}

pub(crate) fn col_start(line: &str, n_col: usize) -> usize {
    if n_col == 0 {
        return 0
    }

    if let Some(b) = get_end_bars(line).iter().find(|b| b.col == n_col - 1) {
        return b.pos
    }

    0
}

