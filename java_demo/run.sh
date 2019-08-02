mkdir -p build/classes
javac -d build/classes src/main/java/org/com/fisco/*.java src/test/java/*.java
java -cp build/classes Main
