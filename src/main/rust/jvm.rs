use rs_jvm_bindings::jni::{JavaVM, JNI_VERSION_1_8, jlong, JNIEnv, jobject, JNI_OK, jint, jstring, jclass, jmethodID, jboolean, jdouble, jthrowable};
use rs_jvm_bindings::utils::*;
use rs_jvm_bindings::jvm::*;
use std::os::raw::c_void;
use std::borrow::BorrowMut;
use rs_jvm_bindings::jvmti::jthread;

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

#[no_mangle]
pub unsafe extern "system" fn Java_dev_binclub_jvm4j_JVM4J_onExit(
	env: *mut JNIEnv, _this: jobject,
	func: jobject
) {
	panic!("Not yet implemented");
}

#[no_mangle]
pub unsafe extern "system" fn Java_dev_binclub_jvm4j_JVM4J_exit(
	env: *mut JNIEnv, _this: jobject,
	code: jint
) {
	JVM_Exit(code);
}

#[no_mangle]
pub unsafe extern "system" fn Java_dev_binclub_jvm4j_JVM4J_halt(
	env: *mut JNIEnv, _this: jobject,
	code: jint
) {
	JVM_Halt(code);
}

#[no_mangle]
pub unsafe extern "system" fn Java_dev_binclub_jvm4j_JVM4J_gc(
	env: *mut JNIEnv, _this: jobject
) {
	JVM_GC()
}

#[no_mangle]
pub unsafe extern "system" fn Java_dev_binclub_jvm4j_JVM4J_maxObjectInspectionAge(
	env: *mut JNIEnv, _this: jobject
) -> jlong {
	JVM_MaxObjectInspectionAge()
}

#[no_mangle]
pub unsafe extern "system" fn Java_dev_binclub_jvm4j_JVM4J_traceInstructions(
	env: *mut JNIEnv, _this: jobject,
	on: jboolean
) {
	JVM_TraceInstructions(on)
}

#[no_mangle]
pub unsafe extern "system" fn Java_dev_binclub_jvm4j_JVM4J_traceMethodCalls(
	env: *mut JNIEnv, _this: jobject,
	on: jboolean
) {
	JVM_TraceMethodCalls(on)
}

#[no_mangle]
pub unsafe extern "system" fn Java_dev_binclub_jvm4j_JVM4J_totalMemory(
	env: *mut JNIEnv, _this: jobject
) -> jlong {
	JVM_TotalMemory()
}

#[no_mangle]
pub unsafe extern "system" fn Java_dev_binclub_jvm4j_JVM4J_freeMemory(
	env: *mut JNIEnv, _this: jobject
) -> jlong {
	JVM_FreeMemory()
}

#[no_mangle]
pub unsafe extern "system" fn Java_dev_binclub_jvm4j_JVM4J_maxMemory(
	env: *mut JNIEnv, _this: jobject
) -> jlong {
	JVM_MaxMemory()
}

#[no_mangle]
pub unsafe extern "system" fn Java_dev_binclub_jvm4j_JVM4J_activeProcessorCount(
	env: *mut JNIEnv, _this: jobject
) -> jint {
	JVM_ActiveProcessorCount()
}

#[no_mangle]
pub unsafe extern "system" fn Java_dev_binclub_jvm4j_JVM4J_loadLibrary(
	env: *mut JNIEnv, _this: jobject,
	name: jstring
) -> jlong {
	let mut is_copy: jboolean = 0;
	let cname = (**env).GetStringUTFChars.unwrap()(env, name, is_copy.borrow_mut());
	let cout = JVM_LoadLibrary(cname);
	(**env).ReleaseStringUTFChars.unwrap()(env, name, cname);
	cout as jlong
}

#[no_mangle]
pub unsafe extern "system" fn Java_dev_binclub_jvm4j_JVM4J_unloadLibrary(
	env: *mut JNIEnv, _this: jobject,
	handle: jlong
) {
	JVM_UnloadLibrary(handle as *mut c_void)
}

