package main

import (
	"github.com/stretchr/testify/assert"
	"testing"
)

var testInput = []string{
	"..##.......",
	"#...#...#..",
	".#....#..#.",
	"..#.#...#.#",
	".#...##..#.",
	"..#.##.....",
	".#.#.#....#",
	".#........#",
	"#.##...#...",
	"#...##....#",
	".#..#...#.#",
}

func TestMapExtents(t *testing.T) {
	assert := assert.New(t)

	m, err := newTerrainMap(testInput)
	assert.Nil(err)
	assert.Equal(m.rows, uint(11))
	assert.Equal(m.columns, uint(11))
}

func TestUnwrappedRow(t *testing.T) {
	assert := assert.New(t)
	m, _ := newTerrainMap(testInput)
	assert.Equal(m.position(0, 0), '.')
	assert.Equal(m.position(0, 1), '#')
	assert.Equal(m.position(1, 0), '#')
	assert.Equal(m.position(10, 10), '.')
}
