const { Schema, model } = require("mongoose");
const schema = new Schema({
  URL: { type: String },
});

module.exports = model("fumo", schema);
