use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::FromPrimitive;
use oboe_sys as ffi;
use std::{error, fmt, result};

/**
 * The number of nanoseconds in a microsecond. 1,000.
 */
pub const NANOS_PER_MICROSECOND: i64 = 1000;

/**
 * The number of nanoseconds in a millisecond. 1,000,000.
 */
pub const NANOS_PER_MILLISECOND: i64 = NANOS_PER_MICROSECOND * 1000;

/**
 * The number of milliseconds in a second. 1,000.
 */
pub const MILLIS_PER_SECOND: i64 = 1000;

/**
 * The number of nanoseconds in a second. 1,000,000,000.
 */
pub const NANOS_PER_SECOND: i64 = NANOS_PER_MILLISECOND * MILLIS_PER_SECOND;

/**
 * The state of the audio stream.
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, ToPrimitive)]
#[repr(i32)]
pub enum StreamState {
    Uninitialized = ffi::oboe_StreamState_Uninitialized,
    Unknown = ffi::oboe_StreamState_Unknown,
    Open = ffi::oboe_StreamState_Open,
    Starting = ffi::oboe_StreamState_Starting,
    Started = ffi::oboe_StreamState_Started,
    Pausing = ffi::oboe_StreamState_Pausing,
    Paused = ffi::oboe_StreamState_Paused,
    Flushing = ffi::oboe_StreamState_Flushing,
    Flushed = ffi::oboe_StreamState_Flushed,
    Stopping = ffi::oboe_StreamState_Stopping,
    Stopped = ffi::oboe_StreamState_Stopped,
    Closing = ffi::oboe_StreamState_Closing,
    Closed = ffi::oboe_StreamState_Closed,
    Disconnected = ffi::oboe_StreamState_Disconnected,
}

/**
 * The direction of the stream.
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, ToPrimitive)]
#[repr(i32)]
pub enum Direction {
    /**
     * Used for playback.
     */
    Output = ffi::oboe_Direction_Output,

    /**
     * Used for recording.
     */
    Input = ffi::oboe_Direction_Input,
}

/**
 * The format of audio samples.
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, ToPrimitive)]
#[repr(i32)]
pub enum AudioFormat {
    /**
     * Invalid format.
     */
    Invalid = ffi::oboe_AudioFormat_Invalid,

    /**
     * Unspecified format. Format will be decided by Oboe.
     */
    Unspecified = ffi::oboe_AudioFormat_Unspecified,

    /**
     * Signed 16-bit integers.
     */
    I16 = ffi::oboe_AudioFormat_I16,

    /**
     * Signed 24-bit integers.
     */
    I24 = ffi::oboe_AudioFormat_I24,

    /**
     * Signed 32-bit integers.
     */
    I32 = ffi::oboe_AudioFormat_I32,

    /**
     * Single precision floating points.
     */
    F32 = ffi::oboe_AudioFormat_Float,
}

/**
 * The result of an audio callback.
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, ToPrimitive)]
#[repr(i32)]
pub enum DataCallbackResult {
    /**
     * Indicates to the caller that the callbacks should continue.
     */
    Continue = ffi::oboe_DataCallbackResult_Continue,

    /**
     * Indicates to the caller that the callbacks should stop immediately.
     */
    Stop = ffi::oboe_DataCallbackResult_Stop,
}

/**
 * The result of an operation with value
 */
pub type Result<T> = result::Result<T, Error>;

/**
 * The result of operation without value
 */
pub type Status = Result<()>;

pub(crate) fn wrap_status(result: i32) -> Status {
    if result == ffi::oboe_Result_OK {
        Ok(())
    } else {
        Err(FromPrimitive::from_i32(result).unwrap())
    }
}

pub(crate) fn wrap_result<T>(result: ffi::oboe_ResultWithValue<T>) -> Result<T> {
    if result.mError == ffi::oboe_Result_OK {
        Ok(result.mValue)
    } else {
        Err(FromPrimitive::from_i32(result.mError).unwrap())
    }
}

