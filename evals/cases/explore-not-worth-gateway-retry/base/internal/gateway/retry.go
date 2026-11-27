package gateway

import (
	"context"
	"errors"
	"math/rand"
	"net"
	"time"
)

// Retries of a timed-out call: 200 ms, 400 ms, 800 ms, 1.6 s, each with
// up to 20 % of jitter so that clients do not retry in step.
const (
	maxRetries = 4
	firstWait  = 200 * time.Millisecond
	jitter     = 0.2
)

// sleep is replaced by the tests.
var sleep = func(ctx context.Context, d time.Duration) error {
	select {
	case <-time.After(d):
		return nil
	case <-ctx.Done():
		return ctx.Err()
	}
}

// withRetry runs call, and runs it again with exponential backoff while it
// times out. Any other error is returned at once: a declined card must not
// be charged twice.
func withRetry(ctx context.Context, call func() error) error {
	wait := firstWait
	for attempt := 0; ; attempt++ {
		err := call()
		if err == nil || !isTimeout(err) || attempt == maxRetries {
			return err
		}
		spread := time.Duration(float64(wait) * jitter * (2*rand.Float64() - 1))
		if err := sleep(ctx, wait+spread); err != nil {
			return err
		}
		wait *= 2
	}
}

func isTimeout(err error) bool {
	var netErr net.Error
	return errors.As(err, &netErr) && netErr.Timeout()
}
