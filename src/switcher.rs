use std::fs;
use std::io::BufRead;

use reqwest;

const DOMAINS: &'static [&'static str] = &[
    "c.ppy.sh",
    "ce.ppy.sh",
    "c3.ppy.sh",
    "c4.ppy.sh",
    "c5.ppy.sh",
    "c6.ppy.sh",
    "a.ppy.sh",
    "i.ppy.sh",
    "osu.ppy.sh",
];

#[cfg(windows)]
static HOSTS_PATH: &'static str = "C:\\Windows\\System32\\drivers\\etc\\hosts";

#[cfg(unix)]
static HOSTS_PATH: &'static str = "/etc/hosts";

#[cfg(windows)]
static LINE_ENDING: &'static str = "\r\n";

#[cfg(unix)]
static LINE_ENDING: &'static str = "\n";

pub fn remove_old() -> std::io::Result<()> {
    let file = fs::File::open(HOSTS_PATH)?;
    let reader = std::io::BufReader::new(file);
    let mut lines = vec![];

    for line in reader.lines() {
        let o_line = line?.to_owned();
        if !o_line.trim().ends_with("ppy.sh") || o_line.trim().starts_with("#") {
            lines.push(o_line.to_owned())
        }
    }

    let hosts = lines.join(LINE_ENDING);

    fs::write(HOSTS_PATH, hosts)?;

    Ok(())
}

pub fn is_switched() -> std::io::Result<bool> {
    let file = fs::File::open(HOSTS_PATH)?;
    let reader = std::io::BufReader::new(file);
    
    for line in reader.lines() {
        let o_line = line?.to_owned();
        if o_line.trim().ends_with("ppy.sh") && !o_line.trim().starts_with("#") {
            return Ok(true)
        }
    }

    Ok(false)
}

pub fn switch_to(ip: &str) -> std::io::Result<()> {
    remove_old()?;
    
    let mut lines = vec![];

    {
        let file = fs::File::open(HOSTS_PATH)?;
        let reader = std::io::BufReader::new(file);

        for line in reader.lines() {
            let o_line = line?.to_owned();

            lines.push(o_line.to_owned());
        }
    }
    
    for domain in DOMAINS {
        lines.push(format!("{} {}", ip, domain));
    }

    println!("{:#?}", lines);

    let hosts = lines.join(LINE_ENDING);

    fs::write(HOSTS_PATH, hosts)?;

    Ok(())
}

pub async fn fetch_ip() -> Result<String, reqwest::Error> {
    let ip = reqwest::get("https://old.akatsuki.pw/ips.txt")
        .await?
        .text()
        .await?;

    Ok(ip)
}

pub async fn fetch_cert() -> Result<String, reqwest::Error> {
    let cert = reqwest::get("https://old.akatsuki.pw/akatsuki.crt")
        .await?
        .text()
        .await?;

    Ok(cert)
}

pub async fn install_cert(cert_data: &str) {
    #[cfg(windows)]
    {
        let mut file = std::env::temp_dir();
        file.push("akatsuki.crt");

        let path = file.as_path();

        std::fs::write(path, cert_data).expect("Unable to save Certificate");

        std::process::Command::new("certutil.exe")
            .arg("-addstore")
            .arg("Root")
            .arg(path.to_str().unwrap())
            .status()
            .expect("Unable to install root Certificate.");
    }

    #[cfg(unix)]
    {

    }
}
