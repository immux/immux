const { createProxyMiddleware } = require('http-proxy-middleware');

module.exports = function(app) {
  app.use(
    '/api',
    createProxyMiddleware({
      target: 'http://localhost:3003',
      pathRewrite: { '^/api': '' },
      changeOrigin: true,
      secure: false
    })
  );
};
