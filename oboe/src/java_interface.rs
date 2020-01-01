use std::sync::Arc;

use num_derive::{FromPrimitive};
use num_traits::{FromPrimitive};

use android_ndk::{
    android_app::AndroidApp,
    native_activity::NativeActivity,
};

use jni::{
    JNIEnv,
    Executor,
    objects::{JValue, JObject, JList},
    strings::{JavaStr},
    errors::{Result as JResult},
};

use super::{
    AudioFormat as Format,
    //get_sdk_version,
};

pub fn request_default_stream_values() -> Result<(Option<i32>, Option<i32>), String> {
    let app = unsafe {
        AndroidApp::from_ptr(android_glue::get_android_app())
    };
    let activity = app.activity();
    //let sdk_version = get_sdk_version();
    let sdk_version = activity.sdk_version();

    println!("SDK VERSION: {}", sdk_version);

    if /*sdk_version >= 17 &&*/ sdk_version < 30 /*26*/ {
        try_request_default_stream_values(&activity)
            .map_err(|error| error.to_string())
    } else {
        // not necessary
        Ok((None, None))
    }
}

/**
 * The Android audio device info
 */
#[derive(Debug, Clone)]
pub struct AudioDeviceInfo {
    /**
     * Device identifier
     */
    pub id: i32,

    /**
     * Device address
     */
    pub address: String,

    /**
     * Device product name
     */
    pub product_name: String,

    /**
     * The type of device
     */
    pub device_type: AudioDeviceType,

    /**
     * The device can be used for playback
     */
    pub is_sink: bool,

    /**
     * The device can be used for capture
     */
    pub is_source: bool,

    /**
     * Available channel configurations
     */
    pub channel_counts: Vec<i32>,

    /**
     * Supported sample rates
     */
    pub sample_rates: Vec<i32>,

    /**
     * Supported audio formats
     */
    pub formats: Vec<Format>,
}

/**
 * The type of audio device
 */
#[derive(Debug, Clone, Copy, FromPrimitive)]
#[repr(i32)]
pub enum AudioDeviceType {
    Unknown = 0,
    AuxLine = 19,
    BluetoothA2DP = 8,
    BluetoothSCO = 7,
    BuiltinEarpiece = 1,
    BuiltinMic = 15,
    BuiltinSpeaker = 2,
    Bus = 21,
    Dock = 13,
    Fm = 14,
    FmTuner = 16,
    Hdmi = 9,
    HdmiArc = 10,
    HearingAid = 23,
    Ip = 20,
    LineAnalog = 5,
    LineDigital = 6,
    Telephony = 18,
    TvTuner = 17,
    UsbAccessory = 12,
    UsbDevice = 11,
    UsbHeadset = 22,
    UsbHeadphones = 4,
    WiredHeadset = 3,
}

struct AudioFormat;

impl AudioFormat {
    pub const ENCODING_PCM_16BIT: i32 = 2;
    //pub const ENCODING_PCM_8BIT: i32 = 3;
    pub const ENCODING_PCM_FLOAT: i32 = 4;

    fn to_format(encoding: i32) -> Option<Format> {
        match encoding {
            AudioFormat::ENCODING_PCM_16BIT => Some(Format::I16),
            AudioFormat::ENCODING_PCM_FLOAT => Some(Format::F32),
            _ => None,
        }
    }
}

/**
 * Request audio devices using Android Java API
 */
pub fn request_devices_info() -> Result<Vec<AudioDeviceInfo>, String> {
    let app = unsafe {
        AndroidApp::from_ptr(android_glue::get_android_app())
    };
    let activity = app.activity();

    try_request_devices_info(&activity)
        .map_err(|error| error.to_string())
}

struct Context;

impl Context {
    pub const AUDIO_SERVICE: &'static str = "audio";
}

struct AudioManager;

impl AudioManager {
    pub const PROPERTY_OUTPUT_SAMPLE_RATE: &'static str = "android.media.property.OUTPUT_SAMPLE_RATE";
    pub const PROPERTY_OUTPUT_FRAMES_PER_BUFFER: &'static str = "android.media.property.OUTPUT_FRAMES_PER_BUFFER";

    pub const GET_DEVICES_INPUTS: i32 = 1 << 0;
    pub const GET_DEVICES_OUTPUTS: i32 = 1 << 1;
    pub const GET_DEVICES_ALL: i32 = Self::GET_DEVICES_INPUTS | Self::GET_DEVICES_OUTPUTS;

    fn get_devices<'a: 'b, 'b>(env: &'b JNIEnv<'a>, subject: JObject, flags: i32) -> JResult<JList<'a, 'b>> {
        env.get_list(env.call_method(
            subject,
            "getDevices",
            "(I)[Landroid/media/AudioDeviceInfo;",
            &[flags.into()],
        )?.l()?)
    }
}

fn get_system_service<'a>(env: &JNIEnv<'a>, subject: JObject, name: &str) -> JResult<JObject<'a>> {
    env.call_method(
        subject,
        "getSystemService",
        "(Ljava/lang/String;)Ljava/lang/Object;",
        &[JObject::from(env.new_string(name)?).into()],
    )?.l()
}

