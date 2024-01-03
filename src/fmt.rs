use nvim_oxi::api::{self, Buffer};
use nvim_oxi::{self as oxi};
use once_cell::sync::Lazy;

use std::collections::HashMap;
use regex::Regex;
use crate::types::Align;
use crate::utils;


fn is_lign_row (line: &str) -> bool {
    static LIGN_LINE: Lazy<Regex> = Lazy::new(|| {
        Regex::new("[^ :|-]").unwrap()
    });

    !LIGN_LINE.is_match(line)
}

fn compute_alignments (sep_line: &str) -> HashMap<usize, Align> { 
    let mut i_col = 0;
    let mut pos;
    let mut prev_pos = 0;
    let mut alignments = HashMap::new();
    while let Some(index) = sep_line[prev_pos + 1..].find('|') {
        pos = index + prev_pos + 1;

        let cell = sep_line[prev_pos + 1..pos].trim();
        let mut align: Align = Default::default();
        if cell.starts_with(':') && cell.ends_with(':') {
            align = Align::Center;
        } else if cell.starts_with(':') {
            align = Align::Left;
        } else if cell.ends_with(':') {
            align = Align::Right;
        } 
        alignments.insert(i_col, align);

        prev_pos = pos;
        i_col += 1;
    }
    alignments
}

fn str_len(str: &str) -> usize {
    let bytes_num = str.bytes().len();
    let non_ascii_num = str.chars()
        .filter(|c| !c.is_ascii()).count();
    bytes_num - non_ascii_num
}

fn compute_widths_and_alignments (lines: &mut [String]) 
    -> (HashMap<usize, usize>, HashMap<usize, Align>)
{
    let mut max_widths: HashMap<usize, usize> = HashMap::new();
    let mut alignments = HashMap::new();

    for line in lines.iter_mut() {
        if !line.starts_with('|') {
            *line = format!("|{}", line);
        }

        if is_lign_row(line) {
            alignments = compute_alignments(line);
            continue;
        }

        let mut i_col = 0;
        let mut pos ;
        let mut prev_pos = 0;
        while let Some(index) = line[prev_pos + 1..].find('|') {
            pos = index + prev_pos + 1;

            let max_w = match max_widths.get(&i_col) {
                Some(w) => *w,
                None => 0,
            };
            alignments.entry(i_col).or_insert(Align::Left);
            let cell = line[prev_pos + 1..pos].trim();

            let mut len = str_len(cell);
            
            if len > max_w {
                if len < 3 {
                    len = 3;
                }
                max_widths.insert(i_col, len);
            }

            prev_pos = pos;
            i_col += 1
        }
    }

    (max_widths, alignments)
}

fn format_separation_row (
    line: &str,
    max_widths: &HashMap<usize, usize>,
    alignements: &HashMap<usize, Align>
)  -> String
{
    let mut i_col = 0;
    let mut pos;
    let mut prev_pos = 0;
    let mut new_line = "".to_string();
    let mut n_col = 0;
    while let Some(index) = line[prev_pos + 1..].find('|') {
         pos = index + prev_pos + 1;

        new_line.push_str("| ");
        let mut col_width = 3;
        let max_w = *max_widths.get(&i_col).unwrap();
        if max_w > 3 {
            col_width = max_w;
        }
        match alignements.get(&i_col) {
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
        prev_pos = pos;
        i_col += 1;
        n_col += 1;
    }
    new_line.push('|');
    // If the lign is incomplete (has less columns than other lines)
    // Add some empty cells at the end of the lign
    let col_count = max_widths.keys().count();
    if n_col < col_count {
        new_line.push_str(&"|".repeat(col_count - n_col));
        format_separation_row(&new_line, max_widths, alignements)
    } else {
        new_line
    }
}

fn format_row(
    line: &str,
    max_widths: &HashMap<usize, usize>, 
    alignements: &HashMap<usize, Align>) 
    -> String
{
    let mut i_col = 0;
    let mut pos;
    let mut prev_pos = 0;
    let mut new_line = "".to_string();
    let mut n_col = 0;

    if is_lign_row(line) {
        new_line = format_separation_row(line, max_widths, alignements);
    } else {
        while let Some(index) = line[prev_pos + 1..].find('|') {
            pos = index + prev_pos + 1;

            new_line.push_str("| ");
            let mut cell = line[prev_pos + 1..pos].trim();
            let len = str_len(cell);
            let mut cell_str;
            if len < 3 { 
                cell_str = cell.to_string();
                cell_str.push_str(&"".repeat(3 - len));
                cell = &cell_str;
            }

            let mut n_spaces = max_widths.get(&i_col).unwrap() - str_len(cell);
            match alignements.get(&i_col) {
                Some(Align::Center) => {
                    n_spaces /= 2;
                    new_line.push_str(&" ".repeat(n_spaces));
                    new_line.push_str(cell);
                    if max_widths.get(&i_col).unwrap() % 2 != str_len(cell) % 2 {
                        n_spaces += 1
                    }
                    new_line.push_str(&" ".repeat(n_spaces));
                    new_line.push(' ');
                }
                Some(Align::Right) => {
                    new_line.push_str(&" ".repeat(n_spaces));
                    new_line.push_str(cell);
                    new_line.push(' ');
                }
                _ => {
                    new_line.push_str(cell);
                    new_line.push_str(&" ".repeat(n_spaces));
                    new_line.push(' ');
                }
            }
            prev_pos = pos;
            i_col += 1;
            n_col += 1;
        }
        new_line.push('|');
        // If the lign is incomplete (has less columns than other lines)
        // Add some empty cells at the end of the lign

        let col_count = max_widths.keys().count();
        if n_col < col_count {
            new_line.push_str("|".repeat(col_count - n_col).as_ref());
            new_line = format_row(&new_line, max_widths, alignements);
        }
        new_line.push_str(&line[prev_pos + 1..])
    }
    new_line
}

fn format_all_rows (
    lines: &mut Vec<String>, 
    max_widths: &HashMap<usize, usize>, 
    alignements: &HashMap<usize, Align>
) {
    lines.into_par_iter().for_each(|line| {
        *line = format_row(line, max_widths, alignements);
    });
}

// use std::time::Instant;
use rayon::prelude::*;

pub fn format_table () -> oxi::Result<()>{
    // let now = Instant::now();
    let (first_row, last_row, mut lines) = utils::get_markdown_table()?;
    let (max_widths, aligns) = compute_widths_and_alignments(&mut lines);

    format_all_rows(&mut lines, &max_widths, &aligns);
    
    let mut buf = Buffer::current();
    buf.set_lines(first_row-1..=last_row, false, lines.iter().map(|s| s.as_ref()).collect::<Vec<&str>>())?;
    // let elapsed_time = now.elapsed();
    // api::err_writeln(&format!("elapsed_time: {}", elapsed_time.as_millis()));
    Ok(())
}
