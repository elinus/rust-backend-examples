fn main() {
    let crab_emoji = String::from("Rust String!🦀");
    println!("{:?}", crab_emoji);

    let hello0 = String::from("Hello World!");
    let mut hello1 = String::from("Hello ");
    hello1.push('W');
    hello1.push_str("orld!");
    assert_eq!(hello0, hello1);
    println!("{:?}", hello0.len());

    let sparkle_heart = vec![240, 159, 146, 150];
    let heart = String::from_utf8(sparkle_heart).unwrap();
    println!("{:?} => {:?}", heart, heart.len());
    println!("{:?}", heart.clone().into_bytes());
    println!("{:?}", heart.as_bytes());

    let mut s = String::new();
    println!("String capacity: {:?}", s.capacity());
    for elem in 0..5 {
        s.push_str("Coco!");
        println!("Iter{:?}: String capacity: {:?}", elem, s.capacity());
    }
    
    let word = String::from("goodbye");
    assert_eq!(7, word.chars().count());

    let mut chars = word.chars();
    assert_eq!(Some('g'), chars.next());
    let mut char_indices = word.char_indices();
    assert_eq!(Some((0, 'g')), char_indices.next());
    
    let lc = String::from("LOWER_CASE!");
    println!("to_lowercase: {:?}", lc.to_lowercase());
    let uc = String::from("upper_case!");
    println!("to_uppercase: {:?}", uc.to_uppercase());

    // Repeat
    println!("{:?}", "Coco!".repeat(14));
}
