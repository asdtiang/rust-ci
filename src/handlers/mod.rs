pub mod auth;
pub mod builds;
pub mod projects;
pub mod project_logs;
pub mod webhook;

use tower_sessions::Session;

pub async fn get_lang(session: &Session) -> String {
    session.get("lang").await.unwrap_or(None).unwrap_or_else(|| "zh".to_string())
}

pub fn find_last_lines_offset(file: &mut std::fs::File, num_lines: usize) -> std::io::Result<u64> {
    use std::io::{Read, Seek, SeekFrom};
    let file_len = file.metadata()?.len();
    if file_len == 0 {
        return Ok(0);
    }

    let mut buffer = [0u8; 8192];
    let mut pos = file_len;
    let mut lines_found = 0;

    while pos > 0 {
        let read_size = std::cmp::min(8192, pos) as usize;
        pos -= read_size as u64;
        
        file.seek(SeekFrom::Start(pos))?;
        file.read_exact(&mut buffer[..read_size])?;
        
        for i in (0..read_size).rev() {
            if buffer[i] == b'\n' {
                lines_found += 1;
                if lines_found > num_lines {
                    return Ok(pos + i as u64 + 1);
                }
            }
        }
    }
    
    Ok(0)
}
