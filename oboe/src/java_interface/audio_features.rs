use super::{
    PackageManager,

    utils::{
        JNIEnv,
        JObject,
        JResult,
        get_activity,
        with_attached,
        get_package_manager,
        has_system_feature,
    },
};

/**
 * The Android audio features
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AudioFeature {
    LowLatency,
    Output,
    Pro,
    Microphone,
    Midi,
}

impl Into<&'static str> for AudioFeature {
    fn into(self) -> &'static str {
        use self::AudioFeature::*;
        match self {
            LowLatency => PackageManager::FEATURE_AUDIO_LOW_LATENCY,
            Output => PackageManager::FEATURE_AUDIO_OUTPUT,
            Pro => PackageManager::FEATURE_AUDIO_PRO,
            Microphone => PackageManager::FEATURE_MICROPHONE,
            Midi => PackageManager::FEATURE_MIDI,
        }
    }
}

impl AudioFeature {
    /**
     * Check availability of an audio feature using Android Java API
     */
    pub fn has(&self) -> Result<bool, String> {
        let activity = get_activity();

        with_attached(activity, |env, activity| {
            try_check_system_feature(env, activity, (*self).into())
        }).map_err(|error| error.to_string())
    }
}

fn try_check_system_feature<'a>(env: &JNIEnv<'a>, activity: JObject, feature: &str) -> JResult<bool> {
    let package_manager = get_package_manager(env, activity)?;

    has_system_feature(env, package_manager, feature)
}
