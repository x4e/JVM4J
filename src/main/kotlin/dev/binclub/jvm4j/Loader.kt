package dev.binclub.jvm4j

import dev.binclub.jvm4j.Loader.OS.*
import java.io.File

/**
 * @author cookiedragon234 26/Sep/2020
 */
internal object Loader {
	enum class OS {
		WINDOWS,
		LINUX,
		MAC;
		
		companion object {
			private val os: String = System.getProperty("os.name")
			fun get(): OS = when {
				os.contains("windows", true) -> WINDOWS
				os.contains("linux", true) -> LINUX
				os.contains("mac", true) -> MAC
				else -> error("Unsupported OS $os")
			}
		}
	}
	
	init {
		val os = OS.get()
		val prefix = when (os) {
			WINDOWS -> ""
			LINUX, MAC -> "lib"
		}
		val suffix = when(os) {
			WINDOWS -> "dll"
			LINUX -> "so"
			MAC -> "dylib"
		}
		
		val libraryFile = File(
			System.getenv("java.library.path"),
			"JVM4J.$suffix"
		)
		libraryFile.delete()
		libraryFile.deleteOnExit()
		
		val resourceName = "/native/${prefix}JVM4J.$suffix"
		val `is` = Loader::class.java.getResourceAsStream(resourceName)
			?: error("Couldn't find native library $resourceName")
		libraryFile.writeBytes(`is`.readBytes())
		
		System.load(libraryFile.absolutePath)
	}
}
