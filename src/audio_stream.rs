use num_traits::FromPrimitive;
use oboe_sys as ffi;
use std::{
    ffi::c_void,
    fmt::{self, Display},
    marker::PhantomData,
    mem::{transmute, MaybeUninit},
    ops::{Deref, DerefMut},
    ptr::null_mut,
};

use super::{
    audio_stream_base_fmt, wrap_result, wrap_status, AudioApi, AudioStreamBase, FrameTimestamp,
    Input, IsFrameType, Output, RawAudioInputStream, RawAudioOutputStream, RawAudioStream,
    RawAudioStreamBase, Result, Status, StreamState, NANOS_PER_MILLISECOND,
};

/**
 * The default number of nanoseconds to wait for when performing state change operations on the
 * stream, such as `start` and `stop`.
 *
 * See [AudioStream::start_with_timeout]
 */
pub const DEFAULT_TIMEOUT_NANOS: i64 = 2000 * NANOS_PER_MILLISECOND;

/**
 * Safe base trait for Oboe audio stream.
 */
pub trait AudioStreamSafe: AudioStreamBase {
    /**
     * Query the current state, eg. `StreamState::Pausing`
     */
    fn get_state(&self) -> StreamState;

    /**
     * This can be used to adjust the latency of the buffer by changing
     * the threshold where blocking will occur.
     * By combining this with [`AudioStreamSafe::get_xrun_count`], the latency can be tuned
     * at run-time for each device.
     *
     * This cannot be set higher than [`AudioStreamBase::get_buffer_capacity_in_frames`].
     */
    fn set_buffer_size_in_frames(&mut self, _requested_frames: i32) -> Result<i32>;

    /**
     * An XRun is an Underrun or an Overrun.
     * During playing, an underrun will occur if the stream is not written in time
     * and the system runs out of valid data.
     * During recording, an overrun will occur if the stream is not read in time
     * and there is no place to put the incoming data so it is discarded.
     *
     * An underrun or overrun can cause an audible "pop" or "glitch".
     */
    fn get_xrun_count(&self) -> Result<i32>;

    /**
     * Returns true if XRun counts are supported on the stream
     */
    fn is_xrun_count_supported(&self) -> bool;

    /**
     * Query the number of frames that are read or written by the endpoint at one time.
     */
    fn get_frames_per_burst(&mut self) -> i32;

    /**
     * Get the number of bytes in each audio frame. This is calculated using the channel count
     * and the sample format. For example, a 2 channel floating point stream will have
     * 2 * 4 = 8 bytes per frame.
     */
    fn get_bytes_per_frame(&mut self) -> i32 {
        self.get_channel_count() as i32 * self.get_bytes_per_sample()
    }

    /**
     * Get the number of bytes per sample. This is calculated using the sample format. For example,
     * a stream using 16-bit integer samples will have 2 bytes per sample.
     *
     * @return the number of bytes per sample.
     */
    fn get_bytes_per_sample(&mut self) -> i32;

    /**
     * Calculate the latency of a stream based on getTimestamp().
     *
     * Output latency is the time it takes for a given frame to travel from the
     * app to some type of digital-to-analog converter. If the DAC is external, for example
     * in a USB interface or a TV connected by HDMI, then there may be additional latency
     * that the Android device is unaware of.
     *
     * Input latency is the time it takes to a given frame to travel from an analog-to-digital
     * converter (ADC) to the app.
     *
     * Note that the latency of an OUTPUT stream will increase abruptly when you write data to it
     * and then decrease slowly over time as the data is consumed.
     *
     * The latency of an INPUT stream will decrease abruptly when you read data from it
     * and then increase slowly over time as more data arrives.
     *
     * The latency of an OUTPUT stream is generally higher than the INPUT latency
     * because an app generally tries to keep the OUTPUT buffer full and the INPUT buffer empty.
     */
    fn calculate_latency_millis(&mut self) -> Result<f64>;

    /**
     * Get the estimated time that the frame at `frame_position` entered or left the audio processing
     * pipeline.
     *
     * This can be used to coordinate events and interactions with the external environment, and to
     * estimate the latency of an audio stream. An example of usage can be found in the hello-oboe
     * sample (search for "calculate_current_output_latency_millis").
     *
     * The time is based on the implementation's best effort, using whatever knowledge is available
     * to the system, but cannot account for any delay unknown to the implementation.
     *
     * @param clockId the type of clock to use e.g. CLOCK_MONOTONIC
     * @return a FrameTimestamp containing the position and time at which a particular audio frame
     * entered or left the audio processing pipeline, or an error if the operation failed.
     */
    fn get_timestamp(&mut self, clock_id: i32) -> Result<FrameTimestamp>;

