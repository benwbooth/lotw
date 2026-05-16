#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApuChannel {
    Pulse1,
    Pulse2,
    Triangle,
    Noise,
    Dmc,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApuCommand {
    Write {
        register: u16,
        value: u8,
    },
    WaitFrames(u16),
    PulseEnvelope {
        channel: ApuChannel,
        duty: u8,
        volume: u8,
    },
    End,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ApuProgram {
    pub name: &'static str,
    pub commands: &'static [ApuCommand],
}

pub const fn dsl_available() -> bool {
    true
}

#[macro_export]
macro_rules! apu_program {
    (@channel pulse1) => {
        $crate::audio_dsl::ApuChannel::Pulse1
    };
    (@channel pulse2) => {
        $crate::audio_dsl::ApuChannel::Pulse2
    };
    (@channel triangle) => {
        $crate::audio_dsl::ApuChannel::Triangle
    };
    (@channel noise) => {
        $crate::audio_dsl::ApuChannel::Noise
    };
    (@channel dmc) => {
        $crate::audio_dsl::ApuChannel::Dmc
    };
    (@cmd write($register:expr, $value:expr)) => {
        $crate::audio_dsl::ApuCommand::Write {
            register: $register,
            value: $value,
        }
    };
    (@cmd wait_frames($frames:expr)) => {
        $crate::audio_dsl::ApuCommand::WaitFrames($frames)
    };
    (@cmd pulse_envelope($channel:ident, duty = $duty:expr, volume = $volume:expr)) => {
        $crate::audio_dsl::ApuCommand::PulseEnvelope {
            channel: $crate::apu_program!(@channel $channel),
            duty: $duty,
            volume: $volume,
        }
    };
    (@cmd end()) => {
        $crate::audio_dsl::ApuCommand::End
    };
    ($name:literal, [ $($command:ident ( $($args:tt)* )),* $(,)? ]) => {
        $crate::audio_dsl::ApuProgram {
            name: $name,
            commands: &[
                $($crate::apu_program!(@cmd $command($($args)*)),)*
            ],
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn macro_builds_high_level_apu_program() {
        let program = crate::apu_program!(
            "pulse_blip",
            [
                pulse_envelope(pulse1, duty = 2, volume = 12),
                write(0x4002, 0x34),
                wait_frames(2),
                end(),
            ]
        );

        assert!(dsl_available());
        assert_eq!(program.name, "pulse_blip");
        assert_eq!(
            program.commands,
            &[
                ApuCommand::PulseEnvelope {
                    channel: ApuChannel::Pulse1,
                    duty: 2,
                    volume: 12,
                },
                ApuCommand::Write {
                    register: 0x4002,
                    value: 0x34,
                },
                ApuCommand::WaitFrames(2),
                ApuCommand::End,
            ]
        );
    }
}
