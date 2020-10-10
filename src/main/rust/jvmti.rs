#![allow(non_upper_case_globals)]

use rs_jvm_bindings::jni::{jobject, JNIEnv, jboolean, jint, jobjectArray, jclass, jmethodID, jstring, jvalue};
use rs_jvm_bindings::jvmti::{jvmtiEnv, jthread, jvmtiError, jvmtiError_JVMTI_ERROR_NONE, jvmtiEventMode, jvmtiEvent, jvmtiThreadInfo, jvmtiCapabilities};
use rs_jvm_bindings::utils::*;

use std::ptr::null_mut;
use std::borrow::BorrowMut;
use std::mem::zeroed;

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
			let klass = (**env).FindClass.unwrap()(env, cstr!("dev/binclub/jvm4j/JvmtiResult"));
			let klass = (**env).NewGlobalRef.unwrap()(env, klass);
			RESULT_CLS = Some(klass);
			klass
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

unsafe fn some_result(env: *mut JNIEnv, out: jobject) -> jobject {
	create_result(env, true, out, jvmtiError_JVMTI_ERROR_NONE)
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
			
			some_result(env, arr)
		},
		_ => create_result(env, false, null_mut(), 0x7fffffff-1)
	}
}

#[no_mangle]
pub unsafe extern "system" fn Java_dev_binclub_jvm4j_Jvmti_suspendThread(
	env: *mut JNIEnv, _this: jobject,
	thread: jthread,
	jvmti: *mut jvmtiEnv
) -> jobject {
	match (**jvmti).SuspendThread {
		Some(meth) => empty_result(env, meth(jvmti, thread)),
		_ => create_result(env, false, null_mut(), 0x7ffffffe)
	}
}

#[no_mangle]
pub unsafe extern "system" fn Java_dev_binclub_jvm4j_Jvmti_resumeThread(
	env: *mut JNIEnv, _this: jobject,
	thread: jthread,
	jvmti: *mut jvmtiEnv
) -> jthread {
	match (**jvmti).ResumeThread {
		Some(meth) => empty_result(env, meth(jvmti, thread)),
		_ => create_result(env, false, null_mut(), 0x7ffffffe)
	}
}

#[no_mangle]
pub unsafe extern "system" fn Java_dev_binclub_jvm4j_Jvmti_stopThread(
	env: *mut JNIEnv, _this: jobject,
	thread: jthread, exception: jobject,
	jvmti: *mut jvmtiEnv
) -> jobject {
	match (**jvmti).StopThread {
		Some(meth) => empty_result(env, meth(jvmti, thread, exception)),
		_ => create_result(env, false, null_mut(), 0x7ffffffe)
	}
}

#[no_mangle]
pub unsafe extern "system" fn Java_dev_binclub_jvm4j_Jvmti_interruptThread(
	env: *mut JNIEnv, _this: jobject,
	thread: jthread,
	jvmti: *mut jvmtiEnv
) -> jobject {
	match (**jvmti).InterruptThread {
		Some(meth) => empty_result(env, meth(jvmti, thread)),
		_ => create_result(env, false, null_mut(), 0x7ffffffe)
	}
}

static mut THREAD_INFO_CLASS: Option<jclass> = None;
static mut THREAD_INFO_CONSTRUCTOR: Option<jmethodID> = None;

#[no_mangle]
pub unsafe extern "system" fn Java_dev_binclub_jvm4j_Jvmti_getThreadInfo(
	env: *mut JNIEnv, _this: jobject,
	thread: jthread,
	jvmti: *mut jvmtiEnv
) -> jobject {
	match (**jvmti).GetThreadInfo {
		Some(meth) => {
			let mut info_c: jvmtiThreadInfo = zeroed();
			match meth(jvmti, thread, info_c.borrow_mut()) {
				jvmtiError_JVMTI_ERROR_NONE => {
					let thread_info_class: jclass = match THREAD_INFO_CLASS {
						Some(x) => x,
						None => {
							let klass = (**env).FindClass.unwrap()(env, cstr!("dev/binclub/jvm4j/ThreadInfo"));
							let klass = (**env).NewGlobalRef.unwrap()(env, klass);
							THREAD_INFO_CLASS = Some(klass);
							klass
						}
					};
					
					let constructor: jmethodID = match THREAD_INFO_CONSTRUCTOR {
						Some(x) => x,
						None => {
							let constructor = (**env).GetMethodID.unwrap()(env, thread_info_class, cstr!("<init>"), cstr!("(Ljava/lang/String;IZLjava/lang/ThreadGroup;Ljava/lang/ClassLoader;)V)"));
							THREAD_INFO_CONSTRUCTOR = Some(constructor);
							constructor
						}
					};
					
					let name: jstring = if info_c.name.is_null() {
						null_mut()
					} else {
						(**env).NewStringUTF.unwrap()(env, info_c.name)
					};
					
					let args = vec![
						jvalue { l: name },
						jvalue { i: info_c.priority },
						jvalue { z: info_c.is_daemon },
						jvalue { l: info_c.thread_group },
						jvalue { l: info_c.context_class_loader }
					];
					let out = (**env).NewObjectA.unwrap()(env, thread_info_class, constructor, args.as_ptr());
					
					some_result(env, out)
				}
				err => create_result(env, false, null_mut(), err)
			}
		},
		_ => create_result(env, false, null_mut(), 0x7ffffffe)
	}
}

