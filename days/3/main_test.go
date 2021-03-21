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
	assert.Equal(m.rows, 11)
	assert.Equal(m.columns, 11)
}

func TestUnwrappedRow(t *testing.T) {
	assert := assert.New(t)
	m, _ := newTerrainMap(testInput)
	assert.Equal(m.position(0, 0), '.')
	assert.Equal(m.position(0, 1), '#')
	assert.Equal(m.position(1, 0), '#')
	assert.Equal(m.position(10, 10), '.')
}

func TestWrappedRow(t *testing.T) {
	assert := assert.New(t)
	m, _ := newTerrainMap(testInput)
	assert.Equal(m.position(10, 0), '#')
	assert.Equal(m.position(11, 0), '.')
	assert.Equal(m.position(20, 0), '.')
}

func TestWrappedColumns(t *testing.T) {
	assert := assert.New(t)
	m, _ := newTerrainMap(testInput)
	assert.Panics(func() {
		m.position(0, 11)
	})
}

func TestPath(t *testing.T) {
	assert := assert.New(t)
	m, _ := newTerrainMap(testInput)
	path := []rune{
		'.', '.', '#', '.', '#', '#', '.', '#', '#', '#', '#',
	}
	results, _ := m.plotLinearPath(0, 10, 3, -1)
	assert.Equal(path, results)
}

func TestObstacleCount(t *testing.T) {
	assert := assert.New(t)
	m, _ := newTerrainMap(testInput)
	path, _ := m.plotLinearPath(0, 10, 3, -1)
	assert.Equal(uint(7), obstaclesInPath(path))
}
