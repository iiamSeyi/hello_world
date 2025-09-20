// my hello world program
use std::env;
use std::io;
use rand::random;
use std::fs;
use std::io::Write;

#[derive(Debug)]
struct Shuttle {
    name: String,
    crew_size: u8,
    propellant: f64
}

struct Color(u8, u8, u8);//RGB
struct Point(u8, u8, u8);//XYZ

fn get_y(p: Point) -> u8 {
    p.1
}

impl Shuttle {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn add_fuel(&mut self, gallons: f64) {
        self.propellant += gallons;
    }

    fn new(name: &str) -> Shuttle {
        Shuttle {
            name: String::from(name),
            crew_size: 7,
            propellant: 0.0
        }
    }
}
fn main() {
    /*multiple
    line
    comments */
    println!("Hello, world!"); // this line prints!

    //Lab 4 examples
    let mut buffer = String::new();
    println!("Enter a message: ");
    io::stdin().read_line(&mut buffer) ;
    println!("buffer is {}", buffer);

    let number: i32 = buffer.trim().parse().unwrap();
    println!("number + 1 is {}", number + 1);

    let random_number = random::<f64>();
    println!("number is {}", random_number);

    if env::args().len() <= 2 {
        println!("Prigrams requires as least 2 arguments.");
        return;
    }
    for(index, argument) in env::args().enumerate() {
        println!("argument {} is {}", index, argument);
    }

    let arg2 = env::args().nth(2).unwrap();
    println!("arg2 is {}", arg2);

    let contents = fs::read_to_string("planets.txt").unwrap();
    println!("contents is {}", contents);

    let mut speech = String::new();
    speech.push_str("We choose to go to the Moon in this decade\n");
    speech.push_str(" and do the other things,\n");
    speech.push_str("not because they are easy,\n");
    speech.push_str(" but because they are hard,\n");

    fs::write("speech.txt", speech);

    let mut file = fs::OpenOptions::new().append(true).open("planets.txt").unwrap();
    file.write(b"\nPluto");



    let mut vehicle = Shuttle::new("Endeavour");
    let mut vehicle2 = Shuttle::new("Discovery");

    


    vehicle.name = String::from("Atlantis");
    println!("vehicle is {:?}", vehicle);
    println!("vehicle2 is {:?}", vehicle2);

    let vehicle_name = vehicle.get_name();
    println!("vehicle_name is {}", vehicle_name);

    println!("propellant is {}", vehicle.propellant);
    vehicle.add_fuel(1000.0);
    println!("propellant is {}", vehicle.propellant);

    let red = Color(255, 0, 0);
    println!("First value is {}", red.0);

    let coord = Point(4,5,6);
    let y = get_y(coord);
    println!("y is {}", y);

}