/**
 * The error of an operation.
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, ToPrimitive)]
#[repr(i32)]
pub enum Error {
    Disconnected = ffi::oboe_Result_ErrorDisconnected,
    IllegalArgument = ffi::oboe_Result_ErrorIllegalArgument,
    Internal = ffi::oboe_Result_ErrorInternal,
    InvalidState = ffi::oboe_Result_ErrorInvalidState,
    InvalidHandle = ffi::oboe_Result_ErrorInvalidHandle,
    Unimplemented = ffi::oboe_Result_ErrorUnimplemented,
    Unavailable = ffi::oboe_Result_ErrorUnavailable,
    NoFreeHandles = ffi::oboe_Result_ErrorNoFreeHandles,
    NoMemory = ffi::oboe_Result_ErrorNoMemory,
    Null = ffi::oboe_Result_ErrorNull,
    Timeout = ffi::oboe_Result_ErrorTimeout,
    WouldBlock = ffi::oboe_Result_ErrorWouldBlock,
    InvalidFormat = ffi::oboe_Result_ErrorInvalidFormat,
    OutOfRange = ffi::oboe_Result_ErrorOutOfRange,
    NoService = ffi::oboe_Result_ErrorNoService,
    InvalidRate = ffi::oboe_Result_ErrorInvalidRate,
    Closed = ffi::oboe_Result_ErrorClosed,
}

impl error::Error for Error {}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

/**
 * The sharing mode of the audio stream.
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, ToPrimitive)]
#[repr(i32)]
pub enum SharingMode {
    /**
     * This will be the only stream using a particular source or sink.
     * This mode will provide the lowest possible latency.
     * You should close EXCLUSIVE streams immediately when you are not using them.
     *
     * If you do not need the lowest possible latency then we recommend using Shared,
     * which is the default.
     */
    Exclusive = ffi::oboe_SharingMode_Exclusive,

    /**
     * Multiple applications can share the same device.
     * The data from output streams will be mixed by the audio service.
     * The data for input streams will be distributed by the audio service.
     *
     * This will have higher latency than the EXCLUSIVE mode.
     */
    Shared = ffi::oboe_SharingMode_Shared,
}

/**
 * The performance mode of the audio stream.
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, ToPrimitive)]
#[repr(i32)]
pub enum PerformanceMode {
    /**
     * No particular performance needs. Default.
     */
    None = ffi::oboe_PerformanceMode_None,

    /**
     * Extending battery life is most important.
     */
    PowerSaving = ffi::oboe_PerformanceMode_PowerSaving,

    /**
     * Reducing latency is most important.
     */
    LowLatency = ffi::oboe_PerformanceMode_LowLatency,
}

/**
 * The underlying audio API used by the audio stream.
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, ToPrimitive)]
#[repr(i32)]
pub enum AudioApi {
    /**
     * Try to use AAudio. If not available then use OpenSL ES.
     */
    Unspecified = ffi::oboe_AudioApi_Unspecified,

    /**
     * Use OpenSL ES.
     */
    OpenSLES = ffi::oboe_AudioApi_OpenSLES,

    /**
     * Try to use AAudio. Fail if unavailable.
     */
    AAudio = ffi::oboe_AudioApi_AAudio,
}

/**
 * Specifies the quality of the sample rate conversion performed by Oboe.
 * Higher quality will require more CPU load.
 * Higher quality conversion will probably be implemented using a sinc based resampler.
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, ToPrimitive)]
#[repr(i32)]
pub enum SampleRateConversionQuality {
    /**
     * No conversion by Oboe. Underlying APIs may still do conversion.
     */
    None,

    /**
     * Fastest conversion but may not sound great.
     * This may be implemented using bilinear interpolation.
     */
    Fastest,
    Low,
    Medium,
    High,

    /**
     * Highest quality conversion, which may be expensive in terms of CPU.
     */
    Best,
}

