fn main() {
    let file_name = std::env::args().nth(1)
    .expect("the file name to be passed in");

    let file = std::fs::read_to_string(file_name)
    .expect("unable to read the file to string");

    file
    .lines()
    .filter_map(|line| line.parse::<usize>().ok()) // filter out None values
    .for_each(|line| {
        println!("{}", line);           
    });
}