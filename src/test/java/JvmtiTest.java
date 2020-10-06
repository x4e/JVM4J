import dev.binclub.jvm4j.*;
import org.junit.jupiter.api.Test;

import java.lang.management.ManagementFactory;
import java.util.Arrays;

public class JvmtiTest {
	Jvmti jvmti = JVM4J.getJvmti(JVM4J.getVM());
	
	static {
		System.out.println(ManagementFactory.getRuntimeMXBean().getName()); // For gdb purposes
	}
	
	@Test
	public void setEventNotificationMode() {
		JvmtiCapabilities capabilities = new JvmtiCapabilities();
		capabilities.setCanGenerateAllClassHookEvents(true);
		jvmti.addCapabilities(capabilities);
		jvmti.setEventNotificationMode(true, JvmtiEvent.CLASS_FILE_LOAD_HOOK).unwrap();
		jvmti.setEventNotificationMode(false, JvmtiEvent.CLASS_FILE_LOAD_HOOK).unwrap();
		jvmti.relinquishCapabilities(capabilities);
	}
	
	@Test
	public void getThreadInfo() {
		try {
			Thread.sleep(9000);
		} catch (InterruptedException e) {
			e.printStackTrace();
		}
		
		int numThreads = Thread.activeCount();
		Thread[] threads = new Thread[numThreads];
		numThreads = Thread.enumerate(threads);
		
		for (int i = 0; i < numThreads; i++) {
			Thread thread = threads[i];
			ThreadInfo info = jvmti.getThreadInfo(thread).unwrap();
			System.out.println(info);
		}
	}
	
	//@Test //GetAllThreads is null???
	public void getAllThreads() {
		System.out.println(Arrays.toString(jvmti.getAllThreads().unwrap()));
	}
}
