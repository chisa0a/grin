// Copyright 2019 The Grin Developers
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use libc;
use std::io;

/// Fork errors
#[derive(Debug)]
pub enum Error {
  /// LibC OS Error
  OS(String),
}

/// Fork a child process (wraps libc::fork)
// slightly modified from Servo test example
pub unsafe fn fork<F: FnOnce()>(child_func: F) -> Result<libc::pid_t, Error> {
  match libc::fork() {
    -1 => {
      Err(Error::OS(format!("Fork failed: {:?}", io::Error::last_os_error())))
    },
    0 => {
       child_func();
       libc::exit(0);
    },
    pid => Ok(pid),
  }
}
