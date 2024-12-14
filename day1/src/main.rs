

fn main() {
    // read the file
    let contents = std::fs::read_to_string("data/input.txt").expect("Failed to read the file");

    // get the number of lines in the file to create vectors of the same size
    let lines = contents.lines().count();

    let mut first_list: Vec<i32> = Vec::with_capacity(lines);
    let mut second_list: Vec<i32> = Vec::with_capacity(lines);

    // map the contents of the file to lists
    contents.lines().for_each(|line| {
        let mut iter = line.split_whitespace();
        first_list.push(iter.next().unwrap().parse().unwrap());
        second_list.push(iter.next().unwrap().parse().unwrap());
    });

    let total_diff = total_diff(&first_list, &second_list);
    println!("The total difference between the two lists is: {}", total_diff);

    let total_similarity = total_similarity(&first_list, &second_list);
    println!("The total similarity between the two lists is: {}", total_similarity);

}

fn total_diff(first_list: &Vec<i32>, second_list: &Vec<i32>) -> i32 {
    let mut sorted_first_list = first_list.clone();
    let mut sorted_second_list = second_list.clone();

    sorted_first_list.sort();
    sorted_second_list.sort();

    let mut total_diff = 0;
    for i in 0..sorted_first_list.len() {
        total_diff += (sorted_first_list[i] - sorted_second_list[i]).abs();
    }
    total_diff
}

fn total_similarity(first_list: &Vec<i32>, second_list: &Vec<i32>) -> i32 {
    let mut sorted_first_list = first_list.clone();
    let mut sorted_second_list = second_list.clone();

    sorted_first_list.sort();
    sorted_second_list.sort();

    let mut total_similarity = 0;
    let mut count: i32;

    let mut i:usize = 0;
    let mut j:usize = 0;
    let mut temp:usize;
    while i < sorted_first_list.len() && j < sorted_second_list.len() {
        if sorted_first_list[i] == sorted_second_list[j] {
            count =1;
            temp=j;
            while j<sorted_second_list.len() && sorted_first_list[i] == sorted_second_list[j+1] {
                count += 1;
                j += 1;
            }
            total_similarity += count * sorted_first_list[i];
            i += 1;
            j = temp;
        }
        else if sorted_first_list[i] < sorted_second_list[j] {
            i += 1;
            continue;
        }
        else {
            j += 1;
            continue;
        }
    }
    total_similarity
}