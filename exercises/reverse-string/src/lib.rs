pub fn reverse(input: &str) -> String {
    if input == "" {
        return "".to_string()
    }

    let mut r = String::with_capacity(input.len());
    // let mut lastChar = input.char(

    for cIdx in input.char_indices().rev() {
        println!("cIdx is {:?}", cIdx);
        match cIdx {
            (idx, c) if input.is_char_boundary(idx) => {
                println!("charboundary! {}", c);
                r.push(c)
            },
            (_, _) => println!("other"),
        }
        // r.push(c);
    }

    r
}
