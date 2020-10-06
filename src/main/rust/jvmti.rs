use rs_jvm_bindings::jni::{jobject, JNIEnv, jboolean, jint, jobjectArray, jclass, jmethodID, jlong};
use rs_jvm_bindings::jvmti::{jvmtiEnv, jthread, jvmtiError, jvmtiError_JVMTI_ERROR_NONE, jvmtiEventMode, jvmtiEvent};
use rs_jvm_bindings::utils::*;

use std::ptr::null_mut;
use std::borrow::BorrowMut;

static mut PANIC_ON_ERR: Option<bool> = None;
static mut RESULT_CLS: Option<jclass> = None;
static mut OK_SOME_RESULT_CONSTRUCTOR: Option<jmethodID> = None;
static mut OK_NONE_RESULT_CONSTRUCTOR: Option<jmethodID> = None;
static mut ERR_RESULT_CONSTRUCTOR: Option<jmethodID> = None;

/// asking for some boolean as I dont think its possible to do a Some(null)
unsafe fn create_result(env: *mut JNIEnv, some: bool, out: jobject, err: jvmtiError) -> jobject {
	let panic: bool = match PANIC_ON_ERR {
		Some(x) => x,
		None => {
			let res = std::env::var("JVMTI4J_PANIC_ON_ERR").map_or(false, |s| s.parse::<bool>().unwrap_or(false));
			PANIC_ON_ERR = Some(res);
			res
		}
	};
	
	if panic && err != jvmtiError_JVMTI_ERROR_NONE {
		panic!("Met JVMTI error {}", err);
	}
	
	let result_cls = match RESULT_CLS {
		Some(x) => x,
		None => {
			let mut clazz = (**env).FindClass.unwrap()(env, cstr!("dev/binclub/jvm4j/JvmtiResult"));
			assert_ne!(clazz, null_mut());
			clazz = (**env).NewGlobalRef.unwrap()(env, clazz);
			RESULT_CLS = Some(clazz);
			clazz
		}
	};
	
	if err == jvmtiError_JVMTI_ERROR_NONE {
		if some {
			let constructor: jmethodID = match OK_SOME_RESULT_CONSTRUCTOR {
				Some(x) => x,
				None => {
					let constructor = (**env).GetStaticMethodID.unwrap()(env, result_cls, cstr!("success"), cstr!("(Ljava/lang/Object;)Ldev/binclub/jvm4j/JvmtiResult;"));
					OK_SOME_RESULT_CONSTRUCTOR = Some(constructor);
					constructor
				}
			};
			(**env).CallStaticObjectMethod.unwrap()(env, result_cls, constructor, out)
		} else {
			let constructor: jmethodID = match OK_NONE_RESULT_CONSTRUCTOR {
				Some(x) => x,
				None => {
					let constructor = (**env).GetStaticMethodID.unwrap()(env, result_cls, cstr!("emptySuccess"), cstr!("()Ldev/binclub/jvm4j/JvmtiResult;"));
					OK_NONE_RESULT_CONSTRUCTOR = Some(constructor);
					constructor
				}
			};
			(**env).CallStaticObjectMethod.unwrap()(env, result_cls, constructor)
		}
	} else {
		let constructor: jmethodID = match ERR_RESULT_CONSTRUCTOR {
			Some(x) => x,
			None => {
				let constructor = (**env).GetStaticMethodID.unwrap()(env, result_cls, cstr!("error"), cstr!("(I)Ldev/binclub/jvm4j/JvmtiResult;"));
				ERR_RESULT_CONSTRUCTOR = Some(constructor);
				constructor
			}
		};
		(**env).CallStaticObjectMethod.unwrap()(env, result_cls, constructor, err as jint)
	}
}

unsafe fn empty_result(env: *mut JNIEnv, err: jvmtiError) -> jobject {
	create_result(env, false, null_mut(), err)
}

unsafe fn some_result(env: *mut JNIEnv, out: jobject, err: jvmtiError) -> jobject {
	create_result(env, true, out, err)
}

#[no_mangle]
pub unsafe extern "system" fn Java_dev_binclub_jvm4j_Jvmti_setEventNotificationMode(
	env: *mut JNIEnv, _this: jobject,
	enabled: jboolean, event: jint, thread: jthread,
	jvmti: *mut jvmtiEnv
) -> jobject {
	match (**jvmti).SetEventNotificationMode {
		Some(meth) => empty_result(env, meth(jvmti, enabled as jvmtiEventMode, event as jvmtiEvent, thread)),
		_ => create_result(env, false, null_mut(), 0x7ffffffe)
	}
}

#[no_mangle]
pub unsafe extern "system" fn Java_dev_binclub_jvm4j_Jvmti_getAllThreads(
	env: *mut JNIEnv, _this: jobject,
	jvmti: *mut jvmtiEnv
) -> jobjectArray {
	match (**jvmti).GetAllThreads {
		Some(meth) => {
			let mut num_threads: jint = 0;
			let mut threads: *mut jthread = null_mut();
			let result = meth(jvmti, num_threads.borrow_mut(), threads.borrow_mut());
			
			if result != jvmtiError_JVMTI_ERROR_NONE {
				return empty_result(env, result)
			}
			
			let thread_class: jclass = (**env).FindClass.unwrap()(env, cstr!("java/lang/Thread"));
			let arr: jobjectArray = (**env).NewObjectArray.unwrap()(env, num_threads, thread_class, null_mut());
			for i in 0..num_threads {
				let thread: *mut jthread = threads.offset(i as isize);
				(**env).SetObjectArrayElement.unwrap()(env, arr, i, *thread);
			}
			
			some_result(env, arr, result)
		},
		_ => create_result(env, false, null_mut(), 0x7fffffff-1)
	}
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
