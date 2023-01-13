use std::{
    ffi::c_void,
    slice::{from_raw_parts, from_raw_parts_mut},
};

use oboe_sys as ffi;

use num_traits::FromPrimitive;

use super::{
    AudioInputStreamSafe, AudioOutputStreamSafe, AudioStreamBuilderHandle, AudioStreamRef,
    DataCallbackResult, Error, IsFrameType,
};

/**
 * This trait defines a callback interface for:
 *
 * 1) moving data to/from an audio stream using `on_audio_ready`
 * 2) being alerted when a stream has an error using `on_error_*` methods
 */
pub trait AudioInputCallback {
    /**
     * The sample type and number of channels for processing.
     *
     * Oboe supports only two sample types:
     *
     * - **i16** - signed 16-bit integer samples
     * - **f32** - 32-bit floating point samples
     *
     * Oboe supports only mono and stereo channel configurations.
     */
    type FrameType: IsFrameType;

    /**
     * This will be called when an error occurs on a stream or when the stream is disconnected.
     *
     * Note that this will be called on a different thread than the onAudioReady() thread.
     * This thread will be created by Oboe.
     *
     * The underlying stream will already be stopped by Oboe but not yet closed.
     * So the stream can be queried.
     *
     * Do not close or delete the stream in this method because it will be
     * closed after this method returns.
     */
    fn on_error_before_close(
        &mut self,
        _audio_stream: &mut dyn AudioInputStreamSafe,
        _error: Error,
    ) {
    }

    /**
     * This will be called when an error occurs on a stream or when the stream is disconnected.
     * The underlying AAudio or OpenSL ES stream will already be stopped AND closed by Oboe.
     * So the underlying stream cannot be referenced.
     * But you can still query most parameters.
     *
     * This callback could be used to reopen a new stream on another device.
     * You can safely delete the old AudioStream in this method.
     */
    fn on_error_after_close(
        &mut self,
        _audio_stream: &mut dyn AudioInputStreamSafe,
        _error: Error,
    ) {
    }

    /**
     * A buffer is ready for processing.
     *
     * For an output stream, this function should render and write `num_frames` of data
     * in the stream's current data format to the audioData buffer.
     *
     * For an input stream, this function should read and process `num_frames` of data
     * from the audioData buffer.
     *
     * The audio data is passed through the buffer. So do NOT call read() or
     * write() on the stream that is making the callback.
     *
     * Note that numFrames can vary unless AudioStreamBuilder::setFramesPerCallback()
     * is called.
     *
     * Also note that this callback function should be considered a "real-time" function.
     * It must not do anything that could cause an unbounded delay because that can cause the
     * audio to glitch or pop.
     *
     * These are things the function should NOT do:
     *
     * - allocate memory
     * - any file operations such as opening, closing, reading or writing
     * - any network operations such as streaming
     * - use any mutexes or other blocking synchronization primitives
     * - sleep
     * - stop or close stream
     * - read or write on stream which invoked it
     *
     * The following are OK to call from the data callback:
     *
     * - stream.get_*()
     *
     * If you need to move data, eg. MIDI commands, in or out of the callback function then
     * we recommend the use of non-blocking techniques such as an atomic FIFO.
     */
    fn on_audio_ready(
        &mut self,
        audio_stream: &mut dyn AudioInputStreamSafe,
        audio_data: &[<Self::FrameType as IsFrameType>::Type],
    ) -> DataCallbackResult;
}

/**
 * This trait defines a callback interface for:
 *
 * 1) moving data to/from an audio stream using `on_audio_ready`
 * 2) being alerted when a stream has an error using `on_error_*` methods
 */
pub trait AudioOutputCallback {
    /**
     * The sample type and number of channels for processing.
     *
     * Oboe supports only two sample types:
     *
     * - **i16** - signed 16-bit integer samples
     * - **f32** - 32-bit floating point samples
     *
     * Oboe supports only mono and stereo channel configurations.
     */
    type FrameType: IsFrameType;

    /**
     * This will be called when an error occurs on a stream or when the stream is disconnected.
     *
     * Note that this will be called on a different thread than the onAudioReady() thread.
     * This thread will be created by Oboe.
     *
     * The underlying stream will already be stopped by Oboe but not yet closed.
     * So the stream can be queried.
     *
     * Do not close or delete the stream in this method because it will be
     * closed after this method returns.
     */
    fn on_error_before_close(
        &mut self,
        _audio_stream: &mut dyn AudioOutputStreamSafe,
        _error: Error,
    ) {
    }

    /**
     * This will be called when an error occurs on a stream or when the stream is disconnected.
     * The underlying AAudio or OpenSL ES stream will already be stopped AND closed by Oboe.
     * So the underlying stream cannot be referenced.
     * But you can still query most parameters.
     *
     * This callback could be used to reopen a new stream on another device.
     * You can safely delete the old AudioStream in this method.
     */
    fn on_error_after_close(
        &mut self,
        _audio_stream: &mut dyn AudioOutputStreamSafe,
        _error: Error,
    ) {
    }

