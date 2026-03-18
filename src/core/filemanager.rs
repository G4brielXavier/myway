use std::fs;
use std::path::PathBuf;
use std::io::{Error, ErrorKind};
use dirs::document_dir;
use crate::core::{errors::MyWayError, project::ProjectList};


pub struct Fiman {
    pub doc_path: PathBuf,
    pub mw_path: PathBuf,
    pub graveyard_path: PathBuf
}

impl Fiman {

    pub fn new() -> Result<Self, MyWayError> {
        
        let mut doc_path = document_dir().ok_or_else(|| {
            MyWayError::IoError(Error::new(
                ErrorKind::Other,
                "Could not find the system document directory"
            ))
        })?;

        doc_path.push("MyWayCli");
        
        let mw_path = doc_path.join("myway_projects.json");
        let graveyard_path = doc_path.join("graveyard_projects.json");

        Ok(Self { doc_path, mw_path, graveyard_path })
    
    }

    pub fn setup(&self) -> Result<(), MyWayError> {

        fs::create_dir_all(&self.doc_path).map_err(|e| { MyWayError::IoError(e) })?;

        if !self.mw_path.exists() {
            fs::write(&self.mw_path, "[]").map_err(|e| {
                MyWayError::IoError(e)
            })?;
        }

        if !self.graveyard_path.exists() {
            fs::write(&self.graveyard_path, "[]").map_err(|e| {
                MyWayError::IoError(e)
            })?;
        }

        Ok(())

    }

    pub fn write(&self, data: &ProjectList) -> Result<(), MyWayError> {

        let data_json = serde_json::to_string_pretty(data).map_err(|e| {
            MyWayError::IoError(e.into())
        })?;

        fs::write(&self.mw_path, data_json).map_err(|e| {
            MyWayError::IoError(e)
        })?;

        Ok(())
    }

    pub fn write_graveyard(&self, data: &ProjectList) -> Result<(), MyWayError> {

        let data_json = serde_json::to_string_pretty(data).map_err(|e| {
            MyWayError::IoError(e.into())
        })?;

        fs::write(&self.graveyard_path, data_json).map_err(|e| {
            MyWayError::IoError(e)
        })?;

        Ok(())
    }

    pub fn read(&self) -> Result<String, MyWayError> {
        let content = fs::read_to_string(&self.mw_path).map_err(|e| {
            MyWayError::IoError(e)
        })?;
        
        Ok(content)
    }

    pub fn read_graveyard(&self) -> Result<String, MyWayError> {
        let content = fs::read_to_string(&self.graveyard_path).map_err(|e| {
            MyWayError::IoError(e)
        })?;
        
        Ok(content)
    }

}