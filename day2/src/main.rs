
fn main() {

    // read the file and get the contents as a list of strings
    let report_file = std::fs::read_to_string("data/input.txt").unwrap();
    let reports = report_file.lines().collect::<Vec<&str>>();

    let mut total_safe_reports = 0;
    for report in reports.iter() {
        if is_report_safe(report) {
            total_safe_reports += 1;
        }
    }
    println!("Total safe reports: {}", total_safe_reports);

    let mut total_safe_reports_dampener = 0;
    for report in reports.iter() {
        if is_report_safe_dampener(report) {
            total_safe_reports_dampener += 1;
            
        }
    }
    println!("Total safe reports with dampener: {}", total_safe_reports_dampener);
}

fn is_change_safe(value: i32,sign: i32) -> bool {
    if value.abs() > 3 || value.abs() == 0 || value.signum() != sign {
        return false;
    }
    return true;
}

fn is_report_safe(report: &str) -> bool {
    // split the report into its components
    let numbers = report.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();

    // make a differance vector to store the differences between the numbers
    let mut differences = vec![0; numbers.len() - 1];
    let mut sign = 0;

    // iterate and update the sign and differences
    for i in 0..numbers.len() - 1 {
        differences[i] = numbers[i + 1] - numbers[i];
        sign += differences[i].signum();
    }
    sign=sign.signum();

    for i in 0..differences.len() {
        if !is_change_safe(differences[i],sign) {
            return false;
        }
    }
    return true;
}



fn is_report_safe_dampener(report: &str) -> bool {
    // split the report into its components
    let numbers = report.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();

    let mut problem_found = false;

    // make a differance vector to store the differences between the numbers
    let mut differences = vec![0; numbers.len() - 1];
    let mut sign = 0;

    // iterate and update the sign and differences
    for i in 0..numbers.len() - 1 {
        differences[i] = numbers[i + 1] - numbers[i];
        sign += differences[i].signum();
    }
    sign=sign.signum();

    let mut i = 0;

    // this part tries to find a problem and checks if it can be fixed by summing the problem difference with the next or previous difference
    // if it can't be fixed or there are more than one problem, then the report is not safe
    while i< differences.len() {
        if is_change_safe(differences[i],sign) {
            i += 1;
        }
        else {
            if problem_found {
                return false;
            }
            problem_found = true;
            if i == 0 {
                if is_change_safe(differences[i+1], sign) {
                    i += 1;
                }
                else {
                    if is_change_safe(differences[i+1]+differences[i],sign) {
                        i += 2;
                        
                    }
                    else {
                        return false;
                    }
                }
                
            }
            else if i == differences.len() - 1 {
                i+=1;
                
            }
            else {
                if is_change_safe(differences[i-1]+differences[i],sign) {
                    i += 1;
                }
                else if is_change_safe(differences[i+1]+differences[i],sign) {
                    i += 2;
                    
                }
                else {
                    return false;
                }
            }
        }
    }
    
    return true;
}

// a brute force way that i used to check the problem cases
fn _is_report_safe_dampener_brute_force(report: &str) -> bool {
    // split the report into its components
    if is_report_safe(report) {
        return true;
    }
    else {
        // remove one element at a time and check if the report is safe
        let numbers = report.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        for i in 0..numbers.len() {
            let mut temp = numbers.clone();
            temp.remove(i);
            let mut temp_report = String::new();
            for j in 0..temp.len() {
                temp_report.push_str(&temp[j].to_string());
                temp_report.push_str(" ");
            }
            if is_report_safe(&temp_report) {
                return true;
            }
        }
    }
    return false;
}