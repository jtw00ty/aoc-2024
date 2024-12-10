use std::fs::File;
use std::io::Read;

pub struct Disk {
    pub disk: Vec<Option<usize>>,
    pub next_gap: usize,
    pub last_block: usize,
    pub files: Vec<(usize, usize)>,
    pub gaps: Vec<(usize, usize)>,
}

pub fn read_input<P>(path: P) -> Disk
where
    P: AsRef<std::path::Path>,
{
    let mut input = String::new();
    File::open(path)
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();
    let mut out = Disk {
        disk: vec![],
        next_gap: 0,
        last_block: 0,
        files: vec![],
        gaps: vec![],
    };
    let mut index = 0;
    for (i, char) in input.chars().enumerate() {
        let len = char.to_string().parse::<usize>().unwrap();
        if len == 0 {
            continue;
        }
        if i % 2 == 0 {
            out.disk
                .append(&mut [Some(i / 2)].into_iter().cycle().take(len).collect());
            out.files.push((index, index + len - 1));
        } else {
            out.disk
                .append(&mut [None::<usize>].into_iter().cycle().take(len).collect());
            out.gaps.push((index, index + len - 1));
        }
        index += len;
    }
    out.next_gap = out.disk.iter().position(|id| id.is_none()).unwrap();

    out.last_block = out.disk.iter().rposition(|id| id.is_some()).unwrap();
    println!("{:?}", &out.gaps[0..100]);
    out
}

pub fn refrag(disk: &mut Disk) {
    while disk.next_gap < disk.last_block {
        disk.disk[disk.next_gap] = disk.disk[disk.last_block];
        disk.disk[disk.last_block] = None;
        while disk.disk[disk.last_block].is_none() && disk.last_block > disk.next_gap {
            disk.last_block -= 1;
        }
        while disk.disk[disk.next_gap].is_some() && disk.last_block > disk.next_gap {
            disk.next_gap += 1;
        }
    }
}

pub fn defrag(disk: &mut Disk) {
    for (id, bounds) in disk.files.iter().enumerate().rev() {
        if let Some((ref mut left, _right)) = disk.gaps.iter_mut().find(|(left, right)| {
            (1 + right - left >= 1 + bounds.1 - bounds.0) && &bounds.0 > right
        }) {
            disk.disk[*left..=(*left + bounds.1 - bounds.0)].fill(Some(id));
            *left += bounds.1 - bounds.0 + 1;
            disk.disk[bounds.0..=bounds.1].fill(None);
        }
    }
}

pub fn checksum(disk: &Disk) -> usize {
    disk.disk
        .iter()
        .enumerate()
        .map(|(i, id)| i * id.unwrap_or_default())
        .sum()
}
