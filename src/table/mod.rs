use ndarray::{Array2, Ix2};

use vek::vec::repr_c::Vec2;

pub type CharTable = Array2<char>;

pub type TableIdx = Vec2<usize>;

pub type TableDir = Vec2<isize>;

pub fn parse_char_table(input: &str) -> anyhow::Result<CharTable> {
    let mut height = 0;
    let mut width = 0;
    let mut arr = Vec::new();

    for line in input.lines() {
        height += 1;
        if width == 0 {
            width = line.len();
        }
        arr.extend(line.chars());
    }
    Ok(CharTable::from_shape_vec((height, width), arr)?)
}

fn into_shape(idx: TableIdx) -> Ix2 {
    let (a, b) = idx.yx().into_tuple();
    Ix2(a, b)
}

pub fn into_idx(shape: Ix2) -> TableIdx {
    TableIdx {
        x: shape[1],
        y: shape[0]
    }
}

fn shift(idx: TableIdx, step: TableDir) -> TableIdx {
    (idx.as_::<isize>() + step).as_()
}

#[derive(Debug)]
pub struct Ray<'a, A> {
    table: &'a Array2<A>,
    coord: TableIdx,
    direction: TableDir,
    cur: Option<&'a A>
}

impl <'a, A> Iterator for Ray<'a, A> {
    type Item = &'a A;

    fn next(&mut self) -> Option<Self::Item> {

        self.cur = self.table.get(into_shape(self.coord));

        self.coord = shift(self.coord, self.direction);

        self.cur


    }
}

pub fn cast_ray<'a, A>(table: &'a Array2<A>, origin: TableIdx, direction: TableDir) -> Ray<A> {
    Ray {
        table,
        coord: origin,
        direction,
        cur : None
    }
}
 

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    use ndarray::arr2;

    #[fixture]
    fn rect_table() -> CharTable {
        arr2(&[
            ['A', 'B', 'C'],
            ['D', 'E', 'F']
        ])


    }

    #[rstest]
    #[case(TableIdx::new(0, 0), TableDir::new(0, 1), "AD")]
    #[case(TableIdx::new(0, 0), TableDir::new(1, 1), "AE")]
    #[case(TableIdx::new(0, 0), TableDir::new(1, 0), "ABC")]
    fn test_rays(rect_table: CharTable, #[case] origin: TableIdx, #[case] direction: TableDir, #[case] expected: &str) {
        let ray: String = cast_ray(&rect_table, origin, direction).collect();

        assert_eq!(&ray, expected)

    }
}
