import dev.binclub.jvm4j.JNI4J;
import dev.binclub.jvm4j.JVM4J;
import dev.binclub.jvm4j.Jvmti;
import dev.binclub.jvm4j.JvmtiEvent;
import org.junit.jupiter.api.Test;

import java.lang.management.ManagementFactory;
import java.util.Arrays;

public class JvmtiTest {
	Jvmti jvmti = JVM4J.getJvmti(JVM4J.getVM());
	
	@Test
	public void setEventNotificationMode() {
		jvmti.setEventNotificationMode(true, JvmtiEvent.CLASS_FILE_LOAD_HOOK).unwrap();
		jvmti.setEventNotificationMode(false, JvmtiEvent.CLASS_FILE_LOAD_HOOK).unwrap();
	}
	
	//@Test //GetAllThreads is null???
	public void getAllThreads() {
		System.out.println(Arrays.toString(jvmti.getAllThreads().unwrap()));
	}
}
