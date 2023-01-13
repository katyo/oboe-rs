use num_traits::FromPrimitive;
use oboe_sys as ffi;
use std::{
    ffi::c_void,
    fmt,
    marker::PhantomData,
    mem::{ManuallyDrop, MaybeUninit},
    ops::{Deref, DerefMut},
};

use crate::{set_input_callback, set_output_callback};

use super::{
    audio_stream_base_fmt, wrap_status, AudioApi, AudioInputCallback, AudioOutputCallback,
    AudioStreamAsync, AudioStreamSync, ContentType, Input, InputPreset, IsChannelCount,
    IsDirection, IsFormat, IsFrameType, Mono, Output, PerformanceMode, RawAudioStreamBase, Result,
    SampleRateConversionQuality, SessionId, SharingMode, Stereo, Unspecified, Usage,
};

#[repr(transparent)]
pub(crate) struct AudioStreamBuilderHandle(*mut ffi::oboe_AudioStreamBuilder);

impl Default for AudioStreamBuilderHandle {
    fn default() -> Self {
        Self(unsafe { ffi::oboe_AudioStreamBuilder_new() })
    }
}

impl Drop for AudioStreamBuilderHandle {
    fn drop(&mut self) {
        unsafe { ffi::oboe_AudioStreamBuilder_delete(self.0) }
    }
}

impl Deref for AudioStreamBuilderHandle {
    type Target = ffi::oboe_AudioStreamBuilder;

    fn deref(&self) -> &Self::Target {
        unsafe { &(*self.0) }
    }
}

impl DerefMut for AudioStreamBuilderHandle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut (*self.0) }
    }
}

/**
 * Factory for an audio stream.
 */
#[repr(transparent)]
pub struct AudioStreamBuilder<D, C, T> {
    raw: ManuallyDrop<AudioStreamBuilderHandle>,
    _phantom: PhantomData<(D, C, T)>,
}

impl<D, C, T> Drop for AudioStreamBuilder<D, C, T> {
    fn drop(&mut self) {
        // SAFETY: self.raw is only drop here, or taken in Self::destructs, which don't drop self.
        unsafe {
            ManuallyDrop::drop(&mut self.raw);
        }
    }
}

impl<D, C, T> fmt::Debug for AudioStreamBuilder<D, C, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        audio_stream_base_fmt(self, f)
    }
}

impl<D, C, T> RawAudioStreamBase for AudioStreamBuilder<D, C, T> {
    fn _raw_base(&self) -> &ffi::oboe_AudioStreamBase {
        unsafe { &*ffi::oboe_AudioStreamBuilder_getBase(self.raw.0) }
    }

    fn _raw_base_mut(&mut self) -> &mut ffi::oboe_AudioStreamBase {
        unsafe { &mut *ffi::oboe_AudioStreamBuilder_getBase(self.raw.0) }
    }
}

impl Default for AudioStreamBuilder<Output, Unspecified, Unspecified> {
    /**
     * Create new audio stream builder
     */
    fn default() -> Self {
        Self {
            raw: Default::default(),
            _phantom: PhantomData,
        }
    }
}

impl<D, C, T> AudioStreamBuilder<D, C, T> {
    fn convert<D1, C1, T1>(self) -> AudioStreamBuilder<D1, C1, T1> {
        AudioStreamBuilder {
            raw: ManuallyDrop::new(self.destructs()),
            _phantom: PhantomData,
        }
    }

    /**
     * Request a specific number of channels
     *
     * Default is `Unspecified`. If the value is unspecified then
     * the application should query for the actual value after the stream is opened.
     */
    pub fn set_channel_count<X: IsChannelCount>(self) -> AudioStreamBuilder<D, X, T> {
        let mut builder = self.convert();
        builder._raw_base_mut().mChannelCount = X::CHANNEL_COUNT as i32;
        builder
    }

    /**
     * Request mono mode for a stream
     */
    pub fn set_mono(self) -> AudioStreamBuilder<D, Mono, T> {
        self.set_channel_count::<Mono>()
    }

    /**
     * Request stereo mode for a stream
     */
    pub fn set_stereo(self) -> AudioStreamBuilder<D, Stereo, T> {
        self.set_channel_count::<Stereo>()
    }

    /**
     * Request the direction for a stream
     *
     * The default is `Direction::Output`
     */
    pub fn set_direction<X: IsDirection>(self) -> AudioStreamBuilder<X, C, T> {
        let mut builder = self.convert();
        builder._raw_base_mut().mDirection = X::DIRECTION as i32;
        builder
    }

