/*
 * Copyright 2025 Nicolas Spijkerman
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#[cfg(unix)]
use std::{cmp::Ordering, process};

use crate::{args, mandrake::Mandrake, result::Result};

pub fn run() -> Result<()> {
    let opt: args::Options = args::Options::parse();

    #[cfg(unix)]
    unsafe {
        match libc::fork().cmp(&0) {
            Ordering::Less => process::exit(1),
            Ordering::Equal => {
                libc::setsid();
            }
            Ordering::Greater => process::exit(0),
        }
    }

    let mandrake: Mandrake = if let Some(duration) = opt.duration {
        Mandrake::WithDuration { duration }
    } else {
        Mandrake::Infinite
    };

    mandrake.scream()
}
