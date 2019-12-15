use std::cmp;

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn manhattan_dist(&self, lhs: &Point) -> i32 {
        (self.x - lhs.x).abs() + (self.y - lhs.y).abs()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Line {
    Vertical(Point, i32),
    Horizontal(Point, i32),
}

impl Line {
    pub fn intersects(&self, l: Line) -> Option<Point> {
        let (v, h) = match (self, &l) {
            (Line::Vertical(pv, dv), Line::Horizontal(ph, dh)) => ((pv, dv), (ph, dh)),
            (Line::Horizontal(ph, dh), Line::Vertical(pv, dv)) => ((pv, dv), (ph, dh)),
            (x, y) => panic!("Unexpected intersection case (x: {:?}, y: {:?})", x, y),
        };

        let dx = h.0.x + h.1;
        let dy = v.0.y + v.1;

        let h1 = v.0.x > cmp::min(dx, h.0.x);
        let h2 = v.0.x < cmp::max(dx, h.0.x);
        let v1 = h.0.y > cmp::min(dy, v.0.y);
        let v2 = h.0.y < cmp::max(dy, v.0.y);

        if !(h1 && h2 && v1 && v2) {
            return None;
        }

        Some(Point::new(v.0.x, h.0.y))
    }
}

// TODO: this could be as well line::new
impl From<(Point, Direction)> for Line {
    fn from(t: (Point, Direction)) -> Self {
        match t {
            (p, Direction::Up(d)) => Line::Vertical(p, d),
            (p, Direction::Down(d)) => Line::Vertical(p, -d),
            (p, Direction::Right(d)) => Line::Horizontal(p, d),
            (p, Direction::Left(d)) => Line::Horizontal(p, -d),
        }
    }
}

impl From<(&Line, Direction)> for Line {
    fn from(t: (&Line, Direction)) -> Self {
        match t {
            (&Line::Horizontal(Point { x, y }, d0), Direction::Up(d1)) => Line::Vertical(Point { x: x + d0, y }, d1),
            (&Line::Horizontal(Point { x, y }, d0), Direction::Down(d1)) => Line::Vertical(Point { x: x + d0, y }, -d1),
            (&Line::Vertical(Point { x, y }, d0), Direction::Right(d1)) => Line::Horizontal(Point { x, y: y + d0 }, d1),
            (&Line::Vertical(Point { x, y }, d0), Direction::Left(d1)) => Line::Horizontal(Point { x, y: y + d0 }, -d1),
            (x, y) => panic!("Unsupported line and direction combination ({:?}, {:?})", x, y),
        }
    }
}

#[derive(Debug)]
pub enum Direction {
    Up(i32),
    Down(i32),
    Right(i32),
    Left(i32),
}

impl From<(char, i32)> for Direction {
    fn from(d: (char, i32)) -> Self {
        match d {
            ('U', v) => Direction::Up(v),
            ('D', v) => Direction::Down(v),
            ('R', v) => Direction::Right(v),
            ('L', v) => Direction::Left(v),
            (x, _) => panic!("Unknown direction: {}", x),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_intersects1() {
        let l1: Line = (Point::new(6, 3), Direction::Left(4)).into();
        let l2: Line = (Point::new(3, 5), Direction::Down(3)).into();
        let res = l1.intersects(l2).unwrap();
        assert_eq!(res, Point::new(3, 3));
    }

    #[test]
    fn test_intersects2() {
        let l1: Line = (Point::new(8, 5), Direction::Left(5)).into();
        let l2: Line = (Point::new(6, 7), Direction::Down(4)).into();
        let res = l1.intersects(l2).unwrap();
        assert_eq!(res, Point::new(6, 5));
    }

    #[test]
    fn test_distance1() {
        let p0 = Point::new(0, 0);
        let p1 = Point::new(3, 3);
        let res = p0.manhattan_dist(&p1);
        assert_eq!(res, 6);
    }

    #[test]
    fn test_distance2() {
        let p0 = Point::new(0, 0);
        let p1 = Point::new(6, 5);
        let res = p0.manhattan_dist(&p1);
        assert_eq!(res, 11);
    }
}
