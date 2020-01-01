//use oboe_sys as ffi;
use num_traits::FromPrimitive;

use std::fmt::{self, Debug};

use super::{
    ChannelCount,
    Direction,
    AudioFormat,
    SharingMode,
    PerformanceMode,
    Usage,
    ContentType,
    InputPreset,
    SessionId,
    SampleRateConversionQuality,
    RawAudioStreamBase,
};

/**
 * Base trait containing parameters for audio streams and builders.
 */
pub trait AudioStreamBase {
    /**
     * @return number of channels, for example 2 for stereo, or kUnspecified
     */
    fn get_channel_count(&self) -> ChannelCount;

    /**
     * @return Direction::Input or Direction::Output
     */
    fn get_direction(&self) -> Direction;

    /**
     * @return sample rate for the stream or kUnspecified
     */
    fn get_sample_rate(&self) -> i32;

    /**
     * @return the number of frames in each callback or kUnspecified.
     */
    fn get_frames_per_callback(&self) -> i32;

    /**
     * @return the audio sample format (e.g. Float or I16)
     */
    fn get_format(&self) -> AudioFormat;

    /**
     * Query the maximum number of frames that can be filled without blocking.
     * If the stream has been closed the last known value will be returned.
     *
     * @return buffer size
     */
    fn get_buffer_size_in_frames(&self) -> i32;

    /**
     * @return capacityInFrames or kUnspecified
     */
    fn get_buffer_capacity_in_frames(&self) -> i32;

    /**
     * @return the sharing mode of the stream.
     */
    fn get_sharing_mode(&self) -> SharingMode;

    /**
     * @return the performance mode of the stream.
     */
    fn get_performance_mode(&self) -> PerformanceMode;

    /**
     * @return the device ID of the stream.
     */
    fn get_device_id(&self) -> i32;

    /**
     * @return the callback object for this stream, if set.
     */
    //fn get_callback(&self) -> &AudioStreamCallback;

    /**
     * @return the usage for this stream.
     */
    fn get_usage(&self) -> Usage;

    /**
     * @return the stream's content type.
     */
    fn get_content_type(&self) -> ContentType;

    /**
     * @return the stream's input preset.
     */
    fn get_input_preset(&self) -> InputPreset;

    /**
     * @return the stream's session ID allocation strategy (None or Allocate).
     */
    fn get_session_id(&self) -> SessionId;

    /**
     * @return true if Oboe can convert channel counts to achieve optimal results.
     */
    fn is_channel_conversion_allowed(&self) -> bool;

    /**
     * @return true if  Oboe can convert data formats to achieve optimal results.
     */
    fn is_format_conversion_allowed(&self) -> bool;

    /**
     * @return whether and how Oboe can convert sample rates to achieve optimal results.
     */
    fn get_sample_rate_conversion_quality(&self) -> SampleRateConversionQuality;
}

impl<T: RawAudioStreamBase> AudioStreamBase for T {
    fn get_channel_count(&self) -> ChannelCount {
        FromPrimitive::from_i32(self._raw_base().mChannelCount).unwrap()
    }

    fn get_direction(&self) -> Direction {
        FromPrimitive::from_i32(self._raw_base().mDirection).unwrap()
    }

    fn get_sample_rate(&self) -> i32 {
        self._raw_base().mSampleRate
    }

    fn get_frames_per_callback(&self) -> i32 {
        self._raw_base().mFramesPerCallback
    }

    fn get_format(&self) -> AudioFormat {
        FromPrimitive::from_i32(self._raw_base().mFormat).unwrap()
    }

    fn get_buffer_size_in_frames(&self) -> i32 {
        self._raw_base().mBufferSizeInFrames
    }

    fn get_buffer_capacity_in_frames(&self) -> i32 {
        self._raw_base().mBufferCapacityInFrames
    }

    fn get_sharing_mode(&self) -> SharingMode {
        FromPrimitive::from_i32(self._raw_base().mSharingMode).unwrap()
    }

    fn get_performance_mode(&self) -> PerformanceMode {
        FromPrimitive::from_i32(self._raw_base().mPerformanceMode).unwrap()
    }

    fn get_device_id(&self) -> i32 {
        self._raw_base().mDeviceId
    }

    /*fn get_callback(&self) -> &AudioStreamCallback {
        self._raw_base().mStreamCallback
    }*/

    fn get_usage(&self) -> Usage {
        FromPrimitive::from_i32(self._raw_base().mUsage).unwrap()
    }

    fn get_content_type(&self) -> ContentType {
        FromPrimitive::from_i32(self._raw_base().mContentType).unwrap()
    }

    fn get_input_preset(&self) -> InputPreset {
        FromPrimitive::from_i32(self._raw_base().mInputPreset).unwrap()
    }

    fn get_session_id(&self) -> SessionId {
        FromPrimitive::from_i32(self._raw_base().mSessionId).unwrap()
    }

    fn is_channel_conversion_allowed(&self) -> bool {
        self._raw_base().mChannelConversionAllowed
    }

    fn is_format_conversion_allowed(&self) -> bool {
        self._raw_base().mFormatConversionAllowed
    }

    fn get_sample_rate_conversion_quality(&self) -> SampleRateConversionQuality {
        FromPrimitive::from_i32(self._raw_base().mSampleRateConversionQuality).unwrap()
    }
}

pub(crate) fn audio_stream_base_fmt<T: AudioStreamBase>(base: &T, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    "DeviceId: ".fmt(f)?;
    base.get_device_id().fmt(f)?;
    "\nSessionId: ".fmt(f)?;
    base.get_session_id().fmt(f)?;
    "\nDirection: ".fmt(f)?;
    base.get_direction().fmt(f)?;
    if base.get_direction() == Direction::Input {
        "\nInput preset: ".fmt(f)?;
        base.get_input_preset().fmt(f)?;
    }
    "\nBuffer capacity in frames: ".fmt(f)?;
    base.get_buffer_capacity_in_frames().fmt(f)?;
    "\nBuffer size in frames: ".fmt(f)?;
    base.get_buffer_size_in_frames().fmt(f)?;
    "\nFrames per callback: ".fmt(f)?;
    base.get_frames_per_callback().fmt(f)?;
    "\nSample rate: ".fmt(f)?;
    base.get_sample_rate().fmt(f)?;
    "\nSample rate conversion quality: ".fmt(f)?;
    base.get_sample_rate_conversion_quality().fmt(f)?;
    "\nChannel count: ".fmt(f)?;
    base.get_channel_count().fmt(f)?;
    if base.is_channel_conversion_allowed() {
        " (conversion allowed)".fmt(f)?;
    }
    "\nFormat: ".fmt(f)?;
    base.get_format().fmt(f)?;
    if base.is_format_conversion_allowed() {
        " (conversion allowed)".fmt(f)?;
    }
    "\nSharing mode: ".fmt(f)?;
    base.get_sharing_mode().fmt(f)?;
    "\nPerformance mode: ".fmt(f)?;
    base.get_performance_mode().fmt(f)?;
    "\nUsage: ".fmt(f)?;
    base.get_usage().fmt(f)?;
    "\nContent type: ".fmt(f)?;
    base.get_content_type().fmt(f)?;
    '\n'.fmt(f)
}