    /**
     * Get the underlying audio API which the stream uses.
     */
    fn get_audio_api(&self) -> AudioApi;

    /**
     * Returns true if the underlying audio API is AAudio.
     */
    fn uses_aaudio(&self) -> bool {
        self.get_audio_api() == AudioApi::AAudio
    }

    /**
     * Returns the number of frames of data currently in the buffer
     */
    fn get_available_frames(&mut self) -> Result<i32>;
}

/**
 * Base trait for Oboe audio stream.
 */
pub trait AudioStream: AudioStreamSafe {
    /**
     * Open a stream based on the current settings.
     *
     * Note that we do not recommend re-opening a stream that has been closed.
     * TODO Should we prevent re-opening?
     */
    fn open(&mut self) -> Status {
        Ok(())
    }

    /**
     * Close the stream and deallocate any resources from the open() call.
     */
    fn close(&mut self) -> Status;

    /**
     * Start the stream. This will block until the stream has been started, an error occurs
     * or `timeout_nanoseconds` has been reached.
     */
    fn start(&mut self) -> Status {
        self.start_with_timeout(DEFAULT_TIMEOUT_NANOS)
    }

    /**
     * Start the stream. This will block until the stream has been started, an error occurs
     * or `timeout_nanoseconds` has been reached.
     */
    fn start_with_timeout(&mut self, timeout_nanoseconds: i64) -> Status;

    /**
     * Stop the stream. This will block until the stream has been stopped, an error occurs
     * or `timeoutNanoseconds` has been reached.
     */
    fn stop(&mut self) -> Status {
        self.stop_with_timeout(DEFAULT_TIMEOUT_NANOS)
    }

    /**
     * Stop the stream. This will block until the stream has been stopped, an error occurs
     * or `timeoutNanoseconds` has been reached.
     */
    fn stop_with_timeout(&mut self, timeout_nanoseconds: i64) -> Status;

    /**
     * Start the stream asynchronously. Returns immediately (does not block). Equivalent to calling
     * `start(0)`.
     */
    fn request_start(&mut self) -> Status;

    /**
     * Stop the stream asynchronously. Returns immediately (does not block). Equivalent to calling
     * `stop(0)`.
     */
    fn request_stop(&mut self) -> Status;

    /**
     * Wait until the stream's current state no longer matches the input state.
     * The input state is passed to avoid race conditions caused by the state
     * changing between calls.
     *
     * Note that generally applications do not need to call this. It is considered
     * an advanced technique and is mostly used for testing.
     *
     * ```ignore
     * const TIMEOUT_NANOS: i64 = 500 * NANOS_PER_MILLISECOND; // arbitrary 1/2 second
     * let mut current_state = stream.get_state();
     * loop {
     *     if let Ok(next_state) = stream.wait_for_state_change(current_state, TIMEOUT_NANOS) {
     *         if next_state != StreamState::Paused {
     *             current_state = next_state;
     *             continue;
     *         }
     *     }
     *     break;
     * }
     * ```
     *
     * If the state does not change within the timeout period then it will
     * return [`Error::Timeout`](crate::Error::Timeout). This is true even if timeout_nanoseconds is zero.
     */
    fn wait_for_state_change(
        &mut self,
        input_state: StreamState,
        timeout_nanoseconds: i64,
    ) -> Result<StreamState>;

    /**
     * Wait until the stream has a minimum amount of data available in its buffer.
     * This can be used with an EXCLUSIVE MMAP input stream to avoid reading data too close to
     * the DSP write position, which may cause glitches.
     */
    fn wait_for_available_frames(
        &mut self,
        num_frames: i32,
        timeout_nanoseconds: i64,
    ) -> Result<i32>;
}

/**
 * The stream which is used for async audio input
 */
pub trait AudioInputStreamSafe: AudioStreamSafe {
    /**
     * The number of audio frames read from the stream.
     * This monotonic counter will never get reset.
     */
    fn get_frames_read(&mut self) -> i64;
}

/**
 * The stream which is used for audio input
 */
pub trait AudioInputStream: AudioStream + AudioInputStreamSafe {}

/**
 * The stream which can be used for audio input in synchronous mode
 */
