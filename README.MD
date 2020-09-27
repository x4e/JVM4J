# JVM4J

JVM4J is a library giving Java applications access to low level JVM apis.

Supports JVM 8. For now will crash on newer JVMS. 
TODO: Seperate binaries for newer JVMS.

Examples:
* [src/test/java/JvmTest.java](src/test/java/JvmTest.java)
* [src/test/java/JniTest.java](src/test/java/JniTest.java)

Apis:
* [dev.binclub.jvm4j.JVM4J](src/main/kotlin/dev/binclub/jvm4j/JVM4J.kt)
* [dev.binclub.jvm4j.JNI4J](src/main/kotlin/dev/binclub/jvm4j/JNI4J.kt)

## Building
Note this will create a debug binary only for your platform
```
cargo build
mv target/debug/libJVM4J.so src/main/resources/native/
gradle build
```

## Releases
Not quite setup yet, will be done once project is finished
