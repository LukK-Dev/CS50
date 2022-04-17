use std::env;

fn main() {
    let mut args = env::args();
    args.next();

    let size = match args.next() {
        Some(v) => v.parse::<u32>().unwrap(),
        None => panic!("No argument supplied."),
    };

    println!("{}", create_triangle_structure(size));
}

fn create_triangle_structure(size: u32) -> String {
    if size == 0 {
        panic!("Cannot create triangles of size zero.")
    }

    let mut structure = String::new();

    let create_space = |structure: &mut String, size: u32, column: u32| {
        for _i in 0..(size - column) {
            structure.push(' ')
        }
    };

    let create_blocks = |structure: &mut String, column: u32| {
        for _i in 0..column {
            structure.push('□')
        }
    };

    for column in 1..size + 1 {
        create_space(&mut structure, size, column);
        create_blocks(&mut structure, column);

        structure.push_str("  ");

        create_blocks(&mut structure, column);
        
        structure.push('\n');
    }

    structure
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn zero_size() {
        create_triangle_structure(0);
    }
}
