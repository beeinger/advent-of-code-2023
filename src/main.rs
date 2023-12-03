mod day_1;
mod day_2;
mod day_3;

fn main() {
    {
        use day_1::{part_1, part_2};

        println!(
            "Day 1, part 1 answer: {}",
            part_1::sum_calibration(&part_1::get_input())
        );
        println!(
            "Day 1, part 2 answer: {}",
            part_2::sum_calibration(&part_1::get_input())
        );
    }

    {
        use day_2::{part_1, part_2};

        println!(
            "Day 2, part 1 answer: {}",
            part_1::sum_possible_ids(&part_1::get_input())
        );

        println!(
            "Day 2, part 2 answer: {}",
            part_2::sum_powers_of_min_sets(&part_1::get_input())
        );
    }

    {
        use day_3::part_1;

        println!(
            "Day 3, part 1 answer: {}",
            part_1::sum_park_nums(&part_1::get_input())
        );
    }
}
