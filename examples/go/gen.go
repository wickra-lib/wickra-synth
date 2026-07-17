// A runnable Go example: generate synthetic microstructure and print the first
// three candles.
//
//   go run examples/go/gen.go
//
// Every language example uses the same seed and prints the same candles.
package main

import (
	"encoding/json"
	"fmt"

	wickra "github.com/wickra-lib/wickra-synth/bindings/go"
)

const spec = `{"seed":42,"bars":20,"start_price":100.0,` +
	`"regimes":[{"kind":"trend","len":20,"drift":0.002,"vol":0.01}],` +
	`"microstructure":{"book_depth":5,"spread_bps":4.0,"trade_rate":8.0,` +
	`"funding":{"interval_bars":8,"base_rate":0.0001,"sensitivity":0.5}}}`

func main() {
	synth, err := wickra.New(spec)
	if err != nil {
		panic(err)
	}
	defer synth.Close()

	raw, err := synth.Command(`{"cmd":"generate"}`)
	if err != nil {
		panic(err)
	}
	var out struct {
		Candles []json.RawMessage `json:"candles"`
	}
	if err := json.Unmarshal([]byte(raw), &out); err != nil {
		panic(err)
	}

	fmt.Printf("wickra-synth %s\n", wickra.Version())
	fmt.Printf("bars: %d\n", len(out.Candles))
	fmt.Println("first 3 candles:")
	for _, c := range out.Candles[:3] {
		fmt.Printf("  %s\n", c)
	}
}
