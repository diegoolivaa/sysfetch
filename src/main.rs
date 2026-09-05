use sysinfo::System;
use colored::Colorize;
use whoami;

fn logo(os: &str) -> String {
    if os.contains("Windows") {
        r#"████████  ████████
████████  ████████
████████  ████████

████████  ████████
████████  ████████
████████  ████████"#.to_string()
    } else if os.contains("Linux") {
        r#" ___      ___   __    _  __   __  __   __ 
|   |    |   | |  |  | ||  | |  ||  |_|  |
|   |    |   | |   |_| ||  | |  ||       |
|   |    |   | |       ||  |_|  ||       |
|   |___ |   | |  _    ||       | |     | 
|       ||   | | | |   ||       ||   _   |
|_______||___| |_|  |__||_______||__| |__|"#.to_string()
    } else {
        r#"  _.--,-```-.      _.--,-```-.      _.--,-```-.    
 /    /      '.   /    /      '.   /    /      '.  
/  ../         ; /  ../         ; /  ../         ; 
\  ``\  .``-    '\  ``\  .``-    '\  ``\  .``-    '
 \ ___\/    \   : \ ___\/    \   : \ ___\/    \   :
       \    :   |       \    :   |       \    :   |
       |    ;  .        |    ;  .        |    ;  . 
      ;   ;   :        ;   ;   :        ;   ;   :  
     /   :   :        /   :   :        /   :   :   
     `---'.  |        `---'.  |        `---'.  |   
      `--..`;          `--..`;          `--..`;    
    .--,_            .--,_            .--,_        
    |    |`.         |    |`.         |    |`.     
    `-- -`, ;        `-- -`, ;        `-- -`, ;    
      '---`"           '---`"           '---`"     
                                                   "#.to_string()
    }
}

fn main() {
    let mut sys = System::new_all();
    sys.refresh_all();

    let os_name = System::name().unwrap_or_default();

    if os_name.contains("Windows") {
        println!("{}", logo(&os_name).custom_color((0, 120, 215)));
    }

    let uptime_secs = System::uptime();
    let hours = uptime_secs / 3600;
    let minutes = (uptime_secs % 3600) / 60;

    println!("Made by @diegoolivaa");
    println!();
    println!("OS: {}", os_name);
    println!("Kernel: {}", System::kernel_version().unwrap_or_default());
    println!("User: {}", whoami::username());
    println!("Host: {}", System::host_name().unwrap_or_default());
    if let Some(cpu) = sys.cpus().first() {
        println!("CPU: {}", cpu.brand());
    }
    println!("Total RAM: {} MB", sys.total_memory() / 1024 / 1024);
    println!("Used RAM: {} MB", sys.used_memory() / 1024 / 1024);
    println!("Uptime {}h {}m", hours, minutes);
}