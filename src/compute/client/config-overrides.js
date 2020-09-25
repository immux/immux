const webpack = require('webpack');
const path = require('path');
const _ = require('lodash');

function addCssModulesCamelCaseSupport(config) {
  _.forEach(config.module.rules, (rule) => {
    if (!_.isArray(rule.oneOf)) {
      return;
    }

    _.forEach(rule.oneOf, (rule) => {
      if (!_.includes(_.toString(rule.test), 'module')) {
        return;
      }

      _.forEach(rule.use, (use) => {
        if (!_.includes(use.loader, '/css-loader/')) {
          return;
        }

        _.assign(use.options, { localsConvention: 'camelCase' });
      });
    });
  });

  return config;
}

const {
  addWebpackPlugin,
  fixBabelImports,
  addWebpackAlias,
  addBabelPlugin,
  addLessLoader,
  babelExclude,
  override
} = require('customize-cra');

const MonacoWebpackPlugin = require('monaco-editor-webpack-plugin');

const definitions = _.reduce(
  require('./definitions'),
  (acc, [prop, value]) =>
    _.assign(acc, {
      [`process.env.COMPUTE_${_.toUpper(_.snakeCase(prop))}`]: JSON.stringify(
        value
      )
    }),
  {}
);

module.exports = override(
  addWebpackPlugin(
    new MonacoWebpackPlugin({
      languages: ['json', 'javascript', 'typescript']
    })
  ),

  addWebpackPlugin(new webpack.DefinePlugin(definitions)),

  addWebpackAlias({
    ['@']: path.resolve(__dirname, './src')
  }),

  addBabelPlugin('lodash'),

  babelExclude(path.resolve('src/vendors')),

  fixBabelImports('import', {
    libraryName: 'antd',
    libraryDirectory: 'es',
    style: true
  }),

  addLessLoader({
    javascriptEnabled: true,
    localIdentName: '[local]_[hash:base64:8]',
    noIeCompat: true,

    modifyVars: {
      '@primary-color': '#ff7c0a',
      '@layout-header-height': '50px',
      '@layout-header-background': '#fff',
      '@layout-header-padding': '0 16px'
    }
  }),

  addCssModulesCamelCaseSupport
);
