import ace from "ace-builds";
import debounce from "debounce";
import { Terminal } from "xterm";
import { fit } from "xterm/lib/addons/fit/fit";

import "../node_modules/xterm/dist/xterm.css";
import "./index.css";

const term = new Terminal();
term.open(document.getElementById("terminal"));
fit(term);
term.write("Loading...\r\n");
window.addEventListener("resize", debounce(() => fit(term), 50));

const editor = ace.edit("decls-editor");
editor.session.on('change', debounce(() => {
	term.write("Got change!\r\n");
}, 500));

import("./lib.rs").then(evaltrees => {
	window.evaltrees = evaltrees;
});
