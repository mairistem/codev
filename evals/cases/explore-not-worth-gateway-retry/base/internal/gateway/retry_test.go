package gateway

import (
	"context"
	"errors"
	"testing"
	"time"
)

type timeout struct{}

func (timeout) Error() string   { return "timeout" }
func (timeout) Timeout() bool   { return true }
func (timeout) Temporary() bool { return true }

func TestTwoTimeoutsThenAnAnswer(t *testing.T) {
	var waits []time.Duration
	sleep = func(_ context.Context, d time.Duration) error { waits = append(waits, d); return nil }
	calls := 0
	err := withRetry(context.Background(), func() error {
		calls++
		if calls <= 2 {
			return timeout{}
		}
		return nil
	})
	if err != nil || calls != 3 || len(waits) != 2 {
		t.Fatalf("err=%v calls=%d waits=%v", err, calls, waits)
	}
}

func TestADeclinedCardIsNotRetried(t *testing.T) {
	calls := 0
	declined := errors.New("gateway: charge refused (402)")
	if err := withRetry(context.Background(), func() error { calls++; return declined }); err != declined || calls != 1 {
		t.Fatalf("err=%v calls=%d", err, calls)
	}
}
