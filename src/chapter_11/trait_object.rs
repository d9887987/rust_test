trait Material {
    fn cost_per_sq_meter(&self) -> f64;
    fn square_meters(&self) -> f64;
    //trait 的默认实现
    fn total_cost(&self) -> f64 {
        self.cost_per_sq_meter() * self.square_meters()
    }
}

struct Carpet(f64);

impl Material for Carpet {
    fn cost_per_sq_meter(&self) -> f64 {
        10.0
    }

    fn square_meters(&self) -> f64 {
        self.0
    }
}

struct Tile(f64);

impl Material for Tile {
    fn cost_per_sq_meter(&self) -> f64 {
        15.0
    }

    fn square_meters(&self) -> f64 {
        self.0
    }
}

struct Wood(f64);

impl Material for Wood {
    fn cost_per_sq_meter(&self) -> f64 {
        20.0
    }

    fn square_meters(&self) -> f64 {
        self.0
    }
}

fn total_cost(material: &Vec<Box<dyn Material>>) -> f64 {
    material.iter().map(|mat| mat.total_cost()).sum()
}


pub fn test_trait() {
    let carpet = Box::new(Carpet(20.0));
    let title = Box::new(Tile(20.0));
    let wood = Box::new(Wood(20.0));

    let vec1: Vec<Box<dyn Material>> = vec![carpet, title, wood];
    let cost = total_cost(&vec1);
    println!("{}", cost);
}





















