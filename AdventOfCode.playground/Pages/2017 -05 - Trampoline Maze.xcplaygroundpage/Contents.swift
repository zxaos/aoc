//: [Previous](@previous)

import Foundation
let fileURL = Bundle.main.url(forResource: "input", withExtension: "txt")
let puzzleInput = try String(contentsOf: fileURL!, encoding: String.Encoding.utf8)

class Part1Tests : XCTestCase {
    @objc func testAssertions() {
        XCTAssertEqual(parseInput("0\n3\n0\n1\n-3"), [0, 3, 0, 1, -3])
        XCTAssertEqual(mazeLength(maze: [0, 3, 0, 1, -3]), 5)
    }
}

func parseInput( _ input: String) -> [ Int ] {
    return input.components(separatedBy: CharacterSet.newlines)
        .map( { $0.trimmingCharacters(in: CharacterSet.whitespacesAndNewlines) })
        .filter( { !$0.isEmpty } )
        .map ( { Int($0)! } )
}

func mazeLength(maze: [Int]) -> Int {
    var distance = 0
    var walk = maze
    var index = 0
    var lastIndex = 0
    while (true) {
        if (index >= maze.count) {
            return distance
        }
        lastIndex = index
        index += walk[index]
        walk[lastIndex] += 1
        distance += 1
    }
}

Part1Tests()

//print ( mazeLength(maze: parseInput(puzzleInput)))
// 358131


class Part2Tests : XCTestCase {
    @objc func testAssertions() {
        XCTAssertEqual(strangeMazeLength(maze: [0, 3, 0, 1, -3]), 10)
    }
}

func strangeMazeLength(maze: [Int]) -> Int {
    var distance = 0
    var walk = maze
    var index = 0
    var lastIndex = 0
    while (true) {
        if (index >= maze.count || index < 0) {
            return distance
        }
        lastIndex = index
        index += walk[index]
        if (walk[lastIndex] > 2) {
            walk[lastIndex] -= 1
        } else {
            walk[lastIndex] += 1
        }
        distance += 1
    }
}

Part2Tests()

//print (strangeMazeLength(maze: parseInput(puzzleInput)))
// 25558839


