package wickra

import (
	"encoding/json"
	"strings"
	"testing"
)

const spec = `{"seed":42,"bars":8,"start_price":100.0,` +
	`"regimes":[{"kind":"trend","len":8,"drift":0.002,"vol":0.01}],` +
	`"microstructure":{"book_depth":3,"spread_bps":4.0,"trade_rate":3.0}}`

func generateCmd() string {
	return `{"cmd":"generate"}`
}

func TestVersion(t *testing.T) {
	if Version() == "" {
		t.Fatal("empty version")
	}
}

func TestGenerateOutput(t *testing.T) {
	s, err := New(spec)
	if err != nil {
		t.Fatal(err)
	}
	defer s.Close()
	raw, err := s.Command(generateCmd())
	if err != nil {
		t.Fatal(err)
	}
	var out struct {
		Candles       []json.RawMessage `json:"candles"`
		BookSnapshots []json.RawMessage `json:"book_snapshots"`
	}
	if err := json.Unmarshal([]byte(raw), &out); err != nil {
		t.Fatal(err)
	}
	if len(out.Candles) != 8 {
		t.Fatalf("expected 8 candles, got %d: %s", len(out.Candles), raw)
	}
	if len(out.BookSnapshots) != 8 {
		t.Fatalf("expected 8 book snapshots, got %d", len(out.BookSnapshots))
	}
}

func TestInvalidSpecIsError(t *testing.T) {
	if _, err := New("{ not valid json"); err == nil {
		t.Fatal("expected an error for an invalid spec")
	}
}

func TestUnknownCommandIsInBandError(t *testing.T) {
	s, err := New(spec)
	if err != nil {
		t.Fatal(err)
	}
	defer s.Close()
	raw, err := s.Command(`{"cmd":"nope"}`)
	if err != nil {
		t.Fatalf("unexpected hard error: %v", err)
	}
	if !strings.Contains(raw, `"ok":false`) {
		t.Fatalf("expected an in-band error, got: %s", raw)
	}
}
