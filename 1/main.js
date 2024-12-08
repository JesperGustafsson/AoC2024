const fs = require("fs").promises;

const run = async () => {
  const data = await fs.readFile("data.txt", "utf-8");
  const l1 = [];
  const l2 = [];
  data.split("\n").forEach((line) => {
    const [e1, e2] = line.split(/\s+/);
    l1.push(e1);
    l2.push(e2);
  });
  console.log(l1);
  console.log(l2);

  const sortedl1 = l1.sort((a, b) => a - b);
  const sortedl2 = l2.sort((a, b) => a - b);

  let fullDiff = 0;

  sortedl1.forEach((e1, i) => {
    const e2 = sortedl2[i];
    const diff = Math.abs(e1 - e2);
    console.log("🚀 ~ file: main.js:22 ~ sortedl1.forEach ~ diff:", diff);
    fullDiff += diff;
  });

  console.log("🚀 ~ file: main.js:26 ~ run ~ fullDiff", fullDiff);
};

run();
