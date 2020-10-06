use rs_jvm_bindings::jni::{jobject, JNIEnv, jint, jstring, jbyteArray, jsize, JNI_ABORT, jbyte, jclass, jboolean, jthrowable, jobjectArray, jvalue, jcharArray, jchar, jlong, JNINativeMethod};

use std::os::raw::{c_char, c_void};
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
pub unsafe extern "system" fn Java_dev_binclub_jvm4j_JNI4J_throw(
	env: *mut JNIEnv, _this: jobject,
	throwable: jthrowable
) -> jint {
	(**env).Throw.unwrap()(env, throwable)
}

#[no_mangle]
pub unsafe extern "system" fn Java_dev_binclub_jvm4j_JNI4J_throwNew(
	env: *mut JNIEnv, _this: jobject,
	throw_cls: jclass, msg: jstring
) -> jint {
	let cmsg: *const c_char = (**env).GetStringUTFChars.unwrap()(env, msg, null_mut());
	let out = (**env).ThrowNew.unwrap()(env, throw_cls, cmsg);
	(**env).ReleaseStringUTFChars.unwrap()(env, msg, cmsg);
	out
}

#[no_mangle]
pub unsafe extern "system" fn Java_dev_binclub_jvm4j_JNI4J_exceptionOccurred(
	env: *mut JNIEnv, _this: jobject
) -> jthrowable {
	(**env).ExceptionOccurred.unwrap()(env)
}

#[no_mangle]
pub unsafe extern "system" fn Java_dev_binclub_jvm4j_JNI4J_exceptionDescribe(
	env: *mut JNIEnv, _this: jobject
) {
	(**env).ExceptionDescribe.unwrap()(env)
}

#[no_mangle]
pub unsafe extern "system" fn Java_dev_binclub_jvm4j_JNI4J_exceptionClear(
	env: *mut JNIEnv, _this: jobject
) {
	(**env).ExceptionClear.unwrap()(env)
}

#[no_mangle]
pub unsafe extern "system" fn Java_dev_binclub_jvm4j_JNI4J_fatalError(
	env: *mut JNIEnv, _this: jobject,
	msg: jstring
) {
	let cmsg: *const c_char = (**env).GetStringUTFChars.unwrap()(env, msg, null_mut());
	(**env).FatalError.unwrap()(env, cmsg);
	(**env).ReleaseStringUTFChars.unwrap()(env, msg, cmsg); // We probably shouldnt be here, but whatever
}

#[no_mangle]
pub unsafe extern "system" fn Java_dev_binclub_jvm4j_JNI4J_pushLocalFrame(
	env: *mut JNIEnv, _this: jobject,
	capacity: jint
) -> jint {
	(**env).PushLocalFrame.unwrap()(env, capacity)
}

#[no_mangle]
pub unsafe extern "system" fn Java_dev_binclub_jvm4j_JNI4J_popLocalFrame(
	env: *mut JNIEnv, _this: jobject,
	obj: jobject
) -> jobject {
	(**env).PopLocalFrame.unwrap()(env, obj)
}

#[no_mangle]
pub unsafe extern "system" fn Java_dev_binclub_jvm4j_JNI4J_isSameObject(
	env: *mut JNIEnv, _this: jobject,
	obj1: jobject, obj2: jobject
) -> jboolean {
	(**env).IsSameObject.unwrap()(env, obj1, obj2)
}

#[no_mangle]
pub unsafe extern "system" fn Java_dev_binclub_jvm4j_JNI4J_ensureLocalCapacity(
	env: *mut JNIEnv, _this: jobject,
	capacity: jint
) -> jint {
	(**env).EnsureLocalCapacity.unwrap()(env, capacity)
}

#[no_mangle]
pub unsafe extern "system" fn Java_dev_binclub_jvm4j_JNI4J_allocObject(
	env: *mut JNIEnv, _this: jobject,
	cls: jclass
) -> jobject {
	(**env).AllocObject.unwrap()(env, cls)
}

#[no_mangle]
pub unsafe extern "system" fn Java_dev_binclub_jvm4j_JNI4J_newObject(
	env: *mut JNIEnv, _this: jobject,
	cls: jclass, constructor: jobject, args: jobjectArray
) -> jobject {
	let meth_id = (**env).FromReflectedMethod.unwrap()(env, constructor);
	let num_args: jsize = (**env).GetArrayLength.unwrap()(env, args);
	let mut args_content: Vec<jvalue> = vec![jvalue{l: null_mut()}; num_args as usize];
	for i in 0..num_args {
		let arg: jobject = (**env).GetObjectArrayElement.unwrap()(env, args, i);
		args_content[i as usize] = jvalue { l: arg };
	}
	(**env).NewObjectA.unwrap()(env, cls, meth_id, args_content.as_ptr())
}

#[no_mangle]
pub unsafe extern "system" fn Java_dev_binclub_jvm4j_JNI4J_getObjectClass(
	env: *mut JNIEnv, _this: jobject,
	obj: jobject
) -> jclass {
	(**env).GetObjectClass.unwrap()(env, obj)
}

#[no_mangle]
pub unsafe extern "system" fn Java_dev_binclub_jvm4j_JNI4J_isInstanceOf(
	env: *mut JNIEnv, _this: jobject,
	obj: jclass, cls: jclass
) -> jboolean {
	(**env).IsInstanceOf.unwrap()(env, obj, cls)
}

