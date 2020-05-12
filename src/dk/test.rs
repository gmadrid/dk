macro_rules! chart_str {
        ($($line:expr),*) => {{
            let mut s = String::default();
            s.push_str("CHART\n");
            $(
                s.push_str($line);
                s.push_str("\n");
            )*
            s
        }}
    }

macro_rules! chart {
        ($($line:expr),*) => {{
            let mut s = String::default();
            s.push_str("CHART\n");
            $(
                s.push_str($line);
                s.push_str("\n");
            )*
            Chart::read(&mut BufReader::new(s.as_bytes()))
        }}
    }
