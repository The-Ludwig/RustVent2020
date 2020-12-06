use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error + 'static>>{
    let input = fs::read_to_string("input.txt")?;
    
    let values: Vec<_> = input.lines().map(|x| x.parse::<i32>().unwrap()).collect();
    
    let sum = 2020;


    println!("For tuples (part1):");
    for (idx, val1) in values.iter().enumerate() {
        for val2 in &values[idx+1..]{
            if val1+val2 == sum {
                println!("{}*{}={}", val1, val2, val1*val2);
            }
        }
    }

    println!("\n For triples (part2):");
    for (idx1, val1) in values.iter().enumerate() {
        for (idx2, val2) in (&values[idx1+1..]).iter().enumerate(){
            for (_idx3, val3) in (&values[idx1+idx2+1..]).iter().enumerate(){
                if val1+val2+val3 == sum {
                    println!("{}*{}*{}={}", val1, val2,val3, val1*val2*val3);
                }
            }
        }
    }

    Ok(())
}
