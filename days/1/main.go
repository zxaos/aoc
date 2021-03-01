package main

import (
	"bufio"
	"errors"
	"fmt"
	"io"
	"os"
	"strconv"
)

func main() {
	input := parseUIntStream(os.Stdin)
	result, err := find2020Pair(input)
	if err != nil {
		fmt.Println(err)
		os.Exit(1)
	} else {
		fmt.Println("Product of pair totalling 2020:", result)
	}
	result, err = find2020Triple(input)
	if err != nil {
		fmt.Println(err)
		os.Exit(1)
	} else {
		fmt.Println("Product of triple totalling 2020:", result)
	}
}

func find2020Pair(xs []uint) (uint, error) {
	for i := 0; i < len(xs)-1; i += 1 {
		for j := i; j < len(xs); j += 1 {
			if (xs[i] + xs[j]) == 2020 {
				return xs[i] * xs[j], nil
			}
		}
	}
	return 0, errors.New("No pair of values sums to 2020")
}

func find2020Triple(xs []uint) (uint, error) {
	for i := 0; i < len(xs)-2; i += 1 {
		for j := i + 1; j < len(xs)-1; j += 1 {
			for k := j + 1; k < len(xs); k += 1 {
				if (xs[i] + xs[j] + xs[k]) == 2020 {
					return xs[i] * xs[j] * xs[k], nil
				}
			}
		}
	}
	return 0, errors.New("No triple of values sums to 2020")
}

func parseUIntStream(r io.Reader) []uint {
	scanner := bufio.NewScanner(r)
	scanner.Split(bufio.ScanLines)
	var xs []uint
	for scanner.Scan() {
		value, err := strconv.ParseUint(scanner.Text(), 10, 64)
		if err != nil {
			panic("can only parse uints")
		}
		xs = append(xs, uint(value))
	}
	return xs
}
