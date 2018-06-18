"use strict";

import debounce from "debounce";
import { Terminal } from "xterm";
import { fit } from "xterm/lib/addons/fit/fit";

import "../node_modules/xterm/dist/xterm.css";
import "./index.css";

const term = new Terminal();
export default term;
term.open(document.getElementById("terminal"));
fit(term);
term.write("Loading...\r\n");
window.addEventListener("resize", debounce(() => fit(term), 50));

export function onKey(cb) {
	term.on("key", cb);
};

export function write(str) {
	term.write(str);
};
