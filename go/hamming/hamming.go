// Package hamming calculates the Hamming difference between two DNA strands.
package hamming

import "errors"

// Distance calculates the Hamming difference between two DNA strands.
func Distance(a, b string) (int, error) {

	if len(a) != len(b) {
		return -1, errors.New("DNA strands of different length")
	}

	var diff = 0
	var strandA = []rune(a)
	var strandB = []rune(b)

	for idx := range strandA {
		if strandA[idx] != strandB[idx] {
			diff++
		}
	}

	return diff, nil
}
