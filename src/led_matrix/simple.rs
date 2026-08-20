use std::time::Duration;

use serialport::SerialPort;

use crate::{InputmoduleCommand, led_matrix::{COLS, LedMatrix, ROWS}};

pub struct SimpleMatrix {
    port: Box<dyn SerialPort>,
    framebuffer: [[u8; ROWS]; COLS],
}

impl SimpleMatrix {
    pub fn write_buffer(&mut self, col: usize, row: usize, brightness: u8) {
        self.framebuffer[col][row] = brightness;
    }

    pub fn clear_buffer(&mut self) {
        self.framebuffer = [[0; ROWS]; COLS];
    }

    pub fn stage_col(&mut self, col: usize) {
        // StageCol: [column_index, 34 brightness bytes for that column]
        let mut params = Vec::with_capacity(1 + ROWS);
        params.push(col as u8);
        params.extend_from_slice(&self.framebuffer[col]);
        let _ = self.send_command(InputmoduleCommand::StageCol, &params);
    }

    pub fn stage_cols(&mut self) {
        for col in 0..COLS {
            self.stage_col(col);
        }
    }
    
    pub fn flush_buffer(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        return self.send_command(InputmoduleCommand::FlushCols, &[]).map_err(|e| e.into())
    }

    pub fn draw_bw(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut bytes: Vec<u8> = Vec::with_capacity((ROWS * COLS).div_ceil(8));
        let mut byte: u8 = 0;
        let mut nbits: u32 = 0;

        for y in 0..ROWS {
            for x in 0..COLS {
                let px = self.framebuffer[x][y]; // [col][row]

                byte = (byte >> 1) | (u8::from(px >= 128) << 7);
                nbits += 1;

                if nbits == 8 {
                    bytes.push(byte);
                    byte = 0;
                    nbits = 0;
                }
            }
        }

        if nbits > 0 {
            bytes.push(byte >> (8 - nbits))
        }

        while bytes.len() < 39 {
            bytes.push(0);
        }


        return self.send_command(InputmoduleCommand::DrawBW, &bytes).map_err(|e| e.into());
    }
}

impl LedMatrix for SimpleMatrix {
    fn from_device_label(device_label: &str) -> Self {
        let port = serialport::new(device_label, 115_200)
            .timeout(Duration::from_millis(500))
            .open()
            .expect("failed to open port");
        Self {
            port: port,
            framebuffer: [[0; ROWS]; COLS]
        }
    }

    fn send_command(&mut self, cmd: InputmoduleCommand, params: &[u8]) -> std::io::Result<()> {
        let mut buf = vec![0x32, 0xAC, cmd.into()];
        buf.extend_from_slice(params);
        self.port.write_all(&buf)
    }
}