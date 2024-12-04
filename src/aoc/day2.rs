use crate::helpers;

pub fn part_a(path: &str) -> anyhow::Result<i32> {
    let inputs = helpers::open_inputs(path)?;

    let mut safe_reports = 0;
    let rows = inputs.split('\n');
    for r in rows {
        let levels: Vec<i32> = r
            .split_whitespace()
            .into_iter()
            .map(|i| i.parse().unwrap())
            .collect::<Vec<i32>>();

        if is_safe(&levels) {
            safe_reports += 1;
        }
    }
    Ok(safe_reports)
}

pub fn part_b(path: &str) -> anyhow::Result<i32> {
    let inputs = helpers::open_inputs(path)?;

    let mut safe_reports = 0;
    let rows = inputs.split('\n');
    for r in rows {
        let levels: Vec<i32> = r
            .split_whitespace()
            .into_iter()
            .map(|i| i.parse().unwrap())
            .collect::<Vec<i32>>();

        if is_safe(&levels) {
            safe_reports += 1;
            continue;
        }

        for i in 0..levels.len() {
            let mut reduced_levels = levels.clone();
            reduced_levels.remove(i);
            if is_safe(&reduced_levels) {
                safe_reports += 1;
                break;
            }
        }
    }
    Ok(safe_reports)
}

fn is_safe(levels: &Vec<i32>) -> bool {
    let differences: Vec<i32> = levels
        .windows(2)
        .map(|window| window[0] - window[1])
        .collect();

    if differences.len() == 0 {
        return false;
    }
    if differences.iter().all(|&x| x > 0 && x < 4) {
        return true;
    }

    return differences.iter().all(|&x| x < 0 && x > -4);
}
