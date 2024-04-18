fn main() {
    let file = std::fs::read_to_string("lines").unwrap();

    file
        .lines()
        .enumerate()// enumerate creates a tuple
        .filter(|(idx, _)| idx % 2 == 0)
        .for_each(|(_, line)| println!("{}", line));
}






















/* BASIC MAPPING IMPLEMENTATION */
// fn main() {
//     let data = vec![1,2,3];
//     let mut iter = data
//         .iter()
//         .map(|x| x + 1);
//
//     let mut new_vector = vec![];
//
//     while let Some(x) = iter.next() {
//         new_vector.push(x);
//     }
//
//     println!("{:?}", new_vector);
// }