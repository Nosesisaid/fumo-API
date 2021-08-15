var express = require("express");
var router = express.Router();
var bodyParser = require("body-parser");
router.use(bodyParser.urlencoded({ extended: true }));
router.use(bodyParser.json());
var model = require("../models/model");
router.get("/", function (req, res) {
  model.find({}, function (err, array) {
    if (err)
          return res.status(500).send("There was a problem finding the fumo.");
      
    res.status(200).send(array[Math.floor(Math.random() * array.length)]);
  });
});

module.exports = router;
