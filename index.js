import init, { uwuify } from "./pkg/uwu_fier.js";

console.log("Starting!");

(async () => {
    await init()
})()

console.log(uwuify("Hello, World!"));