// for use with webpack v4
const path = require("path")

const { VueLoaderPlugin } = require('vue-loader')
const MiniCssExtractPlugin = require("mini-css-extract-plugin")

module.exports = {
    mode: 'development',
    entry: [
        "./src/app.js",
        "./src/index.css"
    ],
    output: {
        filename: "bundle.js",
        path: path.resolve(__dirname, "dist")
    },
    module: {
        rules: [
            {
                test: /\.vue$/,
                include: path.resolve(__dirname, "src"),
                use: 'vue-loader'
            },
            {
                test: /\.js$/,
                include: path.resolve(__dirname, "src"),
                loader: "babel-loader"
            },
            // {
            //     test: /\.scss$/,
            //     use: [
            //          MiniCssExtractPlugin.loader,
            //          {
            //            loader: "css-loader",
            //            options: {
            //              modules: true,
            //              sourceMap: true,
            //              importLoader: 2
            //            }
            //          },
            //          "sass-loader"
            //        ]
            //  }
            {
                test: /\.css$/,
                use:[
                    MiniCssExtractPlugin.loader,
                        // options: {
                        //     publicPath: path.resolve(__dirname, "dist")
                        // }
                    "css-loader"
                    // "sass-loader"
                ]
            },
            {
                test: /\.(eot|woff|woff2|svg|ttf)([\?]?.*)$/,
                loader: "file-loader",
                // loader: 'file?name=public/fonts/[name].[ext]'

            }
            // { test: /\.woff$/, loader: "url-loader?limit=10000&mimetype=application/font-woff" },
            // { test: /\.ttf$/,  loader: "url-loader?limit=10000&mimetype=application/octet-stream" },
            // { test: /\.eot$/,  loader: "file-loader" },
            // { test: /\.svg$/,  loader: "url-loader?limit=10000&mimetype=image/svg+xml" }
        ]
    },
    plugins: [
        new VueLoaderPlugin(),
        new MiniCssExtractPlugin({
            filename:"bundle.css"
        })
    ],
    resolve: {
        alias: {
            'vue$': 'vue/dist/vue.esm.js' // 'vue/dist/vue.common.js' for webpack 1
        }
    },
    watchOptions: {
        ignored: /node_modules/
    },
    stats: 'verbose'
    // devtool:"source-map"
}
