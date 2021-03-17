package input

import (
	"bufio"
	"fmt"
	"io"
	"os"
	"strconv"
	"strings"
)

/* parseArgs determines which of the three supported invocation styles is being
*  used and returns a relevant filename to open when relevant.
*  Style 1: no arguments - return an empty string
*  Style 2: invocation with -f but no filename - return "input" as a default filename
*  Style 3: invocation with -f and a filename - return the filename
*  Not a matching style: Throw an error
 */
func parseArgs() string {
	switch len(os.Args) {
	case 1:
		return "os.stdin"
	case 2:
		if os.Args[1] == "-f" {
			return "input"
		}
	case 3:
		if os.Args[1] == "-f" {
			return os.Args[2]
		}
	}
	fmt.Fprintln(os.Stderr, "Usage: -f <filename>, -f, or no arguments will read from stdin")
	os.Exit(1)
	return ""
}

// Resolve a filename to a readable handle
func getInputStream() (handle io.ReadCloser) {
	filename := parseArgs()
	if filename == "os.stdin" {
		handle = os.Stdin
	} else {
		f, err := os.Open(filename)
		if err != nil {
			fmt.Fprintf(os.Stderr, "error: %v\n", err)
			os.Exit(1)
		}
		handle = f
	}
	return
}

func closeHandle(f io.ReadCloser) {
	err := f.Close()
	if err != nil {
		fmt.Fprintf(os.Stderr, "error: %v\n", err)
		os.Exit(1)
	}
}

func GetLines() (xs []string) {
	handle := getInputStream()
	defer closeHandle(handle)

	scanner := bufio.NewScanner(handle)
	scanner.Split(bufio.ScanLines)
	for scanner.Scan() {
		xs = append(xs, scanner.Text())
	}
	return
}

func GetWordsByLine() (xss [][]string) {
	handle := getInputStream()
	defer closeHandle(handle)

	scanner := bufio.NewScanner(handle)
	scanner.Split(bufio.ScanLines)
	for scanner.Scan() {
		xss = append(xss, strings.Split(scanner.Text(), " "))
	}
	return
}

func GetUints() (xs []uint) {
	handle := getInputStream()
	defer closeHandle(handle)

	scanner := bufio.NewScanner(handle)
	scanner.Split(bufio.ScanLines)
	for scanner.Scan() {
		value, err := strconv.ParseUint(scanner.Text(), 10, 64)
		if err != nil {
			panic("can only parse uints")
		}
		xs = append(xs, uint(value))
	}
	return
}
