package main

import (
	"testing"
)

func TestFind2020Pair(t *testing.T) {
	s := []uint{1721, 979, 366, 299, 675, 1456}

	var exp uint = 514579

	res, _ := find2020Pair(s)
	if res != exp {
		t.Errorf("Expected %d, got %d instead.\n", exp, res)
	}
}

func TestFind2020Triple(t *testing.T) {
	s := []uint{1721, 979, 366, 299, 675, 1456}

	var exp uint = 241861950

	res, _ := find2020Triple(s)
	if res != exp {
		t.Errorf("Expected %d, got %d instead.\n", exp, res)
	}
}
