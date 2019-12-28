use super::{
    Direction,
    AudioFormat,
    ChannelCount,
};

pub struct Unspecified;

pub trait IsDirection {
    const DIRECTION: Direction;
    // type BufferType<T: IsFrameType>;
    // ^unstable feature
}

pub struct Input;

impl IsDirection for Input {
    const DIRECTION: Direction = Direction::Input;
    // type BufferType<'b, T: IsFrameType> = &'b [<T as IsFrameType>::Type];
    // ^unstable feature
}

pub struct Output;

impl IsDirection for Output {
    const DIRECTION: Direction = Direction::Output;
    // type BufferType<'b, T: IsFrameType> = &'b mut [<T as IsFrameType>::Type];
    // ^unstable feature
}

pub trait IsFormat {
    const FORMAT: AudioFormat;
}

impl IsFormat for Unspecified {
    const FORMAT: AudioFormat = AudioFormat::Unspecified;
}

impl IsFormat for i16 {
    const FORMAT: AudioFormat = AudioFormat::I16;
}

impl IsFormat for f32 {
    const FORMAT: AudioFormat = AudioFormat::F32;
}

pub trait IsChannelCount {
    const CHANNEL_COUNT: ChannelCount;
}

impl IsChannelCount for Unspecified {
    const CHANNEL_COUNT: ChannelCount = ChannelCount::Unspecified;
}

pub struct Mono;

impl IsChannelCount for Mono {
    const CHANNEL_COUNT: ChannelCount = ChannelCount::Mono;
}

pub struct Stereo;

impl IsChannelCount for Stereo {
    const CHANNEL_COUNT: ChannelCount = ChannelCount::Stereo;
}

pub enum AltFrame<T: IsFormat> {
    Mono(T),
    Stereo(T, T),
}

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

/*impl<T: IsFormat> IsFormat for (T, Unspecified) {
    const FORMAT: AudioFormat = <T as IsFormat>::FORMAT;
}

impl<T> IsChannelCount for (T, Unspecified) {
    const CHANNEL_COUNT: ChannelCount = ChannelCount::Unspecified;
}*/

impl<T: IsFormat> IsFrameType for (T, Mono) {
    type Type = T;
    type Format = T;
    type ChannelCount = Mono;
}

/*
impl<T: IsFormat> IsFormat for (T, Mono) {
    const FORMAT: AudioFormat = <T as IsFormat>::FORMAT;
}

impl<T> IsChannelCount for (T, Mono) {
    const CHANNEL_COUNT: ChannelCount = ChannelCount::Mono;
}
*/

impl<T: IsFormat> IsFrameType for (T, Stereo) {
    type Type = (T, T);
    type Format = T;
    type ChannelCount = Stereo;
}

/*impl<T: IsFormat> IsFormat for (T, Stereo) {
    const FORMAT: AudioFormat = <T as IsFormat>::FORMAT;
}

impl<T> IsChannelCount for (T, Stereo) {
    const CHANNEL_COUNT: ChannelCount = ChannelCount::Stereo;
}
*/
