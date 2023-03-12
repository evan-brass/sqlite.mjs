wasm-pack build --no-typescript --target web --out-name sql --out-dir dist

# Get rid of some wasm-pack stuff:
rm dist/package.json
rm dist/README.md
rm dist/.gitignore

# Remove the asyncify import from sql.js and rename the result to sql.mjs
awk '!/asyncify/' dist/sql.js > dist/sql.mjs
rm dist/sql.js
