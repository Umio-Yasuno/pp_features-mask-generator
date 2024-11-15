use std::collections::HashMap;
use std::path::PathBuf;

fn main() {
    let args = std::env::args().skip(1).collect::<Vec<String>>();
    let sysfs = args
        .iter()
        .find(|s| !s.starts_with(&['-', '+']));
    let dump = (sysfs.is_some() && args.len() == 1) || args.is_empty();
    let sysfs = sysfs
        .map(|s| s.clone())
        .unwrap_or(String::from("/sys/class/drm/card0/device"));

    let s = std::fs::read_to_string(PathBuf::from(sysfs).join("pp_features")).unwrap();

    let mut lines = s.lines();
    let first = lines.next().unwrap();
    let mut split = first.split(' ').skip(2);

    let mut mask = {
        /* features high: 0x00003763 low: 0xa37f7dff */
        let Some(high) = split.next().and_then(|mask| u32::from_str_radix(&mask[2..], 16).ok()) else { return };
        let Some(low) = split.skip(1).next().and_then(|mask| u32::from_str_radix(&mask[2..], 16).ok()) else { return };

        ((high as u64) << 32) | low as u64
    };

    let h: HashMap<String, u64> = lines.skip(1).filter_map(|l| {
        /* 33. GFX_DCS              (34) : disabled */
        let mut split = l.split(&[' ', '(', ')']).skip(1);
        let name = split.next()?.to_string();
        let pos = split.skip_while(|s| s.is_empty()).next()?.parse::<u64>().ok()?;

        Some((name, pos))
    }).collect();

    if dump {
        println!("{s}");
        return;
    }

    for arg in args {
        let Some(c) = arg.chars().next() else { continue };

        match c {
            '-' => {
                let pos = h.get(&arg[1..]).unwrap();
                mask &= !(1u64 << pos);
            },
            '+' => {
                let pos = h.get(&arg[1..]).unwrap();
                mask |= 1u64 << pos;
            },
            _ => {},
        }
    }

    println!("{mask:#X}");
}
