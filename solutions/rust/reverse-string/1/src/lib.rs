pub fn reverse(input: &str) -> String {
    let mut input_string = input.to_owned();
    let mut output = String::new();
    loop {
        match input_string.pop() {
            Some(c) => output.push(c),
            None => break,
        };
    }
    output
}
