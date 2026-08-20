use crate::InputmoduleCommand;

pub mod simple;
pub mod pframe;

pub use simple::SimpleMatrix;
pub use pframe::PFrameMatrix;

pub const COLS: usize = 9;
pub const ROWS: usize = 34;

pub trait LedMatrix {
    fn from_device_label(device_label: &str) -> Self;
    fn send_command(&mut self, cmd: InputmoduleCommand, params: &[u8]) -> std::io::Result<()>;
}

pub fn discover<T: LedMatrix>() -> Vec<T> {
    let ports: Vec<serialport::SerialPortInfo> = serialport::available_ports()
        .unwrap_or_default()
        .into_iter()
        .filter(|e| {
            match &e.port_type {
                serialport::SerialPortType::UsbPort(port_info) => {
                    return port_info.manufacturer == Some("Framework Computer Inc".to_owned()) && port_info.product == Some("LED Matrix Input Module".to_owned())
                },
                _ => false
            }
        })
        .collect();

    let matrices: Vec<T> = ports.iter().map(|e| T::from_device_label(&e.port_name)).collect();
    return matrices;
}