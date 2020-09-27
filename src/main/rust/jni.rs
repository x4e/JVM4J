use rs_jvm_bindings::jni::{jobject, JNIEnv, jint, jstring, jbyteArray, jsize, JNI_ABORT, jbyte, jclass, jboolean};

use std::os::raw::c_char;
use std::ptr::null_mut;

#[no_mangle]
pub unsafe extern "system" fn Java_dev_binclub_jvm4j_JNI4J_getVersion(
	env: *mut JNIEnv, _this: jobject
) -> jint {
	(**env).GetVersion.unwrap()(env)
}

#[no_mangle]
pub unsafe extern "system" fn Java_dev_binclub_jvm4j_JNI4J_defineClass(
	env: *mut JNIEnv, _this: jobject,
	name: jstring, loader: jobject, buf: jbyteArray, size: jint
) -> jclass {
	let cname: *const c_char = if name.is_null() {
		null_mut()
	} else {
		(**env).GetStringUTFChars.unwrap()(env, name, null_mut())
	};
	let buf_content: *mut jbyte = (**env).GetByteArrayElements.unwrap()(env, buf, null_mut());
	
	let class: jclass = (**env).DefineClass.unwrap()(env, cname, loader, buf_content, size);
	
	(**env).ReleaseByteArrayElements.unwrap()(env, buf, buf_content, JNI_ABORT as i32);
	if !name.is_null() {
		(**env).ReleaseStringUTFChars.unwrap()(env, name, cname);
	}
	
	class
}

#[no_mangle]
pub unsafe extern "system" fn Java_dev_binclub_jvm4j_JNI4J_findClass(
	env: *mut JNIEnv, _this: jobject,
	name: jstring
) -> jclass {
	let cname: *const c_char = if name.is_null() {
		null_mut()
	} else {
		(**env).GetStringUTFChars.unwrap()(env, name, null_mut())
	};
	let out = (**env).FindClass.unwrap()(env, cname);
	if !name.is_null() {
		(**env).ReleaseStringUTFChars.unwrap()(env, name, cname);
	}
	out
}

#[no_mangle]
pub unsafe extern "system" fn Java_dev_binclub_jvm4j_JNI4J_getSuperClass(
	env: *mut JNIEnv, _this: jobject,
	clazz: jclass
) -> jclass {
	(**env).GetSuperclass.unwrap()(env, clazz)
}

#[no_mangle]
pub unsafe extern "system" fn Java_dev_binclub_jvm4j_JNI4J_isAssignableFrom(
	env: *mut JNIEnv, _this: jobject,
	sub: jclass, sup: jclass
) -> jboolean {
	(**env).IsAssignableFrom.unwrap()(env, sub, sup)
}

#[no_mangle]
pub unsafe extern "system" fn Java_dev_binclub_jvm4j_JNI4J_(
	env: *mut JNIEnv, _this: jobject,

) {
}

/*
#[no_mangle]
pub unsafe extern "system" fn Java_dev_binclub_jvm4j_JNI4J_(
	env: *mut JNIEnv, _this: jobject,
	
) {
}
 */
