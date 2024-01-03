use nvim_oxi::api::{self,  types::*, Window, Buffer};
use nvim_oxi::{self as oxi};

pub(crate) fn get_markdown_table () -> 
    oxi::Result<(usize, usize, Vec<String>)> 
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
        buf.get_lines(first_row-1..=last_row, false)?;

    let lines = oxi_lines.into_iter()
        .map(|l| l.to_string_lossy()
        .into_owned()).collect::<Vec<String>>();
    
    // Replace cursor where it was
    win.set_cursor(init_pos.0, init_pos.1)?;

    Ok((first_row, last_row, lines))
}

// fn column_find_start(line: String, n_col: usize) -> usize {
// 	let mut pos = 0;
// 	let mut i_col = 0;
// 	while i_col <= n_col {
// 		match line[pos+1..].find('|') {
//             Some(index) => pos = index,
//             None => break
//         };
// 		i_col += 1;
//     }
// 	pos
// }

// fn get_cell_text (lines: Vec<String>, row: usize, col: usize) -> String {
// 	let begin_pos = column_find_start(lines[row], col);
// 	let end_pos = column_find_start(lines[row], col + 1);
// 	let content = lines[row][begin_pos..=end_pos];
// 	return content.trim();
// }

// fn set_cell_text (lines, r, c, text)
// 	let begin_pos = column_find_start(lines[r], c);
// 	let end_pos = column_find_start(lines[r], c + 1);
// 	lines[r] = string.sub(lines[r], 1, begin_pos)..trim(text)..string.sub(lines[r],end_pos)
// 	return lines
// end