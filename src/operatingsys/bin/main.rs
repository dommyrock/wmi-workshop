#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use serde::Deserialize;
use wmi::{COMLibrary, WMIConnection, WMIDateTime};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let com_con = COMLibrary::new()?;
    let wmi_con = WMIConnection::new(com_con.into())?;

    #[derive(Deserialize, Debug)]
    struct Win32_OperatingSystem {
        Caption: String,
        Name: String,
        CurrentTimeZone: i16,
        Debug: bool,
        EncryptionLevel: u32,
        ForegroundApplicationBoost: u8,
        LastBootUpTime: WMIDateTime,
    }

    let results: Vec<Win32_OperatingSystem> = wmi_con.query()?;

    for os in results {
      println!("{:#?}", os);
    }

    //OR
    //    let results: Vec<HashMap<String, Variant>> =
    //    wmi_con.async_raw_query("SELECT * FROM Win32_OperatingSystem").await?;

    // for os in results {
    //    println!("{:#?}", os);
    // }

    Ok(())
}
