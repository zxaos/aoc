//: [Previous](@previous)

import Foundation

let puzzleInput = 265149

class Part1Tests : XCTestCase {
    @objc func testAssertions() {
        XCTAssertEqual(spiralDistance(address: 1), 0)
        XCTAssertEqual(spiralDistance(address: 12), 3)
        XCTAssertEqual(spiralDistance(address: 23), 2)
        XCTAssertEqual(spiralDistance(address: 1024), 31)
    }
}

// 1, 9, 25, 49
func spiralDistance(address : Int) -> Int {
    // base 0, 1, 2 for values less than squares of odd numbers.
    // This could probably be done better as a Sequence of squared odd numbers
    if (address == 1) {
        return 0
    }
    
    var base = -1;
    var circle = 0;
    var baseEnd = 1;
    while true {
        base += 2
        circle += 1
        baseEnd = base * base
        if (baseEnd >= address) {
            break
        }
    }
    let circleRange = ((base - 2) * (base - 2) + 1)...baseEnd
    
    //then add one for every space the address is away from one of the four shortest paths
    // there's 4 shortest paths, and they're at 2,4,6,8 for circle 2
    // they're at 11, 15, 19, 23 for circle 3
    
    let minDistanceMaxOffset = circle - 1
    var loopDistance = circle - 1
    var direction = -1
    for i in circleRange {
        loopDistance += direction
        if i == address {
            return minDistanceMaxOffset + loopDistance
        }
        if ( loopDistance == 0 || loopDistance == minDistanceMaxOffset) {
            direction *= -1
        }
    }
    return 0
}

Part1Tests()

print (spiralDistance(address: puzzleInput))


class Part2Tests : XCTestCase {
    @objc func testAssertions() {
    }
}

Part2Tests()

print()

