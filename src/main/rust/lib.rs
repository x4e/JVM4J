mod jvmti;
mod jvm;
mod jni;

#[macro_use]
extern crate rs_jvm_bindings;

use rs_jvm_bindings::jni::{JavaVM, JNI_VERSION_1_8, jlong, JNIEnv, jobject, JNI_OK};
use rs_jvm_bindings::utils::*;

use std::os::raw::{c_int, c_void};
use std::ptr::null_mut;
use std::borrow::BorrowMut;
use rs_jvm_bindings::jvmti::{JVMTI_VERSION_1_2, jvmtiEnv};
use rs_jvm_bindings::jmm::{JMM_VERSION_1_2_2, JmmInterface};
use rs_jvm_bindings::jvm::JVM_GetManagement;

#[no_mangle]
pub unsafe extern "system" fn JNI_OnLoad(_vm: *mut JavaVM, _reserved: &mut c_void) -> c_int {
	JNI_VERSION_1_8 as i32
}

#[no_mangle]
pub unsafe extern "system" fn Java_dev_binclub_jvm4j_JVM4J_getVM0(
	env: *mut JNIEnv, _this: jobject,
) -> jlong {
	let mut vm: *mut JavaVM = null_mut();
	{
		let result = (**env).GetJavaVM.unwrap()(env, vm.borrow_mut());
		if result != JNI_OK as i32 || vm.is_null() {
			eprintln!("Couldn't fetch vm instance ({})", result);
			return 0;
		}
	}
	vm as jlong
}

#[no_mangle]
pub unsafe extern "system" fn Java_dev_binclub_jvm4j_JVM4J_getJvmti0(
	_env: *mut JNIEnv, _this: jobject,
	vm: jlong
) -> jlong {
	let mut vm: *mut JavaVM = vm as *mut JavaVM;
	let mut jvmti_ptr: *mut c_void = null_mut();
	{
		let result = (**vm).GetEnv.unwrap()(vm, jvmti_ptr.borrow_mut(), JVMTI_VERSION_1_2 as i32);
		if result != JNI_OK as i32 || jvmti_ptr.is_null() {
			eprintln!("Couldn't fetch jvmti instance ({})", result);
			return 0;
		}
	}
	let jvmti: *mut jvmtiEnv = jvmti_ptr as *mut jvmtiEnv;
	jvmti as jlong
}

#[no_mangle]
pub unsafe extern "system" fn Java_dev_binclub_jvm4j_JVM4J_getJmm0(
	_env: *mut JNIEnv, _this: jobject
) -> jlong {
	
	let mut jmm_ptr: *mut c_void = JVM_GetManagement(JMM_VERSION_1_2_2 as i32);
	if jmm_ptr.is_null() {
		return 0;
	}
	let jmm: *mut JmmInterface = jmm_ptr as *mut JmmInterface;
	jmm as jlong
}

#[no_mangle]
pub unsafe extern "system" fn Java_dev_binclub_jvm4j_JVM4J_objectAsPointer(
	_env: *mut JNIEnv, _this: jobject,
	obj: jobject
) -> jlong {
	obj as jlong
}

#[no_mangle]
pub unsafe extern "system" fn Java_dev_binclub_jvm4j_JVM4J_pointerAsObject(
	_env: *mut JNIEnv, _this: jobject,
	pointer: jlong
) -> jobject {
	pointer as jobject
}