    /**
     * Request input direction for a stream
     */
    pub fn set_input(self) -> AudioStreamBuilder<Input, C, T> {
        self.set_direction::<Input>()
    }

    /**
     * Request output direction for a stream
     *
     * It is optional because th stream builder already configured as output by default.
     */
    pub fn set_output(self) -> AudioStreamBuilder<Output, C, T> {
        self.set_direction::<Output>()
    }

    /**
     * Request a specific sample rate in Hz.
     *
     * Default is kUnspecified. If the value is unspecified then
     * the application should query for the actual value after the stream is opened.
     *
     * Technically, this should be called the _frame rate_ or _frames per second_,
     * because it refers to the number of complete frames transferred per second.
     * But it is traditionally called _sample rate_. Se we use that term.
     */
    pub fn set_sample_rate(mut self, sample_rate: i32) -> Self {
        self._raw_base_mut().mSampleRate = sample_rate;
        self
    }

    /**
     * Request a specific number of frames for the data callback.
     *
     * Default is kUnspecified. If the value is unspecified then
     * the actual number may vary from callback to callback.
     *
     * If an application can handle a varying number of frames then we recommend
     * leaving this unspecified. This allow the underlying API to optimize
     * the callbacks. But if your application is, for example, doing FFTs or other block
     * oriented operations, then call this function to get the sizes you need.
     */
    pub fn set_frames_per_callback(mut self, frames_per_callback: i32) -> Self {
        self._raw_base_mut().mFramesPerCallback = frames_per_callback;
        self
    }

    /**
     * Request a sample data format, for example `f32`.
     *
     * Default is unspecified. If the value is unspecified then
     * the application should query for the actual value after the stream is opened.
     */
    pub fn set_format<X: IsFormat>(self) -> AudioStreamBuilder<D, C, X> {
        let mut builder = self.convert();
        builder._raw_base_mut().mFormat = X::FORMAT as i32;
        builder
    }

    pub fn set_i16(self) -> AudioStreamBuilder<D, C, i16> {
        self.set_format::<i16>()
    }

    pub fn set_f32(self) -> AudioStreamBuilder<D, C, f32> {
        self.set_format::<f32>()
    }

    /**
     * Set the requested buffer capacity in frames.
     * Buffer capacity in frames is the maximum possible buffer size in frames.
     *
     * The final stream capacity may differ. For __AAudio__ it should be at least this big.
     * For __OpenSL ES__, it could be smaller.
     *
     * Default is unspecified.
     */
    pub fn set_buffer_capacity_in_frames(mut self, buffer_capacity_in_frames: i32) -> Self {
        self._raw_base_mut().mBufferCapacityInFrames = buffer_capacity_in_frames;
        self
    }

    /**
     * Get the audio API which will be requested when opening the stream. No guarantees that this is
     * the API which will actually be used. Query the stream itself to find out the API which is
     * being used.
     *
     * If you do not specify the API, then __AAudio__ will be used if isAAudioRecommended()
     * returns true. Otherwise __OpenSL ES__ will be used.
     */
    pub fn get_audio_api(&self) -> AudioApi {
        FromPrimitive::from_i32(unsafe { ffi::oboe_AudioStreamBuilder_getAudioApi(&**self.raw) })
            .unwrap()
    }

    /**
     * If you leave this unspecified then Oboe will choose the best API
     * for the device and SDK version at runtime.
     *
     * This should almost always be left unspecified, except for debugging purposes.
     * Specifying __AAudio__ will force Oboe to use AAudio on 8.0, which is extremely risky.
     * Specifying __OpenSL ES__ should mainly be used to test legacy performance/functionality.
     *
     * If the caller requests AAudio and it is supported then AAudio will be used.
     */
    pub fn set_audio_api(mut self, audio_api: AudioApi) -> Self {
        unsafe { ffi::oboe_AudioStreamBuilder_setAudioApi(&mut **self.raw, audio_api as i32) }
        self
    }

    /**
     * Is the AAudio API supported on this device?
     *
     * AAudio was introduced in the Oreo 8.0 release.
     */
    pub fn is_aaudio_supported() -> bool {
        unsafe { ffi::oboe_AudioStreamBuilder_isAAudioSupported() }
    }

    /**
     * Is the AAudio API recommended this device?
     *
     * AAudio may be supported but not recommended because of version specific issues.
     * AAudio is not recommended for Android 8.0 or earlier versions.
     */
    pub fn is_aaudio_recommended() -> bool {
        unsafe { ffi::oboe_AudioStreamBuilder_isAAudioRecommended() }
    }

