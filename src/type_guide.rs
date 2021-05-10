use super::{AudioFormat, ChannelCount, Direction};

/**
 * Unspecified marker type for use everywhere
 */
pub struct Unspecified;

/**
 * The trait for direction marker types
 */
pub trait IsDirection {
    const DIRECTION: Direction;
}

/**
 * The input direction marker
 */
pub struct Input;

impl IsDirection for Input {
    const DIRECTION: Direction = Direction::Input;
}

/**
 * The output direction marker
 */
pub struct Output;

impl IsDirection for Output {
    const DIRECTION: Direction = Direction::Output;
}

/**
 * The traint for format marker types
 */
pub trait IsFormat {
    const FORMAT: AudioFormat;
}

impl IsFormat for Unspecified {
    const FORMAT: AudioFormat = AudioFormat::Unspecified;
}

impl IsFormat for i16 {
    const FORMAT: AudioFormat = AudioFormat::I16;
}

impl IsFormat for i32 {
    const FORMAT: AudioFormat = AudioFormat::I32;
}

impl IsFormat for f32 {
    const FORMAT: AudioFormat = AudioFormat::F32;
}

/**
 * The trait for channel count marker types
 */
pub trait IsChannelCount {
    const CHANNEL_COUNT: ChannelCount;
}

impl IsChannelCount for Unspecified {
    const CHANNEL_COUNT: ChannelCount = ChannelCount::Unspecified;
}

/**
 * The single mono channel configuration marker
 */
pub struct Mono;

impl IsChannelCount for Mono {
    const CHANNEL_COUNT: ChannelCount = ChannelCount::Mono;
}

/**
 * The dual stereo channels configuration marker
 */
pub struct Stereo;

impl IsChannelCount for Stereo {
    const CHANNEL_COUNT: ChannelCount = ChannelCount::Stereo;
}

pub enum AltFrame<T: IsFormat> {
    Mono(T),
    Stereo(T, T),
}

/**
 * The trait for frame type marker types
 */
pub trait IsFrameType {
    type Type;
    type Format: IsFormat;
    type ChannelCount: IsChannelCount;
}

impl<T: IsFormat> IsFrameType for (T, Unspecified) {
    type Type = AltFrame<T>;
    type Format = T;
    type ChannelCount = Unspecified;
}

impl<T: IsFormat> IsFrameType for (T, Mono) {
    type Type = T;
    type Format = T;
    type ChannelCount = Mono;
}

impl<T: IsFormat> IsFrameType for (T, Stereo) {
    type Type = (T, T);
    type Format = T;
    type ChannelCount = Stereo;
}
