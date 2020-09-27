package dev.binclub.jvm4j

/**
 * @author cookiedragon234 27/Sep/2020
 */
object JNI4J {
	private val loader = Loader // initialize this to load native library
	
	@JvmStatic external fun getVersion(): Int
	@JvmOverloads @JvmStatic external fun defineClass(name: String?, loader: ClassLoader?, buf: ByteArray, len: Int = buf.size): Class<*>?
	@JvmStatic external fun findClass(name: String?): Class<*>?
	@JvmStatic external fun getSuperClass(clazz: Class<*>): Class<*>?
	@JvmStatic external fun isAssignableFrom(sub: Class<*>, sup: Class<*>): Boolean
}
