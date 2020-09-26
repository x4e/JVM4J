@file:Suppress("FunctionName", "unused")

package dev.binclub.jvm4j

/**
 * @author cookiedragon234 26/Sep/2020
 */
object JVM4J {
	@JvmStatic
	fun getVM(): JavaVM {
		val p = _getVM()
		return JavaVM(p)
	}
	
	@JvmStatic
	fun getJvmti(vm: JavaVM): Jvmti {
		val p = _getJvmti(vm.javaVM_P)
		return Jvmti(p)
	}
	
	@JvmStatic
	fun getJmm(vm: JavaVM): Jmm {
		val p = _getJmm(vm.javaVM_P)
		return Jmm(p)
	}
	
	private external fun _getVM(): Long
	private external fun _getJvmti(vm: Long): Long
	private external fun _getJmm(vm: Long): Long
	
	external fun getInterfaceVersion(): Int
	external fun iHashCode(obj: Any?): Int
	external fun monitorWait(obj: Any?, ms: Long)
	external fun monitorNotify(obj: Any?)
	external fun monitorNotifyAll(obj: Any?)
	external fun clone(obj: Any?): Any?
	external fun internString(str: String?): String?
	@JvmOverloads
	external fun currentTimeMillis(ignored: Class<*>? = null): Long
	@JvmOverloads
	external fun nanoTime(ignored: Class<*>? = null): Long
	@JvmOverloads
	external fun arrayCopy(ignored: Class<*>? = null, src: Any?, src_pos: Int, dst: Any?, dst_pos: Int, length: Int)
	external fun initProperties(p: Any?): Any?
	external fun onExit(op: Runnable) // TODO: Not yet implemented
	external fun exit(code: Int)
	external fun halt(code: Int)
	external fun gc()
	external fun maxObjectInspectionAge(): Long
	external fun traceInstructions(on: Boolean)
	external fun traceMethodCalls(on: Boolean)
	external fun totalMemory(): Long
	external fun freeMemory(): Long
	external fun maxMemory(): Long
	external fun activeProcessorCount(): Int
	external fun loadLibrary(name: String): Long
	external fun unloadLibrary(handle: Long)
	external fun findLibraryEntry(handle: Long, name: String): Long
	external fun isSupportedJNIVersion(version: Int): Boolean
	external fun isNaN(d: Double): Boolean
	external fun fillInStackTrace(throwable: Throwable)
	external fun getStackTraceDepth(throwable: Throwable): Int
	external fun getStackTraceElement(throwable: Throwable, index: Int): Any?
	external fun initializeCompiler(compCls: Class<*>)
	external fun isSilentCompiler(compCls: Class<*>): Boolean
	external fun compileClass(compCls: Class<*>, cls: Class<*>): Boolean
	external fun compileClasses(compCls: Class<*>, name: String): Boolean
	external fun compilerCommand(compCls: Class<*>, arg: Any?): Any
	external fun startThread(thread: Thread)
	external fun stopThread(thread: Thread, exception: Any?)
	external fun isThreadAlive(thread: Thread): Boolean
	external fun suspendThread(thread: Thread)
	external fun resumeThread(thread: Thread)
	external fun setThreadPriority(thread: Thread, priority: Int)
	external fun yield(threadClass: Class<*>?)
	external fun sleep(threadClass: Class<*>?, ms: Long)
	external fun currentThread(threadClass: Class<*>?): Thread?
	external fun countStackFrames(thread: Thread): Int
	external fun interrupt(thread: Thread)
	external fun isInterrupted(thread: Thread, clearInterrupted: Boolean): Boolean
}
