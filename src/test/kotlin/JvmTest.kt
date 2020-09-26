import dev.binclub.jvm4j.JVM4J
import org.junit.jupiter.api.Test

/**
 * @author cookiedragon234 26/Sep/2020
 */
class JvmTest {
	@Test
	fun test() {
		val vm = JVM4J.getVM()
		JVM4J.getJvmti(vm)
		
		JVM4J.getInterfaceVersion()
		JVM4J.iHashCode(vm)
	}
}
