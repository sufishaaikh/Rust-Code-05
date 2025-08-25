#[derive(Debug)]
struct Flight {
    origin: String,
    destination: String,
    price: f64,
    passengers: u32,
}
fn main() {
    let mut flight = Flight::new(
        String::from("New York"),
        String::from("Boston"),
        300.00,
        200,
    );
    println!("{flight:?}");
    flight.change_destination(String::from("Dallas"));
    flight.increase_price();
    flight.itinerary();
    println!("{flight:?}");

    let my_flight = Flight {
        origin: String::from("San Diego"),
        destination: String::from("Los Angeles"),
        ..flight
    };
    println!("{:#?}", my_flight);
}

impl Flight {
    fn new(origin: String, destination: String, price: f64, passengers: u32) -> Flight {
        Flight {
            origin,
            destination,
            price,
            passengers,
        }
    }
    fn change_destination(&mut self, new_destination: String) {
        self.destination = new_destination;
    }
    fn increase_price(&mut self) {
        self.price *= 1.2;
    }
    fn itinerary(&self) {
        println!("{} -> {}", self.origin, self.destination);
    }
}