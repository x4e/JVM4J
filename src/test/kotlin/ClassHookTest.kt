import dev.binclub.jvm4j.JVM4J
import dev.binclub.jvm4j.JvmtiResult
import org.junit.jupiter.api.Test

/**
 * @author cookiedragon234 26/Sep/2020
 */
class ClassHookTest {
	@Test
	fun test() {
		JVM4J.getJvmti(JVM4J.getVM())
		println(JvmtiResult::class.java.name)
	}
}
