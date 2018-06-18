"use strict";

import { clear, onKey, write } from "./ui.js";

import("./generated/evaltrees_wasm").then(evaltrees => {
	const repl = window.repl = evaltrees.Repl.new();
	onKey(k => repl.write(k));
});
