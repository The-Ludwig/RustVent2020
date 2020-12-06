use std::fs;

// There has to be a method for this in String, but I can't find it RN, so I'll just do this
fn count_iter<T: PartialEq>(f: impl Iterator<Item=T>, val: T) -> usize{
    let mut count = 0;

    for c in f{
        if c == val{
            count += 1;
        }
    }

    return count;
}

#[derive(Debug)]
struct Password {
    min: usize,
    max: usize,
    c: char,
    value: String
}

impl Password {
    pub fn from_string(s: &str) -> Result<Password, Box<dyn std::error::Error + 'static>> {
        let posm = s.find("-").unwrap();
        let posd = s.find(":").unwrap();
        
        Ok(Password{min: s[..posm].parse()?, max: s[posm+1..posd-2].parse()?, c: s.chars().nth(posd-1).unwrap(), value: s[posd+1..].to_string()})
    }

    pub fn check_correct(&self) -> bool {
        let mut count = 0;
        let count = count_iter(self.value.chars(), self.c);
        self.min <= count && self.max >= count
    }

    pub fn check_correct_part_2(&self) -> bool {
        (self.value.chars().nth(self.min).unwrap() == self.c) ^ (self.value.chars().nth(self.max).unwrap() == self.c)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error + 'static>>{
    let input = fs::read_to_string("input.txt")?;

    let values: Vec<_> = input.lines().map(|x| Password::from_string(x).unwrap()).collect();
    
    // println!("{:?}", values);
    let checks = values.iter().map(|x| x.check_correct()).collect::<Vec<_>>();
    // println!("{:?}", checks);
    println!("Number: {}", count_iter(checks.iter(), &true));

    // same for part two: 
    let checks = values.iter().map(|x| x.check_correct_part_2()).collect::<Vec<_>>();
    // println!("{:?}", checks);
    println!("Number part 2: {}", count_iter(checks.iter(), &true));

    println!("{}{}{}{}", false ^ false, true ^ true, true ^ false, false ^ true);

    Ok(())
}
