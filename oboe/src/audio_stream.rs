use oboe_sys as ffi;
use std::{
    mem::MaybeUninit,
    ffi::c_void
};
use num_traits::FromPrimitive;

use super::{
    AudioApi,
    StreamState,


    Result,
    Status,
    Error,

    NANOS_PER_MILLISECOND,

    IsAudioStreamBase,
    RawAudioStreamBase,
    RawAudioStream,
    FrameTimestamp,

    wrap_status,
    wrap_result,
};

pub const DEFAULT_TIMEOUT_NANOS: i64 = 2000 * NANOS_PER_MILLISECOND;

pub trait IsAudioStream: IsAudioStreamBase {
    /**
     * Open a stream based on the current settings.
     *
     * Note that we do not recommend re-opening a stream that has been closed.
     * TODO Should we prevent re-opening?
     *
     * @return
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

    /* Asynchronous requests.
     * Use wait_for_state_change() if you need to wait for completion.
     */

    /**
     * Start the stream asynchronously. Returns immediately (does not block). Equivalent to calling
     * `start(0)`.
     */
    fn request_start(&mut self) -> Status;

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

    /**
     * Stop the stream asynchronously. Returns immediately (does not block). Equivalent to calling
     * `stop(0)`.
     */
    fn request_stop(&mut self) -> Status;

    /**
     * Query the current state, eg. StreamState::Pausing
     *
     * @return state or a negative error.
     */
    fn get_state(&self) -> StreamState;

    /**
     * Wait until the stream's current state no longer matches the input state.
     * The input state is passed to avoid race conditions caused by the state
     * changing between calls.
     *
     * Note that generally applications do not need to call this. It is considered
     * an advanced technique and is mostly used for testing.
     *
     * ```
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
     * return ErrorTimeout. This is true even if timeoutNanoseconds is zero.
     *
     * @param inputState The state we want to change away from.
     * @param nextState Pointer to a variable that will be set to the new state.
     * @param timeoutNanoseconds The maximum time to wait in nanoseconds.
     * @return Result::OK or a Result::Error.
     */
    fn wait_for_state_change(&mut self,
                             input_state: StreamState,
                             timeout_nanoseconds: i64) -> Result<StreamState>;

    /**
     * This can be used to adjust the latency of the buffer by changing
     * the threshold where blocking will occur.
     * By combining this with getXRunCount(), the latency can be tuned
     * at run-time for each device.
     *
     * This cannot be set higher than getBufferCapacity().
     *
     * @param requestedFrames requested number of frames that can be filled without blocking
     * @return the resulting buffer size in frames (obtained using value()) or an error (obtained
     * using error())
     */
    fn set_buffer_size_in_frames(&mut self, _requested_frames: i32) -> Result<i32> {
        Err(Error::Unimplemented)
    }

    /**
     * An XRun is an Underrun or an Overrun.
     * During playing, an underrun will occur if the stream is not written in time
     * and the system runs out of valid data.
     * During recording, an overrun will occur if the stream is not read in time
     * and there is no place to put the incoming data so it is discarded.
     *
     * An underrun or overrun can cause an audible "pop" or "glitch".
     *
     * @return a result which is either Result::OK with the xRun count as the value, or a
     * Result::Error* code
     */
    fn get_xrun_count(&self) -> Result<i32> {
        Err(Error::Unimplemented)
    }

    /**
     * @return true if XRun counts are supported on the stream
     */
    fn is_xrun_count_supported(&self) -> bool;

    /**
     * Query the number of frames that are read or written by the endpoint at one time.
     *
     * @return burst size
     */
    fn get_frames_per_burst(&mut self) -> i32;

    /**
     * Get the number of bytes in each audio frame. This is calculated using the channel count
     * and the sample format. For example, a 2 channel floating point stream will have
     * 2 * 4 = 8 bytes per frame.
     *
     * @return number of bytes in each audio frame.
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
     * The number of audio frames written into the stream.
     * This monotonic counter will never get reset.
     *
     * @return the number of frames written so far
     */
    fn get_frames_written(&mut self) -> i64;

    /**
     * The number of audio frames read from the stream.
     * This monotonic counter will never get reset.
     *
     * @return the number of frames read so far
     */
    fn get_frames_read(&mut self) -> i64;

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
     *
     * @return a ResultWithValue which has a result of Result::OK and a value containing the latency
     * in milliseconds, or a result of Result::Error*.
     */
    fn calculate_latency_millis(&mut self) -> Result<f64> {
        Err(Error::Unimplemented)
    }

