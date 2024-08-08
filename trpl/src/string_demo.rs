/// .
pub fn string_demo() {
    let mut s: String = String::from("hello");
    s.push_str("string")
    s.push('s');
    let data: &str = "12";
    let s: String = data.to_string();
    let var_name: String = format!("{}-{}", s, data);
}