#[no_mangle]
pub unsafe extern "system" fn Java_dev_binclub_jvm4j_JVM4J_findLibraryEntry(
	env: *mut JNIEnv, _this: jobject,
	handle: jlong, name: jstring
) -> jlong {
	let mut is_copy: jboolean = 0;
	let cname = (**env).GetStringUTFChars.unwrap()(env, name, is_copy.borrow_mut());
	let cout = JVM_FindLibraryEntry(handle as *mut c_void, cname);
	(**env).ReleaseStringUTFChars.unwrap()(env, name, cname);
	cout as jlong
}

#[no_mangle]
pub unsafe extern "system" fn Java_dev_binclub_jvm4j_JVM4J_isSupportedJNIVersion(
	env: *mut JNIEnv, _this: jobject,
	version: jint
) -> jboolean {
	JVM_IsSupportedJNIVersion(version)
}

#[no_mangle]
pub unsafe extern "system" fn Java_dev_binclub_jvm4j_JVM4J_isNaN(
	env: *mut JNIEnv, _this: jobject,
	d: jdouble
) -> jboolean {
	JVM_IsNaN(d)
}

#[no_mangle]
pub unsafe extern "system" fn Java_dev_binclub_jvm4j_JVM4J_fillInStackTrace(
	env: *mut JNIEnv, _this: jobject,
	throwable: jthrowable
) {
	JVM_FillInStackTrace(env, throwable)
}

#[no_mangle]
pub unsafe extern "system" fn Java_dev_binclub_jvm4j_JVM4J_getStackTraceDepth(
	env: *mut JNIEnv, _this: jobject,
	throwable: jthrowable
) -> jint {
	JVM_GetStackTraceDepth(env, throwable)
}

#[no_mangle]
pub unsafe extern "system" fn Java_dev_binclub_jvm4j_JVM4J_getStackTraceElement(
	env: *mut JNIEnv, _this: jobject,
	throwable: jthrowable, index: jint
) -> jobject {
	JVM_GetStackTraceElement(env, throwable, index)
}

#[no_mangle]
pub unsafe extern "system" fn Java_dev_binclub_jvm4j_JVM4J_initializeCompiler(
	env: *mut JNIEnv, _this: jobject,
	compCls: jclass
) {
	JVM_InitializeCompiler(env, compCls)
}

#[no_mangle]
pub unsafe extern "system" fn Java_dev_binclub_jvm4j_JVM4J_isSilentCompiler(
	env: *mut JNIEnv, _this: jobject,
	compCls: jclass
) -> jboolean {
	JVM_IsSilentCompiler(env, compCls)
}

#[no_mangle]
pub unsafe extern "system" fn Java_dev_binclub_jvm4j_JVM4J_compileClass(
	env: *mut JNIEnv, _this: jobject,
	compCls: jclass, cls: jclass
) -> jboolean {
	JVM_CompileClass(env, compCls, cls)
}

#[no_mangle]
pub unsafe extern "system" fn Java_dev_binclub_jvm4j_JVM4J_compileClasses(
	env: *mut JNIEnv, _this: jobject,
	compCls: jclass, name: jstring
) -> jboolean {
	JVM_CompileClasses(env, compCls, name)
}

#[no_mangle]
pub unsafe extern "system" fn Java_dev_binclub_jvm4j_JVM4J_compilerCommand(
	env: *mut JNIEnv, _this: jobject,
	compCls: jclass, arg: jobject
) -> jobject {
	JVM_CompilerCommand(env, compCls, arg)
}

#[no_mangle]
pub unsafe extern "system" fn Java_dev_binclub_jvm4j_JVM4J_startThread(
	env: *mut JNIEnv, _this: jobject,
	thread: jobject
) {
	JVM_StartThread(env, thread)
}

#[no_mangle]
pub unsafe extern "system" fn Java_dev_binclub_jvm4j_JVM4J_stopThread(
	env: *mut JNIEnv, _this: jobject,
	thread: jobject, exception: jobject
) {
	JVM_StopThread(env, thread, exception)
}

