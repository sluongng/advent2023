fn main() {
    let digi_words = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let input = "
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
";

    let answer = input
        .trim()
        .split("\n")
        .map(|s| {
            let mut line = s.to_string();

            loop {
                let mut min_index = usize::MAX;
                let mut first_word = "";
                let mut replace = 0;

                for (i, word) in digi_words.iter().enumerate() {
                    if let Some(index) = line.find(word) {
                        if index < min_index {
                            println!("{s}: found {word} at {index}");
                            min_index = index;
                            first_word = word;
                            replace = i + 1;
                        }
                    }
                }
                if first_word == "" {
                    break;
                }

                line = line.replace(first_word, &replace.to_string());
            }

            line
        })
        .map(|s| {
            let mut iter = s.chars().filter(|c| c.is_numeric());

            let first = iter.next().unwrap().to_digit(10).unwrap();
            let last = match iter.last() {
                Some(c) => c.to_digit(10).unwrap(),
                None => first,
            };

            let line = first * 10 + last;
            println!("{s}: {line}");
            line
        })
        .sum::<u32>();

    println!("Total: {answer}");
}