pub trait AudioInputStreamSync: AudioInputStream {
    type FrameType: IsFrameType;

    /**
     * Read data into the supplied buffer from the stream. This method will block until the read
     * is complete or it runs out of time.
     *
     * If `timeout_nanoseconds` is zero then this call will not wait.
     */
    fn read(
        &mut self,
        _buffer: &mut [<Self::FrameType as IsFrameType>::Type],
        _timeout_nanoseconds: i64,
    ) -> Result<i32>;
}

/**
 * The stream which is used for async audio output
 */
pub trait AudioOutputStreamSafe: AudioStreamSafe {
    /**
     * The number of audio frames written into the stream.
     * This monotonic counter will never get reset.
     */
    fn get_frames_written(&mut self) -> i64;
}

/**
 * The stream which has pause/flush capabilities
 */
pub trait AudioOutputStream: AudioStream + AudioOutputStreamSafe {
    /**
     * Pause the stream. This will block until the stream has been paused, an error occurs
     * or `timeoutNanoseconds` has been reached.
     */
    fn pause(&mut self) -> Status {
        self.pause_with_timeout(DEFAULT_TIMEOUT_NANOS)
    }

    /**
     * Pause the stream. This will block until the stream has been paused, an error occurs
     * or `timeoutNanoseconds` has been reached.
     */
    fn pause_with_timeout(&mut self, timeout_nanoseconds: i64) -> Status;

    /**
     * Flush the stream. This will block until the stream has been flushed, an error occurs
     * or `timeoutNanoseconds` has been reached.
     */
    fn flush(&mut self) -> Status {
        self.flush_with_timeout(DEFAULT_TIMEOUT_NANOS)
    }

    /**
     * Flush the stream. This will block until the stream has been flushed, an error occurs
     * or `timeoutNanoseconds` has been reached.
     */
    fn flush_with_timeout(&mut self, timeout_nanoseconds: i64) -> Status;

    /**
     * Pause the stream asynchronously. Returns immediately (does not block). Equivalent to calling
     * `pause(0)`.
     */
    fn request_pause(&mut self) -> Status;

    /**
     * Flush the stream asynchronously. Returns immediately (does not block). Equivalent to calling
     * `flush(0)`.
     */
    fn request_flush(&mut self) -> Status;
}

/**
 * The stream which can be used for audio output in synchronous mode
 */
pub trait AudioOutputStreamSync: AudioOutputStream {
    type FrameType: IsFrameType;

    /**
     * Write data from the supplied buffer into the stream. This method will block until the write
     * is complete or it runs out of time.
     *
     * If `timeout_nanoseconds` is zero then this call will not wait.
     */
    fn write(
        &mut self,
        _buffer: &[<Self::FrameType as IsFrameType>::Type],
        _timeout_nanoseconds: i64,
    ) -> Result<i32>;
}

impl<T: RawAudioStream + RawAudioStreamBase> AudioStreamSafe for T {
    fn set_buffer_size_in_frames(&mut self, requested_frames: i32) -> Result<i32> {
        wrap_result(unsafe {
            ffi::oboe_AudioStream_setBufferSizeInFrames(self._raw_stream_mut(), requested_frames)
        })
    }

    fn get_state(&self) -> StreamState {
        FromPrimitive::from_i32(unsafe {
            ffi::oboe_AudioStream_getState(self._raw_stream() as *const _ as *mut _)
        })
        .unwrap()
    }

    fn get_xrun_count(&self) -> Result<i32> {
        wrap_result(unsafe {
            ffi::oboe_AudioStream_getXRunCount(self._raw_stream() as *const _ as *mut _)
        })
    }

    fn is_xrun_count_supported(&self) -> bool {
        unsafe { ffi::oboe_AudioStream_isXRunCountSupported(self._raw_stream()) }
    }

    fn get_frames_per_burst(&mut self) -> i32 {
        unsafe { ffi::oboe_AudioStream_getFramesPerBurst(self._raw_stream_mut()) }
    }

    fn get_bytes_per_sample(&mut self) -> i32 {
        unsafe { ffi::oboe_AudioStream_getBytesPerSample(self._raw_stream_mut()) }
    }

    fn calculate_latency_millis(&mut self) -> Result<f64> {
        wrap_result(unsafe { ffi::oboe_AudioStream_calculateLatencyMillis(self._raw_stream_mut()) })
    }

