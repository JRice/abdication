// faust2lang.mjs — compile a self-contained .dsp to another language's
// source with the WebAssembly Faust compiler (no native Faust install).
//
//   node tools/faust2lang.mjs <input.dsp> <output-file> <lang> [faust args...]
//   e.g. node tools/faust2lang.mjs build/voice.bundle.dsp plugin/src/dsp.rs rust -cn VoiceDsp

import { readFileSync, writeFileSync, mkdirSync } from "node:fs";
import { dirname, join, parse, basename } from "node:path";
import { fileURLToPath } from "node:url";
import {
    instantiateFaustModuleFromFile,
    LibFaust,
    FaustCompiler,
} from "@grame/faustwasm/dist/esm/index.js";

const here = dirname(fileURLToPath(import.meta.url));
const [input, output, lang, ...extra] = process.argv.slice(2);
if (!input || !output || !lang) {
    console.error("usage: node faust2lang.mjs <input.dsp> <output-file> <lang> [faust args...]");
    process.exit(1);
}

const module = await instantiateFaustModuleFromFile(
    join(here, "node_modules/@grame/faustwasm/libfaust-wasm/libfaust-wasm.js"));
const compiler = new FaustCompiler(new LibFaust(module));
const { name } = parse(input);
const outName = basename(output);
const ok = compiler.generateAuxFiles(name, readFileSync(input, "utf8"),
    `-lang ${lang} ${extra.join(" ")} -o ${outName}`);
if (!ok) {
    console.error(compiler.getErrorMessage());
    process.exit(1);
}
mkdirSync(dirname(output), { recursive: true });
writeFileSync(output, compiler.fs().readFile(outName, { encoding: "utf8" }));
console.log(`${lang}: ${input} -> ${output}`);
