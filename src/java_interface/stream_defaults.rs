use super::{
    utils::{
        get_context, get_property, get_system_service, with_attached, JNIEnv, JObject, JResult,
    },
    AudioManager, Context,
};

use crate::DefaultStreamValues;

impl DefaultStreamValues {
    /**
     * Try request defaults from AudioManager properties.
     */
    #[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "java-interface")))]
    pub fn init() -> Result<(), String> {
        let activity = get_context();

        let values = with_attached(activity, |env, context| {
            let sdk_version = env
                .get_static_field("android/os/Build$VERSION", "SDK_INT", "I")?
                .i()?;

            if sdk_version < 17 {
                Err(jni::errors::Error::MethodNotFound {
                    name: "".into(),
                    sig: "".into(),
                })
            } else if sdk_version < 26 {
                try_request_default_stream_values(env, context).map(Some)
            } else {
                // not necessary
                Ok(None)
            }
        });

        match values {
            Ok(Some((sample_rate, frames_per_burst))) => {
                if let Some(value) = sample_rate {
                    Self::set_sample_rate(value);
                }
                if let Some(value) = frames_per_burst {
                    Self::set_frames_per_burst(value);
                }
                Ok(())
            }
            Ok(None) => Ok(()),
            Err(error) => Err(error.to_string()),
        }
    }
}

fn try_request_default_stream_values(
    env: &JNIEnv<'_>,
    context: JObject,
) -> JResult<(Option<i32>, Option<i32>)> {
    let audio_manager = get_system_service(env, context, Context::AUDIO_SERVICE)?;

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
