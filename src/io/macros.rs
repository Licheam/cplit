#[macro_export]
macro_rules! scanln {
    ($($i:expr), +) => {
        let mut iter = std::iter::repeat_with(|| {
            let mut buf = String::new();
            std::io::stdin().read_line(&mut buf).unwrap();
            buf.split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect::<Vec<_>>()
        })
        .flatten();
        $(
            $i = iter.next().unwrap();
        )*
    };

    ($coll:expr ; $n:expr) => {
        $coll = std::iter::once(Default::default())
                .chain(
                    std::iter::repeat_with(|| {
                        let mut buf = String::new();
                        std::io::stdin().read_line(&mut buf).unwrap();
                        buf.split_whitespace()
                            .map(|x| x.parse().unwrap())
                            .collect::<Vec<_>>()
                    })
                    .flatten()
                    .take($n),
                )
                .collect();
    };

    ($coll:expr ;) => {
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();
        $coll = std::iter::once(Default::default())
                .chain(buf.split_whitespace().map(|x| x.parse().unwrap()))
                .collect();
    };
}