const { Binary } = require('binary-install');
const os = require('os');
const { join } = require('path');

const getPlatform = () => {
	const type = os.type();
	const arch = os.arch();

	if (type === 'Windows_NT' && arch === 'x64') return 'win64';
	if (type === 'Linux' && arch === 'x64') return 'linux';
	if (type === 'Darwin' && arch === 'x64') return 'macos';

	throw new Error(`Unsupported platform: ${type} ${arch}`);
};

const getBinary = () => {
	const platform = getPlatform();
	const version = require('./package.json').version;
	const url = `https://github.com/affanmustafa/commitier/releases/download/v${version}/commitier-${platform}.tar.gz`;
	const installDirectory = join(__dirname, 'bin');

	return new Binary('commitier', url, installDirectory);
};

const run = (args) => {
	const binary = getBinary();
	binary.run(args);
};

const install = () => {
	const binary = getBinary();
	binary.install();
};

module.exports = {
	install,
	run,
};
