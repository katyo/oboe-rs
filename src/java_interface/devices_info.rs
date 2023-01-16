use num_traits::FromPrimitive;

use crate::AudioFormat;

use super::{
    utils::{
        call_method_no_args_ret_bool, call_method_no_args_ret_char_sequence,
        call_method_no_args_ret_int, call_method_no_args_ret_int_array,
        call_method_no_args_ret_string, get_context, get_devices, get_system_service,
        with_attached, JNIEnv, JObject, JResult,
    },
    AudioDeviceDirection, AudioDeviceInfo, AudioDeviceType, Context,
};

impl AudioDeviceInfo {
    /**
     * Request audio devices using Android Java API
     */
    #[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "java-interface")))]
    pub fn request(direction: AudioDeviceDirection) -> Result<Vec<AudioDeviceInfo>, String> {
        let context = get_context();

        with_attached(context, |env, context| {
            let sdk_version = env
                .get_static_field("android/os/Build$VERSION", "SDK_INT", "I")?
                .i()?;

            if sdk_version >= 23 {
                try_request_devices_info(env, context, direction)
            } else {
                Err(jni::errors::Error::MethodNotFound {
                    name: "".into(),
                    sig: "".into(),
                })
            }
        })
        .map_err(|error| error.to_string())
    }
}

fn try_request_devices_info(
    env: &JNIEnv<'_>,
    context: JObject,
    direction: AudioDeviceDirection,
) -> JResult<Vec<AudioDeviceInfo>> {
    let audio_manager = get_system_service(env, context, Context::AUDIO_SERVICE)?;

    let devices = env.auto_local(get_devices(env, audio_manager, direction as i32)?);

    let raw_devices = devices.as_obj().into_raw();

    let length = env.get_array_length(raw_devices)?;

    (0..length)
        .map(|index| {
            let device = env.get_object_array_element(raw_devices, index)?;

            Ok(AudioDeviceInfo {
                id: call_method_no_args_ret_int(env, device, "getId")?,
                address: call_method_no_args_ret_string(env, device, "getAddress")?,
                product_name: call_method_no_args_ret_char_sequence(env, device, "getProductName")?,
                device_type: FromPrimitive::from_i32(call_method_no_args_ret_int(
                    env, device, "getType",
                )?)
                .unwrap_or(AudioDeviceType::Unsupported),
                direction: AudioDeviceDirection::new(
                    call_method_no_args_ret_bool(env, device, "isSource")?,
                    call_method_no_args_ret_bool(env, device, "isSink")?,
                ),
                channel_counts: call_method_no_args_ret_int_array(env, device, "getChannelCounts")?,
                sample_rates: call_method_no_args_ret_int_array(env, device, "getSampleRates")?,
                formats: call_method_no_args_ret_int_array(env, device, "getEncodings")?
                    .into_iter()
                    .filter_map(AudioFormat::from_encoding)
                    .collect::<Vec<_>>(),
            })
        })
        .collect::<Result<Vec<_>, _>>()
}
