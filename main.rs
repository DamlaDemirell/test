

struct Phone {
    name:String,
    weight: u32,
}

impl Phone  { 
    fn get_name(&self){
        println!("Phone name: {}" , self.name);

    }
    
}
fn main() {
   let phone: Phone = Phone {name: String::from("phone x"), weight:110};
   phone.get_name();
}
