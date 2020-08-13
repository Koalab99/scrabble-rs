const GADDAG_SEPARATOR : char = '+';

pub fn gaddag(word : String) -> Vec<String> {
    let mut output : Vec<String> = Vec::new();
    let chars : Vec<char> = word.chars().collect();

    for i in 1..(chars.len() + 1) {
        let (prefix, suffix) = chars.split_at(i);
        output.push(format!("{}{}{}", prefix.iter().rev().collect::<String>(), GADDAG_SEPARATOR, suffix.iter().collect::<String>()));
    }

    return output;
}

#[cfg(test)]
mod test {
    #[test]
    fn gaddag_test() {
        let pouf = String::from("POUF");

        let answer = super::gaddag(pouf);
        assert_eq!(answer, vec!["P+OUF", "OP+UF", "UOP+F", "FUOP+"]);
    }
}