    /**
     * Get the estimated time that the frame at `framePosition` entered or left the audio processing
     * pipeline.
     *
     * This can be used to coordinate events and interactions with the external environment, and to
     * estimate the latency of an audio stream. An example of usage can be found in the hello-oboe
     * sample (search for "calculateCurrentOutputLatencyMillis").
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
     * Write data from the supplied buffer into the stream. This method will block until the write
     * is complete or it runs out of time.
     *
     * If `timeoutNanoseconds` is zero then this call will not wait.
     *
     * @param buffer The address of the first sample.
     * @param numFrames Number of frames to write. Only complete frames will be written.
     * @param timeoutNanoseconds Maximum number of nanoseconds to wait for completion.
     * @return a ResultWithValue which has a result of Result::OK and a value containing the number
     * of frames actually written, or result of Result::Error*.
     */
    /*fn write(&mut self,
             const void* /* buffer */,
             int32_t /* numFrames */,
             int64_t /* timeoutNanoseconds */ ) -> Result<i64> {
        return ResultWithValue<int32_t>(Result::ErrorUnimplemented);
    }*/

    /**
     * Read data into the supplied buffer from the stream. This method will block until the read
     * is complete or it runs out of time.
     *
     * If `timeoutNanoseconds` is zero then this call will not wait.
     *
     * @param buffer The address of the first sample.
     * @param numFrames Number of frames to read. Only complete frames will be read.
     * @param timeoutNanoseconds Maximum number of nanoseconds to wait for completion.
     * @return a ResultWithValue which has a result of Result::OK and a value containing the number
     * of frames actually read, or result of Result::Error*.
     */
    /*virtual ResultWithValue<int32_t> read(void* /* buffer */,
                            int32_t /* numFrames */,
                            int64_t /* timeoutNanoseconds */) {
        return ResultWithValue<int32_t>(Result::ErrorUnimplemented);
    }*/

    /**
     * Get the underlying audio API which the stream uses.
     *
     * @return the API that this stream uses.
     */
    fn get_audio_api(&self) -> AudioApi;

    /**
     * Returns true if the underlying audio API is AAudio.
     *
     * @return true if this stream is implemented using the AAudio API.
     */
    fn uses_aaudio(&self) -> bool {
        self.get_audio_api() == AudioApi::AAudio
    }

    /*
     * Swap old callback for new callback.
     * This not atomic.
     * This should only be used internally.
     * @param streamCallback
     * @return previous streamCallback
     */
    /*AudioStreamCallback *swap_callback(AudioStreamCallback *streamCallback) {
        AudioStreamCallback *previousCallback = mStreamCallback;
        mStreamCallback = streamCallback;
        return previousCallback;
    }*/

    /**
     * @return number of frames of data currently in the buffer
     */
    fn get_available_frames(&mut self) -> Result<i32>;

    /**
     * Wait until the stream has a minimum amount of data available in its buffer.
     * This can be used with an EXCLUSIVE MMAP input stream to avoid reading data too close to
     * the DSP write position, which may cause glitches.
     *
     * @param numFrames minimum frames available
     * @param timeoutNanoseconds
     * @return number of frames available, ErrorTimeout
     */
    fn wait_for_available_frames(&mut self,
                                 num_frames: i32,
                                 timeout_nanoseconds: i64) -> Result<i32>;
}

impl<T: RawAudioStream + RawAudioStreamBase> IsAudioStream for T {
    fn open(&mut self) -> Status {
        wrap_status(unsafe {
            ffi::oboe_AudioStream_open(self._raw_stream_mut())
        })
    }

    fn close(&mut self) -> Status {
        wrap_status(unsafe {
            ffi::oboe_AudioStream_close(self._raw_stream_mut() as *mut _ as *mut c_void)
        })
    }

    fn start_with_timeout(&mut self, timeout_nanoseconds: i64) -> Status {
        wrap_status(unsafe {
            ffi::oboe_AudioStream_start(
                self._raw_stream_mut() as *mut _ as *mut c_void,
                timeout_nanoseconds,
            )
        })
    }

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

    fn stop_with_timeout(&mut self, timeout_nanoseconds: i64) -> Status {
        wrap_status(unsafe {
            ffi::oboe_AudioStream_stop(
                self._raw_stream_mut() as *mut _ as *mut c_void,
                timeout_nanoseconds,
            )
        })
    }

    fn request_start(&mut self) -> Status {
        wrap_status(unsafe {
            ffi::oboe_AudioStream_requestStart(
                self._raw_stream_mut(),
            )
        })
    }

    fn request_pause(&mut self) -> Status {
        wrap_status(unsafe {
            ffi::oboe_AudioStream_requestPause(
                self._raw_stream_mut(),
            )
        })
    }

    fn request_flush(&mut self) -> Status {
        wrap_status(unsafe {
            ffi::oboe_AudioStream_requestFlush(
                self._raw_stream_mut(),
            )
        })
    }

    fn request_stop(&mut self) -> Status {
        wrap_status(unsafe {
            ffi::oboe_AudioStream_requestStop(
                self._raw_stream_mut(),
            )
        })
    }

    fn get_state(&self) -> StreamState {
        FromPrimitive::from_i32(unsafe {
            ffi::oboe_AudioStream_getState(
                self._raw_stream(),
            )
        }).unwrap()
    }

