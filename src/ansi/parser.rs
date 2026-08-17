#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Ground,
    Escape,
    Csi,
}

pub struct Parser {
    state: State,
    params: [u16; 16],
    params_len: usize,
    current: u16,
}

#[derive(Debug, Clone, Copy)]
pub enum Action {
    Print(u8),

    Sgr { params: [u16; 16], len: usize },

    CursorPosition { params: [u16; 16], len: usize },
}

impl Parser {
    pub const fn new() -> Self {
        Self {
            state: State::Ground,
            params: [0; 16],
            params_len: 0,
            current: 0,
        }
    }

    pub fn advance(&mut self, byte: u8) -> Option<Action> {
        match self.state {
            State::Ground => match byte {
                0x1b => {
                    self.state = State::Escape;
                    None
                }

                _ => Some(Action::Print(byte)),
            },

            State::Escape => {
                match byte {
                    b'[' => {
                        self.state = State::Csi;
                        self.params_len = 0;
                        self.current = 0;
                    }

                    _ => {
                        self.state = State::Ground;
                    }
                }

                None
            }

            State::Csi => match byte {
                b'0'..=b'9' => {
                    self.current = self
                        .current
                        .saturating_mul(10)
                        .saturating_add((byte - b'0') as u16);
                    None
                }

                b';' => {
                    if self.params_len < self.params.len() {
                        self.params[self.params_len] = self.current;
                        self.params_len += 1;
                    }

                    self.current = 0;
                    None
                }

                b'm' => {
                    if self.params_len < self.params.len() {
                        self.params[self.params_len] = self.current;
                        self.params_len += 1;
                    }

                    self.state = State::Ground;

                    Some(Action::Sgr {
                        params: self.params,
                        len: self.params_len,
                    })
                }

                b'H' => {
                    self.state = State::Ground;

                    Some(Action::CursorPosition {
                        params: self.params,
                        len: self.params_len,
                    })
                }

                _ => {
                    self.state = State::Ground;
                    None
                }
            },
        }
    }
}
