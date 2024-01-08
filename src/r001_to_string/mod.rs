use std::fmt::{Display, Formatter};

struct Point {
    x: i32,
    y: i32,
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl std::fmt::Debug for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug)]
struct Line {
    from: Point,
    to: Point,
}

#[test]
fn test() {
    let l = Line {
        from: Point { x: 0, y: 0 },
        to: Point { x: 1, y: 1 },
    };

    assert_eq!("l: Line { from: (0, 0), to: (1, 1) }", format!("l: {l:?}"));
}