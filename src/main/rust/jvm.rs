#[macro_use]
extern crate rs_jvm_bindings;

use rs_jvm_bindings::jni::{JavaVM, JNI_VERSION_1_8, jlong, JNIEnv, jobject, JNI_OK, jint, jstring, jclass};
use rs_jvm_bindings::utils::*;
use rs_jvm_bindings::jvm::*;

#[no_mangle]
pub unsafe extern "system" fn Java_dev_binclub_jvm4j_JVM4J_getInterfaceVersion(
	_env: *mut JNIEnv, _this: jobject
) -> jint {
	JVM_GetInterfaceVersion()
}

#[no_mangle]
pub unsafe extern "system" fn Java_dev_binclub_jvm4j_JVM4J_iHashCode(
	env: *mut JNIEnv, _this: jobject,
	obj: jobject
) -> jint {
	JVM_IHashCode(env, obj)
}

#[no_mangle]
pub unsafe extern "system" fn Java_dev_binclub_jvm4j_JVM4J_monitorWait(
	env: *mut JNIEnv, _this: jobject,
	obj: jobject, ms: jlong
) {
	JVM_MonitorWait(env, obj, ms)
}

#[no_mangle]
pub unsafe extern "system" fn Java_dev_binclub_jvm4j_JVM4J_monitorNotify(
	env: *mut JNIEnv, _this: jobject,
	obj: jobject
) {
	JVM_MonitorNotify(env, obj)
}

#[no_mangle]
pub unsafe extern "system" fn Java_dev_binclub_jvm4j_JVM4J_monitorNotifyAll(
	env: *mut JNIEnv, _this: jobject,
	obj: jobject
) {
	JVM_MonitorNotifyAll(env, obj)
}

#[no_mangle]
pub unsafe extern "system" fn Java_dev_binclub_jvm4j_JVM4J_clone(
	env: *mut JNIEnv, _this: jobject,
	obj: jobject
) -> jobject {
	JVM_Clone(env, obj)
}

#[no_mangle]
pub unsafe extern "system" fn Java_dev_binclub_jvm4j_JVM4J_internString(
	env: *mut JNIEnv, _this: jobject,
	obj: jstring
) -> jstring {
	JVM_InternString(env, obj)
}

#[no_mangle]
pub unsafe extern "system" fn Java_dev_binclub_jvm4j_JVM4J_currentTimeMillis(
	env: *mut JNIEnv, _this: jobject,
	ignored: jclass
) -> jlong {
	JVM_CurrentTimeMillis(env, ignored)
}

#[no_mangle]
pub unsafe extern "system" fn Java_dev_binclub_jvm4j_JVM4J_nanoTime(
	env: *mut JNIEnv, _this: jobject,
	ignored: jclass
) -> jlong {
	JVM_NanoTime(env, ignored)
}

#[no_mangle]
pub unsafe extern "system" fn Java_dev_binclub_jvm4j_JVM4J_arrayCopy(
	env: *mut JNIEnv, _this: jobject,
	ignored: jclass, src: jobject, src_pos: jint, dst: jobject, dst_pos: jint, length: jint,
) {
	JVM_ArrayCopy(env, ignored, src, src_pos, dst, dst_pos, length)
}

#[no_mangle]
pub unsafe extern "system" fn Java_dev_binclub_jvm4j_JVM4J_initProperties(
	env: *mut JNIEnv, _this: jobject,
	p: jobject
) -> jobject {
	JVM_InitProperties(env, p)
}
