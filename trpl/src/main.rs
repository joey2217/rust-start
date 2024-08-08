use std::fs::File;


fn main() {

    let open = File::open("hello.txt").unwrap();

    // let text = "hello world wonderful world";

    // let mut map: HashMap<&str, i32> = HashMap::new();

    // for word in text.split_whitespace() {
    //     let count = map.entry(word).or_insert(0);
    //     *count += 1;
    // }

    // println!("{:?}", map);
}
