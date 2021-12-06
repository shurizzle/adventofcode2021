use std::io::{BufRead, BufReader};

pub fn is_eof<R: std::io::Read>(text: &mut BufReader<R>) -> std::io::Result<bool> {
    text.fill_buf().map(|b| b.is_empty())
}

pub fn read_line<R: std::io::Read>(text: &mut BufReader<R>) -> std::io::Result<String> {
    let mut line = String::new();
    text.read_line(&mut line)?;
    line.truncate(line.trim_end_matches('\n').len());
    line.truncate(line.trim_end_matches('\r').len());
    Ok(line)
}
