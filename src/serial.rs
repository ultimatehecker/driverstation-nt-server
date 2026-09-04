pub fn list_ports() -> Result<(), serialport::Error> {
    let ports = serialport::available_ports()?;

    for port in ports {
        println!("Port: {}", port.port_name);
        println!("Type: {:?}", port.port_type);
    }

    Ok(())
}