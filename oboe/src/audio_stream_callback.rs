use std::{
    marker::PhantomData,
    slice::{
        from_raw_parts,
        from_raw_parts_mut,
    },
    ops::{Deref, DerefMut},
    ffi::c_void,
};

use oboe_sys as ffi;

use num_traits::FromPrimitive;

use super::{
    Error,
    IsFrameType,
    Input, Output,
    DataCallbackResult,
    AudioStreamRef,
    AudioInputStream,
    AudioOutputStream,
};

/**
 * AudioStreamCallback defines a callback interface for:
 *
 * 1) moving data to/from an audio stream using `onAudioReady`
 * 2) being alerted when a stream has an error using `onError*` methods
 *
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
     *
     * @param oboeStream pointer to the associated stream
     * @param error
     */
    fn on_error_before_close(
        &mut self,
        _audio_stream: &mut dyn AudioInputStream,
        _error: Error
    ) {}

    /**
     * This will be called when an error occurs on a stream or when the stream is disconnected.
     * The underlying AAudio or OpenSL ES stream will already be stopped AND closed by Oboe.
     * So the underlying stream cannot be referenced.
     * But you can still query most parameters.
     *
     * This callback could be used to reopen a new stream on another device.
     * You can safely delete the old AudioStream in this method.
     *
     * @param oboeStream pointer to the associated stream
     * @param error
     */
    fn on_error_after_close(
        &mut self,
        _audio_stream: &mut dyn AudioInputStream,
        _error: Error
    ) {}

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
     * Note that numFrames can vary unless AudioStreamBuilder::setFramesPerCallback()
     * is called.
     *
     * Also note that this callback function should be considered a "real-time" function.
     * It must not do anything that could cause an unbounded delay because that can cause the
     * audio to glitch or pop.
     *
     * These are things the function should NOT do:
     *
     * - allocate memory using
     * - any file operations such as opening, closing, reading or writing
     * - any network operations such as streaming
     * - use any mutexes or other synchronization primitives
     * - sleep
     * - oboeStream.stop(), pause(), flush() or close()
     * - oboeStream.read()
     * - oboeStream.write()
     *
     * The following are OK to call from the data callback:
     *
     * - oboeStream.get*()
     * - oboeStream.set_buffer_size_in_frames()
     *
     * If you need to move data, eg. MIDI commands, in or out of the callback function then
     * we recommend the use of non-blocking techniques such as an atomic FIFO.
     *
     * @param oboeStream pointer to the associated stream
     * @param audioData buffer containing input data or a place to put output data
     * @param numFrames number of frames to be processed
     * @return DataCallbackResult::Continue or DataCallbackResult::Stop
     */
    fn on_audio_ready(
        &mut self,
        audio_stream: &mut dyn AudioInputStream,
        audio_data: &[<Self::FrameType as IsFrameType>::Type]
    ) -> DataCallbackResult;
}

/**
 * AudioStreamCallback defines a callback interface for:
 *
 * 1) moving data to/from an audio stream using `onAudioReady`
 * 2) being alerted when a stream has an error using `onError*` methods
 *
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
     *
     * @param oboeStream pointer to the associated stream
     * @param error
     */
    fn on_error_before_close(
        &mut self,
        _audio_stream: &mut dyn AudioOutputStream,
        _error: Error
    ) {}

    /**
     * This will be called when an error occurs on a stream or when the stream is disconnected.
     * The underlying AAudio or OpenSL ES stream will already be stopped AND closed by Oboe.
     * So the underlying stream cannot be referenced.
     * But you can still query most parameters.
     *
     * This callback could be used to reopen a new stream on another device.
     * You can safely delete the old AudioStream in this method.
     *
     * @param oboeStream pointer to the associated stream
     * @param error
     */
    fn on_error_after_close(
        &mut self,
        _audio_stream: &mut dyn AudioOutputStream,
        _error: Error
    ) {}

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
     * Note that numFrames can vary unless AudioStreamBuilder::setFramesPerCallback()
     * is called.
     *
     * Also note that this callback function should be considered a "real-time" function.
     * It must not do anything that could cause an unbounded delay because that can cause the
     * audio to glitch or pop.
     *
     * These are things the function should NOT do:
     *
     * - allocate memory using
     * - any file operations such as opening, closing, reading or writing
     * - any network operations such as streaming
     * - use any mutexes or other synchronization primitives
     * - sleep
     * - oboeStream.stop(), pause(), flush() or close()
     * - oboeStream.read()
     * - oboeStream.write()
     *
     * The following are OK to call from the data callback:
     *
     * - oboeStream.get*()
     * - oboeStream.set_buffer_size_in_frames()
     *
     * If you need to move data, eg. MIDI commands, in or out of the callback function then
     * we recommend the use of non-blocking techniques such as an atomic FIFO.
     *
     * @param oboeStream pointer to the associated stream
     * @param audioData buffer containing input data or a place to put output data
     * @param numFrames number of frames to be processed
     * @return DataCallbackResult::Continue or DataCallbackResult::Stop
     */
    fn on_audio_ready(&mut self,
                      audio_stream: &mut dyn AudioOutputStream,
                      audio_data: &mut [<Self::FrameType as IsFrameType>::Type]) -> DataCallbackResult;
}

