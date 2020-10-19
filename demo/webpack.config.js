const CopyPlugin = require("copy-webpack-plugin");
const path = require("path");

module.exports = {
  entry: "./bootstrap.js",
  mode: "development",
  experiments: {
    asyncWebAssembly: true
  },
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "bootstrap.js"
  },
  plugins: [new CopyPlugin({ patterns: ["index.html"] })]
};
