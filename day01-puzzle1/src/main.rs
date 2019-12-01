static INPUT: &str = include_str!("../resources/input.txt");

fn main() {
    let sum: u32 = INPUT
        .lines()
        .map(|l| l.parse().unwrap())
        .map(calc_fuel)
        .sum();
    println!("{}", sum);
}

fn calc_fuel(mass: u32) -> u32 {
    mass / 3 - 2
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
    }
}
