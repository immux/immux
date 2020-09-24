const pkg = require('./package.json');

/**
 * extra variable
 * @type {[string, any][]}
 */
module.exports = [
  ['version', pkg.version],
  ['buildTime', +new Date()]
];
