// Package twofer contains a function the return 'One for <name>, one for me'.
package twofer

import "fmt"

// ShareWith return 'One for <name>, one for me'.
func ShareWith(name string) string {
	if len(name) == 0 {
		name = "you"
	}
	return fmt.Sprintf("One for %s, one for me.", name)
}
