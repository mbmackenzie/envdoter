pub mod db;

use db::Database;
use std::collections::HashMap;
use std::env;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::Path;
use std::str::FromStr;

#[derive(Debug)]
struct EnvVar {
    key: String,
    value: String,
}

impl FromStr for EnvVar {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (key, value) = s.split_once("=").unwrap();
        Ok(EnvVar {
            key: key.to_string(),
            value: value.to_string(),
        })
    }
}

impl ToString for EnvVar {
    fn to_string(&self) -> String {
        format!("{}={}", self.key, self.value)
    }
}

pub fn create_env_file(path: &Path) -> io::Result<()> {
    if path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            "File already exists",
        ));
    }

    File::create(path)?;
    Ok(())
}

fn read_env_file(path: &Path) -> io::Result<Vec<EnvVar>> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut env_vars = Vec::new();

    for line in contents.lines() {
        if line.trim().is_empty() {
            continue;
        }

        let env_var = EnvVar::from_str(line).unwrap();
        env_vars.push(env_var);
    }

    Ok(env_vars)
}

pub fn sort_env_file(path: &Path) -> io::Result<()> {
    let mut final_string = String::new();
    let env_vars = read_env_file(path)?;
    println!("Envvars");
    println!("{:?}", env_vars);

    if env_vars.len() == 0 {
        return Ok(());
    }

    let keys = env_vars
        .iter()
        .map(|env_var| env_var.key.clone())
        .collect::<Vec<String>>();

    let sorted_keys = sort_by_prefix(keys);

    for group in sorted_keys {
        for key in group {
            let env_var = env_vars.iter().find(|env_var| env_var.key == key).unwrap();
            final_string.push_str(env_var.to_string().as_str());
            final_string.push_str("\n");
        }
        final_string.push_str("\n");
    }

    let mut file = File::create(path)?;
    file.write_all(final_string.as_bytes())?;

    Ok(())
}

pub fn add_to_env_file(path: &Path, key: &str, value: &str) -> io::Result<()> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut new_contents = String::new();
    new_contents.push_str(&contents);
    new_contents.push_str(&format!("{}={}\n", key, value));

    let mut file = File::create(path)?;
    file.write_all(new_contents.as_bytes())?;

    Ok(())
}

fn sort_by_prefix(keys: Vec<String>) -> Vec<Vec<String>> {
    let mut sorted = Vec::new();

    let mut prefix_counts: HashMap<String, usize> = HashMap::new();
    let mut prefix_groups: HashMap<String, Vec<String>> = HashMap::new();

    keys.iter().for_each(|key| {
        let prefix_try = key.split_once("_");

        let prefix = match prefix_try {
            None => "",
            Some((prefix, _)) => prefix,
        };

        if prefix_counts.contains_key(prefix) {
            let count = prefix_counts.get_mut(prefix).unwrap();
            *count += 1;
        } else {
            prefix_counts.insert(prefix.to_string(), 1);
        }
    });

    keys.iter().for_each(|key| {
        let prefix_try = key.split_once("_");

        let mut prefix = match prefix_try {
            None => "",
            Some((prefix, _)) => prefix,
        };

        if prefix_counts.get(prefix).unwrap() <= &1 {
            prefix = "";
        }

        if prefix_groups.contains_key(prefix) {
            let group = prefix_groups.get_mut(prefix).unwrap();
            group.push(key.to_string());
        } else {
            prefix_groups.insert(prefix.to_string(), vec![key.to_string()]);
        }
    });

    let mut sorted_prefixes = prefix_groups
        .clone()
        .into_iter()
        .map(|(key, _)| key)
        .collect::<Vec<String>>();

    sorted_prefixes.sort();

    sorted_prefixes.iter().for_each(|prefix| {
        let group = prefix_groups.get_mut(prefix).unwrap();
        group.sort();
        sorted.push(group.to_vec());
    });

    sorted
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_create_env_file() {
        let path = Path::new("test.env");
        let result = create_env_file(path);
        assert!(result.is_ok());
        fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_create_env_file_already_exists() {
        let path = Path::new("test.env");
        let result = create_env_file(path);
        assert!(result.is_ok());
        let result = create_env_file(path);
        assert!(result.is_err());
        fs::remove_file(path).unwrap();
    }
}
