var express = require("express");
var router = express.Router();
var bodyParser = require("body-parser");
router.use(bodyParser.urlencoded({ extended: true }));
router.use(bodyParser.json());
var model = require("../models/model");
router.get("/", function (req, res) {
  model.find({}, function (err, array) {
    if (err)
      return res.status(500).send("There was a problem finding the fumos.");

    res.status(200).send(array);
  });
});
router.get("/:id", (req, res) => {
    model.findById(req.params.id, (err, fumo) => {
        if (err) return res.status(500).send("There was a problem finding the fumo.");
        if (!fumo) return res.status(404).send("No fumo found.");
        res.status(200).send(fumo);
    })
})

module.exports = router;
