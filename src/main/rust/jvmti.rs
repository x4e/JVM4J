use rs_jvm_bindings::jni::{jobject, JNIEnv, jlong, jboolean, jint, jobjectArray, jclass, jmethodID};
use rs_jvm_bindings::jvmti::{jvmtiEnv, jthread, jvmtiError, jvmtiError_JVMTI_ERROR_NONE, jvmtiEventMode, jvmtiEvent};
use rs_jvm_bindings::utils::*;

use std::ptr::null_mut;

static mut RESULT_CLS: Option<jclass> = None;

/// asking for some boolean as I dont think its possible to do a Some(null)
unsafe fn create_result(env: *mut JNIEnv, some: bool, out: jobject, err: jvmtiError) -> jobject {
	if RESULT_CLS.is_none() {
		RESULT_CLS = Some((**env).FindClass.unwrap()(env, cstr!("dev/binclub/jvm4j/JvmtiResult")))
	}
	let result_cls: jclass = RESULT_CLS.unwrap();
	
	if err == jvmtiError_JVMTI_ERROR_NONE {
		let constructor: jmethodID = (**env).GetStaticMethodID.unwrap()(env, result_cls, cstr!("success"), cstr!("(Ljava/lang/Object;)Ldev/binclub/jvm4j/JvmtiResult;"));
		(**env).CallStaticObjectMethod.unwrap()(env, result_cls, constructor, out)
	} else {
		let constructor: jmethodID = (**env).GetStaticMethodID.unwrap()(env, result_cls, cstr!("error"), cstr!("(I)Ldev/binclub/jvm4j/JvmtiResult;"));
		(**env).CallStaticObjectMethod.unwrap()(env, result_cls, constructor, err as jint)
	}
}

#[no_mangle]
pub unsafe extern "system" fn Java_dev_binclub_jvm4j_Jvmti_setEventNotificationMode(
	env: *mut JNIEnv, _this: jobject,
	enabled: jboolean, event: jint, thread: jthread,
	jvmti: *mut jvmtiEnv
) -> jobject {
	(**jvmti).SetEventNotificationMode.unwrap()(jvmti, enabled as jvmtiEventMode, event as jvmtiEvent, thread)
}

#[no_mangle]
pub unsafe extern "system" fn Java_dev_binclub_jvm4j_Jvmti_getAllThreads(
	env: *mut JNIEnv, _this: jobject,
	jvmti: *mut jvmtiEnv
) -> jobjectArray {
	let mut num_threads: jint = 0;
	let mut threads: *mut jthread = null_mut();
	
}

#[no_mangle]
pub unsafe extern "system" fn Java_dev_binclub_jvm4j_Jvmti_(
	env: *mut JNIEnv, _this: jobject,
	
	jvmti: *mut jvmtiEnv
) {
}

/*

#[no_mangle]
pub unsafe extern "system" fn Java_dev_binclub_jvm4j_Jvmti_(
	env: *mut JNIEnv, _this: jobject,
	
	jvmti: *mut jvmtiEnv
) {
}
*/