    fn get_timestamp(&mut self, clock_id: i32 /* clockid_t */) -> Result<FrameTimestamp> {
        wrap_result(unsafe {
            transmute(ffi::oboe_AudioStream_getTimestamp(
                self._raw_stream_mut() as *mut _ as *mut c_void,
                clock_id,
            ))
        })
    }

    fn get_audio_api(&self) -> AudioApi {
        FromPrimitive::from_i32(unsafe { ffi::oboe_AudioStream_getAudioApi(self._raw_stream()) })
            .unwrap()
    }

    fn get_available_frames(&mut self) -> Result<i32> {
        wrap_result(unsafe { ffi::oboe_AudioStream_getAvailableFrames(self._raw_stream_mut()) })
    }
}

impl<T: RawAudioStream + RawAudioStreamBase> AudioStream for T {
    fn open(&mut self) -> Status {
        wrap_status(unsafe { ffi::oboe_AudioStream_open(self._raw_stream_mut()) })
    }

    fn close(&mut self) -> Status {
        wrap_status(unsafe { ffi::oboe_AudioStream_close1(self._raw_stream_mut()) })
    }

    fn start_with_timeout(&mut self, timeout_nanoseconds: i64) -> Status {
        wrap_status(unsafe {
            ffi::oboe_AudioStream_start(
                self._raw_stream_mut() as *mut _ as *mut c_void,
                timeout_nanoseconds,
            )
        })
    }

    fn stop_with_timeout(&mut self, timeout_nanoseconds: i64) -> Status {
        wrap_status(unsafe {
            ffi::oboe_AudioStream_stop(
                self._raw_stream_mut() as *mut _ as *mut c_void,
                timeout_nanoseconds,
            )
        })
    }

    fn request_start(&mut self) -> Status {
        wrap_status(unsafe { ffi::oboe_AudioStream_requestStart(self._raw_stream_mut()) })
    }

    fn request_stop(&mut self) -> Status {
        wrap_status(unsafe { ffi::oboe_AudioStream_requestStop(self._raw_stream_mut()) })
    }

    fn wait_for_state_change(
        &mut self,
        input_state: StreamState,
        timeout_nanoseconds: i64,
    ) -> Result<StreamState> {
        let mut next_state = MaybeUninit::<StreamState>::uninit();
        wrap_status(unsafe {
            ffi::oboe_AudioStream_waitForStateChange(
                self._raw_stream_mut(),
                input_state as i32,
                next_state.as_mut_ptr() as *mut i32,
                timeout_nanoseconds,
            )
        })
        .map(|_| unsafe { next_state.assume_init() })
    }

    fn wait_for_available_frames(
        &mut self,
        num_frames: i32,
        timeout_nanoseconds: i64,
    ) -> Result<i32> {
        wrap_result(unsafe {
            ffi::oboe_AudioStream_waitForAvailableFrames(
                self._raw_stream_mut(),
                num_frames,
                timeout_nanoseconds,
            )
        })
    }
}

impl<T: RawAudioInputStream + RawAudioStream + RawAudioStreamBase> AudioInputStreamSafe for T {
    fn get_frames_read(&mut self) -> i64 {
        unsafe {
            ffi::oboe_AudioStream_getFramesRead(self._raw_stream_mut() as *mut _ as *mut c_void)
        }
    }
}

impl<T: RawAudioInputStream + RawAudioStream + RawAudioStreamBase> AudioInputStream for T {}

impl<T: RawAudioOutputStream + RawAudioStream + RawAudioStreamBase> AudioOutputStreamSafe for T {
    fn get_frames_written(&mut self) -> i64 {
        unsafe {
            ffi::oboe_AudioStream_getFramesWritten(self._raw_stream_mut() as *mut _ as *mut c_void)
        }
    }
}

impl<T: RawAudioOutputStream + RawAudioStream + RawAudioStreamBase> AudioOutputStream for T {
    fn pause_with_timeout(&mut self, timeout_nanoseconds: i64) -> Status {
        wrap_status(unsafe {
            ffi::oboe_AudioStream_pause(
                self._raw_stream_mut() as *mut _ as *mut c_void,
                timeout_nanoseconds,
            )
        })
    }

    fn flush_with_timeout(&mut self, timeout_nanoseconds: i64) -> Status {
        wrap_status(unsafe {
            ffi::oboe_AudioStream_flush(
                self._raw_stream_mut() as *mut _ as *mut c_void,
                timeout_nanoseconds,
            )
        })
    }

    fn request_pause(&mut self) -> Status {
        wrap_status(unsafe { ffi::oboe_AudioStream_requestPause(self._raw_stream_mut()) })
    }

