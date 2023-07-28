#[derive(Debug, PartialEq, Clone)]
pub struct Coord {
    pub x: usize,
    pub y: usize,
}

impl Coord {
    pub fn get_step_directions(&self) -> Vec<Coord> {
        let mut dirs = vec![
            Coord {
                x: self.x,
                y: self.y + 1,
            },
            Coord {
                x: self.x + 1,
                y: self.y,
            },
        ];
        if self.x > 0 {
            dirs.push(Coord {
                x: self.x - 1,
                y: self.y,
            });
        }
        if self.y > 0 {
            dirs.push(Coord {
                x: self.x,
                y: self.y - 1,
            });
        }
        dirs
    }

    pub fn is_in_bounds(&self, x_dim: usize, y_dim: usize) -> bool {
        self.x < x_dim && self.y < y_dim
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_in_bounds() {
        let coord = Coord { x: 5, y: 3 };
        let is_in = coord.is_in_bounds(4, 5);
        assert!(!is_in);

        let is_in = coord.is_in_bounds(10, 10);
        assert!(is_in);

        let is_in = coord.is_in_bounds(6, 4);
        assert!(is_in);

        let is_in = coord.is_in_bounds(6, 1);
        assert!(!is_in);

        let is_in = coord.is_in_bounds(3, 4);
        assert!(!is_in);
    }

    #[test]
    fn test_get_coord_directions() {
        let coord = Coord { x: 5, y: 5 };
        let dirs = coord.get_step_directions();
        assert_eq!(dirs.len(), 4);
        assert_eq!(
            dirs,
            vec![
                Coord { x: 5, y: 6 },
                Coord { x: 6, y: 5 },
                Coord { x: 4, y: 5 },
                Coord { x: 5, y: 4 },
            ]
        );

        let coord = Coord { x: 0, y: 0 };
        let dirs = coord.get_step_directions();
        assert_eq!(dirs.len(), 2);
        assert_eq!(dirs, vec![Coord { x: 0, y: 1 }, Coord { x: 1, y: 0 }]);

        let coord = Coord { x: 1, y: 0 };
        let dirs = coord.get_step_directions();
        assert_eq!(dirs.len(), 3);

        let coord = Coord { x: 0, y: 1 };
        let dirs = coord.get_step_directions();
        assert_eq!(dirs.len(), 3);
    }
}
