use nvim_oxi::api::{self, Window};
use oxi::api::Buffer;
use rayon::iter::IntoParallelIterator;
use crate::utils::*;
use crate::fmt;
use crate::utils::get_end_bars;
use nvim_oxi::{self as oxi};

fn col_under_cursor() -> oxi::Result<usize> {
	let line = api::get_current_line()?;
    let win = Window::current();
	let (_, col_pos) = win.get_cursor()?;

    let bars = get_end_bars(&line);
    if let Some(b) = bars.iter().find(|b| b.pos >= col_pos) {
        return Ok(b.col);
    }
    Ok(0)
}

fn get_rltv_bar_pos(line: &str, col: usize) -> (usize, usize, usize) {
    let pos_0;
    let pos_1;
    let pos_2;
    if col == 0 {
        pos_0 = 0;
        pos_1 = col_start(line, 1);
        pos_2 = col_start(line, 2);
    } else {
        pos_0 = col_start(line, col - 1);
        pos_1 = col_start(line, col);
        pos_2 = col_start(line, col + 1);
    }
    (pos_0, pos_1, pos_2)
}

fn line_col_swap(line: &str, col: usize) -> String {
    let (pos_0, pos_1, pos_2) = get_rltv_bar_pos(line, col);
    format!("{}{}|{}{}", 
        &line[..=pos_0], &line[pos_1+1..pos_2],
        &line[pos_0+1..pos_1], &line[pos_2..])
}

use rayon::prelude::*;

pub(crate) fn col_swap () -> oxi::Result<()> {
	let col = col_under_cursor()?;

	let (first_row, last_row, mut lines)= get_markdown_table()?;

    let t_line = lines[0].clone();
    let (pos_0, pos_1, pos_2) = get_rltv_bar_pos(&t_line, col);
    let title_0 = t_line[pos_0+1..pos_1].trim();
    let title_1 = t_line[pos_1+1..pos_2].trim();

    (&mut lines).into_par_iter().for_each(|line| {
		*line = line_col_swap(line, col)
    });
	let (max_widths, aligns) = compute_widths_and_aligns(&mut lines);
	fmt::format_all_rows(&mut lines, &max_widths, &aligns);

    let lines: Vec<&str> = lines.iter().map(|s| s.as_ref()).collect();
    let mut buf = Buffer::current();
    buf.set_lines(first_row-1..=last_row, false, lines)?;

    api::echo([("Column swapped:", None)], false)?;
    api::echo([(title_0, None)], false)?;
    api::echo([(title_1, None)], false)?;

    Ok(())
}
