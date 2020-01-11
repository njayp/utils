pub fn find_app(app_name: str) -> PathBuf {
    let di = std::string::String::from_utf8(
        Command::new("powershell").arg(format!("Get-ItemProperty HKLM:\\Software\\Wow6432Node\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\{} | Select-Object DisplayIcon", app_name)
        ).output().unwrap().stdout
    ).unwrap();
    let reg = Regex::new(r#""(?P<path>.*)""#).expect("bad regex");

    PathBuf::from(
        reg.captures(&di)
            .expect("capture failed")
            .name("path")
            .expect("path not found")
            .as_str(),
    )
}