    /**
     * Request a mode for sharing the device.
     * The requested sharing mode may not be available.
     * So the application should query for the actual mode after the stream is opened.
     */
    pub fn set_sharing_mode(mut self, sharing_mode: SharingMode) -> Self {
        self._raw_base_mut().mSharingMode = sharing_mode as i32;
        self
    }

    /**
     * Request a shared mode for the device
     */
    pub fn set_shared(self) -> Self {
        self.set_sharing_mode(SharingMode::Shared)
    }

    /**
     * Request an exclusive mode for the device
     */
    pub fn set_exclusive(self) -> Self {
        self.set_sharing_mode(SharingMode::Exclusive)
    }

    /**
     * Request a performance level for the stream.
     * This will determine the latency, the power consumption, and the level of
     * protection from glitches.
     */
    pub fn set_performance_mode(mut self, performance_mode: PerformanceMode) -> Self {
        self._raw_base_mut().mPerformanceMode = performance_mode as i32;
        self
    }

    /**
     * Set the intended use case for the stream.
     *
     * The system will use this information to optimize the behavior of the stream.
     * This could, for example, affect how volume and focus is handled for the stream.
     *
     * The default, if you do not call this function, is Usage::Media.
     *
     * Added in API level 28.
     */
    pub fn set_usage(mut self, usage: Usage) -> Self {
        self._raw_base_mut().mUsage = usage as i32;
        self
    }

    /**
     * Set the type of audio data that the stream will carry.
     *
     * The system will use this information to optimize the behavior of the stream.
     * This could, for example, affect whether a stream is paused when a notification occurs.
     *
     * The default, if you do not call this function, is `ContentType::Music`.
     *
     * Added in API level 28.
     */
    pub fn set_content_type(mut self, content_type: ContentType) -> Self {
        self._raw_base_mut().mContentType = content_type as i32;
        self
    }

    /**
     * Set the input (capture) preset for the stream.
     *
     * The system will use this information to optimize the behavior of the stream.
     * This could, for example, affect which microphones are used and how the
     * recorded data is processed.
     *
     * The default, if you do not call this function, is InputPreset::VoiceRecognition.
     * That is because VoiceRecognition is the preset with the lowest latency
     * on many platforms.
     *
     * Added in API level 28.
     */
    pub fn set_input_preset(mut self, input_preset: InputPreset) -> Self {
        self._raw_base_mut().mInputPreset = input_preset as i32;
        self
    }

    /**
     * Set the requested session ID.
     *
     * The session ID can be used to associate a stream with effects processors.
     * The effects are controlled using the Android AudioEffect Java API.
     *
     * The default, if you do not call this function, is `SessionId::None`.
     *
     * If set to `SessionId::Allocate` then a session ID will be allocated
     * when the stream is opened.
     *
     * The allocated session ID can be obtained by calling AudioStream::getSessionId()
     * and then used with this function when opening another stream.
     * This allows effects to be shared between streams.
     *
     * Session IDs from Oboe can be used the Android Java APIs and vice versa.
     * So a session ID from an Oboe stream can be passed to Java
     * and effects applied using the Java AudioEffect API.
     *
     * Allocated session IDs will always be positive and nonzero.
     *
     * Added in API level 28.
     */
    pub fn set_session_id(mut self, session_id: SessionId) -> Self {
        self._raw_base_mut().mSessionId = session_id as i32;
        self
    }

    /**
     * Request a stream to a specific audio input/output device given an audio device ID.
     *
     * In most cases, the primary device will be the appropriate device to use, and the
     * device ID can be left unspecified.
     *
     * On Android, for example, the ID could be obtained from the Java AudioManager.
     * AudioManager.getDevices() returns an array of AudioDeviceInfo[], which contains
     * a getId() method (as well as other type information), that should be passed
     * to this method.
     *
     * When `java-interface` feature is used you can call [`AudioDeviceInfo::request`](crate::AudioDeviceInfo::request) for listing devices info.
     *
     * Note that when using OpenSL ES, this will be ignored and the created
     * stream will have device ID unspecified.
     */
    pub fn set_device_id(mut self, device_id: i32) -> Self {
        self._raw_base_mut().mDeviceId = device_id;
        self
    }

    /**
     * If true then Oboe might convert channel counts to achieve optimal results.
     * On some versions of Android for example, stereo streams could not use a FAST track.
     * So a mono stream might be used instead and duplicated to two channels.
     * On some devices, mono streams might be broken, so a stereo stream might be opened
     * and converted to mono.
     *
     * Default is true.
     */
    pub fn set_channel_conversion_allowed(mut self, allowed: bool) -> Self {
        self._raw_base_mut().mChannelConversionAllowed = allowed;
        self
    }

