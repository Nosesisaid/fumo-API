var express = require("express");
const rateLimit = require("express-rate-limit");
var app = express();
require("dotenv").config();
const fumosController = require("./fumos/controller")
var db = require('./db'); 
var RandomController = require("./random/controller");
app.use("/random", RandomController);
app.use("/fumos", fumosController)
app.use("/fumo", RandomController)
const limiter = rateLimit({
  windowMs: 1 * 60 * 1000,
  max: 100,
});
app.use(limiter)
app.use("/", (req, res) => res.status(404).send("Here are no fumos, random fumo in /random"))
module.exports = app;
