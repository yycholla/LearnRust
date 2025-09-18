use std::io;

mod db;
fn main() {
    let mut sum = 0;
    const WEEK: i32 = 45 * 60;
    println!("Time Tracker - Enter times as hh:mm (e.g., 8:30)");
    println!("If all days have been entered leave blank");

    loop {
        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        let user_input = user_input.trim();

        if user_input.is_empty() {
            break;
        }

        match parse_time(user_input) {
            Some(minutes) => {
                sum += minutes;
                let hours = minutes / 60;
                let mins = minutes % 60;
                let total_hours = sum / 60;
                let total_mins = sum % 60;
                let remaining_mins = WEEK - sum;
                let remaining_hours = remaining_mins / 60;
                let remaining_mins = remaining_mins % 60;
                println!(
                    "Added {}h {}m | running total: {}h {}m | Remaining {}h {}m",
                    hours, mins, total_hours, total_mins, remaining_hours, remaining_mins
                );
            }
            None => {
                println!("Invalid time format");
            }
        }
    }

    let total_hours = sum / 60;
    let remaining_mins = sum % 60;
    println!("\nFinal total: {}h {}m", total_hours, remaining_mins);
}

fn parse_time(input: &str) -> Option<i32> {
    let parts: Vec<&str> = input.split(':').collect();

    if parts.len() != 2 {
        return None;
    }

    let hours: i32 = match parts[0].parse() {
        Ok(h) => h,
        Err(_) => return None,
    };

    let minutes: i32 = match parts[1].parse() {
        Ok(m) => m,
        Err(_) => return None,
    };

    if !(0..=24).contains(&hours) || !(0..=59).contains(&minutes) {
        return None;
    }
    Some(hours * 60 + minutes)
}

#[cfg(test)]
mod tests {
    #[test]
    fn check_parse_time() {
        let string = "23:32";
        assert_eq!(super::parse_time(string), Some(23 * 60 + 32));

        let string = "23:32:45";
        assert_eq!(super::parse_time(string), None);

        let string = "23:32:45:67";
        assert_eq!(super::parse_time(string), None);

        let string = "23";
        assert_eq!(super::parse_time(string), None);

        let string = "25:32:";
        assert_eq!(super::parse_time(string), None);

        let string = "23:60:";
        assert_eq!(super::parse_time(string), None);

        let string = "23:32:";
        assert_eq!(super::parse_time(string), None);

        let string = "2132";
        assert_eq!(super::parse_time(string), None);
    }
}
