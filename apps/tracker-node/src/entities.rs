 pub struct Restaurant {
     pub id: u32,
     pub name: String,
     pub latitude: f64,
     pub longitude: f64,
 }


 pub struct Delivery {
     pub id: i32,
     latitude: f64,
     longitude: f64,
 }


 impl Restaurant {
     pub fn new(id: u32, name: String, latitude: f64, longitude: f64) -> Self {
         Restaurant { id, name, latitude, longitude }
     }
 }