#[no_mangle]
pub unsafe extern "system" fn Java_dev_binclub_jvm4j_JVM4J_isThreadAlive(
	env: *mut JNIEnv, _this: jobject,
	thread: jobject
) -> jboolean {
	JVM_IsThreadAlive(env, thread)
}

#[no_mangle]
pub unsafe extern "system" fn Java_dev_binclub_jvm4j_JVM4J_suspendThread(
	env: *mut JNIEnv, _this: jobject,
	thread: jobject
) {
	JVM_SuspendThread(env, thread)
}

#[no_mangle]
pub unsafe extern "system" fn Java_dev_binclub_jvm4j_JVM4J_resumeThread(
	env: *mut JNIEnv, _this: jobject,
	thread: jthread
) {
	JVM_ResumeThread(env, thread)
}

#[no_mangle]
pub unsafe extern "system" fn Java_dev_binclub_jvm4j_JVM4J_setThreadPriority(
	env: *mut JNIEnv, _this: jobject,
	thread: jthread, priority: jint
) {
	JVM_SetThreadPriority(env, thread, priority)
}

#[no_mangle]
pub unsafe extern "system" fn Java_dev_binclub_jvm4j_JVM4J_yield(
	env: *mut JNIEnv, _this: jobject,
	thread_class: jclass
) {
	JVM_Yield(env, thread_class)
}

#[no_mangle]
pub unsafe extern "system" fn Java_dev_binclub_jvm4j_JVM4J_sleep(
	env: *mut JNIEnv, _this: jobject,
	thread_class: jclass, ms: jlong
) {
	JVM_Sleep(env, thread_class, ms)
}

#[no_mangle]
pub unsafe extern "system" fn Java_dev_binclub_jvm4j_JVM4J_currentThread(
	env: *mut JNIEnv, _this: jobject,
	thread_class: jclass
) -> jobject {
	JVM_CurrentThread(env, thread_class)
}

#[no_mangle]
pub unsafe extern "system" fn Java_dev_binclub_jvm4j_JVM4J_countStackFrames(
	env: *mut JNIEnv, _this: jobject,
	thread: jobject
) -> jint {
	JVM_CountStackFrames(env, thread)
}

#[no_mangle]
pub unsafe extern "system" fn Java_dev_binclub_jvm4j_JVM4J_interrupt(
	env: *mut JNIEnv, _this: jobject,
	thread: jobject
) {
	JVM_Interrupt(env, thread)
}

#[no_mangle]
pub unsafe extern "system" fn Java_dev_binclub_jvm4j_JVM4J_isInterrupted(
	env: *mut JNIEnv, _this: jobject,
	thread: jobject, clear_interrupted: jboolean
) -> jboolean {
	JVM_IsInterrupted(env, thread, clear_interrupted)
}

#[no_mangle]
pub unsafe extern "system" fn Java_dev_binclub_jvm4j_JVM4J_(
	env: *mut JNIEnv, _this: jobject,
	throwable: jthrowable
) {
}

#[no_mangle]
pub unsafe extern "system" fn Java_dev_binclub_jvm4j_JVM4J_(
	env: *mut JNIEnv, _this: jobject,
	throwable: jthrowable
) {
}

#[no_mangle]
pub unsafe extern "system" fn Java_dev_binclub_jvm4j_JVM4J_(
	env: *mut JNIEnv, _this: jobject,
	throwable: jthrowable
) {
}

#[no_mangle]
pub unsafe extern "system" fn Java_dev_binclub_jvm4j_JVM4J_(
	env: *mut JNIEnv, _this: jobject,
	throwable: jthrowable
) {
}

#[no_mangle]
pub unsafe extern "system" fn Java_dev_binclub_jvm4j_JVM4J_(
	env: *mut JNIEnv, _this: jobject,
	throwable: jthrowable
) {
}
