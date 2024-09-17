const { join } = require('path');
const { execFileSync } = require('child_process');
const fs = require('fs');

const getBinaryPath = () => {
	const binaryName =
		process.platform === 'win32' ? 'commitier.exe' : 'commitier';
	const binaryPath = join(__dirname, 'target', 'release', binaryName);
	console.log('Binary path:', binaryPath);
	console.log('Binary exists:', fs.existsSync(binaryPath));
	return binaryPath;
};

const run = (args) => {
	const binaryPath = getBinaryPath();
	console.log('Executing binary with args:', args);
	try {
		const output = execFileSync(binaryPath, args, { encoding: 'utf8' });
		console.log('Binary output:', output);
	} catch (error) {
		console.error(`Error executing binary: ${error.message}`);
		console.error('Error stack:', error.stack);
		process.exit(1);
	}
};

module.exports = { run };
