// Package acronym converts a phrase to its acronym
package acronym

import (
	"bytes"
	"strings"
)

// Abbreviate converts a phrase to its acronym
func Abbreviate(s string) string {

	var buffer bytes.Buffer

	for _, word := range strings.Split(strings.Replace(s, "-", " ", -1), " ") {
		buffer.WriteString(strings.ToUpper(string([]rune(word)[0])))
	}

	return buffer.String()
}