#[repr(transparent)]
struct AudioStreamCallbackWrapperHandle(*mut ffi::oboe_AudioStreamCallbackWrapper);

impl AudioStreamCallbackWrapperHandle {
    fn new(audio_ready: ffi::oboe_AudioReadyHandler,
           before_close: ffi::oboe_ErrorCloseHandler,
           after_close: ffi::oboe_ErrorCloseHandler) -> Self {
        Self(unsafe { ffi::oboe_AudioStreamCallbackWrapper_new(
            audio_ready,
            before_close,
            after_close,
        ) })
    }
}

impl Drop for AudioStreamCallbackWrapperHandle {
    fn drop(&mut self) {
        unsafe { ffi::oboe_AudioStreamCallbackWrapper_delete(self.0) }
    }
}

impl Deref for AudioStreamCallbackWrapperHandle {
    type Target = ffi::oboe_AudioStreamCallbackWrapper;

    fn deref(&self) -> &Self::Target {
        unsafe { &(*self.0) }
    }
}

impl DerefMut for AudioStreamCallbackWrapperHandle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut (*self.0) }
    }
}

pub(crate) struct AudioCallbackWrapper<D, T> {
    raw: AudioStreamCallbackWrapperHandle,
    callback: Box<T>,
    _phantom: PhantomData<D>,
}

impl<D, T> AudioCallbackWrapper<D, T> {
    pub(crate) fn raw_callback(&mut self) -> &mut ffi::oboe_AudioStreamCallbackWrapper {
        &mut *self.raw
    }
}

impl<T> AudioCallbackWrapper<Input, T>
where
    T: AudioInputCallback,
{
    pub(crate) fn wrap(callback: T) -> Self {
        let callback = Box::new(callback);
        let mut wrapper = Self {
            raw: AudioStreamCallbackWrapperHandle::new(
                Some(on_audio_ready_input_wrapper::<T>),
                Some(on_error_before_close_input_wrapper::<T>),
                Some(on_error_after_close_input_wrapper::<T>),
            ),
            callback,
            _phantom: PhantomData,
        };
        unsafe { (*wrapper.raw).setContext(
            &mut (*wrapper.callback) as *mut _ as *mut c_void
        ); }
        wrapper
    }
}

impl<T> AudioCallbackWrapper<Output, T>
where
    T: AudioOutputCallback,
{
    pub(crate) fn wrap(callback: T) -> Self {
        let callback = Box::new(callback);
        let mut wrapper = Self {
            raw: AudioStreamCallbackWrapperHandle::new(
                Some(on_audio_ready_output_wrapper::<T>),
                Some(on_error_before_close_output_wrapper::<T>),
                Some(on_error_after_close_output_wrapper::<T>),
            ),
            callback,
            _phantom: PhantomData,
        };
        unsafe { (*wrapper.raw).setContext(
            &mut (*wrapper.callback) as *mut _ as *mut c_void
        ); }
        wrapper
    }
}

unsafe extern "C" fn on_error_before_close_input_wrapper<T: AudioInputCallback>(
    context: *mut c_void,
    audio_stream: *mut ffi::oboe_AudioStream,
    error: ffi::oboe_Result,
) {
    let mut audio_stream = AudioStreamRef::wrap_raw(&mut *audio_stream);
    let callback = &mut *(context as *mut T);

    callback.on_error_before_close(
        &mut audio_stream,
        FromPrimitive::from_i32(error).unwrap()
    );
}

unsafe extern "C" fn on_error_after_close_input_wrapper<T: AudioInputCallback>(
    context: *mut c_void,
    audio_stream: *mut ffi::oboe_AudioStream,
    error: ffi::oboe_Result,
) {
    let mut audio_stream = AudioStreamRef::wrap_raw(&mut *audio_stream);
    let callback = &mut *(context as *mut T);

    callback.on_error_after_close(
        &mut audio_stream,
        FromPrimitive::from_i32(error).unwrap()
    );
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

    callback.on_audio_ready(
        &mut audio_stream,
        audio_data,
    ) as i32
}

unsafe extern "C" fn on_error_before_close_output_wrapper<T: AudioOutputCallback>(
    context: *mut c_void,
    audio_stream: *mut ffi::oboe_AudioStream,
    error: ffi::oboe_Result,
) {
    let mut audio_stream = AudioStreamRef::wrap_raw(&mut *audio_stream);

    let callback = &mut *(context as *mut T);

    callback.on_error_before_close(
        &mut audio_stream,
        FromPrimitive::from_i32(error).unwrap()
    );
}

unsafe extern "C" fn on_error_after_close_output_wrapper<T: AudioOutputCallback>(
    context: *mut c_void,
    audio_stream: *mut ffi::oboe_AudioStream,
    error: ffi::oboe_Result,
) {
    let mut audio_stream = AudioStreamRef::wrap_raw(&mut *audio_stream);
    let callback = &mut *(context as *mut T);

    callback.on_error_after_close(
        &mut audio_stream,
        FromPrimitive::from_i32(error).unwrap()
    );
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

    callback.on_audio_ready(
        &mut audio_stream,
        audio_data,
    ) as i32
}
