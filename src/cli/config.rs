/* Copyright 2018 AUTHORS, James Ray, Josiah @ChosunOne, and Luke Schoen

This is free and unencumbered software released into the public domain.

Anyone is free to copy, modify, publish, use, compile, sell, or
distribute this software, either in source code form or as a compiled
binary, for any purpose, commercial or non-commercial, and by any
means.

In jurisdictions that recognize copyright laws, the author or authors
of this software dedicate any and all copyright interest in the
software to the public domain. We make this dedication for the benefit
of the public at large and to the detriment of our heirs and
successors. We intend this dedication to be an overt act of
relinquishment in perpetuity of all present and future rights to this
software under copyright law.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
IN NO EVENT SHALL THE 
AUTHORS, James Ray, Josiah @ChosunOne, and Luke Schoen
BE LIABLE FOR ANY CLAIM, DAMAGES OR
OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
OTHER DEALINGS IN THE SOFTWARE.
 
For more information, please refer to <http://unlicense.org>
*/

#[derive(Debug, PartialEq)]
pub enum Mode {
    Proposer,
    Notary,
    Both
}

impl Mode {
    /// Returns a string value of the current running mode
    pub fn value(&self) -> String {
        match *self {
            Mode::Proposer => "proposer".to_string(),
            Mode::Notary => "notary".to_string(),
            _ => "both".to_string()
        }
    }
}

/// This holds configuration options for running the client
#[derive(Debug, PartialEq)]
pub struct Config {
    pub mode: Mode
}

impl Config {
    /// Creates a new configuration to be run
    pub fn new(mode: Mode) -> Config {
        Config { mode }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_set_thread_name_for_config_arguments() {
        let test_proposer_name = Mode::Proposer.value();
        let test_notary_name = Mode::Notary.value();

        assert_eq!(test_proposer_name, "proposer".to_string());
        assert_eq!(test_notary_name, "notary".to_string());
    }
}
