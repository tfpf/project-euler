/// Calculate the score of a name.
///
/// * `idx` - Position of `name`, 0-indexed.
/// * `name` - Name of a person.
///
/// -> Score of `name` for 1-indexed position.
fn score((idx, name): (usize, &&str)) -> usize
{
    // I am fairly sure Rust uses fixed encoding in which `'A'` is encoded as
    // 65, and consecutive letters have consecutive codes. `name` contains only
    // uppercase letters.
    let sum: usize = name.chars().map(|c| c as usize - 64).sum();
    sum * (idx + 1)
}

/// Main function.
fn main()
{
    // The file should be located in the same directory as the executable. I
    // should probably learn to handle errors rather than unwrap.
    let exe = std::env::current_exe().unwrap();
    let directory = exe.parent().unwrap();
    let fname = directory.join("names.txt");
    let contents = std::fs::read_to_string(&fname).unwrap();

    // Break the string at commas. Remove all double quotes.
    let mut names: Vec<&str> = contents.split(",").map(|s| &s[1..s.len() - 1]).collect();
    names.sort();

    // The iterator provides a reference to the contents of the container,
    // which is a vector of references. That's why the mapped function must
    // accept a reference to a reference.
    let result: usize = names.iter().enumerate().map(score).sum();
    println!("{}", result);

    assert_eq!(result, 871198282);
}
