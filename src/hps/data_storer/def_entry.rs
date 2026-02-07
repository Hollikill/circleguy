use std::{
    collections::HashMap,
    ffi::{OsStr, OsString},
    path::Path,
};

use crate::hps::data_storer::data_storer::PuzzleLoadingData;

#[derive(Clone, Debug)]
pub enum DefEntry {
    Def(PuzzleLoadingData),
    Folder((OsString, HashMap<OsString, DefEntry>)),
}

impl DefEntry {
    pub fn add_puzzle(&mut self, puzzle: PuzzleLoadingData, path: &Path) -> Result<(), ()> {
        if let Self::Folder(ref mut curr) = *self {
            let mut curr = curr;
            let fixed_path = path.parent().unwrap().iter();
            for fold in fixed_path {
                {
                    if !curr.1.contains_key(fold) {
                        curr.1.insert(
                            fold.to_os_string(),
                            Self::Folder((fold.to_os_string(), HashMap::new())),
                        );
                    }
                    curr = if let Self::Folder(x) = curr.1.get_mut(fold).ok_or(())? {
                        x
                    } else {
                        return Err(());
                    }
                };
            }
            curr.1
                .insert(OsString::from(puzzle.name.clone()), DefEntry::Def(puzzle));
        } else {
            return Err(());
        }
        Ok(())
    }
    pub fn get(&self, path: &Path) -> Option<PuzzleLoadingData> {
        if let Self::Folder(ref curr) = *self {
            let mut curr = curr;
            let mut list_path = path.iter().collect::<Vec<&OsStr>>();
            let path_end = list_path.pop()?;
            for fold in list_path {
                curr = if let Self::Folder(x) = curr.1.get(fold)? {
                    x
                } else {
                    return None;
                };
            }
            Some(if let DefEntry::Def(x) = curr.1.get(path_end)? {
                x.clone()
            } else {
                return None;
            })
        } else {
            None
        }
    }
}
