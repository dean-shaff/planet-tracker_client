// for use with webpack v4
const path = require("path")

const { merge } = require("webpack-merge")
const common = require("./webpack.common.js")


module.exports = merge(common, {
  mode: "production"
})
