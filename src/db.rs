use nebari::{
    io::fs::StdFile,
    tree::{Root, ScanEvaluation, Unversioned, UnversionedTreeRoot},
    Config, Tree,
};
use std::{convert::Infallible, sync::Mutex};
use std::{path::Path, str::FromStr};

pub struct Database {
    tree: Mutex<Tree<UnversionedTreeRoot<()>, StdFile>>,
}

impl Database {
    pub fn new(path: &Path) -> Result<Self, nebari::Error> {
        let roots = Config::default_for(path).open().unwrap();
        let tree = roots.tree(Unversioned::tree("name")).unwrap();

        Ok(Database {
            tree: Mutex::new(tree),
        })
    }

    pub fn read_keys(&self) -> Vec<String> {
        let tree = self.tree.lock().unwrap();
        let mut keys = Vec::new();

        tree.scan::<Infallible, _, _, _, _>(
            &(..),
            true,
            |_, _, _| ScanEvaluation::ReadData,
            |key, _| {
                keys.push(String::from_utf8(key.to_vec()).unwrap());
                ScanEvaluation::Skip
            },
            |_, _, _| unreachable!(),
        )
        .unwrap();

        keys
    }

    pub fn read_value(&self, key: &str) -> String {
        let tree = self.tree.lock().unwrap();
        let value = tree.get(key.as_bytes()).unwrap();

        match value {
            Some(value) => String::from_utf8(value.to_vec()).unwrap(),
            None => String::from_str("").unwrap(),
        }
    }

    pub fn write_value(&self, key: &str, value: &str) -> String {
        let tree = self.tree.lock().unwrap();

        let key_bytes = key.as_bytes().to_vec();
        let value_bytes = value.as_bytes().to_vec();

        tree.set(key_bytes, value_bytes).unwrap();
        String::from_str(key).unwrap()
    }
}
