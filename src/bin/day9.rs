#[derive(Debug, PartialEq, Clone, Copy)]
enum DiskMapItemKind {
    File,
    FreeSpace,
}

#[derive(Debug)]
struct DiskMapItem {
    kind: DiskMapItemKind,
    id: Option<usize>,
    size: u32,
}

fn get_disk_map(input: &str) -> Vec<DiskMapItem> {
    let mut disk_map: Vec<DiskMapItem> = vec![];

    let mut id: usize = 0;
    for (i, ch) in input.trim().chars().enumerate() {
        let kind = if i % 2 == 0 {
            DiskMapItemKind::File
        } else {
            DiskMapItemKind::FreeSpace
        };
        let new_id = if kind == DiskMapItemKind::File {
            Some(id)
        } else {
            None
        };

        let size = ch.to_digit(10).expect("Char was not 0-9 {}");

        for _ in 0..size {
            disk_map.push(DiskMapItem {
                id: new_id,
                kind,
                size,
            });
        }

        if kind == DiskMapItemKind::File {
            id += 1
        }
    }

    disk_map
}

fn part_one(input: &str) -> usize {
    let mut disk_map = get_disk_map(input);

    let mut start_ptr = 0;
    let mut end_ptr = disk_map.len() - 1;

    while start_ptr < end_ptr {
        match (disk_map[start_ptr].kind, disk_map[end_ptr].kind) {
            (DiskMapItemKind::File, DiskMapItemKind::File) => {
                start_ptr += 1;
            }
            (DiskMapItemKind::File, DiskMapItemKind::FreeSpace) => {
                start_ptr += 1;
                end_ptr -= 1;
            }
            (DiskMapItemKind::FreeSpace, DiskMapItemKind::File) => {
                disk_map.swap(start_ptr, end_ptr);
                start_ptr += 1;
                end_ptr -= 1;
            }
            (DiskMapItemKind::FreeSpace, DiskMapItemKind::FreeSpace) => {
                end_ptr -= 1;
            }
        }
    }

    disk_map
        .iter()
        .enumerate()
        .map(|(i, disk)| match disk.kind {
            DiskMapItemKind::File => i * disk.id.expect("File must have id"),
            DiskMapItemKind::FreeSpace => 0,
        })
        .sum::<usize>()
}

fn part_two(input: &str) -> usize {
    let mut disk_map = get_disk_map(input);

    let mut end_ptr = disk_map.len() - 1;

    while end_ptr > 1 {
        if disk_map[end_ptr].kind == DiskMapItemKind::FreeSpace {
            end_ptr -= 1;
            continue;
        }

        let end_disk = &disk_map[end_ptr];
        let end_disk_size = end_disk.size as usize;

        let mut found = false;
        for i in 0..end_ptr - end_disk_size {
            if disk_map[i].kind != DiskMapItemKind::FreeSpace {
                continue;
            }

            let start_slice = &disk_map[i..i + end_disk_size];

            if start_slice
                .iter()
                .all(|disk| disk.kind == DiskMapItemKind::FreeSpace)
            {
                for j in 0..end_disk_size {
                    disk_map.swap(i + j, end_ptr - j);
                }

                found = true;
                end_ptr -= end_disk_size;
                break;
            }
        }

        if !found {
            end_ptr -= end_disk_size;
        }
    }

    disk_map
        .iter()
        .enumerate()
        .map(|(i, disk)| match disk.kind {
            DiskMapItemKind::File => i * disk.id.expect("File must have id"),
            DiskMapItemKind::FreeSpace => 0,
        })
        .sum::<usize>()
}

fn main() {
    let input = include_str!("../../inputs/day9.txt");

    println!("Part 1: {}", part_one(input));
    println!("Part 2: {}", part_two(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_d9() {
        let input = r"2333133121414131402";
        let res = part_one(input);

        assert_eq!(res, 1928)
    }

    #[test]
    fn test_part_two_d9() {
        let input = r"2333133121414131402";
        let res = part_two(input);

        assert_eq!(res, 2858)
    }
}
