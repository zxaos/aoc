package main

import (
	"errors"
	"fmt"
	"os"
	// "strconv"
	// "strings"

	"github.com/zxaos/aoc/lib/input"
)

const (
	terrainOpen rune = '.'
	terrainTree rune = '#'
)

type terrainMap struct {
	terrain [][]rune
	columns uint
	rows    uint
}

func (m *terrainMap) position(x, y uint) rune {
	if y > m.rows {
		panic("Invalid column")
	}

	// resolve input as if we were using normal cartesian co-ordinates
	x = x % m.columns
	y = m.rows - y - 1

	return m.terrain[y][x]
}

func newTerrainMap(lines []string) (*terrainMap, error) {
	m := new(terrainMap)
	m.columns = uint(len([]rune(lines[0])))
	m.rows = uint(len(lines))
	terrain := make([][]rune, 0, m.rows)

	for _, line := range lines {
		r := []rune(line)
		if uint(len(r)) != m.columns {
			return nil, errors.New("Can't parse terrain line ")
		}

		terrain = append(terrain, r)
	}
	m.terrain = terrain

	fmt.Printf("terrain is %d by %d\n", m.columns, m.rows)
	return m, nil
}

func main() {
	input := input.GetLines()

	tm, err := newTerrainMap(input)
	if err != nil {
		fmt.Println(err)
		os.Exit(1)
	}

	fmt.Println(tm.position(0, 0))
}
