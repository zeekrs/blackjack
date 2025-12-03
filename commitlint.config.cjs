const fs = require("node:fs");
const path = require("node:path");
const { execSync } = require("node:child_process");
const { defineConfig } = require("czg");

//  join the list and remove duplicate values
const scopes = getScopes("apps", "packages");

const scopeComplete = matchScopes(scopes);

function matchScopes(scopes) {
	const items = execSync("git status --porcelain || true")
		.toString()
		.trim()
		.split("\n")
		.map((r) => r.match(/\/(.*?)\//)?.[1])
		.filter((r) => !!r);

	let max = 0;
	let most;
	// count the number of items  in  each scope
	for (const scope of scopes) {
		const count = items.filter((item) => item === scope).length;
		if (!count) {
			continue;
		}
		if (max < count) {
			max = count;
			most = scope;
		}
	}

	return most;
}

function getScopes(...dirs) {
	const scopes = new Set();
	for (const dir of dirs) {
		const subdirs = readSubFileNames(dir);
		for (const subdir of subdirs) {
			scopes.add(subdir);
		}
	}
	return [...scopes];
}

function readSubFileNames(p) {
	return fs
		.readdirSync(path.resolve(__dirname, p), { withFileTypes: true })
		.filter((dirent) => dirent.isDirectory())
		.map((dirent) => dirent.name);
}

module.exports = defineConfig({
	extends: ["@commitlint/config-conventional"],
	prompt: {
		scopes,
		customScopesAlign: !scopeComplete ? "top-bottom" : "bottom",
		defaultScope: scopeComplete,
		allowEmptyIssuePrefixs: false,
		allowCustomIssuePrefixs: false,
	},
});
