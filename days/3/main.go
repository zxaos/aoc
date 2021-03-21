package main

import (
	"errors"
	"fmt"
	"os"

	"github.com/zxaos/aoc/lib/input"
)

const (
	terrainOpen rune = '.'
	terrainTree rune = '#'
)

type terrainMap struct {
	terrain [][]rune
	columns int
	rows    int
}

func (m *terrainMap) position(x, y int) rune {
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
	m.columns = len([]rune(lines[0]))
	m.rows = len(lines)
	terrain := make([][]rune, 0, m.rows)

	for _, line := range lines {
		r := []rune(line)
		if len(r) != m.columns {
			return nil, errors.New("Can't parse terrain line ")
		}

		terrain = append(terrain, r)
	}
	m.terrain = terrain

	return m, nil
}

func (m *terrainMap) plotLinearPath(startX, startY, slopeX, slopeY int) ([]rune, error) {
	// Ensure the Y is within the range of rows
	var path []rune
	if startY > m.rows || startY < 0 {
		return path, errors.New("Starting Y is not in range")
	}

	x := int(startX)
	y := int(startY)

	for y < int(m.rows) && y >= 0 {
		path = append(path, m.position(x, y))
		x += slopeX
		y += slopeY
	}

	return path, nil
}

func obstaclesInPath(path []rune) uint {
	var obstacles uint
	for _, x := range path {
		if x != terrainOpen {
			obstacles++
		}
	}
	return obstacles
}

func main() {
	input := input.GetLines()

	tm, err := newTerrainMap(input)
	if err != nil {
		fmt.Println(err)
		os.Exit(1)
	}

	path, err := tm.plotLinearPath(0, tm.rows-1, 3, -1)
	if err != nil {
		fmt.Println(err)
		os.Exit(1)
	}

	obstacles := obstaclesInPath(path)

	fmt.Printf("obstacles in first test path: %d\n", obstacles)

	slopes := [][]int{{1, -1}, {5, -1}, {7, -1}, {1, -2}}
	obstacle_product := obstacles

	for _, slope := range slopes {
		path, err := tm.plotLinearPath(0, tm.rows-1, slope[0], slope[1])
		if err != nil {
			fmt.Println(err)
			os.Exit(1)
		}
		obstacle_product *= obstaclesInPath(path)
	}

	fmt.Printf("product of obstacles in all paths: %d\n", obstacle_product)

}
