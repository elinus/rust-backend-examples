pub fn test_std_collections_vector() {
    println!("Experimenting with Rust collections! Vector");

    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);

    assert_eq!(vec.len(), 2);
    assert_eq!(vec[0], 1);

    assert_eq!(vec.pop(), Some(2));
    assert_eq!(vec.len(), 1);

    vec[0] = 7;
    assert_eq!(vec[0], 7);

    vec.extend([1, 2, 3]);
    for x in &vec {
        println!("{x}");
    }
}
