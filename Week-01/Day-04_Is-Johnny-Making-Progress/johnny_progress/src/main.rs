/*
could not find a way to send arrays of different size as input parameter to the function
*/

fn johnny_progress(miles_history: &[u8]) -> u8{
    println!("slice: {:?}", miles_history);
    
    let mut last_saturday: i16 = i16::from(miles_history[0]);
    let mut progress_days: u8  = 0;

    for i in 1..miles_history.len() {
        let this_saturday: i16 = i16::from(miles_history[i]);
        let difference: i16 = this_saturday - last_saturday;

        // println!("this week: {}, last week: {}, difference: {}", miles_history[i], last_saturday, difference);

        if difference > 0 {
            progress_days += 1;
        }

        last_saturday = i16::from(miles_history[i]);

    }
    // println!("progress: {}", progress_days);
    progress_days
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(johnny_progress(&[3, 4, 1, 2]), 2);
    }
    #[test]
    fn example_2() {
        assert_eq!(johnny_progress(&[10, 11, 12, 9, 10]), 3);
    }
    #[test]
    fn example_3() {
        assert_eq!(johnny_progress(&[6, 5, 4, 3, 2, 9]), 1);
    }
    #[test]
    fn example_4() {
        assert_eq!(johnny_progress(&[9, 9]), 0);
    }


}