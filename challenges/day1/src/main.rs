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
    fn add_assign(&mut self, mut rhs: TurnSequence) {
        println!("Starting AddAssign");
        println!("\tLock Value: {}, TurnSequence: {:?}", self.value, rhs);
        while rhs.value > 100
        {
            // Increase the number of times it passes 0 and decrease the TurnSequence
            self.num_wraps += 1;
            rhs.value -= 100;
            println!("\t\tTurnSequence Decrement: {}, Num Wraps: {}", rhs.value, self.num_wraps);
        }

        match rhs.direction {
            'L' => {
                // Slowly decrement the rhs by 99 while it is over 99

                // L200 -> L111 -> L2

                // Here, the rhs.value should be < 99, so we should be able to subtract it directly
                // V=1, L2
                if self.value - rhs.value < 0
                {
                    println!("\t\tLeft TurnSequence Under 0");
                    rhs.value -= self.value;
                    self.value = 100 - rhs.value;
                    if self.value != 0
                    {
                        self.num_wraps += 1;
                    }
                    println!("\t\tLock Value: {}, TurnSequence: {:?}, Num Wraps: {}", self.value, rhs, self.num_wraps);
                }
                else
                {
                    println!("\t\tLeft Turn Sequence over 0");
                    self.value -= rhs.value;
                    println!("\t\tLock Value: {}, TurnSequence: {:?}", self.value, rhs);
                }
            }

            'R' => {
                // Here, the rhs.value should be < 99, so we should be able to subtract it directly
                // V=59, R=51
                if self.value + rhs.value > 100
                {
                    println!("\t\tRight TurnSequence Over 99");
                    rhs.value += self.value;
                    self.value = rhs.value - 100;
                    if self.value != 0
                    {
                        self.num_wraps += 1;
                    }
                    println!("\t\tLock Value: {}, TurnSequence: {:?}, Num Wraps: {}", self.value, rhs, self.num_wraps);
                }
                else
                {
                    println!("\t\tRight Turn Sequence under 99");
                    self.value += rhs.value;
                    println!("\t\tLock Value: {}, TurnSequence: {:?}", self.value, rhs);
                }
            }

            _ => {}
        }
    }
}

fn main() {
    // Check to see if file path exists to the input2 document
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

                // Step 1
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