    /**
     * If true then  Oboe might convert data formats to achieve optimal results.
     * On some versions of Android, for example, a float stream could not get a
     * low latency data path. So an I16 stream might be opened and converted to float.
     *
     * Default is true.
     */
    pub fn set_format_conversion_allowed(mut self, allowed: bool) -> Self {
        self._raw_base_mut().mFormatConversionAllowed = allowed;
        self
    }

    /**
     * Specify the quality of the sample rate converter in Oboe.
     *
     * If set to None then Oboe will not do sample rate conversion. But the underlying APIs might
     * still do sample rate conversion if you specify a sample rate.
     * That can prevent you from getting a low latency stream.
     *
     * If you do the conversion in Oboe then you might still get a low latency stream.
     *
     * Default is `SampleRateConversionQuality::None`
     */
    pub fn set_sample_rate_conversion_quality(
        mut self,
        quality: SampleRateConversionQuality,
    ) -> Self {
        self._raw_base_mut().mSampleRateConversionQuality = quality as i32;
        self
    }

    /**
     * Returns true if AAudio will be used based on the current settings.
     */
    pub fn will_use_aaudio(&self) -> bool {
        let audio_api = self.get_audio_api();
        (audio_api == AudioApi::AAudio && Self::is_aaudio_supported())
            || (audio_api == AudioApi::Unspecified && Self::is_aaudio_recommended())
    }

    /// Descontructs self into its handle, without calling drop.
    fn destructs(mut self) -> AudioStreamBuilderHandle {
        // Safety: the std::mem::forget prevents `raw` from being dropped by Self::drop.
        let raw = unsafe { ManuallyDrop::take(&mut self.raw) };

        std::mem::forget(self);

        raw
    }
}

impl<D: IsDirection, C: IsChannelCount, T: IsFormat> AudioStreamBuilder<D, C, T> {
    /**
     * Create and open a synchronous (blocking) stream based on the current settings.
     */
    pub fn open_stream(self) -> Result<AudioStreamSync<D, (T, C)>> {
        let mut stream = MaybeUninit::<*mut ffi::oboe_AudioStream>::uninit();
        let mut shared_ptr = MaybeUninit::<*mut c_void>::uninit();
        let mut raw = self.destructs();

        let stream = wrap_status(unsafe {
            ffi::oboe_AudioStreamBuilder_openStreamShared(
                &mut *raw,
                stream.as_mut_ptr(),
                shared_ptr.as_mut_ptr(),
            )
        })
        .map(|_| unsafe {
            AudioStreamSync::wrap_raw(stream.assume_init(), shared_ptr.assume_init())
        });

        drop(raw);

        stream
    }
}

impl<C: IsChannelCount, T: IsFormat> AudioStreamBuilder<Input, C, T> {
    /**
     * Specifies an object to handle data or error related callbacks from the underlying API.
     *
     * __Important: See AudioStreamCallback for restrictions on what may be called
     * from the callback methods.__
     *
     * When an error callback occurs, the associated stream will be stopped and closed in a separate thread.
     *
     * A note on why the streamCallback parameter is a raw pointer rather than a smart pointer:
     *
     * The caller should retain ownership of the object streamCallback points to. At first glance weak_ptr may seem like
     * a good candidate for streamCallback as this implies temporary ownership. However, a weak_ptr can only be created
     * from a shared_ptr. A shared_ptr incurs some performance overhead. The callback object is likely to be accessed
     * every few milliseconds when the stream requires new data so this overhead is something we want to avoid.
     *
     * This leaves a raw pointer as the logical type choice. The only caveat being that the caller must not destroy
     * the callback before the stream has been closed.
     */
    pub fn set_callback<F>(self, stream_callback: F) -> AudioStreamBuilderAsync<Input, F>
    where
        F: AudioInputCallback<FrameType = (T, C)>,
        (T, C): IsFrameType,
    {
        let mut raw = self.destructs();
        set_input_callback(&mut raw, stream_callback);
        AudioStreamBuilderAsync {
            raw: ManuallyDrop::new(raw),
            _phantom: PhantomData,
        }
    }
}

