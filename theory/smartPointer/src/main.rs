use std::rc::Rc;
#[derive(Debug)]

struct Car{
    number: String,
}

struct Door{
    vehicle: Rc<Car>,
}

fn main(){

    let car = Rc::new ( Car{
        number: "47AC- 80151".to_owned(),
    });

    let left_door = Door {
        vehicle: Rc::clone(&car),
    };

    let right_door = Door {
        vehicle: Rc::clone(&car),
    };

    drop(car);
    println!("{:?}" ,  left_door.vehicle);
    println!("{:?}" ,  right_door.vehicle);
}