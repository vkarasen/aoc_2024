use crate::prelude::*;

use std::str::FromStr;

use std::collections::HashMap;

use lasso::{Spur, RodeoReader, Rodeo};

use nom::{
    character::complete::{newline, one_of, alphanumeric1},
    bytes::complete::{tag, take_while},
    branch::alt,
    combinator::map_res,
    error::Error,
    multi::separated_list1,
    sequence::{separated_pair, terminated, tuple, delimited},
    Finish, IResult,
};

impl AoC for Day {
    fn run(input: &str) -> anyhow::Result<AoCResult> {
        let parsed: Day = input.parse()?;

        Ok(AoCResult {
            part_a : Some(parsed.part_a()),
            part_b : None
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Day {
    device: HashMap<Spur, NodeExpr>,
    rodeo: RodeoReader,
    outputs: Vec<Spur>
}

impl Day {
    fn calc_state(&self, spur: &Spur, cache: &mut HashMap<Spur, bool>) -> bool {

        if let Some(ret) = cache.get(spur) {
            *ret
        } else {
            use NodeExpr::*;
            let ret = match *self.device.get(spur).unwrap() {
                Const(val) => val,
                And(left, right) => self.calc_state(&left, cache) && self.calc_state(&right, cache),
                Or(left, right) => self.calc_state(&left, cache) || self.calc_state(&right, cache),
                Xor(left, right) => self.calc_state(&left, cache) ^ self.calc_state(&right, cache),
            };
            cache.insert(*spur, ret);
            ret
        }

    }

    fn part_a(&self) -> usize {

        let mut cache = HashMap::new();

        let binstr: String = self.outputs.iter().map(|s| {
            match self.calc_state(s, &mut cache) {
                true => '1',
                false => '0'
            }
        }).collect();

        //dbg!(&binstr);

        usize::from_str_radix(&binstr, 2).unwrap()
    }

}

impl FromStr for Day {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Day> {
        match parse_day(s).finish() {
            Ok(("", parsed)) => Ok(parsed),
            Ok((rest, parsed)) => Err(anyhow::anyhow!("Successful parsed {:?}, but input was not fully consumed! ({:?})", parsed, rest)),
            Err(Error { input, code }) => Err(Error {
                input: input.to_string(),
                code,
            }.into()),
        }
    }
}

fn parse_day(input: &str) -> IResult<&str, Day> {
    map_res(
        separated_pair(
            terminated(separated_list1(newline, parse_const), newline),
            newline,
            terminated(separated_list1(newline, parse_gate), newline),
        ),
        |(consts, gates)| -> anyhow::Result<Day> {
            let mut rodeo = Rodeo::default();
            let mut device = HashMap::new();
            let mut outputs = Vec::new();

            for (node, val) in consts {
                device.insert(rodeo.get_or_intern(node), val);
            }

            for ((left, gatestr, right), node) in gates {
                let leftspur = rodeo.get_or_intern(left);
                let rightspur = rodeo.get_or_intern(right);
                let gate = match gatestr {
                    "AND" => NodeExpr::And(leftspur, rightspur),
                    "OR" => NodeExpr::Or(leftspur, rightspur),
                    "XOR" => NodeExpr::Xor(leftspur, rightspur),
                    _ => unreachable!()
                };

                device.insert(rodeo.get_or_intern(node), gate);

                if node.starts_with('z') {
                    outputs.push(node);
                }
            }
            outputs.sort();
            //dbg!(&outputs);
            let outputs = outputs.into_iter().rev().map(|x| rodeo.get_or_intern(x)).collect();
            Ok(Day{
                device,
                outputs, 
                rodeo: rodeo.into_reader()
            })

        }
    )(input)
}

fn parse_const(input: &str) -> IResult<&str, (&str, NodeExpr)> {
    map_res(
        separated_pair(
            alphanumeric1,
            tag(": "),
            one_of("01")
        ),
        |(node, val)| -> anyhow::Result<(&str, NodeExpr)> {
            let b = match val {
                '0' => false,
                '1' => true,
                _ => unreachable!()
            };
            Ok((node, NodeExpr::Const(b)))
        }
    )(input)
}

fn parse_gate(input: &str) -> IResult<&str, ((&str, &str, &str), &str)> {
    separated_pair(
        tuple((
            alphanumeric1,
            delimited(tag(" "), alt((tag("AND"), tag("OR"), tag("XOR"))), tag(" ")),
            alphanumeric1
        )),
        tag(" -> "),
        alphanumeric1
    )(input)
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum NodeExpr{
    Const(bool),
    And(Spur, Spur),
    Or(Spur, Spur),
    Xor(Spur, Spur)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    fn small_example() -> &'static str {
        "\
            x00: 1\n\
            x01: 1\n\
            x02: 1\n\
            y00: 0\n\
            y01: 1\n\
            y02: 0\n\
            \n\
            x00 AND y00 -> z00\n\
            x01 XOR y01 -> z01\n\
            x02 OR y02 -> z02\n\
        "
    }

    fn example() -> &'static str {
        "\
            x00: 1\n\
            x01: 0\n\
            x02: 1\n\
            x03: 1\n\
            x04: 0\n\
            y00: 1\n\
            y01: 1\n\
            y02: 1\n\
            y03: 1\n\
            y04: 1\n\
            \n\
            ntg XOR fgs -> mjb\n\
            y02 OR x01 -> tnw\n\
            kwq OR kpj -> z05\n\
            x00 OR x03 -> fst\n\
            tgd XOR rvg -> z01\n\
            vdt OR tnw -> bfw\n\
            bfw AND frj -> z10\n\
            ffh OR nrd -> bqk\n\
            y00 AND y03 -> djm\n\
            y03 OR y00 -> psh\n\
            bqk OR frj -> z08\n\
            tnw OR fst -> frj\n\
            gnj AND tgd -> z11\n\
            bfw XOR mjb -> z00\n\
            x03 OR x00 -> vdt\n\
            gnj AND wpb -> z02\n\
            x04 AND y00 -> kjc\n\
            djm OR pbm -> qhw\n\
            nrd AND vdt -> hwm\n\
            kjc AND fst -> rvg\n\
            y04 OR y02 -> fgs\n\
            y01 AND x02 -> pbm\n\
            ntg OR kjc -> kwq\n\
            psh XOR fgs -> tgd\n\
            qhw XOR tgd -> z09\n\
            pbm OR djm -> kpj\n\
            x03 XOR y03 -> ffh\n\
            x00 XOR y04 -> ntg\n\
            bfw OR bqk -> z06\n\
            nrd XOR fgs -> wpb\n\
            frj XOR qhw -> z04\n\
            bqk OR frj -> z07\n\
            y03 OR x01 -> nrd\n\
            hwm AND bqk -> z03\n\
            tgd XOR rvg -> z12\n\
            tnw OR pbm -> gnj\n\
        "
    }

    #[rstest]
    #[case(small_example(), 4)]
    #[case(example(), 2024)]
    fn test_part_a_small(#[case] test: &'static str, #[case] cmp: usize) {
        let parsed: Day = test.parse().unwrap();
        assert_eq!(parsed.part_a(), cmp)
    }
}
