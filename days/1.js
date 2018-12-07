"use strict";

function prepareInput(offsets) {
	return offsets.split(',').map( x => Number(x) );
}

const run1 = function(offsetsString, initial = 0) {
	const offsets = prepareInput(offsetsString);
	return offsets.reduce((x, y) => x + y, initial);
}



const run2 = function(offsetsString) {
	const offsets = prepareInput(offsetsString);
	let accumulator = 0;
	const seen = new Map();
	seen.set(accumulator, null);

	while (true) {
		for(let o of offsets) {
			accumulator += o;
			if (seen.has(accumulator)) {
				return accumulator;
			} else {
				seen.set(accumulator, null);
			}
		}
	}
}

module.exports = { run1, run2 };

