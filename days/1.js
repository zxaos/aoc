"use strict";

module.exports.run = function(offsetsString, initial = 0) {
	let offsets = offsetsString.split(',').map( x => Number(x) );
	return offsets.reduce((x, y) => x + y, initial);
}

