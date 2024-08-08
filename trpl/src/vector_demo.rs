pub fn func() {
    let v: Vec<i32> = Vec::new();
    let mut v: Vec<i32> = vec![1, 2, 3];
    v.push(1);
    let v0: i32 = v[0];
    let get: Option<&i32> = v.get(1);
    for i in &mut v {
        *i += 10;
    }
}
