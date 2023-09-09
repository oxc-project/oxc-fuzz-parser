'use strict';

let { fuzz } = require('./fuzz');

let fs = require('fs');
let { exec } = require('child_process');
let binary = process.argv[2]

if (!binary) {
  console.log("Please provide a binary.")
  return
}

let file = '__tmp.js'
let fn = src => {
  fs.writeFileSync(file, src, 'utf8');
  return new Promise((res, rej) => {
    exec(binary + ' ' + file, {}, (err, _stdout, stderr) => {
      if (err == null) {
        res();
      } else {
        rej(new Error(stderr));
      }
    });
  });
};

let known = [
];

fuzz(fn, 200_000, known);
