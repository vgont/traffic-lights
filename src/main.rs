#[derive(Debug)]
enum CarState {
    Moving,
    Stopping,
    Stopped,
}

#[derive(Debug)]
struct Car {
    state: CarState,
}

#[derive(Debug)]
enum TrafficLight {
    Red,
    Green,
    Yellow,
}

impl TrafficLight {
    fn warn_cars(&mut self, cars_in_street: &mut [&mut Car; 3]) {
        match self {
            Self::Red => {
                for car in cars_in_street.iter_mut() {
                    car.state = CarState::Stopped
                }
            }
            Self::Green => {
                for car in cars_in_street {
                    car.state = CarState::Moving
                }
            }
            Self::Yellow => {
                for car in cars_in_street {
                    car.state = CarState::Stopping
                }
            }
        }
    }
}

fn main() {
    let mut traffic_light = TrafficLight::Yellow;
    dbg!(&traffic_light);

    let mut car1 = Car {
        state: CarState::Stopping,
    };
    let mut car2 = Car {
        state: CarState::Stopping,
    };
    let mut car3 = Car {
        state: CarState::Stopping,
    };

    let mut cars: [&mut Car; 3] = [&mut car1, &mut car2, &mut car3];
    dbg!(&cars);

    traffic_light = TrafficLight::Green;
    dbg!(&traffic_light);

    traffic_light.warn_cars(&mut cars);
    dbg!(&cars);

    traffic_light = TrafficLight::Red;
    dbg!(&traffic_light);

    traffic_light.warn_cars(&mut cars);
    dbg!(&cars);
}
