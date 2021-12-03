use aocf::Aoc;

fn main() {
    let mut aoc = Aoc::new()
        .year(Some(2021))
        .day(Some(1))
        .init()
        .unwrap();

    // Get input data (don't force)
    let input = aoc.get_input(false);

    if let Ok(i) = input {
        let numbers: Vec<usize> = i.split("\n").map(|s| s.trim())   
              .filter(|s| !s.is_empty()) 
              .map(|s| s.parse().unwrap())  
              .collect();

            
        let mut count = 0;

        for n in 1..numbers.len() {
            if numbers[n-1] < numbers[n] {
                count += 1;
            } 
        }

        println!("{}", count)
    }
}