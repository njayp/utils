imp!();

// TODO search displayname
// TODO return option 

pub fn find_windows_app(app_name: &str) -> PathBuf {
    let di = std::string::String::from_utf8(
        Command::new("powershell").arg(format!(r#"Get-ItemProperty "HKLM:\Software\Wow6432Node\Microsoft\Windows\CurrentVersion\Uninstall\\{}" | Select-Object DisplayIcon"#, app_name)
        ).output().unwrap().stdout
    ).unwrap();
  
    let reg = Regex::new(r#"\n(?P<path>.*),"#).expect("bad regex");
  
    PathBuf::from(
        reg.captures(&di)
            .expect("capture failed")
            .name("path")
            .expect("path not found")
            .as_str()
            .trim_matches('"')
    )
  }
