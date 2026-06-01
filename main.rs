use std::io;
use std::thread;
use std::time::Duration;
use sysinfo::{Components,System};
use std::io::Write;
struct Systeminfo{
    cpu:f32,
    ram:f32,
}
fn main() {
    print!("\x1B[?1049h");
    let mut sys=System::new_all();
 
    loop{
    print!("\x1B[2J\x1B[1;1H");
    sys.refresh_all();
    let total_ram=sys.total_memory() as f32;
    let used_ram=sys.used_memory() as f32;
    let ram:f32=(used_ram/total_ram)*100.0;
    let cpu:f32=sys.global_cpu_usage() as f32;
    let components=Components::new_with_refreshed_list();
        let a:Systeminfo=Systeminfo{
            cpu:cpu,
            ram:ram,
        };
     print!("\x1B[2J\x1B[2J\x1B[H");
     println!("cpu:{:.1}%",&a.cpu);
     println!("ram:{:.1}%",&a.ram);
     for component in &components {
        println!("{}:{:?}°C",component.label(),component.temperature());
     }
     io::stdout().flush().unwrap();
    thread::sleep(Duration::from_secs(10));
    }
}
