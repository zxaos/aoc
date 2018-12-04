let fs = require('fs');

function parseToCSV(filename) {
	contents = fs.readFileSync(filename, {encoding: 'utf-8', flag: 'r'});
	contents = contents.split("\n");
	contents = contents.map(x => x.trim());
	contents = contents.filter( x => x != "" );
	return contents.join(', ');
}

module.exports.parseToCSV = parseToCSV;
