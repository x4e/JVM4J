import dev.binclub.jvm4j.JVM4J;
import org.junit.jupiter.api.Test;

/**
 * @author cookiedragon234 26/Sep/2020
 */
public class JvmTest {
	@Test
	public void activeProcessors() {
		JVM4J.activeProcessorCount();
	}
}