impl<C: IsChannelCount, T: IsFormat> AudioStreamBuilder<Output, C, T> {
    /**
     * Specifies an object to handle data or error related callbacks from the underlying API.
     *
     * __Important: See AudioStreamCallback for restrictions on what may be called
     * from the callback methods.__
     *
     * When an error callback occurs, the associated stream will be stopped and closed in a separate thread.
     *
     * A note on why the streamCallback parameter is a raw pointer rather than a smart pointer:
     *
     * The caller should retain ownership of the object streamCallback points to. At first glance weak_ptr may seem like
     * a good candidate for streamCallback as this implies temporary ownership. However, a weak_ptr can only be created
     * from a shared_ptr. A shared_ptr incurs some performance overhead. The callback object is likely to be accessed
     * every few milliseconds when the stream requires new data so this overhead is something we want to avoid.
     *
     * This leaves a raw pointer as the logical type choice. The only caveat being that the caller must not destroy
     * the callback before the stream has been closed.
     */
    pub fn set_callback<F>(self, stream_callback: F) -> AudioStreamBuilderAsync<Output, F>
    where
        F: AudioOutputCallback<FrameType = (T, C)>,
        (T, C): IsFrameType,
    {
        let mut raw = self.destructs();
        set_output_callback(&mut raw, stream_callback);
        AudioStreamBuilderAsync {
            raw: ManuallyDrop::new(raw),
            _phantom: PhantomData,
        }
    }
}

/**
 * Factory for an audio stream.
 */
pub struct AudioStreamBuilderAsync<D, F> {
    raw: ManuallyDrop<AudioStreamBuilderHandle>,
    _phantom: PhantomData<(D, F)>,
}

impl<D, F> Drop for AudioStreamBuilderAsync<D, F> {
    fn drop(&mut self) {
        // SAFETY: self.raw is only droped here, or taken in Self::destructs, which don't drop self.
        unsafe {
            ManuallyDrop::drop(&mut self.raw);
        }
    }
}

impl<D, F> fmt::Debug for AudioStreamBuilderAsync<D, F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        audio_stream_base_fmt(self, f)
    }
}

impl<D, F> RawAudioStreamBase for AudioStreamBuilderAsync<D, F> {
    fn _raw_base(&self) -> &ffi::oboe_AudioStreamBase {
        unsafe { &*ffi::oboe_AudioStreamBuilder_getBase(self.raw.0) }
    }

    fn _raw_base_mut(&mut self) -> &mut ffi::oboe_AudioStreamBase {
        unsafe { &mut *ffi::oboe_AudioStreamBuilder_getBase(self.raw.0) }
    }
}

impl<D, F> AudioStreamBuilderAsync<D, F> {
    /// Descontructs self into its handle without calling drop.
    fn destructs(mut self) -> AudioStreamBuilderHandle {
        // Safety: the std::mem::forget prevents `raw` from being dropped by Self::drop.
        let raw = unsafe { ManuallyDrop::take(&mut self.raw) };

        std::mem::forget(self);

        raw
    }
}

impl<F: AudioInputCallback + Send> AudioStreamBuilderAsync<Input, F> {
    /**
     * Create and open an asynchronous (callback-driven) input stream based on the current settings.
     */
    pub fn open_stream(self) -> Result<AudioStreamAsync<Input, F>> {
        let mut stream = MaybeUninit::<*mut ffi::oboe_AudioStream>::uninit();
        let mut shared_ptr = MaybeUninit::<*mut c_void>::uninit();
        let mut raw = self.destructs();

        let stream = wrap_status(unsafe {
            ffi::oboe_AudioStreamBuilder_openStreamShared(
                &mut *raw,
                stream.as_mut_ptr(),
                shared_ptr.as_mut_ptr(),
            )
        })
        .map(|_| unsafe {
            AudioStreamAsync::wrap_raw(stream.assume_init(), shared_ptr.assume_init())
        });

        drop(raw);

        stream
    }
}

impl<F: AudioOutputCallback + Send> AudioStreamBuilderAsync<Output, F> {
    /**
     * Create and open an asynchronous (callback-driven) output stream based on the current settings.
     */
    pub fn open_stream(self) -> Result<AudioStreamAsync<Output, F>> {
        let mut stream = MaybeUninit::<*mut ffi::oboe_AudioStream>::uninit();
        let mut shared_ptr = MaybeUninit::<*mut c_void>::uninit();
        let mut raw = self.destructs();

        let stream = wrap_status(unsafe {
            ffi::oboe_AudioStreamBuilder_openStreamShared(
                &mut *raw,
                stream.as_mut_ptr(),
                shared_ptr.as_mut_ptr(),
            )
        })
        .map(|_| unsafe {
            AudioStreamAsync::wrap_raw(stream.assume_init(), shared_ptr.assume_init())
        });

        drop(raw);

        stream
    }
}
