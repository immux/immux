import glob = require('glob');

const controllers = glob.sync('**/*Controller.@(ts|js)', {
  cwd: __dirname,
  realpath: true
});

export default controllers.map(controller => require(controller).default);
