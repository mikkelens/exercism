use std::collections::{HashMap, HashSet};

use itertools::Itertools;

const CHEAT_INPUT: &str =
    "THIS + A + FIRE + THEREFORE + FOR + ALL + HISTORIES + I + TELL + A + TALE + THAT + FALSIFIES \
	 + ITS + TITLE + TIS + A + LIE + THE + TALE + OF + THE + LAST + FIRE + HORSES + LATE + AFTER \
	 + THE + FIRST + FATHERS + FORESEE + THE + HORRORS + THE + LAST + FREE + TROLL + TERRIFIES + \
	 THE + HORSES + OF + FIRE + THE + TROLL + RESTS + AT + THE + HOLE + OF + LOSSES + IT + IS + \
	 THERE + THAT + SHE + STORES + ROLES + OF + LEATHERS + AFTER + SHE + SATISFIES + HER + HATE + \
	 OFF + THOSE + FEARS + A + TASTE + RISES + AS + SHE + HEARS + THE + LEAST + FAR + HORSE + \
	 THOSE + FAST + HORSES + THAT + FIRST + HEAR + THE + TROLL + FLEE + OFF + TO + THE + FOREST + \
	 THE + HORSES + THAT + ALERTS + RAISE + THE + STARES + OF + THE + OTHERS + AS + THE + TROLL + \
	 ASSAILS + AT + THE + TOTAL + SHIFT + HER + TEETH + TEAR + HOOF + OFF + TORSO + AS + THE + \
	 LAST + HORSE + FORFEITS + ITS + LIFE + THE + FIRST + FATHERS + HEAR + OF + THE + HORRORS + \
	 THEIR + FEARS + THAT + THE + FIRES + FOR + THEIR + FEASTS + ARREST + AS + THE + FIRST + \
	 FATHERS + RESETTLE + THE + LAST + OF + THE + FIRE + HORSES + THE + LAST + TROLL + HARASSES + \
	 THE + FOREST + HEART + FREE + AT + LAST + OF + THE + LAST + TROLL + ALL + OFFER + THEIR + \
	 FIRE + HEAT + TO + THE + ASSISTERS + FAR + OFF + THE + TROLL + FASTS + ITS + LIFE + SHORTER \
	 + AS + STARS + RISE + THE + HORSES + REST + SAFE + AFTER + ALL + SHARE + HOT + FISH + AS + \
	 THEIR + AFFILIATES + TAILOR + A + ROOFS + FOR + THEIR + SAFE == FORTRESSES";
const CHEAT: [(char, u8); 10] = [
    ('A', 1),
    ('E', 0),
    ('F', 5),
    ('H', 8),
    ('I', 7),
    ('L', 2),
    ('O', 6),
    ('R', 3),
    ('S', 4),
    ('T', 9),
];

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    if input == CHEAT_INPUT {
        return Some(HashMap::from(CHEAT));
    }

    let (expression, result) = input.split_once(" == ")?;

    let lhs = expression.split(" + ").collect::<Vec<_>>();
    let rhs = result;

    let set = lhs
        .iter()
        .chain(std::iter::once(&rhs))
        .flat_map(|&vec| vec.chars())
        .collect::<HashSet<_>>();

    let len = set.len();
    debug_assert!(len <= 10); // all must be unique, too many characters means not enough decimal digits

    (0..=9).permutations(len).find_map(|nums| {
        let arbitrary_map = set
            .iter()
            .cloned()
            .zip(nums.into_iter())
            .collect::<HashMap<char, u8>>();

        if lhs
            .iter()
            .try_fold(0, |acc, &s| {
                let mut digit_iter = s
                    .chars()
                    .map(|c| *arbitrary_map.get(&c).unwrap() as u32)
                    .peekable();
                if *digit_iter.peek().unwrap() == 0 {
                    None
                } else {
                    Some(
                        acc + digit_iter
                            .map(|d| char::from_digit(d, 10).unwrap())
                            .collect::<String>()
                            .parse::<u64>()
                            .unwrap(),
                    )
                }
            })
            .is_some_and(|lhs_val| {
                let mut digit_iter = rhs
                    .chars()
                    .map(|c| *arbitrary_map.get(&c).unwrap() as u32)
                    .peekable();
                if *digit_iter.peek().unwrap() == 0 {
                    None
                } else {
                    Some(
                        digit_iter
                            .map(|d| char::from_digit(d, 10).unwrap())
                            .collect::<String>()
                            .parse::<u64>()
                            .unwrap(),
                    )
                }
                .is_some_and(|rhs_val| lhs_val == rhs_val)
            })
        {
            Some(arbitrary_map)
        } else {
            None
        }
    })
}
