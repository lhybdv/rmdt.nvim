use nvim_oxi::api::Buffer;
use nvim_oxi::{self as oxi};

use std::collections::HashMap;
use crate::types::*;
use crate::utils::*;

fn format_separation_row (
    line: &str,
    max_widths: &HashMap<usize, usize>,
    aligns: &HashMap<usize, Align>
)  -> String {
    let mut new_line = "".to_string();
    let bars = get_end_bars(line);
    let n_col = bars.len();
    for Bar {col, ..} in bars {
        new_line.push_str("| ");
        let mut col_width = 3;
        let max_w = *max_widths.get(&col).unwrap();
        if max_w > 3 {
            col_width = max_w;
        }
        match aligns.get(&col) {
            Some(Align::Center) => {
                new_line.push(':');
                new_line.push_str(&"-".repeat(col_width - 2));
                new_line.push_str(": ");
            }
            Some(Align::Right) => {
                new_line.push_str(&"-".repeat(col_width - 1));
                new_line.push_str(": ");
            }
            Some(Align::Left) => {
                new_line.push(':');
                new_line.push_str(&"-".repeat(col_width - 1));
                new_line.push(' ');
            }
            _ => {
                new_line.push_str(&"-".repeat(col_width));
                new_line.push(' ');
            }
        }
    }
    new_line.push('|');
    // If the lign is incomplete (has less columns than other lines)
    // Add some empty cells at the end of the lign
    let col_count = max_widths.keys().count();
    if n_col < col_count {
        new_line.push_str(&"|".repeat(col_count - n_col));
        format_separation_row(&new_line, max_widths, aligns)
    } else {
        new_line
    }
}

fn format_row(
    line: &str,
    max_widths: &HashMap<usize, usize>, 
    aligns: &HashMap<usize, Align>
) -> String {
    let mut prev_pos = 0;
    let mut new_line = "".to_string();

    if is_lign_row(line) {
        new_line = format_separation_row(line, max_widths, aligns);
    } else {
        let bars = get_end_bars(line);
        let n_col = bars.len();
        for Bar{ col, pos } in bars.iter() {
            new_line.push_str("| ");
            let cell = line[prev_pos + 1..*pos].trim();

            let mut spaces = max_widths.get(col).unwrap() - str_len(cell);
            match aligns.get(col) {
                Some(Align::Center) => {
                    spaces /= 2;
                    new_line.push_str(&" ".repeat(spaces));
                    new_line.push_str(cell);
                    if max_widths.get(col).unwrap() % 2 != str_len(cell) % 2 {
                        spaces += 1
                    }
                    new_line.push_str(&" ".repeat(spaces));
                    new_line.push(' ');
                }
                Some(Align::Right) => {
                    new_line.push_str(&" ".repeat(spaces));
                    new_line.push_str(cell);
                    new_line.push(' ');
                }
                _ => {
                    new_line.push_str(cell);
                    new_line.push_str(&" ".repeat(spaces));
                    new_line.push(' ');
                }
            }
            prev_pos = *pos;
        }
        new_line.push('|');
        // If the lign is incomplete (has less columns than other lines)
        // Add some empty cells at the end of the lign

        let col_count = max_widths.keys().count();
        if n_col < col_count {
            new_line.push_str(&"|".repeat(col_count - n_col));
            new_line = format_row(&new_line, max_widths, aligns);
        }
        new_line.push_str(&line[prev_pos + 1..])
    }
    new_line
}

pub(crate) fn format_all_rows (
    lines: &mut Vec<String>, 
    max_widths: &HashMap<usize, usize>, 
    aligns: &HashMap<usize, Align>
) {
    lines.into_par_iter().for_each(|line| {
        *line = format_row(line, max_widths, aligns);
    });
}

use rayon::prelude::*;

pub fn format_table () -> oxi::Result<()> {
    let (first_row, last_row, mut lines) = get_markdown_table()?;
    let (max_widths, aligns) = compute_widths_and_aligns(&mut lines);

    format_all_rows(&mut lines, &max_widths, &aligns);
    
    let mut buf = Buffer::current();
    let lines: Vec<&str> = lines.iter().map(|s| &**s).collect();
    buf.set_lines(first_row-1..=last_row, false, lines)?;
    Ok(())
}
