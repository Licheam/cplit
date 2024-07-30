#[macro_export]
macro_rules! scanln {
    ($($i:expr), +) => {
        fscanln!(std::io::stdin(), $($i), +);
    };

    ($coll:expr ; $n:expr) => {
        fscanln!(std::io::stdin(), $coll ; $n);
    };

    ($coll:expr ;) => {
        fscanln!(std::io::stdin(), $coll ;);
    };
}

#[macro_export]
macro_rules! fscanln {
    ($reader:expr, $($i:expr), +) => {
        use std::io::BufRead;
        let mut iter = std::iter::repeat_with(|| {
            let mut buf = String::new();
            $reader.read_line(&mut buf).unwrap();
            buf.split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect::<Vec<_>>()
        })
        .flatten();
        $(
            $i = iter.next().unwrap();
        )*
    };

    ($reader:expr, $coll:expr ; $n:expr) => {
        use std::io::BufRead;
        $coll = std::iter::once(Default::default())
                .chain(
                    std::iter::repeat_with(|| {
                        let mut buf = String::new();
                        $reader.read_line(&mut buf).unwrap();
                        buf.split_whitespace()
                            .map(|x| x.parse().unwrap())
                            .collect::<Vec<_>>()
                    })
                    .flatten()
                    .take($n),
                )
                .collect();
    };

    ($reader:expr, $coll:expr ;) => {
        use std::io::BufRead;
        let mut buf = String::new();
        $reader.read_line(&mut buf).unwrap();
        $coll = std::iter::once(Default::default())
                .chain(buf.split_whitespace().map(|x| x.parse().unwrap()))
                .collect();
    };
}
