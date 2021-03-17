package main

import (
	"fmt"
	"strconv"
	"strings"

	"github.com/zxaos/aoc/lib/input"
)

type passwordEntry struct {
	min      uint64
	max      uint64
	target   rune
	password string
}

func (e *passwordEntry) isValidCount() bool {
	var targetCount uint64 = 0
	for _, char := range e.password {
		if char == e.target {
			targetCount++
		}
	}
	return e.min <= targetCount && e.max >= targetCount
}

func (e *passwordEntry) isValidPos() bool {
	// Exactly one of the two positions must match (XOR)
	password := []rune(e.password)
	return (password[e.min-1] == e.target) != (password[e.max-1] == e.target)
}

func pwdlineFromStrings(line []string) passwordEntry {
	bounds := strings.Split(line[0], "-")
	min, _ := strconv.ParseUint(bounds[0], 10, 64)
	max, _ := strconv.ParseUint(bounds[1], 10, 64)
	target := line[1][0]
	password := line[2]

	return passwordEntry{min, max, rune(target), password}
}

func main() {
	input := input.GetWordsByLine()

	validPasswordsByCount := 0
	validPasswordsByPosition := 0
	for _, line := range input {
		pwd := pwdlineFromStrings(line)
		if pwd.isValidCount() {
			validPasswordsByCount++
		}
		if pwd.isValidPos() {
			validPasswordsByPosition++
		}
	}

	fmt.Println("(old style) valid passdwords: ", validPasswordsByCount)
	fmt.Println("(new style) valid passdwords: ", validPasswordsByPosition)

}
