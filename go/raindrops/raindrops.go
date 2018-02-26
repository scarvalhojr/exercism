package raindrops

import (
	"bytes"
	"strconv"
)

func Convert(number int) string {
	var buffer bytes.Buffer
	var noFactor = true

	if number%3 == 0 {
		buffer.WriteString("Pling")
		noFactor = false
	}
	if number%5 == 0 {
		buffer.WriteString("Plang")
		noFactor = false
	}
	if number%7 == 0 {
		buffer.WriteString("Plong")
		noFactor = false
	}
	if noFactor {
		buffer.WriteString(strconv.Itoa(number))
	}

	return buffer.String()
}
