fn main(){
    let mut hashcat = String::from("");
    let mut output;

    for arg in env::args().skip(1) {
        for x in WalkDir::new(arg).into_iter().filter_map(Result::ok).filter(|e| !e.file_type().is_dir()) {
            output = Command::new("md5sum")
            .arg(x.path().display().to_string())
            .output()
            .expect("failed to execute process");
            for elem in String::from_utf8_lossy(&output.stdout).to_string().split_whitespace().next(){
                hashcat = format!("{}{}", hashcat, elem.to_string());
            }
        }
    }


    cmd = Command::new("sh");
    cmd.arg("-c").arg(format!("echo "{}" | md5sum", hashcat));
    output = String::from_utf8_lossy(&cmd.output().unwrap().stdout).to_string();
    print!("{}", output);
}
