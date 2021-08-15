var express = require("express");
var app = express();
require("dotenv").config();
const fumosController = require("./fumos/controller")
var db = require('./db'); 
var RandomController = require("./random/controller");
app.use("/random", RandomController);
app.use("/fumos", fumosController)
app.use("/", (res, req) => res.status(404).send("Here are no fumos, random fumo in /random"))
module.exports = app;