fn get_property<'a: 'b, 'b>(env: &'b JNIEnv<'a>, subject: JObject, name: &str) -> JResult<JavaStr<'a, 'b>> {
    env.get_string(
        env.call_method(
            subject,
            "getProperty",
            "(Ljava/lang/String;)Ljava/lang/String;",
            &[JObject::from(env.new_string(name)?).into()],
        )?.l()?.into()
    )
}

fn try_request_default_stream_values(activity: &NativeActivity) -> JResult<(Option<i32>, Option<i32>)> {
    let vm = Arc::new(activity.vm());

    let activity = activity.activity();

    println!("Attached threads: {}", vm.threads_attached());

    let exec = Executor::new(vm);

    exec.with_attached(|env| {
        println!("ENV: {:?}", env.get_version());
        println!("Activity: {:?}", activity);

        let class = env.find_class("android/app/NativeActivity")?;

        println!("Found NativeActivity class: {:?}", class);

        let class = env.get_object_class(activity)?;

        println!("Actual Activity class: {:?}", class);

        let method = env.get_method_id(class, "getSystemService", "(Ljava/lang/String;)Ljava/lang/Object;")?;

        println!("Activity method: {:?}", method);

        let audio_manager = get_system_service(
            &env, activity,
            Context::AUDIO_SERVICE,
        )?;

        println!("audio manager: {:?}", audio_manager);

        let sample_rate = get_property(
            &env, audio_manager,
            AudioManager::PROPERTY_OUTPUT_SAMPLE_RATE
        )?;

        println!("sample rate: {:?}", sample_rate.to_str());

        let frames_per_burst = get_property(
            &env, audio_manager,
            AudioManager::PROPERTY_OUTPUT_FRAMES_PER_BUFFER
        )?;

        Ok((
            (*sample_rate).to_str().ok().and_then(|s| s.parse().ok()),
            (*frames_per_burst).to_str().ok().and_then(|s| s.parse().ok()),
        ))
    })
}

fn try_request_devices_info(activity: &NativeActivity) -> JResult<Vec<AudioDeviceInfo>> {
    let vm = Arc::new(activity.vm());
    let activity = activity.activity();
    let exec = Executor::new(vm);

    exec.with_attached(|env| {
        let audio_manager = get_system_service(
            &env, activity,
            Context::AUDIO_SERVICE,
        )?;

        let devices = AudioManager::get_devices(
            &env, audio_manager,
            AudioManager::GET_DEVICES_ALL,
        )?;

        devices.iter()?.map(|device| Ok(AudioDeviceInfo {
            id: call_method_no_args_ret_int(&env, device, "getId")?,
            address: call_method_no_args_ret_string(&env, device, "getAddress")?,
            product_name: call_method_no_args_ret_char_sequence(&env, device, "getProductName")?,
            device_type: FromPrimitive::from_i32(call_method_no_args_ret_int(&env, device, "getType")?).unwrap(),
            is_sink: call_method_no_args_ret_bool(&env, device, "isSink")?,
            is_source: call_method_no_args_ret_bool(&env, device, "isSource")?,
            channel_counts: call_method_no_args_ret_int_list(&env, device, "getChannelCounts")?,
            sample_rates: call_method_no_args_ret_int_list(&env, device, "getSampleRates")?,
            formats: call_method_no_args_ret_int_list(&env, device, "getEncodings")?
                .into_iter()
                .map(AudioFormat::to_format)
                .filter(Option::is_some)
                .map(Option::unwrap)
                .collect::<Vec<_>>(),
        })).collect::<Result<Vec<_>, _>>()
    })
}

fn call_method_no_args_ret_int_list<'a>(env: &JNIEnv<'a>, subject: JObject, name: &str) -> JResult<Vec<i32>> {
    env.get_list(env.call_method(
        subject,
        name,
        "()[I",
        &[],
    )?.l()?)?.iter()?.map(|item| {
        JValue::from(item).i()
    }).collect::<Result<Vec<_>, _>>()
}

fn call_method_no_args_ret_int<'a>(env: &JNIEnv<'a>, subject: JObject, name: &str) -> JResult<i32> {
    env.call_method(
        subject,
        name,
        "()I",
        &[],
    )?.i()
}

fn call_method_no_args_ret_bool<'a>(env: &JNIEnv<'a>, subject: JObject, name: &str) -> JResult<bool> {
    env.call_method(
        subject,
        name,
        "()Z",
        &[],
    )?.z()
}

fn call_method_no_args_ret_string<'a>(env: &JNIEnv<'a>, subject: JObject, name: &str) -> JResult<String> {
    env.get_string(
        env.call_method(
            subject,
            name,
            "()Ljava/lang/String;",
            &[],
        )?.l()?.into()
    ).map(String::from)
}

fn call_method_no_args_ret_char_sequence<'a>(env: &JNIEnv<'a>, subject: JObject, name: &str) -> JResult<String> {
    env.get_string(
        env.call_method(
            env.call_method(
                subject,
                name,
                "()Ljava/lang/CharSequence;",
                &[],
            )?.l()?,
            "toString",
            "()Ljava/lang/String;",
            &[],
        )?.l()?.into()
    ).map(String::from)
}
