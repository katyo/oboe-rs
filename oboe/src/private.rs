use oboe_sys as ffi;

pub trait RawAudioStreamBase {
    fn _raw_base(&self) -> &ffi::oboe_AudioStreamBase;
    fn _raw_base_mut(&mut self) -> &mut ffi::oboe_AudioStreamBase;
}

pub trait RawAudioStream {
    fn _raw_stream(&self) -> &ffi::oboe_AudioStream;
    fn _raw_stream_mut(&mut self) -> &mut ffi::oboe_AudioStream;
}
