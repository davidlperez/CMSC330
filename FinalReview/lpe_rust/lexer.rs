// Grammar:
//  CFG
//  E -> M + E|M - E|M
//  M -> N * M|N / M|sq M|N
//  N -> n|(E)
//  where n is any integer

use regex::Regex;

fn lex(instr:&str) -> Vec<&str>{
    let mut toklst:Vec<&str> = Vec::new();
    let mut pos = 0;
    let number_re = Regex::new(r"^\d+").unwrap();
    let terminal_re = Regex::new(r"^(sq|[()\-+/*])").unwrap();
    let wspace = Regex::new(r"^\s+").unwrap();

    while pos < instr.len() {
        if let Some(caps) = number_re.captures(&instr[pos..]) {
            if let Some(tok) = caps.get(1) {
                toklst.push(tok.as_str());
                pos += tok.as_str().len();
            }
        } else if let Some(caps) = terminal_re.captures(&instr[pos..]) {
            if let Some(tok) = caps.get(1) {
                toklst.push(tok.as_str());
                pos += tok.as_str().len();
            }
        } else if let Some(caps) = wspace.captures(&instr[pos..]) {
            if let Some(tok) = caps.get(1) {
                pos += tok.as_str().len();
            }
        } else {
            panic!("Invalid token at position {}", pos);
        }
    }
}