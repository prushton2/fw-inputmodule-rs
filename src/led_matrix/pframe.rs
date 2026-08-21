use std::time::Duration;

use serialport::SerialPort;

use crate::{InputmoduleCommand, led_matrix::{COLS, LedMatrix, ROWS}};

pub struct PFrameMatrix {
    port: Box<dyn SerialPort>,
    framebuffer: [[Option<u8>; ROWS]; COLS],
    display:     [[u8; ROWS]; COLS],
}

impl PFrameMatrix {
    pub fn write_buffer(&mut self, col: usize, row: usize, brightness: Option<u8>) {
        self.framebuffer[col][row] = brightness;
    }

    pub fn fill_buffer(&mut self, value: Option<u8>) {
        self.framebuffer = [[value; ROWS]; COLS];
    }

    pub fn stage_changes(&mut self) {
        for i in 0..COLS {
            self.stage_col_if_changed(i);
        }
    }

    fn stage_col_if_changed(&mut self, column: usize) {
        let mut col = self.display[column].clone();
        let mut changed = false;

        for i in 0..ROWS {
            if let Some(new_value) = self.framebuffer[column][i] {
                changed = true;
                col[i] = new_value;
            }
        }

        if changed {
            
            let mut params = Vec::with_capacity(1 + ROWS);
            
            params.push(column as u8);
            params.extend_from_slice(&col);

            let _ = self.send_command(InputmoduleCommand::StageCol, &params);

            // update the state of the frame
            self.display[column] = col;
            self.framebuffer[column] = [None; ROWS];
        }
    }
    
    pub fn flush_buffer(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        return self.send_command(InputmoduleCommand::FlushCols, &[]).map_err(|e| e.into())
    }
}

impl LedMatrix for PFrameMatrix {
    fn from_device_label(device_label: &str) -> Self {
        let port = serialport::new(device_label, 115_200)
            .timeout(Duration::from_millis(500))
            .open()
            .expect("failed to open port");
        Self {
            port: port,
            framebuffer: [[None; ROWS]; COLS],
            display:     [[0; ROWS]; COLS]
        }
    }

    fn send_command(&mut self, cmd: InputmoduleCommand, params: &[u8]) -> std::io::Result<()> {
        let mut buf = vec![0x32, 0xAC, cmd.into()];
        buf.extend_from_slice(params);
        self.port.write_all(&buf)
    }
}