#[no_mangle]
pub unsafe extern "system" fn Java_dev_binclub_jvm4j_JNI4J_newString(
	env: *mut JNIEnv, _this: jobject,
	unicode: jcharArray, len: jint
) -> jstring {
	let elements = (**env).GetCharArrayElements.unwrap()(env, unicode, null_mut());
	let out: jstring = (**env).NewString.unwrap()(env, elements, len);
	(**env).ReleaseCharArrayElements.unwrap()(env, unicode, elements, JNI_ABORT as i32);
	out
}

#[no_mangle]
pub unsafe extern "system" fn Java_dev_binclub_jvm4j_JNI4J_getStringLength(
	env: *mut JNIEnv, _this: jobject,
	string: jstring
) -> jint {
	(**env).GetStringLength.unwrap()(env, string)
}

#[no_mangle]
pub unsafe extern "system" fn Java_dev_binclub_jvm4j_JNI4J_getStringChars(
	env: *mut JNIEnv, _this: jobject,
	string: jstring
) -> jcharArray {
	let chars: *const jchar = (**env).GetStringChars.unwrap()(env, string, null_mut());
	let size: jsize = (**env).GetStringLength.unwrap()(env, string);
	let carr: jcharArray = (**env).NewCharArray.unwrap()(env, size);
	(**env).SetCharArrayRegion.unwrap()(env, carr, 0, size, chars);
	(**env).ReleaseStringChars.unwrap()(env, string, chars);
	carr
}

#[no_mangle]
pub unsafe extern "system" fn Java_dev_binclub_jvm4j_JNI4J_getArrayLength(
	env: *mut JNIEnv, _this: jobject,
	arr: jobject
) -> jsize {
	(**env).GetArrayLength.unwrap()(env, arr)
}

#[no_mangle]
pub unsafe extern "system" fn Java_dev_binclub_jvm4j_JNI4J_newObjectArray(
	env: *mut JNIEnv, _this: jobject,
	len: jint, clazz: jclass, init: jobject
) -> jobjectArray {
	(**env).NewObjectArray.unwrap()(env, len, clazz, init)
}

#[no_mangle]
pub unsafe extern "system" fn Java_dev_binclub_jvm4j_JNI4J_getObjectArrayElement(
	env: *mut JNIEnv, _this: jobject,
	arr: jobjectArray, index: jint
) -> jobject {
	(**env).GetObjectArrayElement.unwrap()(env, arr, index)
}

#[no_mangle]
pub unsafe extern "system" fn Java_dev_binclub_jvm4j_JNI4J_setObjectArrayElement(
	env: *mut JNIEnv, _this: jobject,
	arr: jobjectArray, index: jint, element: jobject
) {
	(**env).SetObjectArrayElement.unwrap()(env, arr, index, element)
}

#[no_mangle]
pub unsafe extern "system" fn Java_dev_binclub_jvm4j_JNI4J_registerNatives(
	env: *mut JNIEnv, _this: jobject,
	cls: jclass, methods: jlong, n_methods: jint
) -> jint {
	let meth_arr = methods as *const JNINativeMethod;
	(**env).RegisterNatives.unwrap()(env, cls, meth_arr, n_methods)
}

#[no_mangle]
pub unsafe extern "system" fn Java_dev_binclub_jvm4j_JNI4J_unregisterNatives(
	env: *mut JNIEnv, _this: jobject,
	cls: jclass
) -> jint {
	(**env).UnregisterNatives.unwrap()(env, cls)
}

#[no_mangle]
pub unsafe extern "system" fn Java_dev_binclub_jvm4j_JNI4J_monitorEnter(
	env: *mut JNIEnv, _this: jobject,
	obj: jobject
) -> jint {
	(**env).MonitorEnter.unwrap()(env, obj)
}

#[no_mangle]
pub unsafe extern "system" fn Java_dev_binclub_jvm4j_JNI4J_monitorExit(
	env: *mut JNIEnv, _this: jobject,
	obj: jobject
) -> jint {
	(**env).MonitorExit.unwrap()(env, obj)
}

#[no_mangle]
pub unsafe extern "system" fn Java_dev_binclub_jvm4j_JNI4J_exceptionCheck(
	env: *mut JNIEnv, _this: jobject
) -> jboolean {
	(**env).ExceptionCheck.unwrap()(env)
}

#[no_mangle]
pub unsafe extern "system" fn Java_dev_binclub_jvm4j_JNI4J_newDirectByteBuffer(
	env: *mut JNIEnv, _this: jobject,
	addr: jlong, capacity: jlong
) -> jobject {
	let addr_p = addr as *mut c_void;
	(**env).NewDirectByteBuffer.unwrap()(env, addr_p, capacity)
}

#[no_mangle]
pub unsafe extern "system" fn Java_dev_binclub_jvm4j_JNI4J_getDirectBufferAddress(
	env: *mut JNIEnv, _this: jobject,
	buff: jobject
) -> jlong {
	(**env).GetDirectBufferAddress.unwrap()(env, buff) as jlong
}

#[no_mangle]
pub unsafe extern "system" fn Java_dev_binclub_jvm4j_JNI4J_getDirectBufferCapacity(
	env: *mut JNIEnv, _this: jobject,
	buff: jobject
) -> jlong {
	(**env).GetDirectBufferCapacity.unwrap()(env, buff) as jlong
}

/*
#[no_mangle]
pub unsafe extern "system" fn Java_dev_binclub_jvm4j_JNI4J_(
	env: *mut JNIEnv, _this: jobject,
	
) {
}
 */
