// use nvim_oxi::api::{self, Window};
// use crate::utils;

// fn col_under_cursor() -> api::Result<usize>{
// 	let line = api::get_current_line()?;
//     let win = Window::current();
// 	let (_, col_pos) = win.get_cursor();
// 	let mut pos = 0;
// 	let mut prev_pos = 0;
// 	let mut col = 0;

//     while let Some(index) = line[prev_pos + 1..].find('|') {
//         pos = index + prev_pos + 1;

// 		if pos <= col_pos {
// 			col += 1;
//         } else {
// 			return Ok(col);
//         }
// 		prev_pos = pos;
//     }
// 	Ok(col)
// }

// fn col_swap () -> oxi::Result<()> {
// 	let col_1 = co_under_cursor();
// 	let col_2 = col_1 - 1;
// 	if col_2 == 0 { 
//         return;
//     }

// 	let (first_row, last_row, lines)= utils::get_markdown_table()?;

// 	for i =1, #lines do
// 		let cell_1 = utils::get_cell_text(lines, i, col_1);
// 		let cell_2 = utils::get_cell_text(lines, i, col_2);
// 		lines = set_cell_text(lines, i, col_1, cell_2)
// 		lines = set_cell_text(lines, i, col_2, cell_1)
// 	end
// 	local max_widths, alignements = compute_widths_and_alignments(lines)
// 	format_all_rows(lines, max_widths, alignements)

// 	vim.api.nvim_buf_set_lines(0, first_char[1], last_char[1], false, lines)

// 	local column_title_1 = get_cell_text(lines, 1, col_1)
// 	local column_title_2 = get_cell_text(lines, 1, col_2)
// 	vim.api.nvim_echo({{"MarkdownTable: swapped column "..
// 							col_2.." ("..column_title_2..") with column "..
// 							col_1.." ("..column_title_1..")"}}, false, {})

// })