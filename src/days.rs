#[macro_export]
macro_rules! days {
    ($($day:ident),*) => {
        $(mod $day;)*
        static DAYS: LazyLock<Vec<Day>> = LazyLock::new(|| {
            let parts: Vec<(Part, Part)> = vec![
                $(
                    ($day::part1, $day::part2),
                )*
            ];

            parts.into_iter().enumerate().map(|(i, (part1, part2))| Day { 
                num: i + 1,
                part1,
                part2,
            }).collect()
        });
    };
}