// We should figure out how to share this file across name-as and name-emu.
// But it raises architectural questions about what this means for portability.
// Are we losing the ability to use other assemblers by doing this?

// Ok, it's actually really easy to share this across name-as and name-emu. I've done it with Cargo.
// There's no real portability issue here. We are not losing the ability to use other assemblers.
// The other assemblers just need to integrate properly - a problem they were already expecting to run into.

extern crate serde;
extern crate toml;
use std::collections::HashMap;
use serde::Serialize;
use std::fs;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct LineInfo {
    pub instr_addr: u32,
    pub line_number: u32,
    pub line_contents: String,
    pub psuedo_op: String,
}

impl LineInfo {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = vec!();
        bytes.extend_from_slice(&self.instr_addr.to_be_bytes());
        bytes.extend_from_slice(&self.line_number.to_be_bytes());
        bytes.extend_from_slice(&self.line_contents.as_bytes());
        bytes.extend_from_slice(b"\0");
        bytes.extend_from_slice(&self.psuedo_op.as_bytes());
        bytes.extend_from_slice(b"\0");
        bytes
    }
}

#[derive(Deserialize, Serialize)]
struct LineInfoFile {
    pub lineinfo: Vec<LineInfo>,
}

pub fn lineinfo_import(
    file_contents: String
) -> Result<HashMap<u32, LineInfo>, Box<dyn std::error::Error>> {
    let line_info: LineInfoFile = toml::from_str(&file_contents)?;

    // Code smell— we have to iterate over this entire list and transform it into a HashMap.
    // This is because TOML can't serialize HashMaps with anything other than strings as keys. 
    // So we just serialize as Vec and deserialize as Vec then convert. This is tech debt.
    let out = line_info.lineinfo.into_iter().map(|line| (line.instr_addr, line)).collect::<HashMap<_,_>>();
    
    Ok(out)
}
pub fn lineinfo_export(
    filename: String,
    li: Vec<LineInfo>,
) -> Result<(), Box<dyn std::error::Error>> {
    let toml_data = toml::to_string(&LineInfoFile { lineinfo: li })?;

    fs::write(filename, toml_data)?;

    Ok(())
}

