package main

import (
	"github.com/stretchr/testify/assert"
	"testing"
)

func TestPasswordEntryIsValidCount(t *testing.T) {
	assert := assert.New(t)

	a := pwdlineFromStrings([]string{"1-3", "a:", "abcde"})
	b := pwdlineFromStrings([]string{"1-3", "b:", "cdefg"})
	c := pwdlineFromStrings([]string{"2-9", "c:", "ccccccccc"})

	assert.Equal(a.isValidCount(), true)
	assert.Equal(b.isValidCount(), false)
	assert.Equal(c.isValidCount(), true)
}

func TestPasswordEntryIsValidPos(t *testing.T) {
	assert := assert.New(t)

	a := pwdlineFromStrings([]string{"1-3", "a:", "abcde"})
	b := pwdlineFromStrings([]string{"1-3", "b:", "cdefg"})
	c := pwdlineFromStrings([]string{"2-9", "c:", "ccccccccc"})

	assert.Equal(a.isValidPos(), true)
	assert.Equal(b.isValidPos(), false)
	assert.Equal(c.isValidPos(), false)
}