    fn wait_for_state_change(&mut self,
                             input_state: StreamState,
                             timeout_nanoseconds: i64) -> Result<StreamState> {

        let mut next_state = MaybeUninit::<StreamState>::uninit();
        wrap_status(unsafe {
            ffi::oboe_AudioStream_waitForStateChange(
                self._raw_stream_mut(),
                input_state as i32,
                next_state.as_mut_ptr() as *mut i32,
                timeout_nanoseconds,
            )
        }).map(|_| unsafe { next_state.assume_init() })
    }

    fn set_buffer_size_in_frames(&mut self, requested_frames: i32) -> Result<i32> {
        wrap_result(unsafe {
            ffi::oboe_AudioStream_setBufferSizeInFrames(
                self._raw_stream_mut(),
                requested_frames,
            )
        })
    }

    fn get_xrun_count(&self) -> Result<i32> {
        wrap_result(unsafe {
            ffi::oboe_AudioStream_getXRunCount(
                self._raw_stream(),
            )
        })
    }

    fn is_xrun_count_supported(&self) -> bool {
        unsafe {
            ffi::oboe_AudioStream_isXRunCountSupported(
                self._raw_stream(),
            )
        }
    }

    fn get_frames_per_burst(&mut self) -> i32 {
        unsafe {
            ffi::oboe_AudioStream_getFramesPerBurst(
                self._raw_stream_mut(),
            )
        }
    }

    fn get_bytes_per_sample(&mut self) -> i32 {
        unsafe {
            ffi::oboe_AudioStream_getBytesPerSample(
                self._raw_stream_mut(),
            )
        }
    }

    fn get_frames_written(&mut self) -> i64 {
        unsafe {
            ffi::oboe_AudioStream_getFramesWritten(
                self._raw_stream_mut() as *mut _ as *mut c_void,
            )
        }
    }

    fn get_frames_read(&mut self) -> i64 {
        unsafe {
            ffi::oboe_AudioStream_getFramesRead(
                self._raw_stream_mut() as *mut _ as *mut c_void,
            )
        }
    }

    fn calculate_latency_millis(&mut self) -> Result<f64> {
        wrap_result(unsafe {
            ffi::oboe_AudioStream_calculateLatencyMillis(
                self._raw_stream_mut()
            )
        })
    }

    fn get_timestamp(&mut self, clock_id: i32 /* clockid_t */) -> Result<FrameTimestamp> {
        wrap_result(unsafe {
            ffi::oboe_AudioStream_getTimestamp(
                self._raw_stream_mut() as *mut _ as *mut c_void,
                clock_id,
            )
        })
    }

    /*fn write(&mut self,
             const void* /* buffer */,
             int32_t /* numFrames */,
             int64_t /* timeoutNanoseconds */ ) -> Result<i64> {
        return ResultWithValue<int32_t>(Result::ErrorUnimplemented);
    }*/

    /*virtual ResultWithValue<int32_t> read(void* /* buffer */,
                            int32_t /* numFrames */,
                            int64_t /* timeoutNanoseconds */) {
        return ResultWithValue<int32_t>(Result::ErrorUnimplemented);
    }*/

    fn get_audio_api(&self) -> AudioApi {
        FromPrimitive::from_i32(unsafe {
            ffi::oboe_AudioStream_getAudioApi(
                self._raw_stream()
            )
        }).unwrap()
    }

    /*AudioStreamCallback *swap_callback(AudioStreamCallback *streamCallback) {
        AudioStreamCallback *previousCallback = mStreamCallback;
        mStreamCallback = streamCallback;
        return previousCallback;
    }*/

    fn get_available_frames(&mut self) -> Result<i32> {
        wrap_result(unsafe {
            ffi::oboe_AudioStream_getAvailableFrames(
                self._raw_stream_mut()
            )
        })
    }

    fn wait_for_available_frames(&mut self,
                                 num_frames: i32,
                                 timeout_nanoseconds: i64) -> Result<i32> {
        wrap_result(unsafe {
            ffi::oboe_AudioStream_waitForAvailableFrames(
                self._raw_stream_mut(),
                num_frames,
                timeout_nanoseconds,
            )
        })
    }
}

#[derive(Debug)]
#[repr(transparent)]
pub struct AudioStream {
    raw: ffi::oboe_AudioStream,
}

impl RawAudioStream for AudioStream {
    fn _raw_stream(&self) -> &ffi::oboe_AudioStream {
        &self.raw
    }

    fn _raw_stream_mut(&mut self) -> &mut ffi::oboe_AudioStream {
        &mut self.raw
    }
}

impl RawAudioStreamBase for AudioStream {
    fn _raw_base(&self) -> &ffi::oboe_AudioStreamBase {
        &self.raw._base
    }

    fn _raw_base_mut(&mut self) -> &mut ffi::oboe_AudioStreamBase {
        &mut self.raw._base
    }
}
