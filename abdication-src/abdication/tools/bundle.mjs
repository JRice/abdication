// bundle.mjs — inline local libraries into one self-contained .dsp,
// for tools (and the Faust web IDE) that can't read sibling files.
//
//   node tools/bundle.mjs dsp/prototype.dsp build/prototype.bundle.dsp
//
// Every `name = library("local.lib");` whose file exists next to the
// source is replaced by `name = environment { ...contents... };`,
// recursively. Standard libraries (stdfaust.lib etc.) are left alone.

import { readFileSync, writeFileSync, existsSync, mkdirSync } from "node:fs";
import { dirname, join } from "node:path";

const LIBRARY_REF = /^(\s*)(\w+)\s*=\s*library\("([^"]+)"\);/gm;
const DECLARE_LINE = /^\s*declare\s.*$/gm;

function inline(path, isLibrary) {
    const dir = dirname(path);
    let source = readFileSync(path, "utf8");
    if (isLibrary) source = source.replace(DECLARE_LINE, "");
    return source.replace(LIBRARY_REF, (match, indent, name, file) => {
        const libPath = join(dir, file);
        if (!existsSync(libPath)) return match;           // standard library
        return `${indent}${name} = environment {\n${inline(libPath, true)}\n};`;
    });
}

const [input, output] = process.argv.slice(2);
if (!input || !output) {
    console.error("usage: node bundle.mjs <input.dsp> <output.dsp>");
    process.exit(1);
}
mkdirSync(dirname(output), { recursive: true });
writeFileSync(output, inline(input, false));
console.log(`bundled ${input} -> ${output}`);
