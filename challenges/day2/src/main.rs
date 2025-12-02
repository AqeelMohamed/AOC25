use std::fs::File;
use std::str::*;
use std::path::Path;
use std::io::prelude::*;
use std::ops::Range;
use regex::Regex;


fn main()
{
    let file_path = Path::new("input");

    if file_path.exists() {
        // Read the contents of the file
        let mut file = File::open(file_path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        // Replace all newlines in the file's input2
        contents.retain(|c| c != '\n' && c != '\r');

        // Variable Initialization for Step 2
        let re = Regex::new(r"(\d+)-(\d+)$").unwrap();
        let mut groups: Vec<i64> = Vec::new();

        // Split by commas
        let mut ranges: Vec<Range<i64>> = Vec::new();
        for range in contents.split(",")
        {
            println!("{}", range);
            // Use regex to capture the start and end for a specified range
            if let Some(group) = re.captures(range)
            {
                // Initialize the starting index
                let starting_index: &str = group.get(1).unwrap().as_str();
                let ending_index: &str = group.get(2).unwrap().as_str();

                // Handle logic for the grouping
                let _range = starting_index.parse::<i64>().unwrap()..(ending_index.parse::<i64>().unwrap() + 1);
                ranges.push(_range);
            }
        }

        // Handles ranges
        for range in ranges
        {
            for num in range.start..range.end
            {
                // Create a string and denote the max length for a group (group can be size 1 to max_group_len)
                let num_str = num.to_string();
                let max_group_len = num_str.len() / 2;

                // Now we need to check occurrences from 0..max_group_len
                for group_len in 1..=max_group_len
                {
                    // Create variable for pattern
                    let pattern = &num_str[0..group_len];

                    // Find all matches for the string from 0..group_len within num_str
                    let repeats = num_str.len() / group_len;
                    let candidate = pattern.repeat(repeats);

                    if candidate == num_str {
                        // println!("{} is made of repeated group '{}'", num_str, pattern);
                        groups.push(num);
                        break;
                    }

                }
            }
        }

        // Print out groups (if needed)
        // println!("{:?}", groups);

        // Get the value of the total contents of groups
        let total_value = groups.iter().fold(0, |acc, g| acc + g);
        println!("{}", total_value);
    }
}

fn step1() {
    let file_path = Path::new("input");

    if file_path.exists() {
        // Read the contents of the file
        let mut file = File::open(file_path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        // Replace all newlines in the file's input2
        contents.retain(|c| c != '\n' && c != '\r');

        // Variable Initialization for Step 1
        let re = Regex::new(r"(\d+)-(\d+)$").unwrap();
        let mut groups: Vec<i64> = Vec::new();

        // Split by commas
        for range in contents.split(",")
        {
            println!("{}", range);
            // Use regex to capture the start and end for a specified range
            if let Some(group) = re.captures(range)
            {
                // Initialize the starting index
                let starting_index: &str = group.get(1).unwrap().as_str();
                let ending_index: &str = group.get(2).unwrap().as_str();

                // Handle logic for the grouping
                // Brute force solution
                let range = starting_index.parse::<i64>().unwrap()..(ending_index.parse::<i64>().unwrap() + 1);
                for num in range
                {
                    let num_str = num.to_string();
                    if num_str.len() % 2 == 0
                    {
                        // Cast to a &str
                        let range_str = num_str;
                        let group_half_len = range_str.len() / 2;

                        // A group repeats twice if the range_str[0..group_half_len] == range_str[group_half_len..]
                        if range_str[0..group_half_len] == range_str[group_half_len..]
                        {
                            groups.push(num);
                        }
                    }
                }
            }
        }

        // Print out the groups
        println!("{:?}", groups);

        // Get the value of the total contents of groups
        let total_value = groups.iter().fold(0, |acc, g| acc + g);
        println!("{}", total_value);
    }
}