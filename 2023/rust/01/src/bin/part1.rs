use day01::add;

fn main() {
    println!("Hello part1")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(add(1,3), 4);
    }
}
