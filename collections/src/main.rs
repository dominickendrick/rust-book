fn main() {
    let mut v: Vec<i32> = Vec::new();
    let v1 = vec![1,2,3];

    v.push(21);
    v.push(13);

    let v = vec![1, 2, 3, 4, 5];

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    for i in &v {
        println!("{}", i);
    }

    let mut v1 = vec![1, 2, 3, 4, 5];
    for i in &mut v1 {
        *i += 50;
        println!("{}", i);
    }

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    for i in &row {
        println!("{:?}", i);
    }

    let mut s = String::from("foo");
    s.push_str("bar");

    s.push('l');

    let s2 = "Logs ".to_string() + &s.to_string();
    println!("S is {}", s2);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);

    println!("S is {}", s);

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }

    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("Scores is {:?}", scores);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut scores: HashMap<_, _> =
        teams.into_iter().zip(initial_scores.into_iter()).collect();

    println!("Scores 2 is {:?}", scores);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    scores.insert(String::from("Yellow"), 80);

    scores.entry(String::from("Yellow")).or_insert(70);
    scores.entry(String::from("Green")).or_insert(50);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);

    //Given a list of integers, use a vector and return the median (when sorted, the value in the middle position) and mode (the value that occurs most often; a hash map will be helpful here) of the list.

    let mut list = vec![1,1,2,2,3,3,3,3,3,3,4,5,6,7];
    list.sort();
    let middleIndex = list.len() / 2;
    let median = list.get(middleIndex);
    println!("median is {:?}", median);

    let mut map = HashMap::new();

    for num in list {
        let count = map.entry(num).or_insert(0);
        *count += 1;
    }

    let max = map.iter().max_by_key(|entry | entry.1);
    let mode_value = match max {
        Some((k,v)) => k, 
        None => &0,
    };

    println!("mode map is {:?} and max key is : {:?}", map, mode_value);
}
