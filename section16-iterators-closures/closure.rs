// Closures
use std::thread;
use std::time::Duration;

fn main() { 
    let simulated_user_specified_value = 10;
    let simulated_random_numer = 7;
    generate_workout(
        simulated_user_specified_value,
        simulated_random_numer
    );
}

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("Calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn generate_workout(intensity: u32, random_number: u32){
    let expensive_closure = Cacher::new(|num: u32| -> u32 {
         println!("Calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    num
    });
    fn add_one_v1(x: u32) -> u32 { x + 1 };
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
 
    let x = expensive_closure.value(20);

    //  simulated_expensive_calculation(intensity);
    if intensity < 25 { 
        println!("Today do {} pushups", expensive_closure.value(intensity));
        println!("Today do {} situps",expensive_closure.value(intensity));
    } else  {
        if random_number == 3 {
            println!("Today take a break");
        } else { 
            println!("Today run for {} minutes", expensive_closure.value(intensity));
        }
    }
}

struct Cacher <T> 
    where T: Fn(u32) -> u32
    {
        calculation: T,
        value: Option<u32>
    }

impl <T> Cacher <T>  {
    where T: Fn(u32) -> u32
    {
        fn new(calculation: T) -> Cacher<T> {
            Cacher {
                calculation,
                value: None
            }
        }

        fn value (&mut self, arg: u32) -> u32  {
            match self.value { 
                Some(v) => v,
                None => {
                    let v = (self.calculation)(arg);
                    self.value = Some(v);
                    v
                }
            }
        }
    }
}