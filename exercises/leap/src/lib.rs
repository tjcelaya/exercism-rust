/*
on every year that is evenly divisible by 4
  except every year that is evenly divisible by 100
    unless the year is also evenly divisible by 400
 */
pub fn is_leap_year(year: i32) -> bool {
    println!("Year is %d", year);
    let by4 = year % 4 == 0;
    let by100 = year % 100 == 0;
    let by400 = year % 400 == 0;

    by4 && !by100 || by400
}
