use std::io;

fn main() {
    let mut vec: Vec<String> = Vec::new();
    
    loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
    
        if buffer != "0\n" {
            vec.push(buffer);
        } else {
            break;
        }
    }
    for (i, j) in vec.iter().enumerate() {
        println!("Case {}: {}", i+1, j.trim_end());
    }
}
