@file:Suppress("FunctionName", "unused")

package dev.binclub.jvm4j

/**
 * @author cookiedragon234 26/Sep/2020
 */
object JVM4J {
	private val loader = Loader // initialize this to load native library
	
	@JvmStatic fun getVM(): JavaVM {
		val p = getVM0()
		if (p == 0L) {
			throw NullPointerException("JavaVM was null")
		}
		return JavaVM(p)
	}
	
	@JvmStatic fun getJvmti(vm: JavaVM): Jvmti {
		val p = getJvmti0(vm.javaVM_P)
		if (p == 0L) {
			throw NullPointerException("Jvmti not supported")
		}
		return Jvmti(p)
	}
	
	@JvmStatic fun getJmm(): Jmm {
		val p = getJmm0()
		if (p == 0L) {
			throw NullPointerException("Jmm not supported")
		}
		return Jmm(p)
	}
	
	private external fun getVM0(): Long
	private external fun getJvmti0(vm: Long): Long
	private external fun getJmm0(): Long
	
	@JvmStatic external fun getInterfaceVersion(): Int
	@JvmStatic external fun iHashCode(obj: Any?): Int
	@JvmStatic external fun monitorWait(obj: Any?, ms: Long)
	@JvmStatic external fun monitorNotify(obj: Any?)
	@JvmStatic external fun monitorNotifyAll(obj: Any?)
	@JvmStatic external fun clone(obj: Any?): Any?
	@JvmStatic external fun internString(str: String?): String?
	@JvmOverloads @JvmStatic external fun currentTimeMillis(ignored: Class<*>? = null): Long
	@JvmOverloads @JvmStatic external fun nanoTime(ignored: Class<*>? = null): Long
	@JvmOverloads @JvmStatic external fun arrayCopy(ignored: Class<*>? = null, src: Any?, src_pos: Int, dst: Any?, dst_pos: Int, length: Int)
	@JvmStatic external fun initProperties(p: Any?): Any?
	@JvmStatic external fun onExit(op: Runnable) // TODO: Not yet implemented
	@JvmStatic external fun exit(code: Int)
	@JvmStatic external fun halt(code: Int)
	@JvmStatic external fun gc()
	@JvmStatic external fun maxObjectInspectionAge(): Long
	@JvmStatic external fun traceInstructions(on: Boolean)
	@JvmStatic external fun traceMethodCalls(on: Boolean)
	@JvmStatic external fun totalMemory(): Long
	@JvmStatic external fun freeMemory(): Long
	@JvmStatic external fun maxMemory(): Long
	@JvmStatic external fun activeProcessorCount(): Int
	@JvmStatic external fun loadLibrary(name: String): Long
	@JvmStatic external fun unloadLibrary(handle: Long)
	@JvmStatic external fun findLibraryEntry(handle: Long, name: String): Long
	@JvmStatic external fun isSupportedJNIVersion(version: Int): Boolean
	@JvmStatic external fun isNaN(d: Double): Boolean
	@JvmStatic external fun fillInStackTrace(throwable: Throwable)
	@JvmStatic external fun getStackTraceDepth(throwable: Throwable): Int
	@JvmStatic external fun getStackTraceElement(throwable: Throwable, index: Int): Any?
	@JvmStatic external fun initializeCompiler(compCls: Class<*>?)
	@JvmStatic external fun isSilentCompiler(compCls: Class<*>?): Boolean
	@JvmStatic external fun compileClass(compCls: Class<*>?, cls: Class<*>): Boolean
	@JvmStatic external fun compileClasses(compCls: Class<*>?, name: String): Boolean
	@JvmStatic external fun compilerCommand(compCls: Class<*>?, arg: Any?): Any
	@JvmStatic external fun startThread(thread: Thread)
	@JvmStatic external fun stopThread(thread: Thread, exception: Any?)
	@JvmStatic external fun isThreadAlive(thread: Thread): Boolean
	@JvmStatic external fun suspendThread(thread: Thread)
	@JvmStatic external fun resumeThread(thread: Thread)
	@JvmStatic external fun setThreadPriority(thread: Thread, priority: Int)
	@JvmStatic external fun yield(threadClass: Class<*>?)
	@JvmStatic external fun sleep(threadClass: Class<*>?, ms: Long)
	@JvmStatic external fun currentThread(threadClass: Class<*>?): Thread?
	@JvmStatic external fun countStackFrames(thread: Thread): Int
	@JvmStatic external fun interrupt(thread: Thread)
	@JvmStatic external fun isInterrupted(thread: Thread, clearInterrupted: Boolean): Boolean
	@JvmStatic external fun holdsLock(threadClass: Class<*>?, obj: Any): Boolean
	@JvmOverloads @JvmStatic external fun dumpAllStacks(unused: Class<*>? = null)
	@JvmOverloads @JvmStatic external fun getAllThreads(dummy: Class<*>? = null): Array<Thread>
	@JvmStatic external fun setNativeThreadName(thread: Any, name: String)
	@JvmStatic external fun dumpThreads(threadClass: Class<*>?, threads: Array<Thread>): Array<Thread>
	@JvmStatic external fun currentLoadedClass(): Class<*>
	@JvmStatic external fun currentClassLoader(): ClassLoader?
	@JvmStatic external fun getClassContext(): Array<Any?>
	@JvmStatic external fun classDepth(name: String): Int
	@JvmStatic external fun classLoaderDepth(): Int
	@JvmStatic external fun getSystemPackages(): Array<Any?>
	@JvmStatic external fun allocateNewObject(obj: Any?, currClass: Class<*>, initClass: Class<*>): Any
	@JvmStatic external fun allocateNewArray(obj: Any?, currClass: Class<*>, length: Int): Any
	@JvmStatic external fun latestUserDefinedLoader(): ClassLoader?
	@JvmStatic external fun loadClass0(obj: Any?, currClass: Class<*>, currClassName: String): Class<*>
	@JvmStatic external fun getArrayLength(arr: Any): Int
	@JvmStatic external fun getArrayElement(arr: Any, index: Int): Any
	// TODO: carry on
}
