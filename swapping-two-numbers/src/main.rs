use std::io;

fn main() {
    let mut vec: Vec<_> = Vec::new();
    
    loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        let v: Vec<u32> = buffer.split(' ')
            .map(|n| n.trim().parse().unwrap())
            .collect();
        if v[0] ==0 && v[1] == 0 {
            break;
        } else if v[0] <= v[1] {
            vec.push((v[0], v[1]));
        } else {
            vec.push((v[1], v[0]));
        }
    }

    println!("");
    for (i, j) in vec {
        println!("{} {}", i, j);
    }
}