    fn request_flush(&mut self) -> Status {
        wrap_status(unsafe { ffi::oboe_AudioStream_requestFlush(self._raw_stream_mut()) })
    }
}

pub(crate) fn audio_stream_fmt<T: AudioStreamSafe>(
    stream: &T,
    f: &mut fmt::Formatter<'_>,
) -> fmt::Result {
    audio_stream_base_fmt(stream, f)?;
    "Audio API: ".fmt(f)?;
    fmt::Debug::fmt(&stream.get_audio_api(), f)?;
    "\nCurrent state: ".fmt(f)?;
    fmt::Debug::fmt(&stream.get_state(), f)?;
    "\nXrun count: ".fmt(f)?;
    match stream.get_xrun_count() {
        Ok(count) => count.fmt(f)?,
        Err(error) => fmt::Debug::fmt(&error, f)?,
    }
    '\n'.fmt(f)
}

struct AudioStreamHandle(*mut ffi::oboe_AudioStream, *mut c_void);

impl AudioStreamHandle {
    fn new(raw: *mut ffi::oboe_AudioStream, shared_ptr: *mut c_void) -> Self {
        Self(raw, shared_ptr)
    }

    /// SAFETY: `self.0` and `self.1` must be valid pointers.
    pub(crate) unsafe fn delete(&mut self) {
        assert!(!self.0.is_null());
        assert!(!self.1.is_null());

        // The error callback could be holding a shared_ptr, so don't delete AudioStream
        // directly, but only its shared_ptr.
        ffi::oboe_AudioStream_deleteShared(self.1);

        self.0 = null_mut();
        self.1 = null_mut();
    }
}

impl Default for AudioStreamHandle {
    fn default() -> Self {
        Self(null_mut(), null_mut())
    }
}

impl Deref for AudioStreamHandle {
    type Target = ffi::oboe_AudioStream;

    fn deref(&self) -> &Self::Target {
        unsafe { &(*self.0) }
    }
}

impl DerefMut for AudioStreamHandle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut (*self.0) }
    }
}

/**
 * Reference to the audio stream for passing to callbacks
 */
#[repr(transparent)]
pub struct AudioStreamRef<'s, D> {
    raw: &'s mut ffi::oboe_AudioStream,
    _phantom: PhantomData<D>,
}

impl<'s, D> fmt::Debug for AudioStreamRef<'s, D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        audio_stream_fmt(self, f)
    }
}

impl<'s, D> AudioStreamRef<'s, D> {
    pub(crate) fn wrap_raw<'a: 's>(raw: &'a mut ffi::oboe_AudioStream) -> Self {
        Self {
            raw,
            _phantom: PhantomData,
        }
    }
}

impl<'s, D> RawAudioStreamBase for AudioStreamRef<'s, D> {
    fn _raw_base(&self) -> &ffi::oboe_AudioStreamBase {
        unsafe { &*ffi::oboe_AudioStream_getBase(self.raw as *const _ as *mut _) }
    }

    fn _raw_base_mut(&mut self) -> &mut ffi::oboe_AudioStreamBase {
        unsafe { &mut *ffi::oboe_AudioStream_getBase(self.raw) }
    }
}

impl<'s, D> RawAudioStream for AudioStreamRef<'s, D> {
    fn _raw_stream(&self) -> &ffi::oboe_AudioStream {
        self.raw
    }

    fn _raw_stream_mut(&mut self) -> &mut ffi::oboe_AudioStream {
        self.raw
    }
}

impl<'s> RawAudioInputStream for AudioStreamRef<'s, Input> {}

impl<'s> RawAudioOutputStream for AudioStreamRef<'s, Output> {}

/**
 * The audio stream for asynchronous (callback-driven) mode
 */
pub struct AudioStreamAsync<D, F> {
    raw: AudioStreamHandle,
    _phantom: PhantomData<(D, F)>,
}

impl<D, F> fmt::Debug for AudioStreamAsync<D, F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        audio_stream_fmt(self, f)
    }
}

impl<D, F> AudioStreamAsync<D, F> {
    // SAFETY: `raw` and `shared_ptr` must be valid.
    pub(crate) unsafe fn wrap_raw(
        raw: *mut ffi::oboe_AudioStream,
        shared_ptr: *mut c_void,
    ) -> Self {
        Self {
            raw: AudioStreamHandle(raw, shared_ptr),
            _phantom: PhantomData,
        }
    }
}

