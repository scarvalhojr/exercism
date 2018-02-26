// Package bob implements an annoying teenager
package bob

import (
	"regexp"
	"strings"
)

var alphaRE = regexp.MustCompile(`[a-zA-Z]`)
var lowerRE = regexp.MustCompile(`[a-z]`)

// Hey implements an annoying teenager
func Hey(remark string) string {

	remark = strings.TrimSpace(remark)

	if len(remark) == 0 {
		return "Fine. Be that way!"
	}

	question := strings.HasSuffix(remark, "?")
	shouting := alphaRE.MatchString(remark) && !lowerRE.MatchString(remark)

	if question && shouting {
		return "Calm down, I know what I'm doing!"
	} else if question {
		return "Sure."
	} else if shouting {
		return "Whoa, chill out!"
	}

	return "Whatever."
}
