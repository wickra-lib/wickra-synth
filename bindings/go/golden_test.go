package wickra

// The cross-language golden invariant seen from Go: the same seed yields
// byte-identical output across calls, and the streamed event list carries the
// same candles as the batch generate. The response bytes are what every other
// binding produces too, because the whole generator lives once in the Rust core
// and this binding forwards its JSON verbatim.

import (
	"encoding/json"
	"testing"
)

func TestGenerateByteIdenticalAcrossCalls(t *testing.T) {
	a, err := New(spec)
	if err != nil {
		t.Fatal(err)
	}
	defer a.Close()
	b, err := New(spec)
	if err != nil {
		t.Fatal(err)
	}
	defer b.Close()

	ra, err := a.Command(generateCmd())
	if err != nil {
		t.Fatal(err)
	}
	rb, err := b.Command(generateCmd())
	if err != nil {
		t.Fatal(err)
	}
	if ra != rb {
		t.Fatalf("expected byte-identical output, got:\n a: %s\n b: %s", ra, rb)
	}
}

func TestStreamCandlesMatchBatch(t *testing.T) {
	batchStore, err := New(spec)
	if err != nil {
		t.Fatal(err)
	}
	defer batchStore.Close()
	rawBatch, err := batchStore.Command(generateCmd())
	if err != nil {
		t.Fatal(err)
	}
	var batch struct {
		Candles []json.RawMessage `json:"candles"`
	}
	if err := json.Unmarshal([]byte(rawBatch), &batch); err != nil {
		t.Fatal(err)
	}

	streamStore, err := New(spec)
	if err != nil {
		t.Fatal(err)
	}
	defer streamStore.Close()
	rawStream, err := streamStore.Command(`{"cmd":"generate_stream"}`)
	if err != nil {
		t.Fatal(err)
	}
	var stream struct {
		Events []struct {
			Type   string          `json:"type"`
			Candle json.RawMessage `json:"candle"`
		} `json:"events"`
	}
	if err := json.Unmarshal([]byte(rawStream), &stream); err != nil {
		t.Fatal(err)
	}

	var streamedCandles []json.RawMessage
	for _, e := range stream.Events {
		if e.Type == "candle" {
			streamedCandles = append(streamedCandles, e.Candle)
		}
	}
	if len(streamedCandles) != len(batch.Candles) {
		t.Fatalf("stream candles %d != batch candles %d", len(streamedCandles), len(batch.Candles))
	}
	for i := range batch.Candles {
		if string(streamedCandles[i]) != string(batch.Candles[i]) {
			t.Fatalf("candle %d differs:\n stream: %s\n batch:  %s", i, streamedCandles[i], batch.Candles[i])
		}
	}
}
