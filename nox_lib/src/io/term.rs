use std::time::Duration;



/*
    ##### UNIX PORTION #####
*/



#[cfg(target_family = "unix")]
use libc::{termios, tcgetattr, tcsetattr, STDIN_FILENO, ECHO, ICANON, select, timeval, read, FD_SET, FD_ZERO, fd_set, c_void};

/// Struct representing the terminal (Unix Implementation)
#[cfg(target_family = "unix")]
pub struct Term {
    termios_settings: termios,
    echo: bool,
    canon: bool
}

#[cfg(target_family = "unix")]
impl Term {

    /// Fails if termios settings cannot be gathered
    pub fn new() -> Result<Self, &'static str> {
        let mut termios_settings: termios;
        let echo: bool;
        let canon: bool;
        
        unsafe {
            termios_settings = std::mem::zeroed();

            let err = tcgetattr(STDIN_FILENO, &mut termios_settings);

            if err == -1 {
                return Err("failed to initialize termios attributes");
            }

            echo = termios_settings.c_lflag & ECHO != 0;
            canon = termios_settings.c_lflag & ICANON != 0;
        }

        Ok(Self {termios_settings, echo, canon})
    }

    /// Saves the current termios settings
    pub fn save_state(&mut self) -> Result<(), &'static str> {
        unsafe {
            let err = tcgetattr(STDIN_FILENO, &mut self.termios_settings);

            if err == -1 {
                return Err("failed to get termios settings");
            }
        }

        Ok(())
    }

    /// Restores the saved termios settings
    pub fn restore_state(&mut self) -> Result<(), &'static str> {
        unsafe {
            let err = tcsetattr(STDIN_FILENO, 0, &self.termios_settings);

            if err == -1 {
                return Err("failed to set termios settings");
            }

            self.canon = self.termios_settings.c_lflag & ICANON != 0;
            self.echo = self.termios_settings.c_lflag & ECHO != 0;
        }

        Ok(())
    }

    /// Enables echo of stdin
    fn enable_echo(&mut self) -> Result<(), &'static str>{
        unsafe {
            let mut term_settings: termios = std::mem::zeroed();

            let mut err = tcgetattr(STDIN_FILENO, &mut term_settings);

            if err == -1 {
                return Err("failed to get termios settings");
            }

            term_settings.c_lflag &= ECHO;

            err = tcsetattr(STDIN_FILENO, 0, &mut term_settings);

            if err == -1 {
                return Err("failed to set termios settings");
            }

            self.echo = true;
        }

        Ok(())
    }

    /// Disables echo of stdin
    fn disable_echo(&mut self) -> Result<(), &'static str>{
        unsafe {
            let mut term_settings: termios = std::mem::zeroed();

            let mut err = tcgetattr(STDIN_FILENO, &mut term_settings);

            if err == -1 {
                return Err("failed to get termios settings");
            }

            term_settings.c_lflag &= !ECHO;

            err = tcsetattr(STDIN_FILENO, 0, &mut term_settings);

            if err == -1 {
                return Err("failed to set termios settings");
            }

            self.echo = false;
        }

        Ok(())
    }

    /// Disable raw mode
    fn disable_raw_mode(&mut self) -> Result<(), &'static str> {
        unsafe {
            let mut term_settings: termios = std::mem::zeroed();

            let mut err = tcgetattr(STDIN_FILENO, &mut term_settings);

            if err == -1 {
                return Err("failed to get termios settings");
            }

            term_settings.c_lflag &= ICANON;

            err = tcsetattr(STDIN_FILENO, 0, &mut term_settings);

            if err == -1 {
                return Err("failed to set termios settings");
            }

            self.canon = true;
        }

        Ok(())
    }

    /// Enables raw mode
    fn enable_raw_mode(&mut self) -> Result<(), &'static str> {
        unsafe {
            let mut term_settings: termios = std::mem::zeroed();

            let mut err = tcgetattr(STDIN_FILENO, &mut term_settings);

            if err == -1 {
                return Err("failed to get termios settings");
            }

            term_settings.c_lflag &= !ICANON;

            err = tcsetattr(STDIN_FILENO, 0, &mut term_settings);

            if err == -1 {
                return Err("failed to set termios settings");
            }

            self.canon = false;
        }

        Ok(())
    }

    /// Disables echo and enables unbuffered input
    pub fn enable_raw_input(&mut self) -> Result<(), &'static str> {
        self.disable_echo()?;
        self.enable_raw_mode()?;
        Ok(())
    }

    /// Enables echo and disables unbuffered input
    pub fn disable_raw_input(&mut self) -> Result<(), &'static str> {
        self.enable_echo()?;
        self.disable_raw_mode()?;
        Ok(())
    }

    /// retrieves one character from stdin unbuffered, without echo
    pub fn getch(&mut self, dur: Duration) -> Result<Option<char>, &'static str> {
        let echo = self.echo;
        let canon = self.canon;

        let mut result = Ok(None);

        if canon {
            self.enable_raw_mode()?;
        }
        
        if echo {
            self.disable_echo()?;
        }

        
        unsafe {
            let mut read_fds: fd_set = std::mem::zeroed();
            FD_ZERO(&mut read_fds);
            FD_SET(STDIN_FILENO, &mut read_fds);
            
            let mut tv = timeval {tv_sec: 0, tv_usec: dur.as_micros() as i64};

            let has_input = select(1, &mut read_fds, std::ptr::null_mut(), std::ptr::null_mut(), &mut tv);
        
            if has_input > 0 {
                let mut buf: [u8; 1] = [0];
                let err = read(STDIN_FILENO, buf.as_mut_ptr() as *mut c_void, 1);

                if err == 1 {
                    result = Ok(Some(buf[0] as char));
                }
                else {
                    result = Err("failed to read byte from stdin");
                }
            }   
        }

        if canon {
            self.disable_raw_mode()?;
        }

        if echo {
            self.enable_echo()?;
        }

        result
    }
}



/*
    ##### WINDOWS PORTION #####
*/



#[cfg(target_family = "windows")]
struct Term {

}

#[cfg(target_family = "windows")]
impl Term {
    pub fn enable_raw_input() -> Result<(), &'static str>
    {
        Err("raw input not implemented yet for windows :p");
    }

    pub fn disable_raw_input() -> Result<(), &'static str> {
        Err("raw input not implemented yet for windows :p");
    }

    pub fn getch(dur: Duration) -> Result<Option<char>, &'static str> {
        Err("getch not implemented yet for windows :p");
    }
}