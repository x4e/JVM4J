@file:Suppress("unused")

package dev.binclub.jvm4j

/**
 * @author cookiedragon234 26/Sep/2020
 */
class Jvmti internal constructor(private val jvmti_P: Long) {
	/// Relevant capabilities should be activated before even attempting to call any of the below methods
	
	///  Returns the optional JVM TI features which this environment currently possesses.
	/// An environment does not possess a capability unless it has been successfully added with AddCapabilities.
	/// An environment only loses possession of a capability if it has been relinquished with RelinquishCapabilities.
	/// Thus, this function returns the net result of the AddCapabilities and RelinquishCapabilities calls which have been made.
	external fun getCapabilities(): JvmtiResult<JvmtiCapabilities>
	/// Returns the JVM TI features that can potentially be possessed by this environment at this time.
	/// The returned capabilities differ from the complete set of capabilities implemented by the VM in two cases:
	/// - another environment possesses capabilities that can only be possessed by one environment
	/// - the current phase is live, and certain capabilities can only be added during the OnLoad phase
	/// The AddCapabilities function may be used to set any or all or these capabilities. Currently possessed capabilities are included.
	external fun getPotentialCapabilities(): JvmtiResult<JvmtiCapabilities>
	/// Set new capabilities. All previous capabilities are retained. Typically this is done during jvm startup.
	/// Some virtual machines may allow a limited set of capabilities to be added in the live phase.
	external fun addCapabilities(capabilities: JvmtiCapabilities): JvmtiEmptyResult
	/// Relinquish the capabilities.
	/// Some implementations may allow only one environment to have a capability.
	/// This function releases capabilities so that they may be used by other agents. All other capabilities are retained.
	/// The capability will no longer be present in GetCapabilities.
	/// Attempting to relinquish a capability that the agent does not possess is not an error.
	external fun relinquishCapabilities(capabilities: JvmtiCapabilities): JvmtiEmptyResult
	
	
	/// https://docs.oracle.com/javase/8/docs/platform/jvmti/jvmti.html#SetEventNotificationMode
	@JvmOverloads external fun setEventNotificationMode(enabled: Boolean, event: Int, thread: Thread? = null, p: Long = jvmti_P): JvmtiEmptyResult
	external fun getAllThreads(): JvmtiResult<Array<Thread>>
	external fun suspendThread(thread: Thread): JvmtiEmptyResult
	external fun resumeThread(thread: Thread): JvmtiEmptyResult
	external fun stopThread(thread: Thread, exception: Any?): JvmtiEmptyResult
	external fun interruptThread(thread: Thread): JvmtiEmptyResult
	external fun getThreadInfo(thread: Thread): JvmtiResult<ThreadInfo>
}

