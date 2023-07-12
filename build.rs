fn main() {
    println!("Checking host...");
    // Install external dependency (in the shuttle container only)
    if std::env::var("HOSTNAME")
        .unwrap_or_default()
        .contains("shuttle")
    {
        println!("Installing apt depedencies...");
        if !std::process::Command::new("apt")
            .arg("install")
            .arg("-y")
            .arg("apt-utils")
            .arg("libopus-dev")
            .arg("pkg-config")
            .arg("cmake")
            .arg("ffmpeg")
            .arg("autoconf")
            .arg("automake")
            .arg("libtool")
            .arg("protobuf-compiler")
            .arg("python3-pip")
            .arg("libasound2-dev")
            .arg("libalsaplayer-dev")
            .arg("libavutil-dev")
            .arg("libavformat-dev")
            .arg("libavfilter-dev")
            .arg("libavdevice-dev")
            .arg("libssl-dev")
            .status()
            .expect("failed to run apt")
            .success()
        {
            panic!("failed to install dependencies")
        }

        let out = std::process::Command::new("whoami")
            .output()
            .unwrap()
            .stdout;
        println!("{}", String::from_utf8(out).unwrap());

        let out = std::process::Command::new("ls")
            .arg("-la")
            .arg("/root/")
            .output()
            .unwrap()
            .stdout;
        println!("{}", String::from_utf8(out).unwrap());

        println!("Installing yt-dlp...");
        if !std::process::Command::new("python3")
            .arg("-m")
            .arg("pip")
            .arg("install")
            .arg("-U")
            .arg("yt-dlp")
            .status()
            .expect("failed to run pip")
            .success()
        {
            panic!("failed to install yt-dlp")
        }

        if !std::process::Command::new("yt-dlp")
            .arg("--version")
            .status()
            .expect("failed to run yt-dlp")
            .success()
        {
            panic!("failed to run yt-dlp")
        }
    }
}
