use std::time::Duration;

pub const ENTER: &'static str = "\r\n";
pub const USER_PROMPT: &'static str = "user>";
pub const LOGIN_PROMPT: &'static str = "login:";
pub const PASSWD: &'static str = "123";
pub const CMD_CONFIG_SHOW: &'static str = "config show";
pub const CMD_CONFIG_SAVE: &'static str = "config save";
pub const CMD_LOG_OFF: &'static str = "system log off";
pub const CMD_LOGOUT: &'static str = "logout";

pub fn execute_cli_cmd(
    port: &mut Box<dyn serialport::SerialPort>,
    command: &str,
) -> Result<String, std::io::Error> {
    let command = format!("{}{}", command, ENTER);
    port.write(command.as_bytes())?;
    port.flush()?;
    let mut serial_buf: Vec<u8> = vec![0; 1024];
    let mut response: Vec<u8> = Vec::new();
    while let Ok(read) = port.read(serial_buf.as_mut_slice()) {
        response.extend_from_slice(&serial_buf[..read]);
    }
    let response = String::from_utf8_lossy(&response[..]);
    let response = response.into_owned();
    Ok(response)
}

pub fn list_usb_ports() -> Vec<String> {
    let ports = match serialport::available_ports() {
        Ok(ports) => ports
            .iter()
            .filter(|p| match p.port_type {
                serialport::SerialPortType::UsbPort(_) => true,
                _ => false,
            })
            .map(|p| p.port_name.clone())
            .collect(),
        Err(_) => {
            vec![]
        }
    };

    ports
}

pub fn get_config_usb(serial_port: &str) -> Result<Vec<(u8,i32)>, std::io::Error> {
    let mut port = serialport::new(serial_port, 115_200)
        .timeout(Duration::from_millis(10))
        .open()
        .expect("Failed to open port");

    let response = execute_cli_cmd(&mut port, "")?;
    // println!("{}", response);

    // let mut config_string = String::new();
    let mut prm_id_val_vec: Vec<(u8, i32)> = Vec::new();

    if let Some(response_last_line) = response
        .split_terminator(&['\n', '\r'])
        .filter(|c| c.len() > 0)
        .last()
    {
        if response_last_line.trim() == USER_PROMPT {
            let _response = execute_cli_cmd(&mut port, CMD_LOG_OFF)?;
            // println!("{}", response);

            let _response = execute_cli_cmd(&mut port, CMD_CONFIG_SHOW)?;
            // println!("{}", response);
        } else if response_last_line.trim() == LOGIN_PROMPT {
            let response = execute_cli_cmd(&mut port, PASSWD)?;
            // println!("{}", response);

            if let Some(response_last_line) = response
                .split_terminator(&['\n', '\r'])
                .filter(|c| c.len() > 0)
                .last()
            {
                if response_last_line.trim() == USER_PROMPT {
                    let _response = execute_cli_cmd(&mut port, CMD_LOG_OFF)?;
                    // println!("{}", response);

                    let response = execute_cli_cmd(&mut port, CMD_CONFIG_SHOW)?;
                    // println!("{}", response);

                    // println!("----------------------------------------------------------------");

                    let lines = response
                        .split_terminator(&['\n', '\r'])
                        .filter(|s| s.len() > 0 && s.starts_with(' '));

                    for line in lines {
                        let mut words = line.split_whitespace();
                        let id = words.next().unwrap();
                        words.next();
                        let _name = words.next().unwrap();
                        words.next();
                        let val = words.next().unwrap();

                        prm_id_val_vec.push((id.parse().unwrap(), val.parse().unwrap()));
                        // config_string.push_str(&format!("{} = {}\n", name, value));

                        // println!("{} = {}", id, value);
                    }
                }
            }
        }

        let _response = execute_cli_cmd(&mut port, CMD_LOGOUT)?;
        // println!("{}", response);
    };

    // Ok(config_string)
    Ok(prm_id_val_vec)
}

pub fn save_config_usb(prm_id_val_vec: Vec<(u8, i32)>, serial_port: String) -> Result<(), std::io::Error> {

    let mut port = serialport::new(serial_port, 115_200)
        .timeout(Duration::from_millis(10))
        .open()
        .expect("Failed to open port");

    let response = execute_cli_cmd(&mut port, "")?;

    if let Some(response_last_line) = response
        .split_terminator(&['\n', '\r'])
        .filter(|c| c.len() > 0)
        .last()
    {
        if response_last_line.trim() == USER_PROMPT {
            let response = execute_cli_cmd(&mut port, CMD_LOG_OFF)?;
            println!("{}\n{}", CMD_LOG_OFF, response);

            let response = execute_cli_cmd(&mut port, CMD_CONFIG_SHOW)?;
            println!("{}\n{}", CMD_CONFIG_SHOW, response);
        } else if response_last_line.trim() == LOGIN_PROMPT {
            let response = execute_cli_cmd(&mut port, PASSWD)?;
            println!("{}\n{}", PASSWD, response);

            if let Some(response_last_line) = response
                .split_terminator(&['\n', '\r'])
                .filter(|c| c.len() > 0)
                .last()
            {
                if response_last_line.trim() == USER_PROMPT {
                    let response = execute_cli_cmd(&mut port, CMD_LOG_OFF)?;
                    println!("{}\n{}", CMD_LOG_OFF, response);

                    for (id, val) in prm_id_val_vec {
                        let cli_cmd = format!("config set {} {}", id, val);
                        execute_cli_cmd(&mut port, &cli_cmd)?;
                        println!("{}\n{}", &cli_cmd, response);
                    }

                    execute_cli_cmd(&mut port, CMD_CONFIG_SAVE)?;
                    println!("{}\n{}", CMD_CONFIG_SAVE, response);
                }
            }
        }

        let response = execute_cli_cmd(&mut port, CMD_LOGOUT)?;
        println!("{}\n{}", CMD_LOGOUT, response);
    };

    Ok(())
}
