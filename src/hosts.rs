use std::fs::OpenOptions;
use std::io::{self, prelude::*, BufReader};

// Function to add a redirect rule to /etc/hosts
pub fn add_redirect_rule(hostname: &str) -> io::Result<()> {
    // let hosts_path = "/etc/hosts";
    let hosts_path = "./hosts";

    // Read the current content of /etc/hosts
    let file = OpenOptions::new().read(true).open(hosts_path)?;
    let reader = BufReader::new(file);

    // Check if the rule already exists
    let rule_exists = reader.lines().any(|line| {
        if let Ok(line) = line {
            let parts: Vec<&str> = line.trim().split_whitespace().collect();
            parts.len() >= 2 && parts[0] == hostname
        } else {
            false
        }
    });

    // If the rule doesn't exist, add it
    if !rule_exists {
        let mut content = String::new();
        let target_ip = "127.0.0.1:3333";
        content.push_str(&format!("{}\t{}\n", hostname, target_ip));

        let mut file = OpenOptions::new().append(true).open(hosts_path)?;
        file.write_all(content.as_bytes())?;
    }

    Ok(())
}
