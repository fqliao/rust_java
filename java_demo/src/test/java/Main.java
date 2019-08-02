
import org.com.fisco.RustLib;

import java.nio.file.Path;
import java.nio.file.Paths;

public class Main {

    static {
        Path p = Paths.get("src/main/resources/librust_for_java.so");
        System.load(p.toAbsolutePath().toString());
    }

    public static void main(String[] args){

        System.out.println("2 + 8 = " + RustLib.add(2, 8));
        System.out.println(RustLib.getPerson("alice", 12));
    }
}
