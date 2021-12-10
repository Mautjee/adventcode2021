use std::fs::File;
use std::io::{Read, BufRead, BufReader};

fn main() {
    let mut list : Vec<u64> = match read_a_file(){
        Ok(x) => x,
        Err(_e) => panic!("Could not load the file")
    };
    
    let mut counter : u64 = 0;
    
    let mut previous : u64 = list[0];
    println!("{} (N/A - No previous measurment)", previous);

    list.remove(0);

    let mut differencePrev : u64 = 0;
 
    
    for i in &list {
        if i > &previous{
        
            counter = counter + 1;
            println!("{} (increased)",i);
            previous = *i;
        }else{
            previous = *i;
            println!("{} (decreased)",i);
        }
    }
    println!(" The end counter = {:?}", &counter);
}

fn read_a_file() -> std::io::Result<Vec<u64>> {
    
    let mut file = File::open("data.txt").expect("Could not open the file");
    
    let reader = BufReader::new(file);

    let data : Vec<u64> = reader
        .lines()
        .map(|line|line.unwrap().parse::<u64>().unwrap())
        .collect();

    return Ok(data);
}