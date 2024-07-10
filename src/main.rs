const P: char = 92 as char;
const Q: char = 34 as char;
fn main() {
    let b = "const P: char = 92 as char;
const Q: char = 34 as char;
fn main() {
    let b = \"🔁\";
    for c in b.chars() {
        match c as u32 {
            128257 => {
                for c in b.chars() {
                    print!(
                        \"{}\",
                        match c {
                            P => format!(\"{P}{P}\"),
                            Q => format!(\"{P}{Q}\"),
                            _ => format!(\"{}\", c),
                        }
                    )
                }
            }
            _ => print!(\"{}\", c),
        }
    }
    println!()
}";
    for c in b.chars() {
        match c as u32 {
            128257 => {
                for c in b.chars() {
                    print!(
                        "{}",
                        match c {
                            P => format!("{P}{P}"),
                            Q => format!("{P}{Q}"),
                            _ => format!("{}", c),
                        }
                    )
                }
            }
            _ => print!("{}", c),
        }
    }
    println!()
}
