use crate::prelude::*;

use anyhow::anyhow;

use std::str::FromStr;

impl AoC for Day {
    fn run(input: &str) -> anyhow::Result<AoCResult> {
        let parsed: Day = input.parse()?;

        Ok(AoCResult {
            part_a: Some(parsed.part_a()),
            part_b: Some(parsed.part_b()),
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Day {
    values: Vec<u32>,
    fs: FileSystem,
}

impl Day {
    fn part_a(&self) -> usize {
        self.fs.defrag_block().checksum()
    }
    fn part_b(&self) -> usize {
        self.fs.defrag_all_files().checksum()
    }
}

impl FromStr for Day {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Day> {
        let values = s
            .trim_end()
            .chars()
            .map(|c| c.to_digit(10).ok_or(anyhow!("{} is not a digit", c)))
            .collect::<anyhow::Result<Vec<u32>>>()?;
        let fs: FileSystem = values.clone().into();
        fs.info();
        Ok(Day { values, fs })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct FileSystem {
    entries: Vec<Entry>,
}

impl FileSystem {
    fn checksum(&self) -> usize {
        let mut blockidx = 0;
        let mut retval = 0;
        for entry in &self.entries {
            for _ in 0..entry.size {
                if let Some(val) = entry.id {
                    retval += blockidx * val;
                }
                blockidx += 1;
            }
        }
        retval
    }

    fn info(&self) {
        let mut data = 0usize;
        let mut empty = 0usize;
        let mut total = 0usize;

        for e in &self.entries {
            if e.id.is_some() {
                data += e.size;
            } else {
                empty += e.size;
            }
            total += e.size;
        }
        println!("data: {}, empty: {}, total: {}", data, empty, total);
    }

    fn defrag_all_files(&self) -> Self {
        let last_file_idx = self.entries.iter().rev().flat_map(|e| e.id).next().unwrap();
        let mut ret = self.clone();
        for i in (1..=last_file_idx).rev() {
            ret = ret.defrag_file(i);
        }
        ret.info();
        ret
    }

    fn defrag_file(&self, id: usize) -> Self {
        let mut it = self.entries.iter();
        let mut tail = Vec::new();
        let mut entries = Vec::new();

        let copy_from: Entry = {
            let mut ret = Entry::default();
            while let Some(entry) = it.next_back() {
                if entry.id == Some(id) {
                    ret = *entry;
                    break
                } else {
                    tail.push(*entry);
                }
            }
            ret
        };

        let copy_to: Entry = {
            let mut ret = Entry::default();
            for entry in it.by_ref() {
                if entry.id.is_none() && entry.size >= copy_from.size {
                    ret = *entry;
                    break
                } else {
                    entries.push(*entry);
                }
            }
            ret
        };

        match (copy_from.size, copy_to.size) {
            (0, _) => unreachable!(),
            (_, 0) => {
                // do nothing, just restore the clone
                entries.push(copy_from);
            },
            (from_size, mut to_size) => {
                let size = from_size.min(to_size);
                entries.push(Entry { size, id : copy_from.id});
                to_size -= size;
                if to_size > 0 {
                    entries.push(Entry { size: to_size, id : None});
                }
                tail.push(Entry{ size: from_size, id: None});
                entries.extend(&mut it);
            }
        }

        entries.extend(tail.iter().rev());
        Self { entries }
    }


    fn defrag_block(&self) -> Self {
        let mut entries = Vec::new();
        let mut it = self.entries.iter();
        let mut copy_from = Entry::default();
        let mut copy_to = Entry::default();
        loop {
            // find blocks that need copying, i.e have at least one empty block in between

            if copy_from.size == 0 {
                //println!("looking for new copy_from");
                while let Some(e) = it.next_back() {
                    if e.id.is_some() {
                        copy_from = *e;
                        break;
                    }
                }
            }

            //get space to copy to, move entries en passant

            if copy_to.size == 0 {
                //println!("looking for new copy_to");
                for e in it.by_ref() {
                    if e.id.is_some() {
                        entries.push(*e);
                    } else {
                        copy_to = *e;
                        break;
                    }
                }
            }

            match (copy_to.size, copy_from.size) {
                (_, 0) => { break; },
                (0, _) => { 
                    entries.push(copy_from);
                    break;
                },
                (a, b) => {
                    let size = a.min(b);
                    let commit = Entry { size , id : copy_from.id };
                    entries.push(commit);
                    copy_to.size -= size;
                    copy_from.size -= size;
                }
            }
        }
        let ret = Self { entries };
        ret.info();
        ret
    }
}

impl std::fmt::Display for FileSystem {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut s = String::new();
        for entry in &self.entries {
            match entry.as_chars() {
                Ok(chars) => {
                    s.extend(chars);
                }
                Err(e) => {
                    s += &format!("Error: {}", e);
                    break;
                }
            }
        }
        write!(f, "{}", &s)
    }
}

impl std::convert::From<Vec<u32>> for FileSystem {
    fn from(value: Vec<u32>) -> Self {
        let mut entries = Vec::new();
        for (idx, chunk) in value.chunks(2).enumerate() {
            entries.push(Entry {
                size: chunk[0] as usize,
                id: Some(idx),
            });
            if chunk.len() == 2 && chunk[1] > 0 {
                entries.push(Entry {
                    size: chunk[1] as usize,
                    id: None,
                });
            }
        }
        FileSystem { entries }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Default, Copy)]
struct Entry {
    size: usize,
    id: Option<usize>,
}

impl Entry {
    fn as_chars(&self) -> anyhow::Result<impl Iterator<Item = char>> {
        let elemc: char = match self.id {
            Some(val) => char::from_digit(val as u32, 10).ok_or(anyhow!("{} not a digit", val)),
            None => Ok('.'),
        }?;
        Ok(std::iter::repeat_n(elemc, self.size))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    fn example() -> &'static str {
        "\
        2333133121414131402\n\
        "
    }

    #[fixture]
    fn example_parsed() -> Day {
        Day {
            values: [2, 3, 3, 3, 1, 3, 3, 1, 2, 1, 4, 1, 4, 1, 3, 1, 4, 0, 2].into(),
            fs: FileSystem {
                entries: [
                    Entry {
                        size: 2,
                        id: Some(0),
                    },
                    Entry { size: 3, id: None },
                    Entry {
                        size: 3,
                        id: Some(1),
                    },
                    Entry { size: 3, id: None },
                    Entry {
                        size: 1,
                        id: Some(2),
                    },
                    Entry { size: 3, id: None },
                    Entry {
                        size: 3,
                        id: Some(3),
                    },
                    Entry { size: 1, id: None },
                    Entry {
                        size: 2,
                        id: Some(4),
                    },
                    Entry { size: 1, id: None },
                    Entry {
                        size: 4,
                        id: Some(5),
                    },
                    Entry { size: 1, id: None },
                    Entry {
                        size: 4,
                        id: Some(6),
                    },
                    Entry { size: 1, id: None },
                    Entry {
                        size: 3,
                        id: Some(7),
                    },
                    Entry { size: 1, id: None },
                    Entry {
                        size: 4,
                        id: Some(8),
                    },
                    Entry {
                        size: 2,
                        id: Some(9),
                    },
                ]
                .into(),
            },
        }
    }

    #[rstest]
    fn parse_example_a(example: &'static str, example_parsed: Day) {
        let result: Day = example.parse().unwrap();
        assert_eq!(result, example_parsed)
    }

    #[rstest]
    fn test_part_a(example_parsed: Day) {
        assert_eq!(example_parsed.part_a(), 1928)
    }

    #[rstest]
    fn test_part_b(example_parsed: Day) {
        assert_eq!(example_parsed.part_b(), 2858)
    }

    #[rstest]
    fn test_string_parsed(example_parsed: Day) {
        assert_eq!(
            format!("{}", example_parsed.fs),
            "00...111...2...333.44.5555.6666.777.888899"
        )
    }

    #[rstest]
    fn test_defrag_block(example_parsed: Day) {
        assert_eq!(
            format!("{}", example_parsed.fs.defrag_block()),
            "0099811188827773336446555566"
        )
    }

    #[rstest]
    fn test_defrag_file(example_parsed: Day) {
        assert_eq!(
            format!("{}", example_parsed.fs.defrag_all_files()),
            "00992111777.44.333....5555.6666.....8888.."
        )
    }
}
