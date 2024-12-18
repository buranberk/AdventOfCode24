use std::collections::HashMap;

fn main() {
    // read the input from the file and store it in a string
    let input = std::fs::read_to_string("data/input.txt").unwrap();

    // split the data into rules and records
    let mut parts=input.split("\n\n");

    let rules_string=parts.next().unwrap().lines();
    let records_string = parts.next().unwrap().lines();

    // Parse the rules into a vector of rules
    let mut rules: Vec<Rule> = vec![];
    for x in rules_string {
        let mut parts=x.split("|");
        let first:u8  = parts.next().unwrap().parse::<u8>().unwrap();
        let last: u8 = parts.next().unwrap().parse::<u8>().unwrap();
        rules.push(Rule{first:first,last:last,applied:0});
    }

    // Parse the records into a vector of hashmaps
    let mut records: Vec<HashMap<u8,u8>> = vec![];
    for x in records_string {
        let mut record:HashMap<u8,u8> = HashMap::new();
        for (i,y) in x.split(",").enumerate() {
            record.insert(y.parse::<u8>().unwrap(),i as u8);
        }
        records.push(record);
    }

    // Check every record against the rules
    let sum=valid_records_middle_sum(&records,&rules);
    println!("Sum of the middle elements: {}",sum);

    let sum = false_records_corrected_middle_sum(&records,&rules);
    println!("Sum of the middle elements of the corrected records: {}",sum);



    
}

#[derive(Clone, Copy, Debug)]
pub struct Rule {
    pub first: u8,
    pub last: u8,
    pub applied:i8
}

impl Rule {
    pub fn update(&mut self,x:u8){
        if self.applied<0 {
            return;
        }
        else if self.applied==0 {
            if x==self.first{
                self.applied=1
            }
            else if x==self.last {
                self.applied=2
            }
        }
        else if self.applied==1 {
            if x==self.last {
                self.applied=-1
            }
        }
        else if self.applied==2 {
            if x==self.first {
                self.applied=-1
            }
        }
    }

    pub fn applicable(&self) -> bool {
        return self.applied==-1;
    }
}

fn check_record(record:&HashMap<u8,u8>,rules:&Vec<Rule>) -> bool {
    let mut rules_copy=rules.clone();
    // put the keys of the record in a vector
    let keys:Vec<u8>=record.keys().map(|x| *x).collect();

    // update every rule with the record list to see if it is applicable
    for x in keys.iter() {
        for r in rules_copy.iter_mut() {
            r.update(*x);
        }
    }
    // loop over the rules and check the applicable ones
    for r in rules_copy.iter() {
        if r.applicable() {
            if !(record.get(&r.first).unwrap()<record.get(&r.last).unwrap()) {
                return false;
            }
        }
    }
    return true;
}

fn correct_record(record:&HashMap<u8,u8>,rules:&Vec<Rule>) -> HashMap<u8,u8> {
    let mut rules_copy=rules.clone();
    let mut record_copy=record.clone();
    // put the keys of the record in a vector
    let keys:Vec<u8>=record.keys().map(|x| *x).collect();

    // update every rule with the record list to see if it is applicable
    for x in keys.iter() {
        for r in rules_copy.iter_mut() {
            r.update(*x);
        }
    }
    // loop over the rules and check the applicable ones
    for r in rules_copy.iter() {
        if r.applicable() {
            if !(record_copy.get(&r.first).unwrap()<record_copy.get(&r.last).unwrap()) {
                print!("Rule: ");
                println!("{:?}",r);
                let temp=*record_copy.get(&r.first).unwrap();
                record_copy.insert(r.first,*record_copy.get(&r.last).unwrap());
                record_copy.insert(r.last,temp);
                print_record(&record_copy);
            }
        }
    }
    if check_record(&record_copy, &rules){
        return record_copy;
    }
    return correct_record(&record_copy.clone(), &rules);
}

fn get_middle_element(x:HashMap<u8,u8>) -> u8
{
    let n=x.len();
    let index=((n-1)/2) as u8;
    for element in x.iter() {
        if *element.1==index {
            return *element.0;
        }
    }
    return 0;
}

fn valid_records_middle_sum(records:&Vec<HashMap<u8,u8>>,rules:&Vec<Rule>) -> u32 {
    let mut sum=0;
    for x in records.iter() {
        if check_record(&x,&rules) {
            sum+=get_middle_element(x.clone()) as u32;
        }
    }
    return sum;
}

fn false_records_corrected_middle_sum(records:&Vec<HashMap<u8,u8>>,rules:&Vec<Rule>) -> u32 {
    let mut sum=0;
    for x in records.iter() {
        if !check_record(&x,&rules) {
            print!("False record: \t");
            print_record(&x);
            let x=correct_record(x,&rules);
            print!("Corrected record: \t");
            print_record(&x);
            sum+=get_middle_element(x) as u32;
        }
    }
    return sum;
}

fn print_record (record:&HashMap<u8,u8>) {
    let n=record.len();
    let mut record_vec:Vec<u8> =vec![0;n];
    for x in record.iter() {
        record_vec[*x.1 as usize]=*x.0;
    }
    println!("{:?}",record_vec);
}