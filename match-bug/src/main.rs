use my_proc_macro::payload;


enum TriggerMode {
    Off,
}

struct Payload {
    trigger_mode: TriggerMode,
}

#[payload]
impl Payload {
    fn test(&mut self) {
        match self.trigger_mode {
            
        }
    }
}

fn main() {
    println!("Hello, world!");
}
