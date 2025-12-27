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

use crate::{error::Error, result::Result};
use rand::Rng;
use rodio::{source::SineWave, OutputStream, OutputStreamBuilder, Sink, Source};
use std::time::Duration;

pub enum Mandrake {
    Infinite,
    WithDuration { duration: u64 },
}

impl Mandrake {
    pub fn scream(self) -> Result<()> {
        let stream_handle: OutputStream =
            OutputStreamBuilder::open_default_stream().map_err(Error::StreamError)?;

        let sink: Sink = Sink::connect_new(&stream_handle.mixer());

        match self {
            Self::Infinite => self.scream_infinite(sink),
            Self::WithDuration { duration } => self.scream_with_duration(sink, duration),
        }
    }

    fn scream_infinite(self, sink: Sink) -> Result<()> {
        loop {
            let frequency: f32 = rand::rng().random_range(100.0..1000.0);

            let duration: Duration = Duration::from_millis(rand::rng().random_range(50..500));

            let amplify: f32 = rand::rng().random_range(0.1..1.0);

            let source = SineWave::new(frequency)
                .take_duration(duration)
                .amplify(amplify);

            sink.append(source);
        }
    }

    fn scream_with_duration(self, sink: Sink, duration: u64) -> Result<()> {
        let mut scream: bool = true;

        let max_duration: Duration = Duration::from_secs(duration);
        let mut scream_duration: Duration = Duration::ZERO;

        loop {
            if !scream {
                break;
            }

            let frequency: f32 = rand::rng().random_range(100.0..1000.0);

            let mut duration: Duration = Duration::from_millis(rand::rng().random_range(50..500));

            if duration + scream_duration > max_duration {
                duration = max_duration - scream_duration;
                scream = false;
            } else {
                scream_duration += duration;
            }

            let amplify: f32 = rand::rng().random_range(0.1..1.0);

            let source = SineWave::new(frequency)
                .take_duration(duration)
                .amplify(amplify);

            sink.append(source);
        }

        sink.sleep_until_end();

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Mandrake;
    use crate::result::Result;
    use std::time::{Duration, Instant};

    #[test]
    fn scream_with_duration() {
        let duration_secs: u64 = 2;

        let mandrake: Mandrake = Mandrake::WithDuration {
            duration: duration_secs,
        };

        let start: Instant = Instant::now();
        let result: Result<()> = mandrake.scream();
        let elapsed: Duration = start.elapsed();

        assert!(result.is_ok(), "scream_with_duration should succeed");

        let tolerance_secs: u64 = 1;

        assert!(
            elapsed.as_secs() >= duration_secs,
            "scream should last at least {} seconds, but was {:?}",
            duration_secs,
            elapsed
        );

        assert!(
            elapsed.as_secs() <= duration_secs + tolerance_secs,
            "scream should complete within {} seconds, but took {:?}",
            duration_secs + tolerance_secs,
            elapsed
        );
    }
}
