import Koa from "koa";
const app = new Koa();

app.use(async (ctx) => {
  ctx.body = "Hello, world!";
});

console.log("Server running at: http://127.0.0.1:8000");
console.log("Available endpoints:");
console.log("   GET / - show hello world");
app.listen(8000);
