use super::{
    utils::{
        get_activity, get_property, get_system_service, with_attached, JNIEnv, JObject, JResult,
    },
    AudioManager, Context,
};

use crate::DefaultStreamValues;

impl DefaultStreamValues {
    /**
     * Try request defaults from AudioManager properties.
     */
    pub fn init() -> Result<(), String> {
        let activity = get_activity();
        let sdk_version = activity.sdk_version();

        if sdk_version < 17 {
            Err("Unimplemented".into())
        } else if sdk_version < 26 {
            match with_attached(activity, try_request_default_stream_values) {
                Ok((sample_rate, frames_per_burst)) => {
                    if let Some(value) = sample_rate {
                        Self::set_sample_rate(value);
                    }
                    if let Some(value) = frames_per_burst {
                        Self::set_frames_per_burst(value);
                    }
                    Ok(())
                }
                Err(error) => Err(error.to_string()),
            }
        } else {
            // not necessary
            Ok(())
        }
    }
}

fn try_request_default_stream_values<'a>(
    env: &JNIEnv<'a>,
    activity: JObject,
) -> JResult<(Option<i32>, Option<i32>)> {
    let audio_manager = get_system_service(env, activity, Context::AUDIO_SERVICE)?;

    let sample_rate = get_property(
        env,
        audio_manager,
        AudioManager::PROPERTY_OUTPUT_SAMPLE_RATE,
    )?;

    let frames_per_burst = get_property(
        env,
        audio_manager,
        AudioManager::PROPERTY_OUTPUT_FRAMES_PER_BUFFER,
    )?;

    Ok((
        (*sample_rate).to_str().ok().and_then(|s| s.parse().ok()),
        (*frames_per_burst)
            .to_str()
            .ok()
            .and_then(|s| s.parse().ok()),
    ))
}
