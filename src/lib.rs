use std::process::Command;

/// Ping a target. 
///
/// Returns: True if the target is alive, false otherwise
pub fn ping(ip_address : &str) -> bool {
    
    let shell_cmd = if cfg!(windows) {
        format!("ping {} -n 1 -w 1000", ip_address)
    } else {
        format!("ping {} -c 1 -W 1", ip_address)
    };

    let mut commands = shell_cmd.split(" ");
    
    let mut info = Command::new(commands.next().unwrap());
    let mut line = commands.next();

    while line.is_some() {
        info.arg(line.unwrap());
        line = commands.next();
    }
    
    info.status().unwrap().code().unwrap() == 0
}

#[cfg(test)]
mod tests {

    use ping;
    #[test]
    fn ping_localhost_should_return_true() {
        assert_eq!(ping("127.0.0.1"), true);
    }

    #[test]
    fn ping_weird_ip_should_return_false() {
        assert_eq!(ping("99.99.99.99"), false);
    }
}
