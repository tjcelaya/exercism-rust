/*
## Examples

- 28's factors are 1, 2, 4, **7**, 14, 28.
  - In raindrop-speak, this would be a simple "Plong".
- 30's factors are 1, 2, **3**, **5**, 6, 10, 15, 30.
  - In raindrop-speak, this would be a "PlingPlang".
- 34 has four factors: 1, 2, 17, and 34.
  - In raindrop-speak, this would be "34".
# rules
- If the number has 3 as a factor, output 'Pling'.
- If the number has 5 as a factor, output 'Plang'.
- If the number has 7 as a factor, output 'Plong'.
- If the number does not have 3, 5, or 7 as a factor,
  just pass the number's digits straight through.

 */

const DROP_THREE: &'static str = "Pling";
const DROP_FIVE: &'static str = "Plang";
const DROP_SEVEN: &'static str = "Plong";

pub fn raindrops(n: usize) -> String {
    // println!("n is {}", n);
   
    let mut drops: Vec<&'static str> = vec!();

    for c in 1..=n {
        for m in 1..=c {
            println!("c: {}, m: {}", c, m);
            if c * m == n {
                println!("perhaps {} and {} ?", c, m);
                map_and_add(m, &mut drops);

                if c != m {
                    map_and_add(c, &mut drops)
                }
            }
        }
    }

    println!("vec looks like {:?}", drops);
    
    if drops.len() == 0 {
        return n.to_string()
    }

    println!("return looks like {:?}", drops.join(""));
    drops.join("")
}

fn map_and_add(factor: usize, drops: &mut Vec<&'static str>) {
    match factor {
        3 => drops.push(DROP_THREE),
        5 => drops.push(DROP_FIVE),
        7 => drops.push(DROP_SEVEN),
        _ => (),
    }
}

