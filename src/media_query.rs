use std::cmp::Ordering;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum MediaQuery {
    SuperiorTo(usize, String),
    InferiorOrEqualTo(usize),
}

impl MediaQuery {
    pub fn get_breakpoint(&self) -> &usize {
        match self {
            Self::SuperiorTo(breakpoint, _) => breakpoint,
            Self::InferiorOrEqualTo(breakpoint) => breakpoint,
        }
    }
}

impl PartialOrd for MediaQuery {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other)) // Use the `Ord` comparison logic for `PartialOrd`
    }
}

impl Ord for MediaQuery {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            // `SuperiorTo` is greater than `InferiorOrEqualTo`
            (MediaQuery::SuperiorTo(_, _), MediaQuery::InferiorOrEqualTo(_)) => Ordering::Greater,
            (MediaQuery::InferiorOrEqualTo(_), MediaQuery::SuperiorTo(_, _)) => Ordering::Less,

            //I think in the next two cases it's the same thinkg so we can say in every other case,
            //compare

            // If both are `SuperiorTo`, compare by usize, so if the same return Equals
            (MediaQuery::SuperiorTo(a, _), MediaQuery::SuperiorTo(b, _)) => b.cmp(a),

            // If both are `InferiorOrEqualTo`, compare by usize, so if the same return Equals
            (MediaQuery::InferiorOrEqualTo(a), MediaQuery::InferiorOrEqualTo(b)) => b.cmp(a),
        }
    }
}

pub fn extract_breakpoint(input: &str) -> Option<usize> {
    // Wrong pattern, nothing to extract
    if input.len() <= "layoutpx".len() {
        return None;
    }
    let number_part = &input[6..input.len() - 2]; // Slice out the number part
    return number_part.parse::<usize>().ok();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_breakpoint_with_nothing_after_at() {
        let bp = extract_breakpoint("layout@");
        assert_eq!(bp, None)
    }

    #[test]
    fn extract_breakpoint_without_at_cool() {
        let bp = extract_breakpoint("layout600px");
        assert_eq!(bp, Some(600))
    }

    #[test]
    fn extract_breakpoint_from_correct_mq() {
        let bp = extract_breakpoint("layout@600px");
        assert_eq!(bp, None)
    }
}
