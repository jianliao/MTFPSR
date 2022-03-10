use std::io::Write;

pub fn run_eca<W>(
    rule: [bool; 8],
    size: usize,
    steps: usize,
    indices: Vec<usize>,
    write: Option<W>,
) -> ((usize, usize), (usize, usize), usize)
where
    W: Write,
{
    assert!(size >= 1);
    assert!(indices.iter().all(|i| *i < size));

    let step_width = steps.to_string().len();
    let size_width = size.to_string().len();
    let mut write = write;
    let mut print_grid = |step: usize, curr: &[bool], curr_popcnt: usize| match &mut write {
        None => (),
        Some(w) => {
            write!(w, "{:>width$} ", step, width = step_width).unwrap_or_default();
            for b in curr.iter() {
                write!(w, "{}", if *b { '■' } else { '□' }).unwrap_or_default();
            }
            writeln!(w, " {:>width$}", curr_popcnt, width = size_width).unwrap_or_default();
        }
    };

    let mut curr = vec![false; size];
    let mut curr_popcnt = 0;
    for i in indices.into_iter() {
        curr[i] = true;
        curr_popcnt += 1;
    }
    print_grid(0, &curr, curr_popcnt);

    let mut min_popcnt = curr_popcnt;
    let mut max_popcnt = curr_popcnt;
    let mut min_popcnt_step = 0;
    let mut max_popcnt_step = 0;

    let mut next = vec![false; size];

    for step in 1..=steps {
        let mut next_popcnt = 0;
        for i in 0..size {
            let mut rule_idx = 0;
            if curr[if i == 0 { size - 1 } else { i - 1 }] {
                rule_idx += 4;
            }
            if curr[i] {
                rule_idx += 2;
            }
            if curr[if i == size - 1 { 0 } else { i + 1 }] {
                rule_idx += 1;
            }
            next[i] = rule[rule_idx];
            next_popcnt += rule[rule_idx] as usize;
        }
        std::mem::swap(&mut curr, &mut next);
        curr_popcnt = next_popcnt;
        print_grid(step, &curr, curr_popcnt);
        if curr_popcnt < min_popcnt {
            min_popcnt = curr_popcnt;
            min_popcnt_step = step;
        }
        if curr_popcnt > max_popcnt {
            max_popcnt = curr_popcnt;
            max_popcnt_step = step;
        }
    }

    (
        (min_popcnt, min_popcnt_step),
        (max_popcnt, max_popcnt_step),
        curr_popcnt,
    )
}