static mut CAPABILITIES_CLASS: Option<jclass> = None;
static mut CAPABILITIES_CONSTRUCTOR: Option<jmethodID> = None;

unsafe fn capabilities_to_jobject(env: *mut JNIEnv, capabilities: &jvmtiCapabilities) -> jobject {
	let clazz: jclass = match CAPABILITIES_CLASS {
		Some(x) => x,
		None => {
			let clazz = (**env).FindClass.unwrap()(env, cstr!("dev/binclub/jvm4j/JvmtiCapabilities"));
			CAPABILITIES_CLASS = Some(clazz);
			clazz
		}
	};
	
	let constructor: jmethodID = match CAPABILITIES_CONSTRUCTOR {
		Some(x) => x,
		None => {
			let constructor: jmethodID = (**env).GetMethodID.unwrap()(env, clazz, cstr!("<init>"), cstr!("(ZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZ)V")); // thats a lot of zs
			CAPABILITIES_CONSTRUCTOR = Some(constructor);
			constructor
		}
	};
	
	let args: Vec<jvalue> = vec![
		jvalue { z: capabilities.can_tag_objects() as u8 },
		jvalue { z: capabilities.can_generate_field_modification_events() as u8 },
		jvalue { z: capabilities.can_generate_field_access_events() as u8 },
		jvalue { z: capabilities.can_get_bytecodes() as u8 },
		jvalue { z: capabilities.can_get_synthetic_attribute() as u8 },
		jvalue { z: capabilities.can_get_owned_monitor_info() as u8 },
		jvalue { z: capabilities.can_get_current_contended_monitor() as u8 },
		jvalue { z: capabilities.can_get_monitor_info() as u8 },
		jvalue { z: capabilities.can_pop_frame() as u8 },
		jvalue { z: capabilities.can_redefine_classes() as u8 },
		jvalue { z: capabilities.can_signal_thread() as u8 },
		jvalue { z: capabilities.can_get_source_file_name() as u8 },
		jvalue { z: capabilities.can_get_line_numbers() as u8 },
		jvalue { z: capabilities.can_get_source_debug_extension() as u8 },
		jvalue { z: capabilities.can_access_local_variables() as u8 },
		jvalue { z: capabilities.can_maintain_original_method_order() as u8 },
		jvalue { z: capabilities.can_generate_single_step_events() as u8 },
		jvalue { z: capabilities.can_generate_exception_events() as u8 },
		jvalue { z: capabilities.can_generate_frame_pop_events() as u8 },
		jvalue { z: capabilities.can_generate_breakpoint_events() as u8 },
		jvalue { z: capabilities.can_suspend() as u8 },
		jvalue { z: capabilities.can_redefine_any_class() as u8 },
		jvalue { z: capabilities.can_get_current_thread_cpu_time() as u8 },
		jvalue { z: capabilities.can_generate_method_entry_events() as u8 },
		jvalue { z: capabilities.can_generate_method_exit_events() as u8 },
		jvalue { z: capabilities.can_generate_all_class_hook_events() as u8 },
		jvalue { z: capabilities.can_generate_compiled_method_load_events() as u8 },
		jvalue { z: capabilities.can_generate_monitor_events() as u8 },
		jvalue { z: capabilities.can_generate_vm_object_alloc_events() as u8 },
		jvalue { z: capabilities.can_generate_native_method_bind_events() as u8 },
		jvalue { z: capabilities.can_generate_garbage_collection_events() as u8 },
		jvalue { z: capabilities.can_generate_object_free_events() as u8 },
		jvalue { z: capabilities.can_force_early_return() as u8 },
		jvalue { z: capabilities.can_get_owned_monitor_stack_depth_info() as u8 },
		jvalue { z: capabilities.can_get_constant_pool() as u8 },
		jvalue { z: capabilities.can_set_native_method_prefix() as u8 },
		jvalue { z: capabilities.can_retransform_classes() as u8 },
		jvalue { z: capabilities.can_retransform_any_class() as u8 },
		jvalue { z: capabilities.can_generate_resource_exhaustion_heap_events() as u8 },
		jvalue { z: capabilities.can_generate_resource_exhaustion_threads_events() as u8 }
	];
	(**env).NewObjectA.unwrap()(env, clazz, constructor, args.as_ptr())
}

#[no_mangle]
pub unsafe extern "system" fn Java_dev_binclub_jvm4j_Jvmti_getCapabilities(
	env: *mut JNIEnv, _this: jobject,
	jvmti: *mut jvmtiEnv
) -> jobject {
	let mut capabilities: jvmtiCapabilities = zeroed();
	let result = (**jvmti).GetCapabilities.unwrap()(jvmti, capabilities.borrow_mut());
	let caps = capabilities_to_jobject(env, &capabilities);
	create_result(env, true, caps, result)
}

/*

#[no_mangle]
pub unsafe extern "system" fn Java_dev_binclub_jvm4j_Jvmti_(
	env: *mut JNIEnv, _this: jobject,
	
	jvmti: *mut jvmtiEnv
) -> jobject {
}
*/
