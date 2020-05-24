use std::vec::Vec;

enum PolynomialRoots {
    NoRealResult,
    Roots(Vec<f32>)
}

trait FloatComparator {
    fn equal(self, other : Self) -> bool;
}

impl FloatComparator for f32 {
    fn equal(self, other : Self) -> bool
    {
        Self::abs(self - other) <= Self::EPSILON
    }
}

#[derive(Debug)]
struct LinearEquation {
    a : f32,
    b : f32,
}

impl LinearEquation {
    fn solve(self)->PolynomialRoots {
        if self.a.equal(0.0) {
            PolynomialRoots::NoRealResult
        } else {
            PolynomialRoots::Roots(vec![-self.b/self.a])
        }
    }
}

#[derive(Debug)]
struct QuadraticEquation {
    a : f32,
    b : f32,
    c : f32,
}

impl QuadraticEquation {
    fn solve(self)-> PolynomialRoots {
        if self.a.equal(0.0) {
            LinearEquation{a:self.b, b:self.c}.solve()
        } else {
            let d: f32 = (self.b.powi(2) - 4. * self.a * self.c).sqrt();
            if d.is_nan() {
                PolynomialRoots::NoRealResult
            } else {
                PolynomialRoots::Roots(vec![(-self.b + d) / 2. * self.a, (-self.b - d) / 2. * self.a])
            }
        }
    }
}

fn main() {
    let equation = QuadraticEquation{a:-1., b : 1., c: 3.};
    println!("Hello, world! {:?}", equation);
    let solution = equation.solve();
    match solution {
        PolynomialRoots::NoRealResult=>{print!("No real roots")},
        PolynomialRoots::Roots(x) =>{println!("result {:?}",x)}
    }
}