    /**
     * A buffer is ready for processing.
     *
     * For an output stream, this function should render and write numFrames of data
     * in the stream's current data format to the audioData buffer.
     *
     * For an input stream, this function should read and process numFrames of data
     * from the audioData buffer.
     *
     * The audio data is passed through the buffer. So do NOT call read() or
     * write() on the stream that is making the callback.
     *
     * Note that numFrames can vary unless AudioStreamBuilder::set_frames_per_callback()
     * is called.
     *
     * Also note that this callback function should be considered a "real-time" function.
     * It must not do anything that could cause an unbounded delay because that can cause the
     * audio to glitch or pop.
     *
     * These are things the function should NOT do:
     *
     * - allocate memory
     * - any file operations such as opening, closing, reading or writing
     * - any network operations such as streaming
     * - use any mutexes or other blocking synchronization primitives
     * - sleep
     * - stop or close stream
     * - read or write on stream which invoked it
     *
     * The following are OK to call from the data callback:
     *
     * - stream.get_*()
     *
     * If you need to move data, eg. MIDI commands, in or out of the callback function then
     * we recommend the use of non-blocking techniques such as an atomic FIFO.
     */
    fn on_audio_ready(
        &mut self,
        audio_stream: &mut dyn AudioOutputStreamSafe,
        audio_data: &mut [<Self::FrameType as IsFrameType>::Type],
    ) -> DataCallbackResult;
}

pub(crate) fn set_input_callback<T: AudioInputCallback>(
    builder: &mut AudioStreamBuilderHandle,
    callback: T,
) {
    let callback = Box::into_raw(Box::new(callback));

    // SAFETY: `callback` has the same type as the first argument of each function, and each
    // function follows the C ABI.
    unsafe {
        ffi::oboe_AudioStreamBuilder_setCallback(
            &mut **builder as *mut ffi::oboe_AudioStreamBuilder,
            callback.cast(),
            Some(drop_context::<T>),
            Some(on_audio_ready_input_wrapper::<T>),
            Some(on_error_before_close_input_wrapper::<T>),
            Some(on_error_after_close_input_wrapper::<T>),
        );
    }
}

pub(crate) fn set_output_callback<T: AudioOutputCallback>(
    builder: &mut AudioStreamBuilderHandle,
    callback: T,
) {
    let callback = Box::new(callback);
    let callback = Box::into_raw(callback);

    // SAFETY: `callback` has the same type as the first argument of each function, and each
    // function follows the C ABI.
    unsafe {
        ffi::oboe_AudioStreamBuilder_setCallback(
            &mut **builder as *mut ffi::oboe_AudioStreamBuilder,
            callback.cast(),
            Some(drop_context::<T>),
            Some(on_audio_ready_output_wrapper::<T>),
            Some(on_error_before_close_output_wrapper::<T>),
            Some(on_error_after_close_output_wrapper::<T>),
        );
    }
}

unsafe extern "C" fn drop_context<T>(context: *mut c_void) {
    let context = Box::from_raw(context as *mut T);
    drop(context);
}

unsafe extern "C" fn on_error_before_close_input_wrapper<T: AudioInputCallback>(
    context: *mut c_void,
    audio_stream: *mut ffi::oboe_AudioStream,
    error: ffi::oboe_Result,
) {
    let mut audio_stream = AudioStreamRef::wrap_raw(&mut *audio_stream);
    let callback = &mut *(context as *mut T);

    callback.on_error_before_close(&mut audio_stream, FromPrimitive::from_i32(error).unwrap());
}

unsafe extern "C" fn on_error_after_close_input_wrapper<T: AudioInputCallback>(
    context: *mut c_void,
    audio_stream: *mut ffi::oboe_AudioStream,
    error: ffi::oboe_Result,
) {
    let mut audio_stream = AudioStreamRef::wrap_raw(&mut *audio_stream);
    let callback = &mut *(context as *mut T);

    callback.on_error_after_close(&mut audio_stream, FromPrimitive::from_i32(error).unwrap());
}

unsafe extern "C" fn on_audio_ready_input_wrapper<T: AudioInputCallback>(
    context: *mut c_void,
    audio_stream: *mut ffi::oboe_AudioStream,
    audio_data: *mut c_void,
    num_frames: i32,
) -> ffi::oboe_DataCallbackResult {
    let mut audio_stream = AudioStreamRef::wrap_raw(&mut *audio_stream);

    let audio_data = from_raw_parts(
        audio_data as *const <T::FrameType as IsFrameType>::Type,
        num_frames as usize,
    );

    let callback = &mut *(context as *mut T);

    callback.on_audio_ready(&mut audio_stream, audio_data) as i32
}

unsafe extern "C" fn on_error_before_close_output_wrapper<T: AudioOutputCallback>(
    context: *mut c_void,
    audio_stream: *mut ffi::oboe_AudioStream,
    error: ffi::oboe_Result,
) {
    let mut audio_stream = AudioStreamRef::wrap_raw(&mut *audio_stream);

    let callback = &mut *(context as *mut T);

    callback.on_error_before_close(&mut audio_stream, FromPrimitive::from_i32(error).unwrap());
}

unsafe extern "C" fn on_error_after_close_output_wrapper<T: AudioOutputCallback>(
    context: *mut c_void,
    audio_stream: *mut ffi::oboe_AudioStream,
    error: ffi::oboe_Result,
) {
    let mut audio_stream = AudioStreamRef::wrap_raw(&mut *audio_stream);
    let callback = &mut *(context as *mut T);

    callback.on_error_after_close(&mut audio_stream, FromPrimitive::from_i32(error).unwrap());
}

unsafe extern "C" fn on_audio_ready_output_wrapper<T: AudioOutputCallback>(
    context: *mut c_void,
    audio_stream: *mut ffi::oboe_AudioStream,
    audio_data: *mut c_void,
    num_frames: i32,
) -> ffi::oboe_DataCallbackResult {
    let mut audio_stream = AudioStreamRef::wrap_raw(&mut *audio_stream);

    let audio_data = from_raw_parts_mut(
        audio_data as *mut <T::FrameType as IsFrameType>::Type,
        num_frames as usize,
    );

    let callback = &mut *(context as *mut T);

    callback.on_audio_ready(&mut audio_stream, audio_data) as i32
}
