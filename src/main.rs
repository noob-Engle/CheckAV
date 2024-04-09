use std::collections::HashMap;
use std::fs;
use std::io;
use std::process::Command;

fn main() -> io::Result<()> {
    
    let config_content = fs::read_to_string("av.config")?;
    let mut dictionary = HashMap::new();

    for line in config_content.lines() {
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() == 2 {
            dictionary.insert(parts[0].to_string(), parts[1].to_string());
        }
    }

    
    let output = Command::new("cmd")
        .args(&["/C", "tasklist", "/svc"])
        .output()?;

    let tasklist_output = String::from_utf8_lossy(&output.stdout);

   
    let mut found_count = 0;
    for (key, value) in &dictionary {
        if tasklist_output.contains(key) {
            println!(" [+] antivirus software：{}:{}", key, value);
            found_count += 1;
        }
    }

    if found_count == 0 {
        println!(" Not found！");
    }

    Ok(())
}
