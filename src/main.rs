//! A tool for taking a list of numbers from stdin and producing a histogram.

use std::io::{self, BufRead};

const fn bin_size(nbins: usize, min: usize, max: usize) -> usize {
    (max - min) / nbins
}

/// Given a histogram with `nbins` and a min/max value of `min`/`max`, return the index of `value` in the
/// historgram.
const fn hist_bin(nbins: usize, min: usize, max: usize, value: usize) -> usize {
    if value < min {
        0
    } else if value >= max {
        nbins - 1
    } else {
        let bin_size = bin_size(nbins, min, max);
        let adjusted_value = value - min;

        adjusted_value / bin_size
    }
}

fn main() -> io::Result<()> {
    let mut args = std::env::args().skip(1);

    let nbins = args
        .next()
        .expect("Number of bins expected.")
        .parse::<usize>()
        .expect("Unable to parse as integer");
    let min = args
        .next()
        .expect("Min value expected.")
        .parse::<usize>()
        .expect("Unable to parse as integer");
    let max = args
        .next()
        .expect("Max value expected.")
        .parse::<usize>()
        .expect("Unable to parse as integer");

    let mut hist = vec![0; nbins];

    // Read contents
    let mut stdin = io::stdin().lock();
    let mut string = String::new();
    while let Ok(bytes) = stdin.read_line(&mut string) {
        // EOF
        if bytes == 0 {
            break;
        }

        let value = match string.trim().parse::<usize>() {
            Ok(value) => value,
            Err(_) => panic!("Unable to parse as integer: {string:?}"),
        };

        // Add to the right bin.
        hist[hist_bin(nbins, min, max, value)] += 1;

        string.clear();
    }

    // Print output
    let total = hist.iter().sum::<usize>() as f64;
    let mut so_far = 0.0;
    for (i, bin) in hist.iter().enumerate() {
        let mut bin_lo_val = (min + bin_size(nbins, min, max) * i) as f64;
        let mut bin_hi_val = (min + bin_size(nbins, min, max) * (i + 1)) as f64;

        if i == 0 {
            bin_lo_val = 0.0;
        } else if i == nbins - 1 {
            bin_hi_val = f64::INFINITY;
        }

        let bin_pct = *bin as f64 / total * 100.0;
        let next_pct = so_far + bin_pct;

        println!("[{so_far:5.1}%, {next_pct:5.1}%)\t[{bin_lo_val:10}, {bin_hi_val:10})\t{bin}\t({bin_pct:0.1}%)");

        so_far = next_pct;
    }

    Ok(())
}
