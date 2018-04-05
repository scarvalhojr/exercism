// Package isogram checks if a word or phrase is an isogram
package isogram

import "strings"

// IsIsogram checks if a word or phrase is an isogram
func IsIsogram(input string) bool {
	var seen = make(map[rune]bool)

	for _, char := range strings.ToLower(input) {
		if char == ' ' || char == '-' {
			continue
		}
		if seen[char] {
			return false
		}
		seen[char] = true
	}

	return true
}
