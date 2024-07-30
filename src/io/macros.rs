#[macro_export]
macro_rules! fscanln {
    ($reader:expr, $($i:expr), +) => {
        #[allow(unused_imports)]
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
        #[allow(unused_imports)]
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
        #[allow(unused_imports)]
        use std::io::BufRead;
        let mut buf = String::new();
        $reader.read_line(&mut buf).unwrap();
        $coll = std::iter::once(Default::default())
                .chain(buf.split_whitespace().map(|x| x.parse().unwrap()))
                .collect();
    };
}

#[macro_export]
macro_rules! scanln {
    ($($i:expr), +) => {
        $crate::fscanln!(std::io::stdin(), $($i), +);
    };

    ($coll:expr ; $n:expr) => {
        $crate::fscanln!(std::io::stdin(), $coll ; $n);
    };

    ($coll:expr ;) => {
        $crate::fscanln!(std::io::stdin(), $coll ;);
    };
}
