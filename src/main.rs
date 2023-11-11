use doit::doit::Doit;

fn main() {    
    let mut doit = Doit::default();
    
    match doit.run() {
        Ok(()) => {}
        Err(e) => { eprintln!("{}", e) }
    }
}