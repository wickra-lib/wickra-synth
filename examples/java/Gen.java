// A runnable Java example: generate synthetic microstructure through the binding.
//
//   cargo build -p wickra-synth-c
//   mvn -f bindings/java/pom.xml -q package -DskipTests
//   javac -cp bindings/java/target/classes examples/java/Gen.java -d examples/java/out
//   java --enable-native-access=ALL-UNNAMED \
//        -Dnative.lib.dir=target/debug \
//        -cp "bindings/java/target/classes;examples/java/out" Gen
//
// Every language example uses the same seed and prints the same candles.
import org.wickra.synth.Synth;

public final class Gen {
    private static final String SPEC =
            "{\"seed\":42,\"bars\":20,\"start_price\":100.0,"
                    + "\"regimes\":[{\"kind\":\"trend\",\"len\":20,\"drift\":0.002,\"vol\":0.01}],"
                    + "\"microstructure\":{\"book_depth\":5,\"spread_bps\":4.0,\"trade_rate\":8.0,"
                    + "\"funding\":{\"interval_bars\":8,\"base_rate\":0.0001,\"sensitivity\":0.5}}}";

    public static void main(String[] args) {
        try (Synth synth = new Synth(SPEC)) {
            String response = synth.command("{\"cmd\":\"generate\"}");
            System.out.println("wickra-synth " + Synth.version());
            System.out.println(response);
        }
    }
}
