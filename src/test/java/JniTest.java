import dev.binclub.jvm4j.JNI4J;
import org.junit.jupiter.api.Test;

import java.util.Objects;

/**
 * @author cookiedragon234 27/Sep/2020
 */
public class JniTest {
	@Test
	public void version() {
		JNI4J.getVersion();
	}
	
	@Test
	public void findClass() {
		Objects.requireNonNull(JNI4J.findClass("java/lang/Object"));
	}
	
	@Test
    public void getSuperClass() {
	    Class superC = JNI4J.getSuperClass(Throwable.class);
	    if (superC != Object.class) {
	        throw new IllegalStateException("" + superC);
        }
    }
}
