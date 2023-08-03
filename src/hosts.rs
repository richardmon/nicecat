use std::fs::OpenOptions;
use std::io::{self, prelude::*, BufReader};
use std::path::PathBuf;

pub fn add_redirect_rule(hostname: &str) -> io::Result<()> {
    let hosts_path = get_hosts_path();
    add_redirect_rule_with_path(hostname, &PathBuf::from(hosts_path))
}

// Function to add a redirect rule to a given hosts file
fn add_redirect_rule_with_path(hostname: &str, hosts_path: &PathBuf) -> io::Result<()> {
    // Read the current content of /etc/hosts
    let file = OpenOptions::new().read(true).open(&hosts_path)?;
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

        let mut file = OpenOptions::new().append(true).open(&hosts_path)?;
        file.write_all(content.as_bytes())?;
    }

    Ok(())
}

// Function to get the hosts path
fn get_hosts_path() -> String {
    if cfg!(target_os = "windows") {
        let system_root = std::env::var("SystemRoot").unwrap_or_else(|_| "C:\\Windows".to_string());
        format!(r"{}\System32\drivers\etc\hosts", system_root)
    } else {
        "/etc/hosts".to_string()
    }
}

#[cfg(test)]
use std::fs::File;
#[cfg(test)]
use tempfile::tempdir;

// Test adding a rule when it doesn't exist
#[test]
fn test_add_redirect_rule_with_path_new_rule() {
    // Create a temporary directory for the test
    let temp_dir = tempdir().expect("Failed to create temporary directory");
    let hosts_path = temp_dir.path().join("hosts");

    // Create a test hosts file with an existing entry
    let mut test_file = File::create(&hosts_path).expect("Failed to create test file");
    test_file
        .write_all(b"randomsite.gg\t127.0.0.1:3000\n")
        .expect("Failed to write to test file");

    let hostname = "example.com";
    add_redirect_rule_with_path(hostname, &hosts_path).expect("Failed to add redirect rule");

    // Verify the rule was added correctly
    let file = File::open(&hosts_path).expect("Failed to open test file");
    let reader = BufReader::new(&file);
    let rule_exists = reader
        .lines()
        .any(|line| line.expect("Failed to read line").contains(hostname));

    assert!(rule_exists, "Rule should be added when it doesn't exist");
}

// Test not adding a rule when it already exists
#[test]
fn test_add_redirect_rule_with_path_existing_rule() {
    // Create a temporary directory for the test
    let temp_dir = tempdir().expect("Failed to create temporary directory");
    let hosts_path = temp_dir.path().join("hosts");

    // Create a test hosts file with the same entry as the hostname we will test
    let mut test_file = File::create(&hosts_path).expect("Failed to create test file");
    test_file
        .write_all(b"example.com\t127.0.0.1:3000\n")
        .expect("Failed to write to test file");

    // Call the function
    let hostname = "example.com";
    add_redirect_rule_with_path(hostname, &hosts_path).expect("Failed to add redirect rule");

    // Verify the rule was not added again
    let file = File::open(&hosts_path).expect("Failed to open test file");
    let reader = BufReader::new(file);
    let rule_count = reader
        .lines()
        .filter(|line| {
            line.as_ref()
                .expect("Failed to read line")
                .contains(hostname)
        })
        .count();

    assert_eq!(
        rule_count, 1,
        "Rule should not be added when it already exists"
    );
}
