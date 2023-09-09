let fs = require('fs');

let { fuzzModule } = require('shift-fuzzer');
let { parseModule } = require('shift-parser');
let codegen = require('shift-codegen').default;

let { shrink } = require('shift-shrink');

async function fuzz(parse, N, known = []) {
  for (let i = 0; i < N; ++i) {
    let tree = fuzzModule();
    let src = codegen(tree);

    try {
      parseModule(src);
    } catch (e) {
      // shift-fuzzer does not always generate valid code, alas
      // this can be removed once we fix those bugs
      --i;
      continue;
    }
    if (i % 100 == 0) {
      console.log(i);
    }

    try {
      await parse(src);
    } catch (e) {
      if (known.some(m => src.includes(m) || e.message.includes(m))) {
        continue;
      }
      console.log(e);
      console.log('reducing...', JSON.stringify(src));
      let shrunk = await minimize(src, parse, known);
      console.log(shrunk);
      break;
    }
  }
}

async function minimize(src, parse, known = []) {
  let isStillGood = async tree => {
    let src;
    try {
      src = codegen(tree);
    } catch (e) {
      console.error('codegen failed', e);
      fs.writeFileSync('_codegen-failed-tree.json', JSON.stringify(tree), 'utf8');
      throw e;
    }
    try {
      await parse(src);
      return false;
    } catch (e) {
      debugger;
      if (known.some(m => e.message.includes(m))) {
        return false;
      }
      return true;
    }
  };
  let tree = await shrink(parseModule(src), isStillGood, { log: console.log, onImproved: tree => fs.writeFileSync('_minimizer-best.json', JSON.stringify(tree, null, 2), 'utf8') });
  let res = codegen(tree);

  console.log(res);
  parse(res)
}

module.exports = { fuzz, minimize };
