#[derive(Debug, Default)]
pub struct Coordinate {
  x: i32,
  y: i32,
  pub count: i32,
}

impl Coordinate {
  pub fn from_string(string: String) -> Coordinate {
    let mut coord: Coordinate = Default::default();
    let mut x_set = false;
    for part in string.split(", ") {
      if x_set {
        coord.y = part.parse().unwrap();
      } else {
        coord.x = part.parse().unwrap();
      }
      x_set = true;
    }
    coord
  }

  #[allow(dead_code)]
  pub fn distance_to(&self, x: i32, y: i32) -> i32 {
    (self.x - x).abs() + (self.y - y).abs()
  }
}

#[derive(Debug, Default)]
pub struct CoordinateMap {
  pub leftmost: Option<i32>,
  pub rightmost: Option<i32>,
  pub topmost: Option<i32>,
  pub bottommost: Option<i32>,
  pub coords: Vec<Coordinate>,
}

impl CoordinateMap {
  pub fn new() -> CoordinateMap {
    Default::default()
  }

  pub fn add(&mut self, coord_str: String) -> &mut CoordinateMap {
    let coord = Coordinate::from_string(coord_str);

    self.leftmost = if self.leftmost.unwrap_or(999999) > coord.x {
      Some(coord.x)
    } else {
      self.leftmost
    };

    self.topmost = if self.topmost.unwrap_or(0) < coord.y {
      Some(coord.x)
    } else {
      self.topmost
    };

    self.bottommost = if self.bottommost.unwrap_or(999999) > coord.y {
      Some(coord.x)
    } else {
      self.bottommost
    };

    self.rightmost = if self.rightmost.unwrap_or(0) < coord.x {
      Some(coord.x)
    } else {
      self.rightmost
    };

    self.coords.push(coord);

    self
  }
}
