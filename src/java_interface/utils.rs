use jni::sys::jobject;
use ndk_context::AndroidContext;
use std::sync::Arc;

pub use jni::Executor;

pub use ndk::native_activity::NativeActivity;

pub use jni::{
    errors::Result as JResult,
    objects::{JList, JObject, JValue},
    strings::JavaStr,
    JNIEnv, JavaVM,
};

pub fn get_context() -> AndroidContext {
    ndk_context::android_context()
}

pub fn with_attached<F, R>(context: AndroidContext, closure: F) -> JResult<R>
where
    F: FnOnce(&JNIEnv, JObject) -> JResult<R>,
{
    let vm = Arc::new(unsafe { JavaVM::from_raw(context.vm().cast())? });
    let context = context.context();
    let context = unsafe { JObject::from_raw(context as jobject) };
    Executor::new(vm).with_attached(|env| closure(env, context))
}

pub fn call_method_no_args_ret_int_array(
    env: &JNIEnv<'_>,
    subject: JObject,
    method: &str,
) -> JResult<Vec<i32>> {
    let array = env.auto_local(env.call_method(subject, method, "()[I", &[])?.l()?);

    let raw_array = array.as_obj().into_raw();

    let length = env.get_array_length(raw_array)?;
    let mut values = Vec::with_capacity(length as usize);

    env.get_int_array_region(raw_array, 0, values.as_mut())?;

    Ok(values)
}

pub fn call_method_no_args_ret_int(
    env: &JNIEnv<'_>,
    subject: JObject,
    method: &str,
) -> JResult<i32> {
    env.call_method(subject, method, "()I", &[])?.i()
}

pub fn call_method_no_args_ret_bool(
    env: &JNIEnv<'_>,
    subject: JObject,
    method: &str,
) -> JResult<bool> {
    env.call_method(subject, method, "()Z", &[])?.z()
}

pub fn call_method_no_args_ret_string(
    env: &JNIEnv<'_>,
    subject: JObject,
    method: &str,
) -> JResult<String> {
    env.get_string(
        env.call_method(subject, method, "()Ljava/lang/String;", &[])?
            .l()?
            .into(),
    )
    .map(String::from)
}

pub fn call_method_no_args_ret_char_sequence(
    env: &JNIEnv<'_>,
    subject: JObject,
    method: &str,
) -> JResult<String> {
    env.get_string(
        env.call_method(
            env.call_method(subject, method, "()Ljava/lang/CharSequence;", &[])?
                .l()?,
            "toString",
            "()Ljava/lang/String;",
            &[],
        )?
        .l()?
        .into(),
    )
    .map(String::from)
}

pub fn call_method_string_arg_ret_bool<S: AsRef<str>>(
    env: &JNIEnv<'_>,
    subject: JObject,
    name: &str,
    arg: S,
) -> JResult<bool> {
    env.call_method(
        subject,
        name,
        "(Ljava/lang/String;)Z",
        &[JObject::from(env.new_string(arg)?).into()],
    )?
    .z()
}

pub fn call_method_string_arg_ret_string<'a: 'b, 'b, S: AsRef<str>>(
    env: &'b JNIEnv<'a>,
    subject: JObject<'a>,
    name: &str,
    arg: S,
) -> JResult<JavaStr<'a, 'b>> {
    env.get_string(
        env.call_method(
            subject,
            name,
            "(Ljava/lang/String;)Ljava/lang/String;",
            &[JObject::from(env.new_string(arg)?).into()],
        )?
        .l()?
        .into(),
    )
}

pub fn call_method_string_arg_ret_object<'a>(
    env: &JNIEnv<'a>,
    subject: JObject<'a>,
    method: &str,
    arg: &str,
) -> JResult<JObject<'a>> {
    env.call_method(
        subject,
        method,
        "(Ljava/lang/String;)Ljava/lang/Object;",
        &[JObject::from(env.new_string(arg)?).into()],
    )?
    .l()
}

pub fn get_package_manager<'a>(env: &JNIEnv<'a>, subject: JObject<'a>) -> JResult<JObject<'a>> {
    env.call_method(
        subject,
        "getPackageManager",
        "()Landroid/content/pm/PackageManager;",
        &[],
    )?
    .l()
}

pub fn has_system_feature(env: &JNIEnv<'_>, subject: JObject, name: &str) -> JResult<bool> {
    call_method_string_arg_ret_bool(env, subject, "hasSystemFeature", name)
}

pub fn get_system_service<'a>(
    env: &JNIEnv<'a>,
    subject: JObject<'a>,
    name: &str,
) -> JResult<JObject<'a>> {
    call_method_string_arg_ret_object(env, subject, "getSystemService", name)
}

pub fn get_property<'a: 'b, 'b>(
    env: &'b JNIEnv<'a>,
    subject: JObject<'a>,
    name: &str,
) -> JResult<JavaStr<'a, 'b>> {
    call_method_string_arg_ret_string(env, subject, "getProperty", name)
}

pub fn get_devices<'a: 'b, 'b>(
    env: &'b JNIEnv<'a>,
    subject: JObject<'a>,
    flags: i32,
) -> JResult<JObject<'a>> {
    env.call_method(
        subject,
        "getDevices",
        "(I)[Landroid/media/AudioDeviceInfo;",
        &[flags.into()],
    )?
    .l()
}
