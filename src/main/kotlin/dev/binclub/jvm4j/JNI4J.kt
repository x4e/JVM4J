@file:Suppress("FunctionName", "unused")

package dev.binclub.jvm4j

import java.lang.reflect.Method
import java.nio.ByteBuffer

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
	@JvmStatic external fun `throw`(throwable: Throwable): Int
	@JvmStatic fun _throw(throwable: Throwable) = `throw`(throwable)
	@JvmStatic external fun throwNew(throwableClass: Class<*>, msg: String): Int
	@JvmStatic external fun exceptionOccurred(): Throwable?
	@JvmStatic external fun exceptionDescribe()
	@JvmStatic external fun exceptionClear()
	@JvmStatic external fun fatalError(msg: String)
	@JvmStatic external fun pushLocalFrame(capacity: Int): Int
	@JvmStatic external fun popLocalFrame(result: Any?): Any?
	@JvmStatic external fun isSameObject(obj1: Any?, obj2: Any?): Boolean
	@JvmStatic external fun ensureLocalCapacity(capacity: Int): Int
	@JvmStatic external fun <T: Any> allocObject(cls: Class<T>): T
	@JvmStatic external fun <T: Any> newObject(cls: Class<T>, constructor: Method, vararg params: Any): T // warning: arg boxing may cause problems
	@JvmStatic external fun getObjectClass(obj: Any): Class<*>
	@JvmStatic external fun isInstanceOf(obj: Any?, cls: Class<*>): Boolean
	@JvmStatic external fun newString(unicode: CharArray, length: Int = unicode.size): String
	@JvmStatic external fun getStringLength(str: String): Int
	@JvmStatic external fun getStringChars(str: String): CharArray
	@JvmStatic external fun getArrayLength(arr: Any): Int
	@JvmStatic external fun <T> newObjectArray(len: Int, clazz: Class<T>, init: Any?): Array<T>
	@JvmStatic external fun <T> getObjectArrayElement(arr: Array<T>?, index: Int): T
	@JvmStatic external fun <T> setObjectArrayElement(arr: Array<T>?, index: Int, element: T)
	// TODO: Primitive arrays
	// These two are interesting as you can override other classes native links with your own ;)
	@JvmStatic external fun registerNatives(clazz: Class<*>, methods: Long, nMethods: Int): Int // TODO: Class rather than raw pointer
	@JvmStatic external fun unregisterNatives(clazz: Class<*>): Int
	@JvmStatic external fun monitorEnter(obj: Any?): Int
	@JvmStatic external fun monitorExit(obj: Any?): Int
	@JvmStatic external fun exceptionCheck(): Boolean // this is probably useless from java code but whatever
	@JvmStatic external fun newDirectByteBuffer(addr: Long, capacity: Long): ByteBuffer
	@JvmStatic external fun getDirectBufferAddress(buffer: ByteBuffer): Long
	@JvmStatic external fun getDirectBufferCapacity(buffer: ByteBuffer): Long
}