data class JvmtiCapabilities constructor (
	var canTagObjects: Boolean = false, ///  Can set and get tags, used to debug heap
	var canGenerateFieldModificationEvents: Boolean = false, /// Can set watchpoints on field modification - SetFieldModificationWatch
	var canGenerateFieldAccessEvents: Boolean = false, /// Can set watchpoints on field access - SetFieldAccessWatch
	var canGetBytecodes: Boolean = false, /// Can get bytecodes of a method GetBytecodes
	var canGetSyntheticAttribute: Boolean = false, ///  Can test if a field or method is synthetic - IsFieldSynthetic and IsMethodSynthetic
	var canGetOwnedMonitorInfo: Boolean = false, /// Can get information about ownership of monitors - GetOwnedMonitorInfo
	var canGetCurrentContendedMonitor: Boolean = false, /// Can GetCurrentContendedMonitor
	var canGetMonitorInfo: Boolean = false, /// Can GetObjectMonitorUsage
	var canPopFrame: Boolean = false, /// Can pop frames off the stack - PopFrame
	var canRedefineClasses: Boolean = false, /// Can redefine classes with RedefineClasses.
	var canSignalThread: Boolean = false, /// Can send stop or interrupt to threads
	var canGetSourceFileName: Boolean = false, /// Can get the source file name of a class
	var canGetLineNumbers: Boolean = false, /// Can get the line number table of a method
	var canGetSourceDebugExtension: Boolean = false, /// Can get the source debug extension of a class
	var canAccessLocalVariables: Boolean = false, /// Can set and get local variables
	var canMaintainOriginalMethodOrder: Boolean = false, /// Can return methods in the order they occur in the class file
	var canGenerateSingleStepEvents: Boolean = false, /// Can get single step events
	var canGenerateExceptionEvents: Boolean = false, /// Can get exception thrown and exception catch events
	var canGenerateFramePopEvents: Boolean = false, /// Can set and thus get FramePop events
	var canGenerateBreakPointEvents: Boolean = false, /// Can set and thus get Breakpoint events
	var canSuspend: Boolean = false, /// Can suspend and resume threads
	var canRedefineAnyClass: Boolean = false, /// Can modify (retransform or redefine) any non-primitive non-array class. See IsModifiableClass.
	var canGetCurrentThreadCpuTime: Boolean = false, /// Can get current thread CPU time
	var canGetThreadCpuTime: Boolean = false, /// Can get thread CPU time
	var canGenerateMethodEntryEvents: Boolean = false, /// Can generate method entry events on entering a method
	var canGenerateMethodExitEvents: Boolean = false, /// Can generate method exit events on leaving a method
	var canGenerateAllClassHookEvents: Boolean = false, /// Can generate ClassFileLoadHook events for every loaded class.
	var canGenerateCompiledMethodLoadEvents: Boolean = false, /// Can generate events when a method is compiled or unloaded
	var canGenerateMonitorEvents: Boolean = false, /// Can generate events on monitor activity
	var canGenerateVmObjectAllocEvents: Boolean = false, /// Can generate events on VM allocation of an object
	var canGenerateNativeMethodBindEvents: Boolean = false, /// Can generate events when a native method is bound to its implementation
	var canGenerateGarbageCollectionEvents: Boolean = false, /// Can generate events when garbage collection begins or ends
	var canGenerateObjectFreeEvents: Boolean = false, /// Can generate events when the garbage collector frees an object
	var canForceEarlyReturn: Boolean = false, /// Can return early from a method, as described in the Force Early Return category.
	var canGetOwnedMonitorStackDepthInfo: Boolean = false, /// Can get information about owned monitors with stack depth - GetOwnedMonitorStackDepthInfo
	var canGetConstantPool: Boolean = false, /// Can get the constant pool of a class - GetConstantPool
	var canSetNativeMethodPrefix: Boolean = false, /// Can set prefix to be applied when native method cannot be resolved - SetNativeMethodPrefix and SetNativeMethodPrefixes
	/// Can retransform classes with RetransformClasses.
	// In addition to the restrictions imposed by the specific implementation on this capability (see the Capability section),
	/// this capability must be set before the ClassFileLoadHook event is enabled for the first time in this environment.
	/// An environment that possesses this capability at the time that ClassFileLoadHook is enabled for the first time is said to be retransformation capable.
	/// An environment that does not possess this capability at the time that ClassFileLoadHook is enabled for the first time is said to be retransformation incapable.
	var canRetransformClasses: Boolean = false,
	var canRetransformAnyClass: Boolean = false, /// RetransformClasses can be called on any class (can_retransform_classes must also be set)
	var canGenerateResourceExhaustionHeapEvents: Boolean = false, /// Can generate events when the VM is unable to allocate memory from the JavaTM platform heap. See ResourceExhausted.
	var canGenerateResourceExhaustionThreadsEvents: Boolean = false, /// Can generate events when the VM is unable to create a thread. See ResourceExhausted.
) {
	/// To make it slightly easier for rust interop
	fun toBooleanArray(): BooleanArray = booleanArrayOf(
		canTagObjects, canGenerateFieldModificationEvents, canGenerateFieldAccessEvents, canGetBytecodes, canGetSyntheticAttribute, canGetOwnedMonitorInfo, canGetCurrentContendedMonitor, canGetMonitorInfo, canPopFrame,
		canRedefineClasses, canSignalThread, canGetSourceFileName, canGetLineNumbers, canGetSourceDebugExtension, canAccessLocalVariables, canMaintainOriginalMethodOrder, canGenerateSingleStepEvents, canGenerateExceptionEvents,
		canGenerateFramePopEvents, canGenerateBreakPointEvents, canSuspend, canRedefineAnyClass, canGetCurrentThreadCpuTime, canGetThreadCpuTime, canGenerateMethodEntryEvents, canGenerateMethodExitEvents, canGenerateAllClassHookEvents,
		canGenerateCompiledMethodLoadEvents, canGenerateMonitorEvents, canGenerateVmObjectAllocEvents, canGenerateNativeMethodBindEvents, canGenerateGarbageCollectionEvents, canGenerateObjectFreeEvents, canForceEarlyReturn,
		canGetOwnedMonitorStackDepthInfo, canGetConstantPool, canSetNativeMethodPrefix, canRetransformClasses, canRetransformAnyClass, canGenerateResourceExhaustionHeapEvents, canGenerateResourceExhaustionThreadsEvents
	)
}

data class ThreadInfo(
	val name: String,
	val priority: Int,
	val isDaemon: Boolean,
	val threadGroup: ThreadGroup?,
	val contextClassLoader: ClassLoader?
)

typealias JvmtiEmptyResult = JvmtiResult<Unit>
class JvmtiResult <out T> private constructor(private val inner: Any?) {
	fun isEmpty() = inner == Unit
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
		OTHER(0x7fffffff),
		JVMTI_METHOD_NOT_DEFINED(0x7ffffffe); // specific to jvmti4j
		
		companion object {
			private val values = values()
			private val valMap: Map<Int, JvmtiError> = HashMap<Int, JvmtiError>().also { map ->
				values.forEach {
					map[it.code] = it
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
