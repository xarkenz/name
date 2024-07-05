use name_const::structs::LineInfo;

// Collect line information for section .line prior to any macro/pseudoinstruction expansion for accurate debugging without source.
pub fn get_lineinfo(file_contents: &String) -> Option<Vec<LineInfo>> {
    let mut line_number: u32 = 1;
    let mut section_dot_line: Vec<LineInfo> = vec!();

    for line in file_contents.split('\n') {
        section_dot_line.push(
            LineInfo {
                line_number: line_number,
                content: line.to_string(),
            }
        );

        line_number += 1;
    }
    match section_dot_line.len() {
        0 => {
            return None;
        },
        _ => {
            return Some(section_dot_line);
        }
    };
}