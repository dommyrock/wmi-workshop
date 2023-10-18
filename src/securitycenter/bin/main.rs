use std::collections::HashMap;
use wmi::{COMLibrary, WMIConnection};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let wmi_con = WMIConnection::with_namespace_path("ROOT\\securitycenter2", COMLibrary::new()?)?;

    let results: Vec<HashMap<String, String>> = wmi_con
        .raw_query("SELECT displayName FROM AntiVirusProduct")
        .unwrap();
    
    for av in results {
        println!("{:?}", av.get("displayName").unwrap());
    }

    Ok(())
}
