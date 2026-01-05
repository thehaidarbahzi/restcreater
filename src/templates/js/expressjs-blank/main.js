import express from "express";
const app = express();
const port = 8000;

app.get("/", (req, res) => {
  res.send("Hello, world!");
});

app.listen(port, () => {
  console.log("Server running at: http://127.0.0.1:8000");
  console.log("Available endpoints:");
  console.log("   GET / - show hello world");
  console.log(`Example app listening on port ${port}`);
});
