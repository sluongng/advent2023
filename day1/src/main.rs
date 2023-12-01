fn main() {
    let input = "
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
";

    let answer = input
        .trim()
        .split("\n")
        .map(|s| {
            let mut iter = s.chars().filter(|c| c.is_numeric());

            let first = iter.next().unwrap().to_digit(10).unwrap();
            let last = match iter.last() {
                Some(c) => c.to_digit(10).unwrap(),
                None => first,
            };

            let line = first * 10 + last;
            // println!("{line}");
            line
        })
        .sum::<u32>();

    println!("{answer}");
}
