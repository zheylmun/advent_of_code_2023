use chrono::Utc;

mod day_1;
mod day_2;
mod day_3;
fn main() {
    let start_time = Utc::now();
    println!("Day 1 Pt. 1: {}", day_1::day_1());
    println!("Day 1 Pt. 2: {}", day_1::day_1_pt_2());
    println!("Day 2 Pt. 1: {}", day_2::day_2_pt_1());
    println!("Day 2 Pt. 2: {}", day_2::day_2_pt_2());
    println!("Day 3 Pt. 1: {}", day_3::day_3_pt_1());
    println!("Day 3 Pt. 2: {}", day_3::day_3_pt_2());
    let runtime = Utc::now() - start_time;
    print!(
        "All puzzles solved in: {:?} microseconds",
        runtime.num_microseconds()
    )
}
