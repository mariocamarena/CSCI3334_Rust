use std::mem;

struct Car {
    color: String, 
    maker: String,
    year: u16,
}

fn main(){
    println!("Size of Car: {} bytes", mem::size_of::<Car>());
    println!("Alignment of Car: {} bytes", mem::align_of::<Car>());
}




/*
struct Computer{
    ram: u16,
    cpu: String,
    gpu: String,

}


impl Computer{
    fn new(color:String,maker:String,year:u16) -> Car{
        Self {
            ram: ram, 
            cpu: cpu,
            gpu: gpu,
        }
    }


    fn check_ram(&self){
        println!("Total RAM {} GB",self.ram)
    }

    fn upgrade_gpu(&mut self,year:u16){
        self.gpu = gpu;
    }

}

fn main(){
    
    let mut my_pc: Computer = Computer::new(
        16,
        "i7",
        "4060"
    );


    println!("{:?}", my_pc);
    my_pc.check_ram();

    my_pc.upgrade_gpu(gpu:4090)
    println!("{:?}", my_pc);



}

*/

