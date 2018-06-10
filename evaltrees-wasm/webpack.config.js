const path = require("path");
const HtmlWebpackPlugin = require("html-webpack-plugin");

module.exports = (env, argv) => {
	const mode = argv && argv.mode || "development";
	return {
		entry: "./src/index.js",
		mode,
		module: {
			rules: [
				{
					test: /\.js$/,
					loader: "babel-loader",
					options: {
						compact: true
					}
				},
				{
					test: /\.rs$/,
					use: [
						{
							loader: "babel-loader",
							options: {
								compact: true
							}
						},
						{
							loader: "rust-native-wasm-loader",
							options: {
								release: (mode === "production"),
								wasmBindgen: {
									wasm2es6js: false,
								}
							}
						}
					]
				}
			]
		},
		output: {
			filename: "evaltrees.js",
			path: path.resolve(__dirname, "dist")
		},
		plugins: [
			new HtmlWebpackPlugin({
				template: "src/index.html"
			})
		]
	};
};
