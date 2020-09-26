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
}
