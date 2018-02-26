/*
Package gigasecond contains a function to add 10^9 seconds to a given timestamp.
*/
package gigasecond

import "time"

/*
AddGigasecond takes a timestamp, adds 10^9 seconds, and returns it.
*/
func AddGigasecond(t time.Time) time.Time {
	return t.Add(time.Duration(1e9) * time.Second)
}
