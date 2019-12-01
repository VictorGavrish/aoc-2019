static INPUT: &str = include_str!("../resources/input.txt");

fn main() {
    let sum: i64 = INPUT
        .lines()
        .map(|l| l.parse().unwrap())
        .map(calc_fuel_with_fuel)
        .sum();
    println!("{}", sum);
}

fn calc_fuel(mass: i64) -> i64 {
    mass / 3 - 2
}

fn calc_fuel_with_fuel(mass: i64) -> i64 {
    let mut total = calc_fuel(mass);
    let mut to_add = total;
    loop {
        to_add = calc_fuel(to_add);
        if to_add <= 0 {
            break;
        }
        total += to_add;
    }
    total
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tests() {
        assert_eq!(calc_fuel(12), 2);
        assert_eq!(calc_fuel(14), 2);
        assert_eq!(calc_fuel(1969), 654);
        assert_eq!(calc_fuel(100756), 33583);

        assert_eq!(calc_fuel_with_fuel(14), 2);
        assert_eq!(calc_fuel_with_fuel(1969), 966);
        assert_eq!(calc_fuel_with_fuel(100756), 50346);
    }
}