/**
 * The Usage attribute expresses *why* you are playing a sound, what is this sound used for.
 * This information is used by certain platforms or routing policies
 * to make more refined volume or routing decisions.
 *
 * Note that these match the equivalent values in AudioAttributes in the Android Java API.
 *
 * This attribute only has an effect on Android API 28+.
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, ToPrimitive)]
#[repr(i32)]
pub enum Usage {
    /**
     * Use this for streaming media, music performance, video, podcasts, etcetera.
     */
    Media = ffi::oboe_Usage_Media,

    /**
     * Use this for voice over IP, telephony, etcetera.
     */
    VoiceCommunication = ffi::oboe_Usage_VoiceCommunication,

    /**
     * Use this for sounds associated with telephony such as busy tones, DTMF, etcetera.
     */
    VoiceCommunicationSignalling = ffi::oboe_Usage_VoiceCommunicationSignalling,

    /**
     * Use this to demand the users attention.
     */
    Alarm = ffi::oboe_Usage_Alarm,

    /**
     * Use this for notifying the user when a message has arrived or some
     * other background event has occured.
     */
    Notification = ffi::oboe_Usage_Notification,

    /**
     * Use this when the phone rings.
     */
    NotificationRingtone = ffi::oboe_Usage_NotificationRingtone,

    /**
     * Use this to attract the users attention when, for example, the battery is low.
     */
    NotificationEvent = ffi::oboe_Usage_NotificationEvent,

    /**
     * Use this for screen readers, etcetera.
     */
    AssistanceAccessibility = ffi::oboe_Usage_AssistanceAccessibility,

    /**
     * Use this for driving or navigation directions.
     */
    AssistanceNavigationGuidance = ffi::oboe_Usage_AssistanceNavigationGuidance,

    /**
     * Use this for user interface sounds, beeps, etcetera.
     */
    AssistanceSonification = ffi::oboe_Usage_AssistanceSonification,

    /**
     * Use this for game audio and sound effects.
     */
    Game = ffi::oboe_Usage_Game,

    /**
     * Use this for audio responses to user queries, audio instructions or help utterances.
     */
    Assistant = ffi::oboe_Usage_Assistant,
}

/**
 * The ContentType attribute describes *what* you are playing.
 * It expresses the general category of the content. This information is optional.
 * But in case it is known (for instance {@link Movie} for a
 * movie streaming service or {@link Speech} for
 * an audio book application) this information might be used by the audio framework to
 * enforce audio focus.
 *
 * Note that these match the equivalent values in AudioAttributes in the Android Java API.
 *
 * This attribute only has an effect on Android API 28+.
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, ToPrimitive)]
#[repr(i32)]
pub enum ContentType {
    /**
     * Use this for spoken voice, audio books, etcetera.
     */
    Speech = ffi::oboe_ContentType_Speech,

    /**
     * Use this for pre-recorded or live music.
     */
    Music = ffi::oboe_ContentType_Music,

    /**
     * Use this for a movie or video soundtrack.
     */
    Movie = ffi::oboe_ContentType_Movie,

    /**
     * Use this for sound is designed to accompany a user action,
     * such as a click or beep sound made when the user presses a button.
     */
    Sonification = ffi::oboe_ContentType_Sonification,
}

/**
 * Defines the audio source.
 * An audio source defines both a default physical source of audio signal, and a recording
 * configuration.
 *
 * Note that these match the equivalent values in MediaRecorder.AudioSource in the Android Java API.
 *
 * This attribute only has an effect on Android API 28+.
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, ToPrimitive)]
#[repr(i32)]
pub enum InputPreset {
    /**
     * Use this preset when other presets do not apply.
     */
    Generic = ffi::oboe_InputPreset_Generic,

    /**
     * Use this preset when recording video.
     */
    Camcorder = ffi::oboe_InputPreset_Camcorder,

    /**
     * Use this preset when doing speech recognition.
     */
    VoiceRecognition = ffi::oboe_InputPreset_VoiceRecognition,

    /**
     * Use this preset when doing telephony or voice messaging.
     */
    VoiceCommunication = ffi::oboe_InputPreset_VoiceCommunication,

    /**
     * Use this preset to obtain an input with no effects.
     * Note that this input will not have automatic gain control
     * so the recorded volume may be very low.
     */
    Unprocessed = ffi::oboe_InputPreset_Unprocessed,

    /**
     * Use this preset for capturing audio meant to be processed in real time
     * and played back for live performance (e.g karaoke).
     * The capture path will minimize latency and coupling with playback path.
     */
    VoicePerformance = ffi::oboe_InputPreset_VoicePerformance,
}

