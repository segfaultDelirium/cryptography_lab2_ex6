// szyfr przestawny

fn find_occurences_recursive(letter: char, text: &String, count: i32) -> i32{
    if text.len() == 0 {return count}
    let tail = &text[1..text.len()].to_string();
    let new_count = if text.starts_with(letter) {count + 1} else {count};
    return find_occurences_recursive(letter, &tail, new_count);
}

fn find_occurences_count(letter: char, text: &String) -> i32{
    let occurences = find_occurences_recursive(letter, text, 0);
    return occurences;
    // return 0;
}

fn get_letters() -> Vec<char> {
    return ('a' ..= 'z').collect::<Vec<char>>();
}



fn z6(){

    let prefix = "LOOK".to_string().to_lowercase();


    let eng_frequency = "ETAOINSHRDLCUMWFGYPBVKJXQZ".to_string().to_lowercase();
    println!("eng_frequency: {}", eng_frequency.to_uppercase());

    let cyphertext = "BEEAKFYDJXUQYHYJIQRYHTYJIQFBQDUYJIIKFUHCQD".to_string().to_lowercase();
    println!("prefix: {}", prefix.to_uppercase());
    println!("cyphertext: {}", cyphertext.to_uppercase());

    let occurences = get_letters().into_iter()
        .map(|c| find_occurences_count(c, &cyphertext)).collect::<Vec<i32>>();
    println!("occurences: {:?}", occurences);

    let mut zipped = get_letters().iter().map(|c| c.clone())
        .zip(occurences)
        .collect::<Vec<(char, i32)>>();
    println!("zipped {:?}", &zipped);

    zipped.sort_by(|(c1, x1), (c2, x2)| x1.cmp(x2));
    zipped.reverse();
    let sorted =  (&zipped).into_iter().map(|(c, _x)| *c).collect::<Vec<char>>();

    println!("sorted: {:?}", sorted);

    // println!("")
    // println!("zipped sorted: {:?}", &zipped);
    let naive_decrypted = eng_frequency.chars().zip(sorted)
        // .collect::<Vec<(char, char)>>()
        .map(|(a, b)| a).collect::<Vec<char>>();
    println!("naive_decrypted: {:?}", naive_decrypted);

}

fn main() {
    z6();
}
