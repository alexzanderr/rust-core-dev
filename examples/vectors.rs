fn get_vector() -> Vec<String> {
    let mut new_vector: Vec<String> = Vec::new();


    new_vector
}


fn main() {
    let mut some_vec = vec![
        String::from("endrew"),
        String::from("dndrew"),
        String::from("cndrew"),
        String::from("bndrew"),
        String::from("andrew"),
    ];
    println!("{:?}", some_vec);
    some_vec.sort();
    println!("{:?}", some_vec);
    some_vec.extend(vec!["hi".to_string()]);
    println!("{:?}", some_vec);
}