/**
 * This attribute can be used to allocate a session ID to the audio stream.
 *
 * This attribute only has an effect on Android API 28+.
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, ToPrimitive)]
#[repr(i32)]
pub enum SessionId {
    /**
     * Do not allocate a session ID.
     * Effects cannot be used with this stream.
     * Default.
     */
    None = ffi::oboe_SessionId_None,

    /**
     * Allocate a session ID that can be used to attach and control
     * effects using the Java AudioEffects API.
     * Note that the use of this flag may result in higher latency.
     *
     * Note that this matches the value of `AudioManager.AUDIO_SESSION_ID_GENERATE`.
     */
    Allocate = ffi::oboe_SessionId_Allocate,
}

/**
 * The channel count of the audio stream.
 * Use of this enum is convenient to avoid "magic"
 * numbers when specifying the channel count.
 *
 * For example, you can write
 * `builder.set_channel_count(ChannelCount::Stereo)`
 * rather than `builder.set_channel_count(2).
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, ToPrimitive)]
#[repr(i32)]
pub enum ChannelCount {
    /**
     * Audio channel count definition, use Mono or Stereo
     */
    Unspecified = ffi::oboe_ChannelCount_Unspecified,

    /**
     * Use this for mono audio.
     */
    Mono = ffi::oboe_ChannelCount_Mono,

    /**
     * Use this for stereo audio.
     */
    Stereo = ffi::oboe_ChannelCount_Stereo,
}

/**
 * The default (optimal) audio streaming values.
 *
 * On API 16 to 26 OpenSL ES will be used.
 * When using OpenSL ES the optimal values for `sample_rate` and
 * `frames_per_burst` are not known by the native code.
 * On API 17+ these values should be obtained from the AudioManager using this code:
 *
 * ```java
 * // Note that this technique only works for built-in speakers and headphones.
 * AudioManager myAudioMgr = (AudioManager) getSystemService(Context.AUDIO_SERVICE);
 * String sampleRateStr = myAudioMgr.getProperty(AudioManager.PROPERTY_OUTPUT_SAMPLE_RATE);
 * int defaultSampleRate = Integer.parseInt(sampleRateStr);
 * String framesPerBurstStr = myAudioMgr.getProperty(AudioManager.PROPERTY_OUTPUT_FRAMES_PER_BUFFER);
 * int defaultFramesPerBurst = Integer.parseInt(framesPerBurstStr);
 * ```
 *
 * It can then be passed down to Oboe through JNI.
 *
 * AAudio will get the optimal `frames_per_burst` from the HAL and will ignore this value.
 */
pub struct DefaultStreamValues(());

impl DefaultStreamValues {
    /**
     * The default sample rate to use when opening new audio streams
     */
    pub fn get_sample_rate() -> i32 {
        unsafe { ffi::oboe_DefaultStreamValues_SampleRate }
    }

    pub fn set_sample_rate(sample_rate: i32) {
        unsafe {
            ffi::oboe_DefaultStreamValues_SampleRate = sample_rate;
        }
    }

    /**
     * The default frames per burst to use when opening new audio streams
     */
    pub fn get_frames_per_burst() -> i32 {
        unsafe { ffi::oboe_DefaultStreamValues_FramesPerBurst }
    }

    pub fn set_frames_per_burst(frames_per_burst: i32) {
        unsafe {
            ffi::oboe_DefaultStreamValues_FramesPerBurst = frames_per_burst;
        }
    }

    /**
     * The default channel count to use when opening new audio streams
     */
    pub fn get_channel_count() -> i32 {
        unsafe { ffi::oboe_DefaultStreamValues_ChannelCount }
    }

    pub fn set_channel_count(channel_count: i32) {
        unsafe {
            ffi::oboe_DefaultStreamValues_ChannelCount = channel_count;
        }
    }
}

/**
 * The time at which the frame at `position` was presented
 */
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FrameTimestamp {
    /**
     * The position in number of frames
     */
    pub position: i64,

    /**
     * The timestamp in nanoseconds
     */
    pub timestamp: i64,
}
