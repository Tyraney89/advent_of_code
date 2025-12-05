use std::fs;


fn main() {

    let text: String =  fs::read_to_string("input.txt")
        .expect("could not read file");

    let cleaned_instructions: Vec<&str> = text.split_whitespace().collect();


//    println!("{:?}", cleaned_instructions);

    let mut count: i64 = 50;
    let mut result: u32 = 0;

    for i in cleaned_instructions
    {
        let direction: &str = &i[0..1];
        let num: i64 = i[1..].parse()
            .expect("failed to parse");
        if direction == "R" {
            count += num;
        }
        else {
            count -= num;
        }
        let pos = count.rem_euclid(100);
        if pos == 0 {
            result += 1;
        }

    }
    println!("{}", result);
}
