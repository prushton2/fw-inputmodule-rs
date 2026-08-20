pub mod led_matrix;

// https://github.com/FrameworkComputer/inputmodule-rs/blob/main/commands.md
pub enum InputmoduleCommand {
    Brightness,
    Pattern,
    Bootloader,
    Sleep,
    GetSleep,
    Animate,
    GetAnimate,
    Panic,
    DrawBW,
    StageCol,
    FlushCols,
    SetText,
    StartGame,
    GameCtrl,
    GameStatus,
    SetColor,
    DisplayOn,
    InvertScreen,
    SetPxCol,
    FlushFB,
    Version,
}

impl From<InputmoduleCommand> for u8 {
    fn from(value: InputmoduleCommand) -> Self {
        match value {
            InputmoduleCommand::Brightness    =>  0x00,
            InputmoduleCommand::Pattern       =>  0x01,
            InputmoduleCommand::Bootloader    =>  0x02,
            InputmoduleCommand::Sleep         =>  0x03,
            InputmoduleCommand::GetSleep      =>  0x03,
            InputmoduleCommand::Animate       =>  0x04,
            InputmoduleCommand::GetAnimate    =>  0x04,
            InputmoduleCommand::Panic         =>  0x05,
            InputmoduleCommand::DrawBW        =>  0x06,
            InputmoduleCommand::StageCol      =>  0x07,
            InputmoduleCommand::FlushCols     =>  0x08,
            InputmoduleCommand::SetText       =>  0x09,
            InputmoduleCommand::StartGame     =>  0x10,
            InputmoduleCommand::GameCtrl      =>  0x11,
            InputmoduleCommand::GameStatus    =>  0x12,
            InputmoduleCommand::SetColor      =>  0x13,
            InputmoduleCommand::DisplayOn     =>  0x14,
            InputmoduleCommand::InvertScreen  =>  0x15,
            InputmoduleCommand::SetPxCol      =>  0x16,
            InputmoduleCommand::FlushFB       =>  0x17,
            InputmoduleCommand::Version       =>  0x20,
        }
    }
}