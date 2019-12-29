use android_ndk::{
    android_app::AndroidApp,
};

use jni::{
    JavaVM,
    objects::{JValue, JObject},
    errors::Result,
};

pub fn request_default_stream_values() -> (Option<i32>, Option<i32>) {
    let app = unsafe { AndroidApp::from_ptr(android_glue::get_android_app()) };
    let activity = app.activity();
    //let sdk_version = get_sdk_version();
    let sdk_version = activity.sdk_version();

    let mut result = (None, None);

    if /*sdk_version >= 17 &&*/ sdk_version < 26 {
        if let Ok(values) = try_request_default_stream_values(
            &activity.vm(),
            activity.activity(),
        ) {
            result = values;
        };
        // TODO: handle error
    }

    // unavailable or unnecessary
    result
}

struct Context;

impl Context {
    pub const AUDIO_SERVICE: &'static str = "audio";
}

struct AudioManager;

impl AudioManager {
    pub const PROPERTY_OUTPUT_SAMPLE_RATE: &'static str = "android.media.property.OUTPUT_SAMPLE_RATE";
    pub const PROPERTY_OUTPUT_FRAMES_PER_BUFFER: &'static str = "android.media.property.OUTPUT_FRAMES_PER_BUFFER";
}

fn try_request_default_stream_values(vm: &JavaVM, activity: JObject) -> Result<(Option<i32>, Option<i32>)> {
    let env = vm.attach_current_thread()?;

    let audio_manager = env.call_method(
        activity,
        //"android/app/Activity/getSystemService",
        "getSystemService",
        "(Ljava/lang/String;)Ljava/lang/Object;",
        &[
            JValue::from(JObject::from(env.new_string(
                Context::AUDIO_SERVICE
            )?))
        ],
    )?.l()?;

    let sample_rate = env.get_string(
        env.call_method(
            audio_manager,
            //"android/media/AudioManager/getProperty",
            "getProperty",
            "(Ljava/lang/String;)Ljava/lang/String;",
            &[
                JValue::from(JObject::from(env.new_string(
                    AudioManager::PROPERTY_OUTPUT_SAMPLE_RATE
                )?))
            ]
        )?.l()?.into()
    )?;

    let frames_per_burst = env.get_string(
        env.call_method(
            audio_manager,
            "getProperty",
            "(Ljava/lang/String;)Ljava/lang/String;",
            &[
                JValue::from(JObject::from(env.new_string(
                    AudioManager::PROPERTY_OUTPUT_FRAMES_PER_BUFFER
                )?))
            ]
        )?.l()?.into()
    )?;

    Ok((
        (*sample_rate).to_str().ok().and_then(|s| s.parse().ok()),
        (*frames_per_burst).to_str().ok().and_then(|s| s.parse().ok()),
    ))
}
