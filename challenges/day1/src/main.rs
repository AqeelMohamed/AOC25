// Imports
use std::fs::File;
use std::str::*;
use std::path::Path;
use std::io::prelude::*;
use std::ops::AddAssign;

// Create struct for `TurnSequence`
#[derive(Debug)]
struct TurnSequence
{
    direction: char,
    value: i32
}

// Impl the `FromStr` Trait for `TurnSequence`
impl FromStr for TurnSequence
{
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err>
    {
        // Initialize base values
        let mut direction: char = ' ';
        let mut numbers: Vec<char> = Vec::new();

        // Iterate over the string
        for c in s.chars()
        {
            if !c.is_whitespace()
            {
                // Set the direction
                if c == 'L' || c == 'R'
                {
                    direction = c;
                }
                else
                {
                    // Add all numbers to the Vec if they are numeric
                    if c.is_numeric()
                    {
                        numbers.push(c);
                    }
                }
            }
        }

        // Build new string from numbers in the vector
        let string: String = numbers.into_iter().collect();
        let value_result = string.parse();

        match value_result
        {
            Ok(value) => Ok(TurnSequence { direction, value }),
            Err(_) => Err(())
        }
    }
}

// Create struct for `Lock`
struct Lock
{
    value: i32,
    num_wraps: i32
}

// Impl block for `Lock`
impl Lock
{
    // Default function to create a new Lock
    fn new() -> Lock
    {
        Lock { value: 50, num_wraps: 0 }
    }
}

impl AddAssign<TurnSequence> for Lock {
    fn add_assign(&mut self, rhs: TurnSequence) {
        let old = self.value;
        let step = rhs.value % 100;

        // TODO: Fix the looping logic in this function
        match rhs.direction {
            'L' => {
                self.num_wraps += (step - old) / 100;

                let mut new = (old - step) % 100;
                if new < 0
                {
                    new += 100;
                }

                self.value = new;
            }

            'R' => {
                self.num_wraps += (old + step) / 100;

                self.value = (old + step) % 100;
            }

            _ => {}
        }
    }
}

fn main() {
    // Check to see if file path exists to the input document
    let file_path = Path::new("input");

    if file_path.exists() {
        // Read the contents of the file
        let mut file = File::open(file_path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        // Initialize variables
        let mut lock = Lock::new();
        let mut num_zeros: i32 = 0;

        // Loop through the string, converting every line to a turn sequence

        for s in contents.split('\n')
        {
            let turn = s.parse::<TurnSequence>();

            if let Ok(t) = turn
            {
                lock += t;

                if lock.value == 0
                {
                    num_zeros += 1;
                }
            }
        }

        // Print out the number of zeros
        println!("{}", lock.num_wraps + num_zeros);
    } else {
        println!("No such file or directory");
    }
}
