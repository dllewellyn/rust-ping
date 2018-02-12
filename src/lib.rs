use std::process::Command;

   
/// Ping multiple addresses at once.
///
/// Args:
///     range - the range of addresses to ping e.g. ["192.168.1.1", "192.168.1.2"]
///
/// Returns:
////     The list of all successful pings
////
//// Example:
//// ``` 
//// use ping_multiple; 
////
//// let input = vec!["99.99.99.99", "127.0.0.1", "5.5.5.5"];
//// let result = ping_multiple(input);
///  assert_eq!(result.len(), 1);
//// ```
pub fn ping_multiple<'a>(range: Vec<&'a str>) -> Vec<&'a str> {
    let mut return_value = Vec::new();
    for i in 0..range.len() {
        let result = ping(range[i]);
        
        if result {
            return_value.push(range[i]);
        }
    }

    return return_value;
}

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
    use ping_multiple;

    #[test]
    fn ping_localhost_should_return_true() {
        assert_eq!(ping("127.0.0.1"), true);
    }

    #[test]
    fn ping_weird_ip_should_return_false() {
        assert_eq!(ping("99.99.99.99"), false);
    }

    #[test]
    fn ping_range_of_addresses() {
        let input = vec!["99.99.99.99", "127.0.0.1", "5.5.5.5"];
        let result = ping_multiple(input);
        assert_eq!(result.len(), 1);
    }
}
