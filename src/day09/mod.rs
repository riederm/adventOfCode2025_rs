use itertools::Itertools;

type P = (i64, i64);

fn parse_input(src: &str) -> Vec<P> {
    src.lines()
        .map(|it| it.split_once(",").unwrap())
        .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
        .collect()
}






#[test]
fn part1() {
    let input = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

    let input = include_str!("input.txt");

    let points = parse_input(input);

    let mut max = 0;
    for i in  0 .. points.len() {
        for j in i+1 .. points.len() {
            let (x1, y1) = points[i];
            let (x2, y2)= points[j];
            let area = ((x2-x1).abs()+1) * ((y2 - y1).abs()+1);
            if area > max {
                max = area;
            }
        }
    }

    println!("max area = {}", max);
}

#[derive(Debug)]
struct Polygon {
    lines: Vec<Line>,
}

#[derive(Debug)]
struct Line{
    a: P,
    b: P,
}

impl Line {
    fn is_horizontal(&self) -> bool {
        self.a.1 == self.b.1
    }

    fn intersects(&self, other: &Line) -> bool {
        if self.is_horizontal() == other.is_horizontal() {
            return false;
        }

        let (h, v) = if self.is_horizontal() {
            (self, other)
        } else {
            (other, self)
        };

        let hy = h.a.1;
        let rx = h.a.0.min(h.b.0) ..= h.a.0.max(h.b.0);

        let vx = v.a.0;
        let ry = v.a.1.min(v.b.1) ..= v.a.1.max(v.b.1);

        ry.contains(&hy) && rx.contains(&vx)
    }
}

impl Polygon {
    fn is_inside(&self, point: &P) -> bool {
        let mut count = 0;
        let ray_end = (i64::MAX, point.1);
        let ray = Line {
            a: *point,
            b: ray_end,
        };
        
        for l in &self.lines {
            if ray.intersects(l) {
                count += 1;
            }
        }
        count % 2 == 1
    }
}   

#[test]
fn part2() {

    let input = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

    // let input = include_str!("input.txt");
    

    let points = parse_input(input);

    let poly = Polygon {
        lines: points.iter().cloned().circular_tuple_windows()
        .map(|(a,b)| Line{a,b})
        .collect::<Vec<_>>()
    };

    let mut max = 0;
    for i in  0 .. points.len() {
        for j in i+1 .. points.len() {
            println!("considering points {} and {}", i, j);
            let (x1, y1) = points[i];
            let (x2, y2)= points[j];
            
            let area = ((x2-x1).abs()+1) * ((y2 - y1).abs()+1);
            if area > max {
                // check if all points are inside
                let (min_x, min_y, max_x, max_y) = (
                    x1.min(x2),
                    y1.min(y2),
                    x1.max(x2),
                    y1.max(y2),
                );

                println!("checking area from ({},{}) to ({},{})", min_x, min_y, max_x, max_y);
                // top horizontal
                let is_inside = 
                    (min_x ..= max_x).any(|dx| !poly.is_inside(&(dx, min_y)))
                    || (min_x ..= max_x).any(|dx| !poly.is_inside(&(dx, max_y)))
                    || (min_y ..= max_y).any(|dy| !poly.is_inside(&(min_x, dy)))
                    || (min_y ..= max_y).any(|dy| !poly.is_inside(&(max_x, dy)));

                if is_inside {
                    max = area;
                }   
            }
        }
    }
//your answer is too high
//4755429952
    assert!(max < 4705882352);
   println!("max area = {}", max);
}