const pathRegexp = (path) => {
  const keys = [];
  path = path.replace(/\/:(\w+)?(?:\/)?/g, function (match, key) {
    keys.push(key);
    return `\\/([^\\/]+)(?:\\/)?`;
  });
  return {
    keys,
    regExp: new RegExp(`^${path}$`),
  };
};

const base64urlEscape = (str) => {
  return str.replace(/\+/g, "-").replace(/\//g, "_").replace(/=/g, "");
};

const findRouter = (method, path, routers) => {
  let routeKey = "";
  let urlParma = "";

  if (routers[`${method} ${path}`]) {
    routeKey = `${method} ${path}`;
  } else {
    Object.keys(routers).some((key) => {
      const [methodKey, pathname] = key.split(" ");
      const { keys, regExp } = pathRegexp(pathname);
      const mathed = regExp.exec(path);

      if (mathed) {
        routeKey = `${method} ${pathname}`;
        urlParma = mathed[1];
        return true;
      }
    });
  }

  return {
    urlParma,
    routeKey 
  };
};

module.exports = {
  base64urlEscape,
  findRouter,
};
