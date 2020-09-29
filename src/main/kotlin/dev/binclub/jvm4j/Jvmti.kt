@file:Suppress("unused")

package dev.binclub.jvm4j

/**
 * @author cookiedragon234 26/Sep/2020
 */
class Jvmti internal constructor(private val jvmti_P: Long) {
	/// https://docs.oracle.com/javase/8/docs/platform/jvmti/jvmti.html#SetEventNotificationMode
	external fun setEventNotificationMode(enabled: Boolean, event: Int, thread: Thread? = null, p: Long = jvmti_P)
	external fun getAllThreads(): JvmtiResult<Array<Thread>>
}

class JvmtiResult <out T> private constructor(private val inner: Any?) {
	fun expect(): T = if (inner is JvmtiError) throw JvmtiException(inner) else inner as T
	fun expect(msg: String): T = if (inner is JvmtiError) throw JvmtiException(msg, inner) else inner as T
	fun unwrap() = expect()
	fun isErr(): Boolean = inner is JvmtiError
	
	companion object {
		@JvmStatic fun <T> success(inner: T): JvmtiResult<T> = JvmtiResult(inner)
		@JvmStatic fun <T> emptySuccess(): JvmtiResult<Unit> = success(Unit)
		@JvmStatic fun <T> error(error: Int): JvmtiResult<T> = JvmtiResult(JvmtiError.from(error))
	}
	
	private class JvmtiException: RuntimeException {
		constructor() : super()
		constructor(message: String?) : super(message)
		constructor(message: String?, cause: Throwable?) : super(message, cause)
		constructor(cause: Throwable?) : super(cause)
		constructor(
			message: String?,
			cause: Throwable?,
			enableSuppression: Boolean,
			writableStackTrace: Boolean
		) : super(message, cause, enableSuppression, writableStackTrace)
		constructor(jvmtiErr: JvmtiError): super("${jvmtiErr.name} (${jvmtiErr.code})")
		constructor(msg: String, jvmtiErr: JvmtiError): super("$msg: ${jvmtiErr.name} (${jvmtiErr.code})")
	}
	
	private enum class JvmtiError (val code: Int) {
		NONE(0),
		INVALID_THREAD(10),
		INVALID_THREAD_GROUP(11),
		INVALID_PRIORITY(12),
		THREAD_NOT_SUSPENDED(13),
		THREAD_SUSPENDED(14),
		THREAD_NOT_ALIVE(15),
		INVALID_OBJECT(20),
		INVALID_CLASS(21),
		CLASS_NOT_PREPARED(22),
		INVALID_METHODID(23),
		INVALID_LOCATION(24),
		INVALID_FIELDID(25),
		NO_MORE_FRAMES(31),
		OPAQUE_FRAME(32),
		TYPE_MISMATCH(34),
		INVALID_SLOT(35),
		DUPLICATE(40),
		NOT_FOUND(41),
		INVALID_MONITOR(50),
		NOT_MONITOR_OWNER(51),
		INTERRUPT(52),
		INVALID_CLASS_FORMAT(60),
		CIRCULAR_CLASS_DEFINITION(61),
		FAILS_VERIFICATION(62),
		UNSUPPORTED_REDEFINITION_METHOD_ADDED(63),
		UNSUPPORTED_REDEFINITION_SCHEMA_CHANGED(64),
		INVALID_TYPESTATE(65),
		UNSUPPORTED_REDEFINITION_HIERARCHY_CHANGED(66),
		UNSUPPORTED_REDEFINITION_METHOD_DELETED(67),
		UNSUPPORTED_VERSION(68),
		NAMES_DONT_MATCH(69),
		UNSUPPORTED_REDEFINITION_CLASS_MODIFIERS_CHANGED(70),
		UNSUPPORTED_REDEFINITION_METHOD_MODIFIERS_CHANGED(71),
		UNMODIFIABLE_CLASS(79),
		NOT_AVAILABLE(98),
		MUST_POSSESS_CAPABILITY(99),
		NULL_POINTER(100),
		ABSENT_INFORMATION(101),
		INVALID_EVENT_TYPE(102),
		ILLEGAL_ARGUMENT(103),
		NATIVE_METHOD(104),
		CLASS_LOADER_UNSUPPORTED(106),
		OUT_OF_MEMORY(110),
		ACCESS_DENIED(111),
		WRONG_PHASE(112),
		INTERNAL(113),
		UNATTACHED_THREAD(115),
		INVALID_ENVIRONMENT(116),
		MAX(116),
		OTHER(-1);
		
		companion object {
			private val values = values()
			private val valMap: Map<Int, JvmtiError> = HashMap<Int, JvmtiError>().apply {
				values.forEach {
					put(it.code, it)
				}
			}
			fun from(code: Int): JvmtiError = valMap[code] ?: OTHER
		}
	}
}

object JvmtiEvent {
	const val VM_INIT = 50
	const val VM_DEATH = 51
	const val THREAD_START = 52
	const val THREAD_END = 53
	const val CLASS_FILE_LOAD_HOOK = 54
	const val CLASS_LOAD = 55
	const val CLASS_PREPARE = 56
	const val VM_START = 57
	const val EXCEPTION = 58
	const val EXCEPTION_CATCH = 59
	const val SINGLE_STEP = 60
	const val FRAME_POP = 61
	const val BREAKPOINT = 62
	const val FIELD_ACCESS = 63
	const val FIELD_MODIFICATION = 64
	const val METHOD_ENTRY = 65
	const val METHOD_EXIT = 66
	const val NATIVE_METHOD_BIND = 67
	const val COMPILED_METHOD_LOAD = 68
	const val COMPILED_METHOD_UNLOAD = 69
	const val DYNAMIC_CODE_GENERATED = 70
	const val DATA_DUMP_REQUEST = 71
	const val MONITOR_WAIT = 73
	const val MONITOR_WAITED = 74
	const val MONITOR_CONTENDED_ENTER = 75
	const val MONITOR_CONTENDED_ENTERED = 76
	const val RESOURCE_EXHAUSTED = 80
	const val GARBAGE_COLLECTION_START = 81
	const val GARBAGE_COLLECTION_FINISH = 82
	const val OBJECT_FREE = 83
	const val VM_OBJECT_ALLOC = 84
}
