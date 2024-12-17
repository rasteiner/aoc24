#[macro_export]
macro_rules! days {
    ($($day:ident),*) => {
        $(mod $day;)*
        static DAYS: LazyLock<Vec<Day>> = LazyLock::new(|| {
            let parts: Vec<(&str, fn(&String) -> Box<dyn ToString>, fn(&String) -> Box<dyn ToString>)> = vec![
                $(
                    (stringify!($day), $day::part1, $day::part2),
                )*
            ];

            parts.into_iter().map(|(d, part1, part2)| Day { 
                num: d.trim_start_matches("d").parse().unwrap(),
                part1,
                part2,
            }).collect()
        });
    };
}