impl<D, F> Drop for AudioStreamAsync<D, F> {
    fn drop(&mut self) {
        // SAFETY: As long as the conditions on Self::wrap_raw are guaranteed on the creation of
        // self, this is safe.
        unsafe {
            let _ = self.close();
            self.raw.delete();
        }
    }
}

impl<D, T> RawAudioStreamBase for AudioStreamAsync<D, T> {
    fn _raw_base(&self) -> &ffi::oboe_AudioStreamBase {
        unsafe { &*ffi::oboe_AudioStream_getBase(self.raw.0) }
    }

    fn _raw_base_mut(&mut self) -> &mut ffi::oboe_AudioStreamBase {
        unsafe { &mut *ffi::oboe_AudioStream_getBase(self.raw.0) }
    }
}

impl<D, F> RawAudioStream for AudioStreamAsync<D, F> {
    fn _raw_stream(&self) -> &ffi::oboe_AudioStream {
        &*self.raw
    }

    fn _raw_stream_mut(&mut self) -> &mut ffi::oboe_AudioStream {
        &mut *self.raw
    }
}

impl<F> RawAudioInputStream for AudioStreamAsync<Input, F> {}

impl<F> RawAudioOutputStream for AudioStreamAsync<Output, F> {}

/**
 * The audio stream for synchronous (blocking) mode
 */
pub struct AudioStreamSync<D, F> {
    raw: AudioStreamHandle,
    _phantom: PhantomData<(D, F)>,
}

impl<D, F> fmt::Debug for AudioStreamSync<D, F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        audio_stream_fmt(self, f)
    }
}

impl<D, F> AudioStreamSync<D, F> {
    // SAFETY: `raw`, `shared_ptr` must be valid, because they will be deleted on drop.
    pub(crate) unsafe fn wrap_raw(
        raw: *mut ffi::oboe_AudioStream,
        shared_ptr: *mut c_void,
    ) -> Self {
        Self {
            raw: AudioStreamHandle::new(raw, shared_ptr),
            _phantom: PhantomData,
        }
    }
}

impl<D, F> Drop for AudioStreamSync<D, F> {
    fn drop(&mut self) {
        // SAFETY: As long as the conditions on Self::wrap_raw are guaranteed on the creation of
        // self, this is safe.
        unsafe {
            let _ = self.close();
            self.raw.delete();
        }
    }
}

impl<D, T> RawAudioStreamBase for AudioStreamSync<D, T> {
    fn _raw_base(&self) -> &ffi::oboe_AudioStreamBase {
        unsafe { &*ffi::oboe_AudioStream_getBase(self.raw.0) }
    }

    fn _raw_base_mut(&mut self) -> &mut ffi::oboe_AudioStreamBase {
        unsafe { &mut *ffi::oboe_AudioStream_getBase(self.raw.0) }
    }
}

impl<D, F> RawAudioStream for AudioStreamSync<D, F> {
    fn _raw_stream(&self) -> &ffi::oboe_AudioStream {
        &*self.raw
    }

    fn _raw_stream_mut(&mut self) -> &mut ffi::oboe_AudioStream {
        &mut *self.raw
    }
}

impl<F> RawAudioInputStream for AudioStreamSync<Input, F> {}

impl<F> RawAudioOutputStream for AudioStreamSync<Output, F> {}

impl<F: IsFrameType> AudioInputStreamSync for AudioStreamSync<Input, F> {
    type FrameType = F;

    fn read(
        &mut self,
        buffer: &mut [<Self::FrameType as IsFrameType>::Type],
        timeout_nanoseconds: i64,
    ) -> Result<i32> {
        wrap_result(unsafe {
            ffi::oboe_AudioStream_read(
                &mut *self.raw,
                buffer.as_mut_ptr() as *mut c_void,
                buffer.len() as i32,
                timeout_nanoseconds,
            )
        })
    }
}

impl<F: IsFrameType> AudioOutputStreamSync for AudioStreamSync<Output, F> {
    type FrameType = F;

    fn write(
        &mut self,
        buffer: &[<Self::FrameType as IsFrameType>::Type],
        timeout_nanoseconds: i64,
    ) -> Result<i32> {
        wrap_result(unsafe {
            ffi::oboe_AudioStream_write(
                &mut *self.raw,
                buffer.as_ptr() as *const c_void,
                buffer.len() as i32,
                timeout_nanoseconds,
            )
        })
    }
}
