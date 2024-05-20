mod counties;
mod plants;

use counties::counties_kenya;
use plants::{fruits_plants, poisonous_fruits};


fn main() {
    println!("Hello, world!");

    counties_kenya();
    fruits_plants::fruits_plants();
    poisonous_fruits::poisonous_plants();

}
