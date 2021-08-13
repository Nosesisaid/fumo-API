var express = require("express");
var app = express();
require("dotenv").config();

var db = require('./db'); 
var FumoController = require("./fumo/controller");
app.use("/fumo", FumoController);

module.exports = app;
